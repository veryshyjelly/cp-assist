{
  description = "CP Assist";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "cp-assist";
          version = "0.1.0";
          src = ./.;

          # Fix the Cargo.lock issue by pointing to the correct location
          postPatch = ''
            ln -s ${./src-tauri/Cargo.lock} Cargo.lock
          '';

          # Use pre-downloaded pnpm dependencies
          npmDeps = ./pnpmDeps.tar.gz;

          cargoLock.lockFile = ./src-tauri/Cargo.lock;

          nativeBuildInputs = with pkgs; [
            gcc
            rustc
            cargo
            cargo-tauri
            rustfmt
            pnpm
            nodejs_22
            pkg-config
          ];

          buildInputs = with pkgs; [
            gobject-introspection
            at-spi2-atk
            atkmm
            cairo
            gdk-pixbuf
            glib
            gtk3
            harfbuzz
            librsvg
            libsoup_3
            pango
            webkitgtk_4_1
            openssl
          ];

          buildPhase = ''
            tar -xzf $npmDeps -C .
            # Build the Tauri app
            pnpm --offline tauri build
          '';

          installPhase = ''
            mkdir -p $out/bin
            cp src-tauri/target/release/cp-assist $out/bin/

            # Add desktop file and icons if needed
            mkdir -p $out/share/applications
            cp src-tauri/target/release/bundle/linux/cp-assist.desktop $out/share/applications/ || true

            mkdir -p $out/share/icons/hicolor/128x128/apps
            cp src-tauri/icons/128x128.png $out/share/icons/hicolor/128x128/apps/cp-assist.png || true
          '';
        };

        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            gcc
            rustc
            cargo
            cargo-tauri
            rustfmt
            pnpm
            nodejs_22
          ];
          buildInputs = with pkgs; [
            pkg-config
            gobject-introspection
            at-spi2-atk
            atkmm
            cairo
            gdk-pixbuf
            glib
            gtk3
            harfbuzz
            librsvg
            libsoup_3
            pango
            webkitgtk_4_1
            openssl
          ];

          shellHook = ''
            echo "Tauri development environment"
            echo "Run 'pnpm install' to install dependencies"
            echo "Run 'pnpm tauri dev' to start development server"
          '';
        };
      }
    );
}
