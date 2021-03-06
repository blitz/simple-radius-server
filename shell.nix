{ sources ? import nix/sources.nix, nixpkgs ? sources.nixpkgs
, pkgs ? import nixpkgs { } }:
pkgs.mkShell {
  inputsFrom = [
    (import ./nix/release.nix {
      inherit sources nixpkgs pkgs;
    }).simple-radius-server
  ];

  nativeBuildInputs = with pkgs; [
    # Rust development environment
    cargo
    cargo-bloat
    cargo-license
    rustc
    rustfmt
    rust-analyzer

    # Dependency management
    niv

    # Testing with radclient. See the examples at
    # https://wiki.freeradius.org/config/Radclient for how to use
    # this.
    freeradius
  ];
}
