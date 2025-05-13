let pkgs = import <nixpkgs> { }; in
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    gcc
    rustc
    rust-analyzer
    nixd
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

  # Important for runtime library discovery
  shellHook = ''
    export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [
      pkgs.at-spi2-atk
      pkgs.atkmm
      pkgs.cairo
      pkgs.gdk-pixbuf
      pkgs.glib
      pkgs.gtk3
      pkgs.harfbuzz
      pkgs.librsvg
      pkgs.libsoup_3
      pkgs.pango
      pkgs.webkitgtk_4_1
      pkgs.openssl
    ]}:$LD_LIBRARY_PATH
  '';
}
