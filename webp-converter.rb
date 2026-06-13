class WebpConverter < Formula
  desc "Fast CLI tool to convert images to WebP — Rust rewrite"
  homepage "https://github.com/reneboygarcia/webp-converter"
  url "https://github.com/reneboygarcia/webp-converter/archive/refs/tags/v0.2.1.tar.gz"
  sha256 "56a36957a72930a581c4e87f94ecb6df0d18cf31a62601005d830d2ed0be3c26"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  def caveats
    <<~EOS
      Run the interactive CLI:
        webp-convert

      Or batch convert non-interactively:
        webp-convert --input /path/to/images --output ~/Downloads
    EOS
  end

  test do
    assert_match "webp-convert", shell_output("#{bin}/webp-convert --help")
  end
end
