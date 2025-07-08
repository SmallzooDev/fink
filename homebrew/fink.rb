class Fink < Formula
  desc "Beautiful TUI for managing AI prompts with version control"
  homepage "https://github.com/SmallzooDev/fink"
  url "https://github.com/SmallzooDev/fink/releases/download/v0.1.2/fink-v0.1.2-macos.tar.gz"
  sha256 "662804a28440790aec976219424b3f3df64dffa987fe1b83d039ff4419efb280"
  version "0.1.2"
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
