class WebpConverter < Formula
  desc "Fast CLI tool to convert images to WebP — Rust rewrite"
  homepage "https://github.com/reneboygarcia/webp-converter"
  url "https://github.com/reneboygarcia/webp-converter/archive/refs/tags/v0.2.2.tar.gz"
  sha256 "157a5515c9fefba7af5ccb3d5d887d8aa3bd15cba1661dd911fd84d819917bff"
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
