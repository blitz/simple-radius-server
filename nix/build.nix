{ rustPlatform, src }:
rustPlatform.buildRustPackage {
  name = "hello";

  inherit src;

  cargoSha256 = "1sksgrmhsirhmdrx7z4g2xix3h729i1w2c59fhsr617q6ch0n5ri";
  verifyCargoDeps = true;
}
