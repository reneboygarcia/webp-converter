---
title: "Homebrew Installation Build Bottleneck and Dependency Optimization"
date: 2026-06-03
category: build-errors
module: "Homebrew Installation & Package Distribution"
problem_type: build_error
component: tooling
symptoms:
  - "Installation via Homebrew (`brew install webp-converter`) takes over 10 minutes to complete from source."
  - "Build system bootstraps heavy C++ build tools (`cmake`, `ninja`) from source during dependency installation."
  - "Virtual environment sizes are bloated to over 500MB due to legacy unused packages."
  - "Automated Homebrew tests (`brew test webp-converter`) hang and timeout during `webp-convert --help` execution."
root_cause: config_error
resolution_type: dependency_update
severity: high
tags:
  - "homebrew"
  - "pillow"
  - "tqdm"
  - "compilation"
  - "optimization"
  - "virtualenv"
---

# Homebrew Installation Build Bottleneck and Dependency Optimization

## Problem
When installing the `webp-converter` tool via Homebrew from source, the compilation of the `pillow` dependency took over 10 minutes and hung. This made installation of a lightweight command-line utility extremely slow and compromised user experience. Additionally, the local virtual environment was unnecessarily bloated due to legacy packages. Furthermore, running `brew test webp-converter` hung and timed out because the CLI immediately launched into an interactive TUI menu without argument checking.

## Symptoms
- Installing the tap formula using `brew install webp-converter` takes upwards of 10 minutes, eventually getting stuck.
- Running installation processes shows pip downloading and compiling large build-system dependencies like `scikit-build-core` and `pybind11` from source, followed by bootstrapping `cmake` and `ninja` from source.
- Local virtual environments (`venv`) take up 538MB of disk space for a tool that only needs about 47MB.
- `brew test webp-converter` times out with `Timeout::Error: execution expired` at `/opt/homebrew/Cellar/webp-converter/[version]/bin/webp-convert --help` due to it prompting for user input non-interactively.

## What Didn't Work
- Leaving dependencies unconstrained or pinned to the latest versions (`Pillow==12.2.0`). Modern Pillow versions (12.0+) migrated to a build backend utilizing `scikit-build-core` and `pybind11`. Since Homebrew installs python dependencies using the `--no-binary=:all:` flag (which compiles everything from source), this forced pip to compile and bootstrap massive C++ build tools (`cmake`, `ninja`) from scratch, leading to the 10+ minute hang.
- Installing Pillow inside the Homebrew build sandbox without providing required system header packages resulted in compilation failures since Pillow’s C extensions require `jpeg`, `png`, and `webp` headers.
- Calling `system libexec/"bin/pip"` in the formula to compile Pillow manually. Because Homebrew's `virtualenv_create` creates virtual environments using the `--without-pip` flag, there is no `pip` binary inside `libexec/bin`, resulting in command execution failures.
- Running interactive TUI menus without checking if `--help` or `-h` was requested. Automated environments like `brew test` run tests non-interactively (with `fd=0` not being a terminal), causing the menu prompt to block indefinitely.

## Solution
1. **Downgraded Pillow Dependency to `10.4.0`**: Pinned Pillow to `10.4.0` in both tap formulas. Pillow `10.4.0` uses the traditional `setuptools` build backend, compiling directly using the standard system C compiler (`clang`) without bootstrapping `cmake` and `ninja`.
2. **Exposed System Libraries in Formula**: Added system dependencies `depends_on "jpeg-turbo"`, `depends_on "libpng"`, and `depends_on "webp"` inside the Homebrew formulas so their headers are exposed to Pillow within the sandbox during compilation.
3. **Enabled Parallel Compilation**: Added `ENV["MAX_CONCURRENCY"] = ENV.make_jobs.to_s` inside the formula's `install` method. This enables compiling Pillow's C code in parallel using all available CPU cores, reducing its build time to under 15 seconds.
4. **Isolated Pillow Compilation & Disabled Unused Codecs**: Used the host Python pip runner `python3.12 -m pip --python=#{libexec}/bin/python` to install Pillow inside the virtualenv and passed `*std_pip_args(prefix: false, build_isolation: true)` along with custom `--config-settings` to compile only required codecs (disabling `tiff`, `freetype`, `raqm`, `lcms`, `jpeg2000`, `imagequant`, `xcb`, and `avif`). This secures build predictability inside the sandbox.
5. **Trimmed `tqdm` and Switched to `rich.progress`**: Removed `tqdm` from `setup.py`, `requirements.txt`, and Homebrew formulas, replacing it with the built-in `rich.progress` module in `cli.py` to match the existing Retro/Synthwave terminal styling and reduce resource footprint.
6. **Virtual Environment Clean Build**: Recreated the project's virtual environment, pruning residual dependencies from legacy background-removal libraries. This reduced the environment size from **538MB** to **47MB** (~91% size reduction).
7. **Added Argument Interception for Help Command**: Intercepted `--help` and `-h` command-line arguments in `main()` of `webp_converter/cli.py` to print a usage message and exit with code 0 immediately before booting the interactive console, fixing the Homebrew test runner hang.

## Why This Works
- Using a `setuptools`-based Pillow release (`10.4.0`) avoids modern python build backends that trigger bootstrapping massive C++ compilers inside sandboxed python environments.
- Homebrew's `ENV.make_jobs` exposes the number of parallel compile jobs available on the host machine. Assigning this value to Pillow's `MAX_CONCURRENCY` environment variable leverages multi-threading, cutting compilation times significantly.
- Declaring `jpeg-turbo`, `libpng`, and `webp` ensures these libraries are built/linked inside Homebrew before compiling Python wheels, resolving header file paths automatically.
- Passing `prefix: false` to `std_pip_args` ensures pip installs Pillow into the virtual environment's default package directory without forcing the global Cellar prefix.
- Intercepting `--help` stops the program from spawning interactive prompt selectors under non-interactive automated test environments.

## Prevention
- Avoid upgrading Pillow to `12.0.0` or higher in Homebrew tap formulas unless you can guarantee pre-compiled bottles are available or the user has compiler build tools like `cmake` pre-installed globally. Pinned versions below `11.0.0` should be favored.
- Always include `ENV["MAX_CONCURRENCY"] = ENV.make_jobs.to_s` when installing Python packages with C extensions via Homebrew formula resources to optimize compilation.
- Ensure all necessary underlying system dev libraries (headers) are explicitly listed as dependencies (`depends_on`) in Homebrew tap formulas.
- When calling `pip` manually in virtualenv-based Homebrew formulas, execute it via the host Python (`python3.y -m pip --python=#{libexec}/bin/python`) rather than `libexec/"bin/pip"` since virtualenvs are initialized without a standalone `pip` executable.
- Intercept `--help` and `-h` flags at the entrypoint of any TUI/interactive CLI script to ensure automated runner tests exit cleanly with documentation output rather than hanging on prompt loops.
- Minimize project dependencies by leveraging built-in features of packages already required (e.g. using `rich.progress` when `rich` is already required, instead of adding `tqdm`).
