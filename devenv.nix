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
    vulkan-loader
    libxkbcommon
    wayland
    libx11
    libxcursor
    libxi
    libxrandr
  ]);

  env.__EGL_VENDOR_LIBRARY_FILENAMES = "${pkgs.mesa}/share/glvnd/egl_vendor.d/50_mesa.json";
  # Visual tests render with Mesa's software Vulkan driver. The app itself uses OpenGL.
  env.VK_DRIVER_FILES = "${pkgs.mesa}/share/vulkan/icd.d/lvp_icd.x86_64.json";

  scripts.dev-run.exec = "cargo run --locked";
  scripts.check.exec = "cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings";
  scripts.visual-test.exec = ''
    label="''${1:-latest}"
    case "$label" in
      "" | */* | .*) echo "Use a plain label, for example: visual-test iter-0" >&2; exit 2 ;;
    esac
    # visual-test <label> --only a,b runs those scenarios and keeps the label's other captures.
    if [ "$#" -gt 1 ]; then
      if [ "$#" -ne 3 ] || [ "$2" != "--only" ]; then
        echo "Use: visual-test <label> [--only <scenario>[,<scenario>]]" >&2
        exit 2
      fi
      export SPLAT_VISUAL_SCENARIOS="$3"
    fi
    out="$PWD/artifacts/screenshots/$label"
    SPLAT_VISUAL_DIR="$out" SPLAT_VISUAL_VIDEO="''${SPLAT_VISUAL_VIDEO:-1}" \
      cargo test --locked --bin splat visual_suite -- --nocapture
    status=$?
    # Host ImageMagick: it must not load the Nix libraries on the shell's library path.
    if [ -n "''${SPLAT_VISUAL_SCENARIOS:-}" ]; then
      echo "Partial run: no contact sheet."
    elif command -v montage >/dev/null; then
      find "$out" -maxdepth 1 -name '*__*.png' ! -name '*@2x.png' | sort > "$out/.contact-sheet-files"
      env -u LD_LIBRARY_PATH montage @"$out/.contact-sheet-files" \
        -label '%t' -pointsize 9 -tile 8x -geometry 300x225+4+4 -background '#222222' -fill white \
        "$out/contact-sheet.png" || echo "Could not write the contact sheet." >&2
      rm -f "$out/.contact-sheet-files"
    else
      echo "montage not found: no contact sheet." >&2
    fi
    echo "Output: $out"
    exit "$status"
  '';
  # Which captures of two visual-test runs differ, above the rendering noise.
  scripts.visual-compare.exec = ''
    if [ "$#" -ne 2 ]; then
      echo "Use two labels, for example: visual-compare iter-2 iter-3" >&2
      exit 2
    fi
    VISUAL_COMPARE_A="$PWD/artifacts/screenshots/$1" VISUAL_COMPARE_B="$PWD/artifacts/screenshots/$2" \
      cargo test --locked --bin splat visual_compare -- --ignored --nocapture
  '';
  enterTest = "cargo test --locked";
}
