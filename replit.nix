
{ pkgs }: {
  deps = [
    pkgs.nodejs_20
    pkgs.nodePackages.typescript-language-server
    pkgs.yarn
    pkgs.replitPackages.jest
    pkgs.rustc
    pkgs.cargo
    pkgs.rustfmt
    pkgs.clippy
    pkgs.pkg-config
    pkgs.openssl
    pkgs.gtk3
    pkgs.webkitgtk_4_1
    pkgs.libappindicator-gtk3
    pkgs.librsvg
    pkgs.glib
    pkgs.glib.dev
    pkgs.gobject-introspection
  ];
}
