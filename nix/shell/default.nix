pkgs:
pkgs.mkShell {
  name = "Poincare";
  packages = with pkgs; [
    nixd
    alejandra
    statix
    deadnix
    npins
    cargo
    rustToolchains.nightly
    bacon
    openssl
    pkg-config
  ];
}
