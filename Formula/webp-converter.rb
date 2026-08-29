class WebpConverter < Formula
  desc "Fast CLI tool to convert images to WebP — Rust rewrite"
  homepage "https://github.com/reneboygarcia/webp-converter"
  url "https://github.com/reneboygarcia/webp-converter/archive/refs/tags/v0.2.8.tar.gz"
  sha256 "178e6acb921ba0d7e6a405fc7bfcd590a173d5cc38265443f7851c2a5bc60f76"
  license "MIT"
  head "https://github.com/reneboygarcia/webp-converter.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
    bin.install_symlink bin/"webp-convert" => "webp-conv"
  end

  def caveats
    <<~EOS
      Once installed, you can start webp-converter from your terminal:

      Interactive mode:
        webp-convert  (or 'webp-conv')

      Batch convert non-interactively:
        webp-conv --input /path/to/images --output ~/Downloads

      Install / configure CLI:
        webp-conv install

      Check for updates / upgrade:
        webp-conv update

      Uninstall:
        webp-conv delete

      For full usage options:
        webp-conv --help
    EOS
  end

  test do
    system bin/"webp-convert", "--help"
    system bin/"webp-conv", "--help"
  end
end
