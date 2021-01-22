class Trash < Formula
  desc "Trash is a command-line application for moving files and folders into the trash. A safer alternative to rm."
  homepage "https://github.com/JakeChampion/trash"
  url "https://github.com/JakeChampion/trash/releases/download/refs/tags/v1.0.4/trash-x86_64-apple-darwin"
  sha256 ""
  version "refs/tags/v1.0.4"

  def install
    bin.install "trash"
  end
end
