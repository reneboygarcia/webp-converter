# CLAUDE.md - Developer Guide for webp-converter

This guide details the commands, structure, and guidelines for developing and running this project.

## Build and Run Commands

### Development Makefile

The project includes a Makefile to orchestrate development tasks:

```bash
# Installation (creates venv and installs dependencies)
make install

# Run unit tests
make test

# Run Trivy vulnerability scan
make sca

# Generate CycloneDX SBOM (to sbom.cyclonedx.json)
make sbom

# Run pre-merge check (tests + sca)
make pre-merge
# Skip local SCA scan if Trivy/DB is failing (e.g. Docker credential issues)
SKIP_SCA=1 make pre-merge

# Clean build artifacts
make clean
```

### Manual Commands (Fallback)

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

## Testing

### Execution
*   **Run unit tests**:
    ```bash
    python -m unittest discover -s tests
    ```
    (Or using the virtual environment interpreter: `venv/bin/python -m unittest discover -s tests`)

### Test Discipline
- **Location**: All unit tests must live in the `tests/` directory, named with the `test_` prefix (e.g. `tests/test_cli.py`).
- **Isolation**: Use `tempfile` for creating and cleaning up test inputs and outputs to prevent side-effects on the workspace.
- **Assertions**: Validate critical invariants (like color mode `RGBA`, pixel transparency value, or structured metrics dictionaries).
- **Test Integrity**: If a test fails and the expectation is correct, do not modify the test code. Iterate on the package implementation until the test passes.

---

## Code Guidelines & Repository Standards

### Architecture
- **Single Responsibility Principle (SRP)**: Keep console/CLI operations in `cli.py`, terminal formatting in `ui_helpers.py`, image loading/saving/transparency in `image_utils.py`, and resizing in `image_transform.py`.
- **Imports**: Avoid importing modules that are not listed in [setup.py](file:///Users/reneboygarcia/Documents/Github%20Projects/webp_converter/setup.py). (Note: AI background removal using `rembg` has been removed to keep the package lightweight and easily installable via Homebrew).

### Coding Style
- Prefer clean, self-documenting code with meaningful names.
- Always preserve transparency (e.g. alpha channels) when opening and saving image files. Use [save_image_with_transparency](file:///Users/reneboygarcia/Documents/Github%20Projects/webp_converter/webp_converter/image_utils.py) for all saves.
- Keep the user interface interactive using `questionary` and stylized with `rich` panels.
