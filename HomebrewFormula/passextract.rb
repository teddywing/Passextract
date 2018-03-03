class Passextract < Formula
  desc "An interface to easily copy login information from Pass"
  homepage "https://github.com/teddywing/Passextract"
  url "https://github.com/teddywing/Passextract/releases/download/v0.4.0/passextract-0.4.0_osx_amd64.tar.bz2"
  sha256 "622439d19d2cedde4c6666d29cc9fad5487083da3e26660b66910e2f469eb54e"

  def install
    bin.install "passextract"
    bash_completion.install "passextract.bash-completion" => "passextract"
    man1.install "passextract.1"
  end

  def caveats; <<~EOS
    To enable tab completion, add the following to your bash profile:

        source #{etc}/bash_completion.d/passextract
    EOS
  end
end
