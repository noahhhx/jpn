{ pkgs, ... }:

{
  languages.rust = {
    enable = true;
    channel = "stable";
    targets = [ "wasm32-unknown-unknown" ];
  };

  packages = [
    pkgs.cargo-leptos
    pkgs.binaryen # wasm-opt, used by cargo-leptos for release builds
    # must match the wasm-bindgen version pinned in Cargo.toml
    pkgs.wasm-bindgen-cli_0_2_127
  ];
}
