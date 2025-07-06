class Fink < Formula
  desc "Beautiful TUI for managing AI prompts with version control"
  homepage "https://github.com/SmallzooDev/fink"
  url "https://github.com/SmallzooDev/fink/archive/v0.1.0.tar.gz"
  sha256 "PLACEHOLDER_SHA256"
  license "MIT"
  head "https://github.com/SmallzooDev/fink.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    # Test that the binary runs
    system "#{bin}/fink", "--version"
    
    # Test list command
    output = shell_output("#{bin}/fink list 2>&1")
    assert_match(/No prompts found|code-review/, output)
  end
end