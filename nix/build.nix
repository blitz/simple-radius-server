{ rustPlatform, src }:
rustPlatform.buildRustPackage {
  name = "radius-proto";

  inherit src;

  cargoSha256 = "06vh3c87ryp0vzydjvxdmp4w4s6ajicv26jjv74x063s2nd9z7cr";
  verifyCargoDeps = true;
}
