# LinkedIn Post: WebP Converter CLI Launch (Final Draft)
**Date:** 2026-06-03
**Status:** Ready to copy-paste & publish on Thursday

***

Thursday is for Sidequests. 🛠️

Sometimes the best tools are the simple ones you build to scratch your own itch on the side. 

Back in July 2025, while rebuilding our company website, I needed a local way to batch-convert assets into WebP format without relying on web-based converters. 

What started as a quick Thursday sidequest became **webp-converter**—an interactive CLI tool that simplifies image optimization.

Initially, I built it as a TUI, but realized a clean, interactive CLI was much quicker: you just copy and paste the folder path containing the images, and it converts them. It's a simple tool for a specific usecase—not earth-shattering, but it makes my workflow much easier. Just enough for my taste.

I recently updated the implementation so it's installable via Homebrew. 

💻 **To install:**
```sh
brew install reneboygarcia/homebrew-tap/webp-converter
```

Once installed, just run `webp-convert` and follow the interactive prompts.

**Why it's handy:**
- **100% Local & Private:** No uploading assets to third-party sites.
- **Interactive Prompts:** Keyboard-navigable menus (using `questionary`) and rich terminal visuals.
- **Recursive Directories:** Converts everything while preserving folder structures.
- **Skips Existing WebP:** Saves time by skipping already-optimized images.

🔒 **Security:** 
In case you worry about security with new CLI tools, you can ask your favorite ChatGPT to security-scan the repo. The README has a one-click prompt link that scans the codebase for input validation and supply chain safety on the fly. 

Do you have any side projects you built out of necessity that ended up becoming part of your daily workflow? Let me know!

*(Link in comments)*

***

## Checklist before publishing:
1. **GitHub Repository link**: Copy the link to your repository and paste it as the very first comment once you publish the post.
2. **Post Day**: Recommend posting this on a Thursday morning to match the "Thursday is for Sidequests" hook.
3. **Image / Video**: You can attach a screenshot of the CLI tool (located at [screenshot.png](file:///Users/reneboygarcia/Documents/Github%20Projects/webp_converter/docs/screenshot.png)) or a quick terminal recording to make the post visually stand out on feeds.
