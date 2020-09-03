{ rustPlatform, src }:
rustPlatform.buildRustPackage {
  name = "radius-proto";

  inherit src;

  cargoSha256 = "05xr8d4gpw5jpaah3ww3b14kkwrbjglhikb24rja9z8cq49jydky";
  verifyCargoDeps = true;
}
