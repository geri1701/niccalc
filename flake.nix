{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs;
          mkShell {
            buildInputs = [
              cargo
              rustc
              rustfmt
              pre-commit
              rustPackages.clippy
              pkgs.xorg.libX11
              pkgs.xorg.libXext
              pkgs.xorg.libXinerama
              pkgs.xorg.libXcursor
              pkgs.xorg.libXrender
              pkgs.xorg.libXfixes
              pkgs.xorg.libXft
              pkgs.pango
            ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
      });
}
