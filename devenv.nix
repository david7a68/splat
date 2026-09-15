{
  pkgs,
  lib,
  ...
}: {
  imports = [./nix/rust-toolchain.nix];
  packages = with pkgs; [git pkg-config alejandra dbus];
  languages.c.enable = false;

  env.LD_LIBRARY_PATH = lib.makeLibraryPath (with pkgs; [
    libGL
    libxkbcommon
    wayland
    libx11
    libxcursor
    libxi
    libxrandr
  ]);

  env.__EGL_VENDOR_LIBRARY_FILENAMES = "${pkgs.mesa}/share/glvnd/egl_vendor.d/50_mesa.json";

  scripts.dev-run.exec = "cargo run --locked";
  scripts.check.exec = "cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings";
  enterTest = "cargo test --locked";
}
