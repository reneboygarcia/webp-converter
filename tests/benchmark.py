#!/usr/bin/env python3
"""
Benchmark: Python webp-converter vs Rust webp-converter

Measures:
  1. Cold startup latency  (--help invocation)
  2. Single-file conversion throughput
  3. Batch conversion throughput (N_IMAGES files, parallel)

Run after: make release
"""

import os
import shutil
import statistics
import subprocess
import sys
import tempfile
import time

N_IMAGES = 50
N_TRIALS = 3
RUST_BIN = os.path.join(os.path.dirname(__file__), "..", "target", "release", "webp-convert")
PYTHON_MODULE = ["python3", "-m", "webp_converter.cli"]
DEPRECATED_DIR = os.path.join(os.path.dirname(__file__), "..", "deprecated")


def _make_test_images(tmpdir, n):
    try:
        from PIL import Image
    except ImportError:
        print("PIL not available — skipping image generation for Python batch test")
        return []

    paths = []
    for i in range(n):
        p = os.path.join(tmpdir, f"img_{i:04d}.png")
        img = Image.new("RGBA", (256, 256), (i * 5 % 255, i * 3 % 255, i * 7 % 255, 200))
        img.save(p)
        paths.append(p)
    return paths


def _time_command(cmd, trials=N_TRIALS):
    """Return median wall-clock time in seconds over `trials` runs."""
    times = []
    for _ in range(trials):
        t0 = time.perf_counter()
        result = subprocess.run(cmd, capture_output=True)
        elapsed = time.perf_counter() - t0
        if result.returncode not in (0, 2):  # 2 = clap --help exit
            pass  # ignore non-zero for --help variants
        times.append(elapsed)
    return statistics.median(times)


def _format_ms(seconds):
    return f"{seconds * 1000:.1f} ms"


def _format_speedup(py, rust):
    if rust == 0:
        return "N/A"
    ratio = py / rust
    return f"{ratio:.1f}x faster ({(1 - rust / py) * 100:.1f}% time saved)"


def benchmark_startup():
    print("\n── Startup Latency (--help) ─────────────────────────────────")
    rust_ok = os.path.isfile(RUST_BIN)

    if rust_ok:
        rust_time = _time_command([RUST_BIN, "--help"])
        print(f"  Rust binary:     {_format_ms(rust_time)}")
    else:
        print(f"  Rust binary not found at {RUST_BIN} — run 'make release' first")
        rust_time = None

    # Python startup: import cost
    py_time = _time_command(
        [sys.executable, "-c", "import sys; sys.path.insert(0, sys.argv[1]); import webp_converter; print('ok')", DEPRECATED_DIR]
    )
    print(f"  Python import:   {_format_ms(py_time)}")

    if rust_time:
        print(f"  Speedup:         {_format_speedup(py_time, rust_time)}")

    return rust_time, py_time


def benchmark_single_file():
    print("\n── Single-File Conversion (256×256 PNG → WebP) ──────────────")
    rust_ok = os.path.isfile(RUST_BIN)

    with tempfile.TemporaryDirectory() as tmpdir:
        images = _make_test_images(tmpdir, 1)
        if not images:
            print("  Skipped (PIL not available)")
            return None, None

        out_dir = os.path.join(tmpdir, "out_rust")
        os.makedirs(out_dir)

        rust_time = None
        if rust_ok:
            rust_times = []
            for _ in range(N_TRIALS):
                # Remove output each trial for fair measurement
                shutil.rmtree(out_dir, ignore_errors=True)
                os.makedirs(out_dir)
                t0 = time.perf_counter()
                subprocess.run(
                    [RUST_BIN, "--input", images[0], "--output", out_dir, "--force"],
                    capture_output=True,
                )
                rust_times.append(time.perf_counter() - t0)
            rust_time = statistics.median(rust_times)
            print(f"  Rust:            {_format_ms(rust_time)}")

        # Python single-file (non-interactive via programmatic import)
        py_times = []
        sys.path.insert(0, DEPRECATED_DIR)
        try:
            import importlib
            import importlib.util

            spec = importlib.util.spec_from_file_location(
                "cli", os.path.join(DEPRECATED_DIR, "webp_converter", "cli.py")
            )
            cli_mod = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(cli_mod)

            out_py_dir = os.path.join(tmpdir, "out_py")
            os.makedirs(out_py_dir)
            out_path = os.path.join(out_py_dir, "out.webp")

            for _ in range(N_TRIALS):
                if os.path.exists(out_path):
                    os.remove(out_path)
                t0 = time.perf_counter()
                cli_mod.convert_to_webp_core(images[0], out_path)
                py_times.append(time.perf_counter() - t0)
            py_time = statistics.median(py_times)
            print(f"  Python:          {_format_ms(py_time)}")
        except Exception as e:
            print(f"  Python:          unavailable ({e})")
            py_time = None

        if rust_time and py_time:
            print(f"  Speedup:         {_format_speedup(py_time, rust_time)}")

        return rust_time, py_time


