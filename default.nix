{ pkgs ? import <nixpkgs> { } }:

let
  dynamic-linker = pkgs.stdenv.cc.bintools.dynamicLinker;

  binDeps = [ pkgs.nix-prefetch-git ];

in
pkgs.stdenv.mkDerivation rec {
  name = "update-fetch";

  src = builtins.fetchTarball {
    url = "https://justin.gateway.scarf.sh/update-fetch/1.1.0.tgz";
    sha256 = "1qjsaxcld5czp4fv63hl3lrzsr5lvnv89gzh638wixil3a6dp4b5";
  };

  buildInputs = [ pkgs.makeWrapper pkgs.glibc ];

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
