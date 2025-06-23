{
  self,
  pkgs,
  lib,
  inputs,
  ...
}: rec {
  default = r;
  r = pkgs.callPackage ./r.nix {inherit pkgs inputs lib self;};
}
