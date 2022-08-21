{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/660ac43ff9ab1f12e28bfb31d4719795777fe152.tar.gz") {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.cargo
    pkgs.rustc
    pkgs.rustfmt
    pkgs.rust-analyzer
    pkgs.openssl
  ];
  shellHook = ''
    function update_tag() {
      git tag -d $1
      git push --delete origin $1
      git tag $1
      git push --tags
    }
    export -f update_tag
  '';

}
