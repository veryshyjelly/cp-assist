{ pkgs ? import <nixpkgs> {} }:

pkgs.stdenv.mkDerivation {
  name = "cp-assist";
  version = "0.1.0";

  src = ./src-tauri/target/release/cp-assist; # Or point to your binary

  phases = ["installPhase"];

  installPhase = ''
    mkdir -p $out/bin
    cp ${./src-tauri/target/release/cp-assist} $out/bin/cp-assist
    chmod +x $out/bin/cp-assist

    # Create the resources directory structure that matches /usr/lib/cp-assist
    mkdir -p $out/lib/cp-assist
    cp ${./src-tauri/Languages.toml} $out/lib/cp-assist/Languages.toml

    # Add desktop file for application menus
    mkdir -p $out/share/applications
    cat > $out/share/applications/cp-assist.desktop << EOF
    [Desktop Entry]
    Name=CP Assist
    Exec=$out/bin/cp-assist
    Icon=$out/share/icons/hicolor/128x128/apps/cp-assist.png
    Type=Application
    Categories=Development;Utility;
    EOF

    # Add icon
    mkdir -p $out/share/icons/hicolor/128x128/apps
    cp ${./src-tauri/icons/128x128.png} $out/share/icons/hicolor/128x128/apps/cp-assist.png
  '';
}
