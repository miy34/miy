with import <nixpkgs> { };
mkShell rec {
  nativeBuildInputs = [
    # set pkg config path environment var
    pkg-config

    # Rust
    rustup

    gdb
    cmake
    clang

    typos
    committed
  ];

  buildInputs = [
    # required to build GLFW
    ## GLFW Wayland
    extra-cmake-modules
    wayland-scanner
    wayland-protocols
    ## GLFW X11
    xorg.libX11
    xorg.libXrandr
    xorg.libXinerama
    xorg.libXcursor
    xorg.libXi
    libxkbcommon

    wayland
    xwayland

    # audio
    alsa-lib.dev

    mold 

    vulkan-headers
    vulkan-loader
    vulkan-tools
    vulkan-validation-layers
  ];

  # required for tools like RustAnalyzer
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

  LD_LIBRARY_PATH = lib.makeLibraryPath [
    libxkbcommon
    wayland
    libGL
    xorg.libX11
    vulkan-headers
    vulkan-loader
    vulkan-tools
    vulkan-validation-layers
  ];

 shellHook = ''
    export PATH="$HOME/.cargo/bin:$PATH"
    rustup +stable component add rust-analyzer rustfmt cargo
    rustup target add wasm32-unknown-unknown

    # Install Miri if not already installed
    rustup +nightly component add miri

    export VK_LAYER_PATH="${pkgs.vulkan-validation-layers}/share/vulkan/explicit_layer.d";
  '';
}
