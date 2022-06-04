{ pkgs ? import <nixpkgs> { } }:

let
  dynamic-linker = pkgs.stdenv.cc.bintools.dynamicLinker;

  binDeps = [ pkgs.nix-prefetch-git ];

in
pkgs.stdenv.mkDerivation rec {
  name = "nixpkgs-fmt";

  src = builtins.fetchTarball {
    url = "https://justin.gateway.scarf.sh/update-fetch/1.0.0.tgz";
    sha256 = "04b8d3pxkmd6cxclc9p903l110xz59dqgblfqw1f6jwsd641nw6n";
  };

  buildInputs = [ pkgs.makeWrapper ];

  dontStrip = true;

  libPath = pkgs.lib.makeLibraryPath [ pkgs.glibc ];

  unpackPhase = ''
    mkdir -p $out/bin
    TARGET=$out/bin/update-fetch

    cp $src/update-fetch $TARGET
    chmod +wx $TARGET

    patchelf $TARGET \
      --interpreter ${dynamic-linker} \
      --set-rpath ${libPath}

    wrapProgram $TARGET \
      --prefix PATH : ${pkgs.lib.makeBinPath binDeps}
  '';

  dontInstall = true;
}
