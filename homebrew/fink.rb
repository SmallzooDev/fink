class Fink < Formula
  desc "Beautiful TUI for managing AI prompts with version control"
  homepage "https://github.com/SmallzooDev/fink"
  url "https://github.com/SmallzooDev/fink/releases/download/v0.1.1/fink-v0.1.1-macos.tar.gz"
  sha256 "f9169747d50896e32b3d70177b573b0a637e6fdde707c57885c9ed5b69ff3a03"
  version "0.1.1"
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