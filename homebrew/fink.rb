class Fink < Formula
  desc "Beautiful TUI for managing AI prompts with version control"
  homepage "https://github.com/SmallzooDev/fink"
  url "https://github.com/SmallzooDev/fink/releases/download/v0.1.4/fink-v0.1.4-macos.tar.gz"
  sha256 "f04dd9b5d316f6b366241ed15b7ac947eef738683d2112197f8f9f8324f3f283"
  version "0.1.4"
  license "MIT"

  def install
    bin.install "fink"
  end

  test do
    # Test that the binary runs
    system "#{bin}/fink", "--version"
    
    # Test list command
    output = shell_output("#{bin}/fink list 2>&1")
    assert_match(/No prompts found|code-review/, output)
  end
end
