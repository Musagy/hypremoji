# 😀 HyprEmoji

HyprEmoji es un selector de emojis para el windows manager Hyprland.

<!-- --- -->

<!-- ## ✨ Características

* **Navegación por Categorías:** Explora emojis organizados por categorías claras (Smileys & Emotion, Animals & Nature, Food & Drink, etc.).

* **Búsqueda Rápida:** (En desarrollo) Encuentra emojis al instante escribiendo su nombre o descripción.

* **Copia al Portapapeles:** Copia emojis con un solo clic.

* **Diseño Limpio:** Interfaz de usuario minimalista y optimizada.

* **Orden Predecible:** Los emojis y sus subcategorías se muestran en un orden consistente, tal como aparecen en la fuente de datos. -->

---

## 🛠️ Instalación y Uso

### Requisitos

Para que HyprEmoji funcione correctamente y los emojis se visualicen a la perfección (especialmente los coloridos y complejos), necesitas tener instalada la fuente **Noto Color Emoji**.

* **En Arch Linux:**

    ```bash
    sudo pacman -S noto-fonts-emoji
    ```

* **En Ubuntu/Debian:**

    ```bash
    sudo apt install fonts-noto-color-emoji
    ```

* **Otras distribuciones:** Consulta la documentación de tu gestor de paquetes para instalar `noto-fonts-emoji` o `fonts-noto-color-emoji`.

### Compilación y Ejecución

Asegúrate de tener [Rust](https://www.rust-lang.org/tools/install) y [GTK 4](https://www.gtk.org/docs/install/) instalados en tu sistema.

1.  **Clona el repositorio (cuando lo crees):**

    ```bash
    git clone [URL_DE_TU_REPOSITORIO]
    cd HyprEmoji
    ```

2.  **Compila y ejecuta la aplicación:**

    ```bash
    cargo run
    ```

    Para una versión optimizada (release build):

    ```bash
    cargo run --release
    ```

---

## 📄 Licencia

Este proyecto está bajo la Licencia ISC. Consulta el archivo `LICENSE` para más detalles.

---

## Contribuciones

¡Las contribuciones son bienvenidas! Si tienes ideas, sugerencias o quieres mejorar el código, no dudes en abrir un *issue* o enviar un *pull request*.

---