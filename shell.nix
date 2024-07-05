let
  moz_overlay = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  ruststable = (nixpkgs.latest.rustChannels.stable.rust.override {
    extensions = [ "rust-src" "rustfmt-preview" "clippy-preview" ];
  });

  inherit (nixpkgs.lib) optionals;
in with nixpkgs;
mkShell rec {
  buildInputs = [ rustup rust-analyzer ruststable ] ++ optionals stdenv.isDarwin
    (with darwin.apple_sdk.frameworks; [ CoreServices ]);
  RUST_SRC_PATH = "${nixpkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${
      with pkgs;
      lib.makeLibraryPath [ wayland libxkbcommon fontconfig ]
    }";

}
