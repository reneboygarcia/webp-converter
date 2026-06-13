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
*   [docs/solutions/](file:///Users/reneboygarcia/Documents/Github%20Projects/webp_converter/docs/solutions) (Documented Solutions): Searchable knowledge store of past problems (bugs, best practices, workflow patterns), organized by category with YAML frontmatter (`module`, `tags`, `problem_type`). Relevant when implementing or debugging in documented areas.

---

## Architecture Decisions

- **No AI Background Removal**: The AI background removal utility (`rembg`, `onnxruntime`) was intentionally **removed** to keep the tool lightweight and simplify Homebrew installation (preventing bulky compilation issues for users). Do not re-add these dependencies.
- **Homebrew Tap Integration**: The application is distributed through a custom Homebrew tap (`reneboygarcia/homebrew-tap`). 

---

## Maintenance & Update Workflow

> [!IMPORTANT]
> This Homebrew tap release workflow **MUST be executed after every code modification or feature update**. The tap formula should always be kept up-to-date with the main repository.

When performing modifications to the codebase:

1.  Make the required code edits.
2.  Build and verify the tool locally:
    ```bash
    cargo build --release
    ./target/release/webp-convert --help
    cargo test
    ```
3.  Bump version in `Cargo.toml` and compile to update `Cargo.lock`.
4.  Commit and push to `main` on GitHub:
    ```bash
    git add .
    git commit -m "..."
    git push origin main
    ```
5.  Tag the version (e.g. `v0.2.7`) and push:
    ```bash
    git tag vX.Y.Z
    git push origin vX.Y.Z
    ```
6.  Recalculate the archive SHA-256:
    ```bash
    curl -sSL "https://github.com/reneboygarcia/webp-converter/archive/refs/tags/vX.Y.Z.tar.gz" | shasum -a 256
    ```
7.  Update the formula file `Formula/webp-converter.rb` in the `homebrew-tap` repository (cloned under `homebrew-tap/`) with the new URL and checksum.
8.  Stage, commit, and push the updated formula to `reneboygarcia/homebrew-tap` on GitHub:
    ```bash
    git -C homebrew-tap add Formula/webp-converter.rb
    git -C homebrew-tap commit -m "bump webp-converter to vX.Y.Z"
    git -C homebrew-tap push origin main
    ```
9.  Verify the local Homebrew formula is updated:
    ```bash
    brew update
    brew upgrade reneboygarcia/homebrew-tap/webp-converter
    ```
