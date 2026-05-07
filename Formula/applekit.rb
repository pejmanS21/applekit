class Applekit < Formula
  desc "macOS CLI for creating Apple Notes and Apple Reminders"
  homepage "https://github.com/pejmanS21/applekit"
  url "https://github.com/pejmanS21/applekit/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "fa9ed7b36c9c9c34f49da45a718fd529684cbb73b1c460f56c16cf3c117811d7"
  license "MIT"
  head "https://github.com/pejmanS21/applekit.git", branch: "main"

  depends_on "rust" => :build
  depends_on :macos
  uses_from_macos "swift" => :build

  def install
    system "cargo", "install", *std_cargo_args
    system "./scripts/build-swift-helper.sh"
    bin.install "target/helper/ReminderHelper"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/applekit --version")
    assert_path_exists bin/"ReminderHelper"
  end
end
