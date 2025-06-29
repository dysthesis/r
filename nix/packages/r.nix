{
  rustToolchains,
  pkg-config,
  rustPlatform,
  openssl,
  ...
}:
rustPlatform.buildRustPackage rec {
  name = "r";
  version = "0.1.0";
  nativeBuildInputs = [
    pkg-config
    rustToolchains.nightly
  ];
  buildInputs = [openssl];
  RUSTFLAGS = [
    "-Zlocation-detail=none"
    "-Zfmt-debug=none"
  ];
  src = ../../.;
  cargoLock.lockFile = "${src}/Cargo.lock";
}
