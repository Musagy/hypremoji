"""Exercise the production restore function without GTK or a Wayland compositor.

Requires python3 and rustc. The fake wl-copy forks a long-lived clipboard owner
that inherits stdout/stderr, reproducing the pipe lifetime behind issue #15.
"""
import os
from pathlib import Path
import signal
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]


class ClipboardExitTest(unittest.TestCase):
    def test_restore_returns_while_clipboard_owner_is_alive(self):
        source = (ROOT / "src/utils/clipboard_manager.rs").read_text()
        restore = source[source.index("fn set_element_to_clipboard("):]
        with tempfile.TemporaryDirectory() as directory:
            work = Path(directory)
            fake = work / "wl-copy"
            fake.write_text("""#!/usr/bin/env python3
import os, pathlib, sys, time
pathlib.Path(os.environ['CAPTURE']).write_bytes(sys.stdin.buffer.read())
pathlib.Path(os.environ['ARGS']).write_text('\u005cn'.join(sys.argv[1:]))
if os.fork() == 0:
    time.sleep(10)
os._exit(0)
""")
            fake.chmod(0o755)
            harness = work / "restore.rs"
            harness.write_text("""
use std::{fs::File, process::{Command, Stdio}};
struct OriginalClipboardContent { content: Option<String>, mime_type: String }
""" + restore + """
fn main() {
    let args: Vec<String> = std::env::args().collect();
    set_element_to_clipboard(OriginalClipboardContent {
        content: Some(args[1].clone()), mime_type: args[2].clone(),
    });
}
""")
            binary = work / "restore"
            subprocess.run(["rustc", str(harness), "-o", str(binary)], check=True)
            for mime, content, expected_args in [
                ("text/plain", "emoji 😀\n".encode(), "--type\ntext/plain"),
                ("image/png", b"\x89PNG\r\n\x1a\n\x00\xff", ""),
            ]:
                with self.subTest(mime=mime):
                    original = work / "clipboard"
                    original.write_bytes(content)
                    capture, args = work / "capture", work / "args"
                    env = dict(os.environ, PATH=f"{work}:{os.environ['PATH']}",
                               CAPTURE=str(capture), ARGS=str(args))
                    process = subprocess.Popen([str(binary), str(original), mime],
                                               env=env, start_new_session=True,
                                               stdout=subprocess.PIPE, stderr=subprocess.PIPE)
                    try:
                        process.communicate(timeout=2)
                        self.assertEqual(process.returncode, 0)
                        self.assertEqual(capture.read_bytes(), content)
                        self.assertEqual(args.read_text(), expected_args)
                        self.assertFalse(original.exists())
                        # The simulated clipboard owner still exists after restore exits.
                        os.killpg(process.pid, 0)
                    finally:
                        try:
                            os.killpg(process.pid, signal.SIGKILL)
                        except ProcessLookupError:
                            pass
                        process.communicate()


if __name__ == "__main__":
    unittest.main()
