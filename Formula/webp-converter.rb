class WebpConverter < Formula
  desc "Fast CLI tool to convert images to WebP — Rust rewrite"
  homepage "https://github.com/reneboygarcia/webp-converter"
  url "https://github.com/reneboygarcia/webp-converter/archive/refs/tags/v0.2.11.tar.gz"
  sha256 "2b2a03fe4c9b898d5bdf263084cd70915d16c7faa6e7fd73863bef42f7daf719"
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
