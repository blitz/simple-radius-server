{ rustPlatform, src }:
rustPlatform.buildRustPackage {
  name = "radius-proto";

  inherit src;

  cargoSha256 = "1lk505m1f2ad8m75y8cyzqg4i8b6k2rx7wwgmjpclb9yiwb1qhmh";
  verifyCargoDeps = true;
}
