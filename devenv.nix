{
  pkgs,
  lib,
  ...
}: {
  imports = [./nix/rust-toolchain.nix];
  # GPUI builds against fontconfig, FreeType, xkbcommon, Wayland and xcb through pkg-config.
  packages = with pkgs; [git pkg-config alejandra dbus] ++ lib.optionals stdenv.isLinux [util-linux fontconfig freetype libxkbcommon wayland libxcb vulkan-loader];
  languages.c.enable = false;

  env.LD_LIBRARY_PATH = lib.makeLibraryPath (with pkgs; [
    libGL
    vulkan-loader
    libxkbcommon
    wayland
    libx11
    libxcursor
    libxi
    libxrandr
    libxcb
    fontconfig
    freetype
  ]);

  env.__EGL_VENDOR_LIBRARY_FILENAMES = "${pkgs.mesa}/share/glvnd/egl_vendor.d/50_mesa.json";
  # GUI test captures (`gui::harness`) use Mesa's Vulkan drivers: an Intel or AMD GPU when there
  # is one, otherwise the software rasterizer. NVIDIA's own driver is not listed: Nix's Vulkan
  # loader does not find its library.
  env.VK_DRIVER_FILES = builtins.concatStringsSep ":" (
    map (name: "${pkgs.mesa}/share/vulkan/icd.d/${name}_icd.x86_64.json") [
      "intel"
      "radeon"
      "lvp"
    ]
  );

  scripts.dev-run.exec = "cargo run --locked";
  # Test targets unify test-support features from dev dependencies. Lint the normal
  # build on its own so it also covers the live recording and credentials paths.
  scripts.check.exec = "cargo fmt --check && cargo clippy --locked --workspace --lib --bins -- -D warnings && cargo clippy --locked --all-targets -- -D warnings";
  # Run once in the seed worktree before copying target/ to implementers.
  scripts.audit-warm.exec = "cargo test --locked --no-run && check";
  # The GPUI page tests and their captures (`src/gui/`). Captures land in
  # artifacts/screenshots/gpui/. `gui-test [filter] [test args]`: the filter defaults to `gui::`,
  # for example `gui-test gui::pages::run -- --nocapture`.
  scripts.gui-test.exec = ''
    filter="''${1:-gui::}"
    if [ "$#" -gt 0 ]; then
      shift
    fi
    if [ "''${1:-}" = "--" ]; then
      shift
    fi
    cargo test --locked --package splat --lib -- "$filter" "$@"
    status=$?
    echo "Captures: $PWD/artifacts/screenshots/gpui"
    exit "$status"
  '';
  enterTest = "cargo test --locked";
}
