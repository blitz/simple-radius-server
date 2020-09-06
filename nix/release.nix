{ sources ? import ./sources.nix, nixpkgs ? sources.nixpkgs
, pkgs ? import nixpkgs { } }:

let
  simple-radius-server = pkgs.callPackage ./build.nix {
    # TODO This needs gitignore filtering.
    src = ../radius-proto;
  };
in 
{
  inherit simple-radius-server;

  integration-test = import ./integration-test.nix {
    inherit nixpkgs simple-radius-server;
  };
}
