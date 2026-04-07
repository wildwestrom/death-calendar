{
  description = "Death Calendar";

  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = nixpkgs.legacyPackages.${system}.extend (
          final: prev: {
            rustPkgs = import nixpkgs {
              inherit system overlays;
            };
          }
        );
        rust-toolchain = pkgs.rustPkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            bacon
            pkg-config
            libGL
            wrapGAppsHook4 # this is needed for relm4-icons to properly load after gtk::init()
            libadwaita
            fontconfig
            rust-toolchain
          ];

          shellHook = ''
            export GSETTINGS_SCHEMA_DIR=${glib.getSchemaPath gtk4}
          '';
        };
      }
    );
}
