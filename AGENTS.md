# AGENTS.md - AI Agent Context & Workflows

Welcome, AI agent! This repository contains a Python command-line utility for converting images to WebP format.

---

## Directory Structure

*   [webp_converter/](file:///Users/reneboygarcia/Documents/Github%20Projects/webp_converter/webp_converter) (Source Package)
    *   [cli.py](file:///Users/reneboygarcia/Documents/Github%20Projects/webp_converter/webp_converter/cli.py): Main entrypoint containing the interactive console menus, arguments parsing, folder traversal, and orchestration loops.
    *   [ui_helpers.py](file:///Users/reneboygarcia/Documents/Github%20Projects/webp_converter/webp_converter/ui_helpers.py): Styled Rich panels, error/success banners, and confirmation prompts.
    *   [image_utils.py](file:///Users/reneboygarcia/Documents/Github%20Projects/webp_converter/webp_converter/image_utils.py): Lower-level PIL image manipulation logic.
    *   [image_transform.py](file:///Users/reneboygarcia/Documents/Github%20Projects/webp_converter/webp_converter/image_transform.py): Resize operations and supporting transforms.
*   [webp-converter.rb](file:///Users/reneboygarcia/Documents/Github%20Projects/webp_converter/webp-converter.rb): Template for the Homebrew Tap Formula. Used for reference when building updates.
*   [setup.py](file:///Users/reneboygarcia/Documents/Github%20Projects/webp_converter/setup.py): Distribution metadata and python dependencies.
*   [requirements.txt](file:///Users/reneboygarcia/Documents/Github%20Projects/webp_converter/requirements.txt): Pinned requirements.

---

## Architecture Decisions

- **No AI Background Removal**: The AI background removal utility (`rembg`, `onnxruntime`) was intentionally **removed** to keep the tool lightweight and simplify Homebrew installation (preventing bulky compilation issues for users). Do not re-add these dependencies.
- **Homebrew Tap Integration**: The application is distributed through a custom Homebrew tap (`reneboygarcia/homebrew-tap`). 

---

## Maintenance & Update Workflow

When performing modifications to the codebase:

1.  Make the required code edits.
2.  Install and verify the tool locally:
    ```bash
    pip install .
    webp-convert --help
    ```
3.  Commit and push to `main` in `webp_converter`.
4.  Tag the version (e.g. `v0.1.1`) and push:
    ```bash
    git tag vX.Y.Z
    git push origin vX.Y.Z
    ```
5.  Recalculate the archive SHA-256:
    ```bash
    curl -sSL https://github.com/reneboygarcia/webp-converter/archive/refs/tags/vX.Y.Z.tar.gz -o webp-converter.tar.gz
    shasum -a 256 webp-converter.tar.gz
    ```
6.  If package dependencies have changed, run `scratch/generate_resources.py` to regenerate the resource stanzas.
7.  Update the formula file inside the `homebrew-tap` repository under `Formula/webp-converter.rb` with the new URL, checksum, and resources.
8.  Push the updated formula to `reneboygarcia/homebrew-tap` on GitHub.
