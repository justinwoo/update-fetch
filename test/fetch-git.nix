{ pkgs ? import <nixpkgs> {} }:

let
  a = import (
    pkgs.fetchgit {
      url = "https://github.com/justinwoo/empty-repo";
      rev = "108b9ec239caf7a9e39de456d1a80a9c5f4c5afe";
      sha256 = "0sjjj9z1dhilhpc8pq4154czrb79z9cm044jvn75kxcjv6v5l2m5";
    }
  ) {};
in
{
  inherit a;
}
