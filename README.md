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

```
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

```
$ update-fetch spago2nix.nix
Fetched rev and sha256 for justinwoo/spago2nix
Finished spago2nix.nix
```
