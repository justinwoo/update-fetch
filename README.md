# update-fetch

for updating fetch attributes in nix expressions

## Installation

```nix
{ pkgs ? import <nixpkgs> {} }:

import (
  pkgs.fetchFromGitHub {
    owner = "justinwoo";
    repo = "update-fetch";
    rev = "<ref>";
    sha256 = "<sha256>";
  }
) {}
```

Get the latest rev and sha256 by running `nix-prefetch-git https://github.com/justinwoo/update-fetch`

## Usage

```bash
$ update-fetch
Need arguments for what in_files to process.
Usage Examples:
    # update a single in_file
    update-fetch-derivation my-in_file.nix
    # Using fd (sequential)
    fd -e nix -x update-fetch-derivation {}
    # Multiple files
    update-fetch-derivation *.nix
```

## Example

```diff
$ update-fetch default.nix
Fetched rev and sha256 for justinwoo/easy-purescript-nix
Finished default.nix

$ git diff
diff --git a/default.nix b/default.nix
--- a/default.nix
+++ b/default.nix
@@ -21,8 +21,8 @@ in
       pkgs.fetchFromGitHub {
         owner = "justinwoo";
         repo = "easy-purescript-nix";
-        rev = "cc7196bff3fdb5957aabfe22c3fa88267047fe88";
-        sha256 = "1xfl7rnmmcm8qdlsfn3xjv91my6lirs5ysy01bmyblsl10y2z9iw";
+        rev = "927403abd55dfc82824019cc03efbc28047b3d46";
+        sha256 = "1lj1jrrxpzn2lravmam7xbzb2d3bg40yacmvh4m7gc3rmvnc9bh8";
       }
     ) {
       inherit pkgs;
```
