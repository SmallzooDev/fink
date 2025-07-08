class Fink < Formula
  desc "Beautiful TUI for managing AI prompts with version control"
  homepage "https://github.com/SmallzooDev/fink"
  url "https://github.com/SmallzooDev/fink/releases/download/v0.1.3/fink-v0.1.3-macos.tar.gz"
  sha256 "d4386be6855a94e964eaae46145bd2569bc70b37204310ce9fe92b61756c55a9"
  version "0.1.3"
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
