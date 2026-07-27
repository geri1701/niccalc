{
  description = "A nicotine calculator for e-cigarette liquids";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs =
    { self, nixpkgs }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      manifest = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      source = nixpkgs.lib.fileset.toSource {
        root = ./.;
        fileset = nixpkgs.lib.fileset.unions [
          ./Cargo.lock
          ./Cargo.toml
          ./LICENSE.txt
          ./README.md
          ./assets
          ./src
        ];
      };
      formatSource = nixpkgs.lib.fileset.toSource {
        root = ./.;
        fileset = nixpkgs.lib.fileset.unions [
          ./Cargo.toml
          ./flake.nix
          ./src
        ];
      };
      mkPkgs = system: import nixpkgs { inherit system; };
      nativeBuildInputs = pkgs: [
        pkgs.cmake
        pkgs.pkg-config
      ];
      buildInputs = pkgs: [
        pkgs.cairo
        pkgs.expat
        pkgs.fontconfig
        pkgs.freetype
        pkgs.glib
        pkgs.libsysprof-capture
        pkgs.libx11
        pkgs.libxcursor
        pkgs.libxext
        pkgs.libxfixes
        pkgs.libxft
        pkgs.libxinerama
        pkgs.libxrender
        pkgs.pango
      ];
      mkPackage =
        pkgs:
        pkgs.rustPlatform.buildRustPackage {
          pname = manifest.package.name;
          version = manifest.package.version;
          src = source;

          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = nativeBuildInputs pkgs;
          buildInputs = buildInputs pkgs;

          meta = {
            inherit (manifest.package) description homepage;
            license = pkgs.lib.licenses.mit;
            mainProgram = manifest.package.name;
            platforms = supportedSystems;
          };
        };
    in
    {
      packages = forAllSystems (
        system:
        let
          pkgs = mkPkgs system;
        in
        rec {
          niccalc = mkPackage pkgs;
          default = niccalc;
        }
      );

      apps = forAllSystems (system: {
        default = {
          type = "app";
          program = nixpkgs.lib.getExe self.packages.${system}.default;
          meta.description = manifest.package.description;
        };
      });

      checks = forAllSystems (
        system:
        let
          pkgs = mkPkgs system;
        in
        {
          package = self.packages.${system}.default;
          formatting =
            pkgs.runCommand "niccalc-formatting"
              {
                nativeBuildInputs = [
                  pkgs.cargo
                  pkgs.nixfmt
                  pkgs.rustfmt
                ];
                src = formatSource;
              }
              ''
                cp -r "$src" source
                chmod -R u+w source
                cd source
                cargo fmt --check --all
                nixfmt --check flake.nix
                touch "$out"
              '';
        }
      );

      devShells = forAllSystems (
        system:
        let
          pkgs = mkPkgs system;
        in
        {
          default = pkgs.mkShell {
            inputsFrom = [ self.packages.${system}.default ];
            packages = [
              pkgs.cargo
              pkgs.clippy
              pkgs.nixfmt
              pkgs.rustc
              pkgs.rustfmt
            ];
          };
        }
      );

      formatter = forAllSystems (system: (mkPkgs system).nixfmt);
    };
}
