{ pkgs ? import <nixpkgs> { } }:

let
  default = import ../default.nix { inherit pkgs; };
in
default.overrideAttrs (x: {
  src = ../output;
})
