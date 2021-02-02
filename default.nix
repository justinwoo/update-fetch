{ pkgs ? import <nixpkgs> { } }:

let
  dynamic-linker = pkgs.stdenv.cc.bintools.dynamicLinker;

  binDeps = [ pkgs.nix-prefetch-git ];

in
pkgs.stdenv.mkDerivation rec {
  name = "nixpkgs-fmt";

  src = pkgs.fetchurl {
    url = "https://github.com/justinwoo/update-fetch/releases/download/2021-02-03/update-fetch";
    sha256 = "0cp2rm1iyvpgriqwd5m851d7xir1h5d7m0gh8gsfxjgahn885xjv";
  };

  buildInputs = [ pkgs.makeWrapper ];

  dontStrip = true;

  libPath = pkgs.lib.makeLibraryPath [ pkgs.glibc ];

  unpackPhase = ''
    mkdir -p $out/bin
    TARGET=$out/bin/update-fetch

    cp $src $TARGET
    chmod +wx $TARGET

    patchelf $TARGET \
      --interpreter ${dynamic-linker} \
      --set-rpath ${libPath}

    wrapProgram $TARGET \
      --prefix PATH : ${pkgs.lib.makeBinPath binDeps}
  '';

  dontInstall = true;
}
