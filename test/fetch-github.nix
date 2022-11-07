# https://github.com/justinwoo/easy-purescript-nix/blob/7255d015b80d28c7c6db655dda215535cb2d4b41/psc-package2nix.nix

{ pkgs ? import <nixpkgs> { } }:

import
  (
    pkgs.fetchFromGitHub {
      owner = "justinwoo";
      repo = "psc-package2nix";
      rev = "f271b3ad7a8e2931a50b03dafd906262679d527f";
      sha256 = "0dqz0955912jq7imrh09dms4pj3cj4aags666dpg0p5zgk30sgnl";
    }
  )
{
  inherit pkgs;
}
