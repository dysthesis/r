{
  pkg-config,
  rustPlatform,
  cargo,
  rustc,
  openssl,
  ...
}:
rustPlatform.buildRustPackage rec {
  name = "r";
  version = "0.1.0";
  nativeBuildInputs = [
    cargo
    rustc
    pkg-config
  ];
  buildInputs = [openssl];
  src = ../../.;
  cargoLock.lockFile = "${src}/Cargo.lock";
}
