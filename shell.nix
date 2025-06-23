{pkgs ? import <nixpkgs> {}}: let
  overrides = builtins.fromTOML (builtins.readFile ./rust-toolchain.toml);
in
  pkgs.callPackage (
    {
      stdenv,
      mkShell,
      rustPlatform,
    }:
      mkShell {
        strictDeps = true;
        nativeBuildInputs =
          [
            pkgs.rustc
            pkgs.cargo
            pkgs.cargo-binstall
            pkgs.rustfmt
            pkgs.dioxus-cli
            pkgs.wasm-bindgen-cli_0_2_100
            pkgs.lld_20
            rustPlatform.bindgenHook
            pkgs.tailwindcss
	    pkgs.nodejs
          ]
          ++ [
            pkgs.python3
            pkgs.libGL
            pkgs.libGLU
          ]
          ++ [
            pkgs.xorg.libxcb
            pkgs.xorg.libXcursor
            pkgs.xorg.libXrandr
            pkgs.xorg.libXi
            pkgs.pkg-config
          ];
        # libraries here
        buildInputs = with pkgs; [
          nodejs
          rustc
          cargo
          rustfmt
          rust-analyzer
          clippy
          xorg.libX11
          wayland
          libxkbcommon
          trunk
        ];
        RUSTC_VERSION = overrides.toolchain.channel;
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        # https://github.com/rust-lang/rust-bindgen#environment-variables
        shellHook = ''
          export PATH="''${CARGO_HOME:-~/.cargo}/bin":"$PATH"
          export PATH="$PWD/node_modules/.bin/:$PATH"
          export PATH="''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-${stdenv.hostPlatform.rust.rustcTarget}/bin":"$PATH"
          export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [
            "/run/opengl-driver"
            "/run/opengl-driver-32"
            pkgs.libGL
            pkgs.libGLU
            pkgs.vulkan-loader
            pkgs.egl-wayland
            pkgs.wayland
            pkgs.libxkbcommon
            pkgs.xorg.libXcursor
          ]}:$LD_LIBRARY_PATH"
        '';
      }
  ) {}
