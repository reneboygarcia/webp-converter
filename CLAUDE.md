# CLAUDE.md - Developer Guide for webp-converter

This guide details the commands, structure, and guidelines for developing and running this project.

## Build and Run Commands

### Installation
*   **Virtual Environment Setup**:
    ```bash
    python3 -m venv venv
    source venv/bin/activate
    ```
*   **Install Dependencies**:
    ```bash
    pip install -r requirements.txt
    ```
*   **Install Package Locally (Editable Mode)**:
    ```bash
    pip install -e .
    ```

### Execution
*   **Run CLI interactively**:
    ```bash
    python -m webp_converter.cli
    # Or, if installed locally in editable mode:
    webp-convert
    ```
*   **Convert single file directly**:
    ```bash
    python webp_converter.py input.jpg output.webp
    ```

### Cleanup
*   **Clean build artifacts**:
    ```bash
    rm -rf build/ dist/ *.egg-info webp_converter.egg-info
    ```

---

## Code Guidelines & Repository Standards

### Architecture
- **Single Responsibility Principle (SRP)**: Keep console/CLI operations in `cli.py`, terminal formatting in `ui_helpers.py`, image loading/saving/transparency in `image_utils.py`, and resizing in `image_transform.py`.
- **Imports**: Avoid importing modules that are not listed in [setup.py](file:///Users/reneboygarcia/Documents/Github%20Projects/webp_converter/setup.py). (Note: AI background removal using `rembg` has been removed to keep the package lightweight and easily installable via Homebrew).

### Coding Style
- Prefer clean, self-documenting code with meaningful names.
- Always preserve transparency (e.g. alpha channels) when opening and saving image files. Use [save_image_with_transparency](file:///Users/reneboygarcia/Documents/Github%20Projects/webp_converter/webp_converter/image_utils.py) for all saves.
- Keep the user interface interactive using `questionary` and stylized with `rich` panels.
