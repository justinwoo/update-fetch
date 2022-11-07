{ pkgs ? import <nixpkgs> { } }:

let
  dynamic-linker = pkgs.stdenv.cc.bintools.dynamicLinker;

  binDeps = [
    pkgs.nix-prefetch-git
  ];

  libPath = pkgs.lib.makeLibraryPath [ pkgs.glibc ];

  patchelf =
    if pkgs.stdenv.isDarwin
    then ""
    else "patchelf $TARGET --interpreter ${dynamic-linker} --set-rpath ${libPath}";

in
pkgs.stdenv.mkDerivation rec {
  name = "update-fetch";

  src =
    if pkgs.stdenv.isDarwin
    then
      pkgs.fetchzip
        {
          url = "https://justin.gateway.scarf.sh/update-fetch/binaries/2022-11-07/macos.zip";
          sha256 = "BBE3vfP2XRHIEVt9fG87inI3Tca5l/fp9ubIOnzA4iA=";
        }
    else
      pkgs.fetchzip {
        url = "https://justin.gateway.scarf.sh/update-fetch/binaries/2022-11-07/ubuntu.zip";
        sha256 = "6qSRe63F3yaIgM48uwHg2NTsypzs6yxsvc+9k8Rh5I4=";
      };

  buildInputs = [ pkgs.makeWrapper ];

  dontStrip = true;

  unpackPhase = ''
    mkdir -p $out/bin
    TARGET=$out/bin/update-fetch

    cp $src/update-fetch $TARGET
    chmod +wx $TARGET

    ${patchelf}

    wrapProgram $TARGET \
      --prefix PATH : ${pkgs.lib.makeBinPath binDeps}
  '';

  dontInstall = true;
}
