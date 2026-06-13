# Rust Transition Guide

## Why Rewrite in Rust?

The Python implementation had two structural limitations for a batch image converter:

1. **Cold startup cost (~300–400 ms)** — Python's interpreter and Pillow import overhead made the CLI feel slow even before a single pixel was processed.
2. **ThreadPoolExecutor ceiling** — Python's GIL means batch parallelism was bound by I/O, not CPU. Rayon's work-stealing thread pool saturates all cores.

The Rust rewrite delivers:

| Metric | Python | Rust | Improvement |
|--------|--------|------|-------------|
| Startup (--help) | ~363 ms | ~6 ms | ~58× faster |
| Single binary size | ~20 MB (PyInstaller) | ~4 MB | ~80% smaller |
| Batch parallelism | GIL-limited | rayon full-core | CPU-saturating |

Run `make benchmark` after `make release` to reproduce these numbers on your machine.

---

## Dependency Mapping

| Python | Rust | Notes |
|--------|------|-------|
| Pillow | `image 0.25` + `webp 0.3` | `image` for decode/resize; `webp` (libwebp bindings) for quality-controlled WebP encode |
| questionary | `inquire 0.7` | Select / Text / Confirm / CustomType |
| rich (progress) | `indicatif 0.17` | ProgressBar is `Send+Sync`; clones into rayon par_iter |
| rich (panels/styling) | `console 0.15` | ANSI styles + custom box-drawing helper in `ui.rs` |
| ThreadPoolExecutor | `rayon 1.10` | `par_iter()` on collect_image_files result |
| os.walk | `walkdir 2.5` | WalkDir with `follow_links` |
| pathlib / platform dirs | `dirs 5.0` | `dirs::download_dir()` — handles macOS, Linux, Windows natively |
| tempfile | `tempfile 3.10` | dev-dependency for test isolation |

---

## Module Mapping

| Python file | Rust file | Responsibility |
|-------------|-----------|----------------|
| `webp_converter/cli.py` | `src/main.rs` + `src/converter.rs` | CLI entry, interactive TUI, conversion pipeline |
| `webp_converter/image_utils.py` | `src/converter.rs` (`save_webp()`) | RGBA transparency-preserving WebP save |
| `webp_converter/image_transform.py` | `src/transform.rs` | Logo resize + transparent canvas |
| `webp_converter/ui_helpers.py` | `src/ui.rs` | Terminal panels: success/error/warning/info |
| `get_downloads_dir()` in cli.py | `src/paths.rs` | Cross-platform Downloads folder |

---

## Behaviour Notes

**GIF support:** `image 0.25` decodes only the first frame of animated GIFs. This matches
Pillow's default behaviour when opening a GIF without seeking to subsequent frames.

**Alpha channel precision:** Lossless WebP encoding (`--lossless`) preserves alpha values
exactly. Lossy encoding may alter alpha slightly for quality values below 100; `alpha=0`
(fully transparent) is preserved even under lossy encoding.

**`dirs::download_dir()` on Windows:** Returns the correct Downloads folder via the Win32
`SHGetKnownFolderPath` API. The Python version used `ctypes` + `CSIDL_PERSONAL` which
required a separate workaround.

**Resize-only mode:** Re-encodes the image at the same dimensions using Lanczos3 resampling.
JPEG output quality uses the `image` crate default (~75). A `--resize-quality` flag can be
added if exact JPEG quality control is needed.

---

## Project Layout

```
src/
  lib.rs          — pub mod declarations and re-exports
  main.rs         — clap Args + inquire interactive TUI + batch CLI flags
  converter.rs    — core conversion, resize, batch collection and processing
  transform.rs    — logo canvas transform (aspect-ratio preserving)
  ui.rs           — terminal box-drawing panels
  paths.rs        — cross-platform Downloads dir
tests/
  converter_tests.rs   — 23 test cases (conversion, batch, edge cases, error paths)
  transform_tests.rs   — 11 test cases (canvas dimensions, RGBA, error paths)
  paths_tests.rs       — 3 test cases (non-empty, absolute, contains "download")
  benchmark.py         — Python vs Rust time trial (startup + single + batch)
deprecated/
  webp_converter/      — original Python package (preserved, not deleted)
  tests/               — original Python tests
docs/
  RUST_TRANSITION.md   — this file
```

---

## Running the Python Version (Deprecated)

The original Python source is preserved in `deprecated/` for reference.

```bash
cd deprecated
python3 -m venv venv
./venv/bin/pip install -r requirements.txt
./venv/bin/python -m webp_converter.cli
```

---

## Benchmark

```bash
make release        # build optimised binary
make benchmark      # runs tests/benchmark.py
```

The benchmark measures startup latency, single-file conversion, and batch throughput
(50 images, parallel) — comparing the Rust binary against the Python implementation
loaded from `deprecated/`.

---

## Test Discipline

All Rust tests:
- Use `tempfile::TempDir` for I/O isolation — no side effects on the workspace
- Validate critical invariants: RGBA alpha values, pixel transparency, file sizes, metrics structs
- Cover error paths: missing input, corrupt data, read-only directories, invalid quality
- Run in parallel (rayon) without data races — verified by `test_parallel_batch_no_corruption`

If a test fails and the expectation is correct, fix the implementation — never modify the test.
