class Mewn < Formula
  desc "Network monitor with cat mascot"
  homepage "https://github.com/adit-prawira/mewn"
  version "0.1.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/adit-prawira/mewn/releases/download/v0.1.0/mewn-v0.1.0-aarch64-apple-darwin.tar.gz"
      sha256 "ac9bf1fd7d992110c6a021aaa0876cbfbc2d217d057ebf03742dffcad7d35a87"
    else
      url "https://github.com/adit-prawira/mewn/releases/download/v0.1.0/mewn-v0.1.0-x86_64-apple-darwin.tar.gz"
      sha256 "b77073f5338e0ae5c010b5328f053c7d5abe2cd9d91d641d12ff2279d2b55d33"
    end
  end

  def install
    bin.install "mewn"
  end

  def caveats
    <<~EOS
      Setup packet capture permissions:
        sudo mewn setup

      Download GeoIP database:
        IP2LOCATION_LICENSE_KEY=your-key mewn geoip-update
    EOS
  end

  test do
    assert_match "mewn", shell_output("#{bin}/mewn version")
  end
end
