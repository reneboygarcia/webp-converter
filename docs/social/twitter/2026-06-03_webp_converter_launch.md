# Twitter Posts: WebP Converter CLI Launch
**Date:** 2026-06-03
**Status:** Ready to publish

Below are two formats for Twitter: a Twitter Thread (recommended for engagement) and a Single Punchy Tweet.

---

## Format 1: Twitter Thread (Recommended)

### Tweet 1/5 (Hook)
Thursday is for sidequests. 🛠️

Last year, while rebuilding our company site, I needed a local way to batch-convert assets to WebP format.

I didn't want to upload private images to sketchy online converters.

So I built **webp-converter**—an interactive CLI. 👇

---

### Tweet 2/5 (Backstory & Dev Journey)
I originally started building a heavy TUI. 

But I quickly realized a streamlined interactive CLI was way faster for my workflow.

Just paste a folder or image path, configure settings interactively, and go.

No earth-shattering tech, just a workflow multiplier. 🚀

---

### Tweet 3/5 (Key Features)
How it works under the hood:
- 100% local, offline, and private.
- Interactive keyboard menus (powered by questionary + rich).
- Recursive directory support + preserves folder structure.
- Smart overwrite protection.
- Automatically copies existing WebP files without re-encoding.

---

### Tweet 4/5 (Installation)
I just updated the implementation so anyone can install it via Homebrew. 

💻 To install:
```sh
brew install reneboygarcia/homebrew-tap/webp-converter
```

Run `webp-convert` to start optimization.

---

### Tweet 5/5 (Security & Outro)
If you're cautious about new CLI tools, you can ask ChatGPT to scan the repo. 

The README has a one-click ChatGPT security scan link, alongside automated CodeQL + Trivy SCA/SBOM checks.

Try it out and let me know if it helps your dev workflow!

[Link to Repo] (or reply with repo link)

---

## Format 2: Single Punchy Tweet

Thursday is for sidequests. 🛠️ 

I needed a private, local way to batch-convert site assets to WebP format, so I built **webp-converter**.

- 100% local (no sketchy web uploads)
- Interactive CLI prompt style
- Preserves folder structure recursively
- Skipping of already-converted WebP files

Homebrew install:
`brew install reneboygarcia/homebrew-tap/webp-converter`

GitHub Repo: [Link to Repo]
