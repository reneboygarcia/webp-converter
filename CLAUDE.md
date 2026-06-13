# CLAUDE.md — Developer Guide for webp-converter

This project is a Rust CLI tool. The Python source lives in `deprecated/` for reference only.

---

## Build and Run

```bash
# Dev build
cargo build

# Release build
cargo build --release

# Run interactively (dev)
cargo run

# Run installed binary
webp-convert

# Batch mode (non-interactive)
webp-convert --input /path/to/images --output ~/Downloads --quality 80
```

### Makefile shortcuts

```bash
make build        # cargo build
make release      # cargo build --release
make test         # cargo test
make lint         # cargo clippy --all-targets -- -D warnings
make format       # cargo fmt
make sca          # Trivy vulnerability scan
make sbom         # CycloneDX SBOM → sbom.cyclonedx.json
make benchmark    # build --release + python3 tests/benchmark.py
make pre-merge    # test + fmt check + lint + sca
SKIP_SCA=1 make pre-merge   # skip Trivy if DB unavailable
make clean        # cargo clean + rm sbom.cyclonedx.json
```

---

## Testing

```bash
cargo test
```

### Test Discipline
- All tests live in `tests/`, using `tempfile::TempDir` for isolation.
- Validate critical invariants: file existence, RGBA alpha values, metrics structs, error variants.
- **Never modify a test to make it pass** — fix the implementation instead.

---

## Release Checklist

> [!IMPORTANT]
> This Homebrew tap release workflow **MUST be executed after every code modification or feature update**. The tap formula should always be kept up-to-date with the main repository.

Follow this order exactly before every release:

### 1. Pre-flight
```bash
cargo fmt --all -- --check     # must be clean
cargo clippy --all-targets -- -D warnings   # zero warnings
cargo test                     # all tests pass
```

### 2. Bump version
Update `Cargo.toml`:
```toml
version = "0.X.Y"
```
Then rebuild so `Cargo.lock` is updated:
```bash
cargo build
```

### 3. Commit and push
```bash
git add -p          # stage intentionally
git commit -m "..."
git push origin main
```

### 4. Tag the release
```bash
git tag vX.Y.Z
git push origin vX.Y.Z
```

### 5. Get the SHA256 for the tap
```bash
curl -sL "https://github.com/reneboygarcia/webp-converter/archive/refs/tags/vX.Y.Z.tar.gz" | shasum -a 256
```

### 6. Update the Homebrew tap (separate repo)
The tap formula lives in the **`homebrew-tap`** repository.
Local path: `homebrew-tap/Formula/webp-converter.rb` (nested clone in workspace) or `/opt/homebrew/Library/Taps/reneboygarcia/homebrew-tap/Formula/webp-converter.rb`

Update both fields:
```ruby
url "https://github.com/reneboygarcia/webp-converter/archive/refs/tags/vX.Y.Z.tar.gz"
sha256 "<paste SHA256 from step 5>"
```

Then commit and push the tap:
```bash
cd /opt/homebrew/Library/Taps/reneboygarcia/homebrew-tap
git add Formula/webp-converter.rb
git commit -m "chore: bump webp-converter to vX.Y.Z"
git push origin main
```

### 7. Verify
```bash
brew update
brew info reneboygarcia/homebrew-tap/webp-converter   # should show new version
```

---

## Project Structure

```
src/
  lib.rs          — pub mod declarations and re-exports
  main.rs         — clap Args, banner, inquire TUI, RenderConfig
  converter.rs    — core conversion + batch logic
  transform.rs    — logo/batch resize transform
  ui.rs           — terminal panels (show_success, show_error, etc.)
  paths.rs        — get_downloads_dir() cross-platform
tests/
  converter_tests.rs   — 23 tests
  transform_tests.rs   — 11 tests
  paths_tests.rs       — 3 tests
  benchmark.py         — Python vs Rust time trial
docs/
  RUST_TRANSITION.md
  UX_GUIDELINES.md
deprecated/
  webp_converter/      — archived Python source
homebrew-tap/          — gitignored; tap lives at github.com/reneboygarcia/homebrew-tap
```

---

## Architecture

- **SRP**: CLI/TUI in `main.rs`, panels in `ui.rs`, image logic in `converter.rs`, resize in `transform.rs`.
- **Color constant**: `SKY = AnsiValue(81)` (`#5fd7ff`) is the single source of truth for the brand color, exported from `ui.rs`.
- **Transparency**: always use `img.into_rgba8()` before encoding; lossy WebP via `webp 0.3` crate, lossless via `image 0.25`.
- **No new Python dependencies**: the `deprecated/` directory is read-only history.

---

## Coding Style

- Format with `rustfmt` before every commit — CI enforces it.
- Zero clippy warnings — CI enforces `-D warnings`.
- No comments unless the WHY is non-obvious.
- Keep `ui.rs` panels consistent: sky blue (`SKY`) for success/info, red for errors, yellow for warnings.