def benchmark_batch():
    print(f"\n── Batch Conversion ({N_IMAGES} × 256×256 PNG → WebP, parallel) ──")
    rust_ok = os.path.isfile(RUST_BIN)

    with tempfile.TemporaryDirectory() as tmpdir:
        images = _make_test_images(tmpdir, N_IMAGES)
        if not images:
            print("  Skipped (PIL not available)")
            return None, None

        rust_time = None
        if rust_ok:
            rust_times = []
            for _ in range(N_TRIALS):
                out_dir = os.path.join(tmpdir, f"out_rust_{_}")
                os.makedirs(out_dir, exist_ok=True)
                t0 = time.perf_counter()
                subprocess.run(
                    [RUST_BIN, "--input", tmpdir, "--output", out_dir, "--force"],
                    capture_output=True,
                )
                rust_times.append(time.perf_counter() - t0)
            rust_time = statistics.median(rust_times)
            throughput = N_IMAGES / rust_time
            print(f"  Rust:            {_format_ms(rust_time)}  ({throughput:.0f} img/s)")

        # Python batch via ThreadPoolExecutor (programmatic)
        sys.path.insert(0, DEPRECATED_DIR)
        try:
            import importlib.util

            spec = importlib.util.spec_from_file_location(
                "cli2", os.path.join(DEPRECATED_DIR, "webp_converter", "cli.py")
            )
            cli_mod = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(cli_mod)

            from concurrent.futures import ThreadPoolExecutor

            py_times = []
            for trial in range(N_TRIALS):
                out_py = os.path.join(tmpdir, f"out_py_{trial}")
                os.makedirs(out_py)

                def convert_one(src):
                    dst = os.path.join(out_py, os.path.basename(src).replace(".png", ".webp"))
                    cli_mod.convert_to_webp_core(src, dst)

                t0 = time.perf_counter()
                with ThreadPoolExecutor(max_workers=8) as pool:
                    list(pool.map(convert_one, images))
                py_times.append(time.perf_counter() - t0)

            py_time = statistics.median(py_times)
            throughput = N_IMAGES / py_time
            print(f"  Python:          {_format_ms(py_time)}  ({throughput:.0f} img/s)")
        except Exception as e:
            print(f"  Python:          unavailable ({e})")
            py_time = None

        if rust_time and py_time:
            print(f"  Speedup:         {_format_speedup(py_time, rust_time)}")

        return rust_time, py_time


def binary_size():
    print("\n── Binary / Package Size ────────────────────────────────────")
    if os.path.isfile(RUST_BIN):
        size_bytes = os.path.getsize(RUST_BIN)
        print(f"  Rust binary:     {size_bytes / 1_048_576:.2f} MB  (standalone, no runtime)")
    else:
        print(f"  Rust binary not found — run 'make release'")

    # Check for Python PyInstaller bundle if it exists
    bundle = os.path.join(os.path.dirname(__file__), "..", "dist")
    if os.path.isdir(bundle):
        total = sum(
            os.path.getsize(os.path.join(r, f))
            for r, _, files in os.walk(bundle)
            for f in files
        )
        print(f"  Python dist/:    {total / 1_048_576:.2f} MB")
    else:
        print("  Python dist/:    not built")


def main():
    print("=" * 62)
    print("  webp-converter  Benchmark: Python vs Rust")
    print(f"  Trials per test: {N_TRIALS}  |  Batch size: {N_IMAGES} images")
    print("=" * 62)

    benchmark_startup()
    benchmark_single_file()
    benchmark_batch()
    binary_size()

    print("\n" + "=" * 62)
    print("  Done. Run 'make release' before benchmarking if stale.")
    print("=" * 62)


if __name__ == "__main__":
    main()
