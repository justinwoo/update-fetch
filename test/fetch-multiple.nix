{ pkgs ? import <nixpkgs> { } }:

{
  a =
    pkgs.fetchurl {
      url = "https://github.com/justinwoo/empty-repo/archive/108b9ec239caf7a9e39de456d1a80a9c5f4c5afe.tar.gz";
      sha256 = "0bvsp5am5s4zj8wdn07jlwr3zyj9skdvrfizk3jkki1w5h0xmvrn";
    };
  b =
    pkgs.fetchurl {
      url = "https://github.com/justinwoo/empty-repo/archive/108b9ec239caf7a9e39de456d1a80a9c5f4c5afe.tar.gz";
      sha256 = "0bvsp5am5s4zj8wdn07jlwr3zyj9skdvrfizk3jkki1w5h0xmvrn";
    };
  c =
    pkgs.fetchurl {
      url = "https://github.com/justinwoo/empty-repo/archive/108b9ec239caf7a9e39de456d1a80a9c5f4c5afe.tar.gz";
      sha256 = "0bvsp5am5s4zj8wdn07jlwr3zyj9skdvrfizk3jkki1w5h0xmvrn";
    };
  d =
    pkgs.fetchurl {
      url = "https://github.com/justinwoo/empty-repo/archive/108b9ec239caf7a9e39de456d1a80a9c5f4c5afe.tar.gz";
      sha256 = "0bvsp5am5s4zj8wdn07jlwr3zyj9skdvrfizk3jkki1w5h0xmvrn";
    };
  e =
    pkgs.fetchurl {
      url = "https://github.com/justinwoo/empty-repo/archive/108b9ec239caf7a9e39de456d1a80a9c5f4c5afe.tar.gz";
      sha256 = "0bvsp5am5s4zj8wdn07jlwr3zyj9skdvrfizk3jkki1w5h0xmvrn";
    };
}
