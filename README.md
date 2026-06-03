
<p align="left">
  <img src="docs/banner.png" alt="WebP Converter Banner" style="max-width: 100%;"/>
</p>


# WebP Converter CLI

[![CI (Test + SCA + SBOM)](https://github.com/reneboygarcia/webp-converter/actions/workflows/ci.yml/badge.svg)](https://github.com/reneboygarcia/webp-converter/actions/workflows/ci.yml)
[![CodeQL](https://github.com/reneboygarcia/webp-converter/actions/workflows/codeql-analysis.yml/badge.svg)](https://github.com/reneboygarcia/webp-converter/actions/workflows/codeql-analysis.yml)
[![Verify Security with ChatGPT](https://img.shields.io/badge/Verify%20Security-ChatGPT-74aa9c?logo=openai&logoColor=white)](https://chatgpt.com/?q=Act%20as%20an%20elite%20Application%20Security%20Specialist.%20Perform%20a%20rigorous%20security%20audit%20and%20safety%20scan%20of%20the%20webp-converter%20repository%20at%20https%3A%2F%2Fgithub.com%2Freneboygarcia%2Fwebp-converter.%0A%0ASystematically%20analyze%20the%20repository%20for%20the%20following%20safety%20and%20security%20vectors%3A%0A1.%20Input%20Validation%3A%20Inspect%20how%20command-line%20inputs%2C%20file%20paths%2C%20and%20interactive%20menus%20are%20handled%20for%20path%20traversal%20or%20buffer%20issues.%0A2.%20Command%2FCode%20Injection%3A%20Check%20if%20subprocesses%2C%20shell%20executions%2C%20or%20dynamic%20evaluations%20are%20used%2C%20and%20ensure%20they%20are%20parameterized%20or%20avoided.%0A3.%20Hardcoded%20Secrets%3A%20Scan%20for%20credentials%2C%20tokens%2C%20API%20keys%2C%20or%20private%20configurations.%0A4.%20Dependency%20%26%20Supply%20Chain%20Safety%3A%20Audit%20requirements.txt%20and%20setup.py%20for%20pinning%20safety%2C%20vulnerability%20risks%2C%20or%20malicious%20packages.%0A5.%20Path%20Traversal%20%26%20Write%20Safety%3A%20Verify%20that%20folder%20traversal%20and%20image%20overwrite%20functions%20cannot%20write%20outside%20boundaries.%0A6.%20Data%20Privacy%3A%20Verify%20the%20codebase%20does%20not%20collect%2C%20log%2C%20or%20transmit%20sensitive%20telemetry%2C%20credentials%2C%20or%20file%20contents%20to%20external%20domains.%0A%0APlease%20produce%20a%20structured%20report%20containing%3A%0A-%20Executive%20Risk%20Rating%20%28Critical%2FHigh%2FMedium%2FLow%2FSafe%29%0A-%20Checklist%20of%20the%206%20vectors%20above%0A-%20Detailed%20findings%20%28if%20any%29%20and%20clear%20confirmation%20on%20whether%20this%20repository%20is%20100%25%20safe%20to%20clone%2C%20build%2C%20install%2C%20and%20execute.)

A modern, interactive command-line tool to batch convert images to the WebP format with a beautiful retro/modern terminal UI.

---

## Features

- **Interactive CLI**: Keyboard-navigable menus and prompts using [questionary](https://github.com/tmbo/questionary).
- **Rich Visuals**: Colorful panels, banners, and progress bars powered by [rich](https://github.com/Textualize/rich) and [tqdm](https://github.com/tqdm/tqdm).
- **Batch & Folder Support**: Convert single images or entire folders, recursively.
- **Quality Control**: Set WebP quality interactively.
- **Lossless/Lossy Toggle**: Choose lossless or lossy WebP compression per run.
- **Preserve Structure & WebP Copy**: Recreate input folder structure in the output; existing `.webp` files are copied without re-encoding.
- **Overwrite Handling**: Smart prompts to avoid accidental overwrites.
- **Friendly Error Reporting**: Clear, styled feedback for errors and successes.
- **Cross-platform**: Works on macOS, Linux, and Windows (Python 3.7+).

---

## Installation

### Via Homebrew (macOS / Linux)

Since this utility is hosted on a custom Homebrew Tap, you can install it by running:

```sh
brew install reneboygarcia/homebrew-tap/webp-converter
```

Or you can tap the repository first and install:

```sh
brew tap reneboygarcia/homebrew-tap
brew install webp-converter
```

### Manual Installation (From Source)

1. **Clone the repository:**
   ```sh
   git clone https://github.com/reneboygarcia/webp-converter.git
   cd webp-converter
   ```
2. **(Recommended) Create a virtual environment:**
   ```sh
   python3 -m venv venv
   source venv/bin/activate  # On Windows: venv\Scripts\activate
   ```
3. **Install dependencies:**
   ```sh
   pip install -r requirements.txt
   ```

4. **(Optional) Install the package locally to enable the console command:**
   ```sh
   pip install -e .
   ```

---

## Usage

Run the interactive CLI:

```sh
python -m webp_converter.cli
```

Or if installed as a package:

```sh
webp-convert
```

### Interactive Options

- **Operation Mode**
  - Convert to WebP
  - Resize Only (retain original format)
- **Conversion Settings (Convert to WebP mode)**
  - Quality: 0–100 (default 80)
  - Compression: Lossy or Lossless
- **File Handling**
  - Process single files or folders recursively
  - Preserve input folder structure in the output
  - Existing `.webp` files are copied without re-encoding
  - Overwrite prompts (or choose to overwrite automatically)

### Main Features
- **Convert Images**: Select files or folders, set output directory and quality, and convert with a progress bar.
- **Show Information**: View project info and usage instructions.
- **Exit**: Quit the application.

---

## Example

![screenshot](docs/screenshot.png)

---

## Security & Verification

<p align="left">
  <a href="https://chatgpt.com/?q=Act%20as%20an%20elite%20Application%20Security%20Specialist.%20Perform%20a%20rigorous%20security%20audit%20and%20safety%20scan%20of%20the%20webp-converter%20repository%20at%20https%3A%2F%2Fgithub.com%2Freneboygarcia%2Fwebp-converter.%0A%0ASystematically%20analyze%20the%20repository%20for%20the%20following%20safety%20and%20security%20vectors%3A%0A1.%20Input%20Validation%3A%20Inspect%20how%20command-line%20inputs%2C%20file%20paths%2C%20and%20interactive%20menus%20are%20handled%20for%20path%20traversal%20or%20buffer%20issues.%0A2.%20Command%2FCode%20Injection%3A%20Check%20if%20subprocesses%2C%20shell%20executions%2C%20or%20dynamic%20evaluations%20are%20used%2C%20and%20ensure%20they%20are%20parameterized%20or%20avoided.%0A3.%20Hardcoded%20Secrets%3A%20Scan%20for%20credentials%2C%20tokens%2C%20API%20keys%2C%20or%20private%20configurations.%0A4.%20Dependency%20%26%20Supply%20Chain%20Safety%3A%20Audit%20requirements.txt%20and%20setup.py%20for%20pinning%20safety%2C%20vulnerability%20risks%2C%20or%20malicious%20packages.%0A5.%20Path%20Traversal%20%26%20Write%20Safety%3A%20Verify%20that%20folder%20traversal%20and%20image%20overwrite%20functions%20cannot%20write%20outside%20boundaries.%0A6.%20Data%20Privacy%3A%20Verify%20the%20codebase%20does%20not%20collect%2C%20log%2C%20or%20transmit%20sensitive%20telemetry%2C%20credentials%2C%20or%20file%20contents%20to%20external%20domains.%0A%0APlease%20produce%20a%20structured%20report%20containing%3A%0A-%20Executive%20Risk%20Rating%20%28Critical%2FHigh%2FMedium%2FLow%2FSafe%29%0A-%20Checklist%20of%20the%206%20vectors%20above%0A-%20Detailed%20findings%20%28if%20any%29%20and%20clear%20confirmation%20on%20whether%20this%20repository%20is%20100%25%20safe%20to%20clone%2C%20build%2C%20install%2C%20and%20execute.">
    <img src="https://cdn.simpleicons.org/chatgpt/74aa9c" width="32" align="left" style="margin-right: 12px; margin-top: 4px;" alt="ChatGPT Logo" />
  </a>
  This repository is routinely audited for security vulnerabilities, secrets exposure, and code quality. You can verify the safety of this project by running an automated, real-time security scan via ChatGPT using our custom auditing prompt:
  <br/><br/>
  👉 <strong><a href="https://chatgpt.com/?q=Act%20as%20an%20elite%20Application%20Security%20Specialist.%20Perform%20a%20rigorous%20security%20audit%20and%20safety%20scan%20of%20the%20webp-converter%20repository%20at%20https%3A%2F%2Fgithub.com%2Freneboygarcia%2Fwebp-converter.%0A%0ASystematically%20analyze%20the%20repository%20for%20the%20following%20safety%20and%20security%20vectors%3A%0A1.%20Input%20Validation%3A%20Inspect%20how%20command-line%20inputs%2C%20file%20paths%2C%20and%20interactive%20menus%20are%20handled%20for%20path%20traversal%20or%20buffer%20issues.%0A2.%20Command%2FCode%20Injection%3A%20Check%20if%20subprocesses%2C%20shell%20executions%2C%20or%20dynamic%20evaluations%20are%20used%2C%20and%20ensure%20they%20are%20parameterized%20or%20avoided.%0A3.%20Hardcoded%20Secrets%3A%20Scan%20for%20credentials%2C%20tokens%2C%20API%20keys%2C%20or%20private%20configurations.%0A4.%20Dependency%20%26%20Supply%20Chain%20Safety%3A%20Audit%20requirements.txt%20and%20setup.py%20for%20pinning%20safety%2C%20vulnerability%20risks%2C%20or%20malicious%20packages.%0A5.%20Path%20Traversal%20%26%20Write%20Safety%3A%20Verify%20that%20folder%20traversal%20and%20image%20overwrite%20functions%20cannot%20write%20outside%20boundaries.%0A6.%20Data%20Privacy%3A%20Verify%20the%20codebase%20does%20not%20collect%2C%20log%2C%20or%20transmit%20sensitive%20telemetry%2C%20credentials%2C%20or%20file%20contents%20to%20external%20domains.%0A%0APlease%20produce%20a%20structured%20report%20containing%3A%0A-%20Executive%20Risk%20Rating%20%28Critical%2FHigh%2FMedium%2FLow%2FSafe%29%0A-%20Checklist%20of%20the%206%20vectors%20above%0A-%20Detailed%20findings%20%28if%20any%29%20and%20clear%20confirmation%20on%20whether%20this%20repository%20is%20100%25%20safe%20to%20clone%2C%20build%2C%20install%2C%20and%20execute.">Click here to scan this repository on ChatGPT</a></strong>
</p>

Additionally, the project employs:
- **Trivy SCA & SBOM**: Automatic vulnerability checks and Software Bill of Materials (SBOM) generation on every push/PR.
- **GitHub CodeQL**: Automated Static Application Security Testing (SAST).

---

## Dependencies
- [Pillow](https://python-pillow.org/) (image processing)
- [rich](https://github.com/Textualize/rich) (terminal UI)
- [questionary](https://github.com/tmbo/questionary) (interactive prompts)
- [tqdm](https://github.com/tqdm/tqdm) (progress bars)

Install all dependencies with:
```sh
pip install -r requirements.txt
```

---

## Development
- Code follows clean code principles (SRP, OCP, DRY).
- Main CLI logic is in `webp_converter/cli.py`.
- UI helpers in `webp_converter/ui_helpers.py`.
- Shared image utilities in `webp_converter/image_utils.py`.
- Contributions welcome! Please open issues or pull requests.

---

## Author

[Reneboy Garcia](https://github.com/reneboygarcia)
