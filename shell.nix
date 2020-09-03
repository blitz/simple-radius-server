{
  sources ? import nix/sources.nix,
  nixpkgs ? sources.nixpkgs,
  pkgs ? import nixpkgs {}
}:
pkgs.mkShell {
  inputsFrom = [ (import ./. { inherit sources nixpkgs pkgs; })];

  nativeBuildInputs = with pkgs; [
    # Rust development environment
    cargo rustc rustfmt rust-analyzer

    # Dependency management
    niv
  ];
}
