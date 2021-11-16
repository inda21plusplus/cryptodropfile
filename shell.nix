{ pkgs ? import <nixpkgs> {} }:

  # Rolling updates, not deterministic.
  # pkgs = import (fetchTarball("channel:nixpkgs-unstable")) {};
pkgs.mkShell {
  buildInputs = [ pkgs.clang pkgs.cargo ];
}
