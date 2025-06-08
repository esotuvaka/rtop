{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rust-analyzer
    openssl
    pkg-config
    ripgrep
    fd
  ];

  # This makes sure pkg-config can find openssl
  shellHook = ''
    export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig"
    echo "✅ Rust dev shell ready (with OpenSSL, pkg-config, ripgrep, etc)"
  '';
}

