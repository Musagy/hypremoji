EAPI=8

inherit cargo git-r3

DESCRIPTION="A modern emoji picker for Hyprland, written in Rust + GTK4"
HOMEPAGE="https://github.com/Musagy/hypremoji"
EGIT_REPO_URI="https://github.com/Musagy/hypremoji"

LICENSE="ISC"
SLOT="0"
PROPERTIES="live"

DEPEND="gui-libs/gtk:4"
RDEPEND="${DEPEND}
	media-fonts/noto-emoji
	gui-apps/wl-clipboard
	gui-wm/hyprland"

BDEPEND="dev-lang/rust virtual/pkgconfig"

src_unpack() {
	git-r3_src_unpack
	cargo_live_src_unpack
}

src_compile() {
	cargo_src_compile
}

src_install() {
	dobin target/*/hypremoji || die

	insinto /usr/share/hypremoji/assets
	doins -r assets/*

	insinto /usr/share/hypremoji
	doins config/hypremoji.conf
	doins config/config.json

	dodoc LICENSE
}

pkg_postinst() {
    if [[ ${D} == "/" ]]; then
        source "${FILESDIR}/hypremoji.install"
    fi
}
