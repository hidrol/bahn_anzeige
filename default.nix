{ pkgs ? import <nixpkgs> { } }:
let manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
pkgs.rustPlatform.buildRustPackage rec {
  pname = manifest.name;
  version = manifest.version;
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;
  #src = ./.;
  nativeBuildInputs = with pkgs; [ pkg-config ];
  # buildInputs = [ pkgs.pkg-config  ];
  buildInputs = with pkgs; [ cargo rustc openssl ];
  postFixup = ''
      cp -r static $out/bin
    # mkdir -p $out/bin
    # cp hello-world $out/bin
  '';



}
