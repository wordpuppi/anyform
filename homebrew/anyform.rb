class Anyform < Formula
  desc "Any database. Any form. Zero hassle."
  homepage "https://github.com/epenabella/anyform"
  version "0.4.0"
  license "MIT"

  on_macos do
    on_intel do
      url "https://github.com/epenabella/anyform/releases/download/v#{version}/anyform-darwin-amd64"
      sha256 "CHECKSUM_PLACEHOLDER"

      def install
        bin.install "anyform-darwin-amd64" => "anyform"
      end
    end
    on_arm do
      url "https://github.com/epenabella/anyform/releases/download/v#{version}/anyform-darwin-arm64"
      sha256 "CHECKSUM_PLACEHOLDER"

      def install
        bin.install "anyform-darwin-arm64" => "anyform"
      end
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/epenabella/anyform/releases/download/v#{version}/anyform-linux-amd64"
      sha256 "CHECKSUM_PLACEHOLDER"

      def install
        bin.install "anyform-linux-amd64" => "anyform"
      end
    end
    on_arm do
      url "https://github.com/epenabella/anyform/releases/download/v#{version}/anyform-linux-arm64"
      sha256 "CHECKSUM_PLACEHOLDER"

      def install
        bin.install "anyform-linux-arm64" => "anyform"
      end
    end
  end

  test do
    system "#{bin}/anyform", "--version"
  end
end
