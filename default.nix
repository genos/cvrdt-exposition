let
  pkgs = import
    (fetchTarball "https://github.com/NixOS/nixpkgs/archive/20.03.tar.gz") { };
in pkgs.stdenv.mkDerivation {
  name = "cvrdt-exposition";
  buildInputs = [ pkgs.cargo ];
}
