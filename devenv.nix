{
  pkgs,
  lib,
  ...
}: {
  imports = [./nix/rust-toolchain.nix];
  packages = with pkgs; [git pkg-config alejandra dbus] ++ lib.optionals stdenv.isLinux [util-linux];
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
  # Visual test captures use Mesa's Vulkan drivers: an Intel or AMD GPU when there is one,
  # otherwise the software rasterizer. The app itself uses OpenGL. NVIDIA's own driver is not
  # listed: Nix's Vulkan loader does not find its library.
  env.VK_DRIVER_FILES = builtins.concatStringsSep ":" (
    map (name: "${pkgs.mesa}/share/vulkan/icd.d/${name}_icd.x86_64.json") [
      "intel"
      "radeon"
      "lvp"
    ]
  );

  scripts.dev-run.exec = "cargo run --locked";
  scripts.check.exec = "cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings";
  # Run once in the seed worktree before copying target/ to implementers.
  scripts.audit-warm.exec = "cargo test --locked --no-run && check";
  # The full merge gate: run visual scenarios once, with captures, among all tests.
  scripts.audit-gate.exec = ''
    if [ "$#" -gt 1 ]; then
      echo "Use: audit-gate [label]" >&2
      exit 2
    fi
    label="''${1:-latest}"
    case "$label" in
      "" | */* | .*) echo "Use a plain capture label." >&2; exit 2 ;;
    esac
    # Feedback-only settings must not turn a merge gate into a partial/report run.
    unset SPLAT_VISUAL_SCENARIOS SPLAT_VISUAL_UPDATE_BASELINE SPLAT_VISUAL_DIAGNOSE
    unset SPLAT_DM_AUDIT_WRITE SPLAT_PLAIN_EVENING_WRITE
    export SPLAT_VISUAL_ENFORCE=baseline
    # Cargo serializes compilation in this target directory, but releases its
    # build lock before running tests. Let fmt/Clippy overlap test execution.
    check &
    check_pid=$!
    out="$PWD/artifacts/screenshots/$label"
    export SPLAT_VISUAL_CHROME_HINT="$(printf \
      'After reviewing chrome changes, run: devenv shell -- visual-baseline-chrome %q\nThen rerun the full gate: devenv shell -- audit-gate %q' \
      "$label" "$label")"
    SPLAT_VISUAL_DIR="$out" SPLAT_VISUAL_VIDEO="''${SPLAT_VISUAL_VIDEO:-1}" \
      cargo test --locked -- --nocapture
    status=$?
    wait "$check_pid"
    check_status=$?
    if [ "$check_status" -ne 0 ]; then
      status=$check_status
    fi
    if [ "''${SPLAT_VISUAL_CONTACT_SHEET:-0}" = 1 ]; then
      visual-contact-sheet "$label"
    else
      echo "No contact sheet. Use: visual-contact-sheet $label"
    fi
    echo "Output: $out"
    exit "$status"
  '';
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
      cargo test --locked --package splat --lib visual_suite -- --nocapture
    status=$?
    if [ -n "''${SPLAT_VISUAL_SCENARIOS:-}" ]; then
      echo "Partial run: no contact sheet."
    elif [ "''${SPLAT_VISUAL_CONTACT_SHEET:-0}" = 1 ]; then
      visual-contact-sheet "$label"
    else
      echo "No contact sheet. Use: visual-contact-sheet $label"
    fi
    echo "Output: $out"
    exit "$status"
  '';
  # Measure the audit's evening (G6) and write the plain reference (G7) from this source.
  # Run it on main only: the files it writes are what every later change is compared with.
  scripts.audit-freeze.exec = ''
    if [ "$#" -gt 1 ]; then
      echo "Use: audit-freeze [label]" >&2
      exit 2
    fi
    label="''${1:-freeze}"
    case "$label" in
      "" | */* | .*) echo "Use a plain capture label." >&2; exit 2 ;;
    esac
    SPLAT_VISUAL_ENFORCE=report SPLAT_DM_AUDIT_WRITE=1 SPLAT_PLAIN_EVENING_WRITE=1 \
      visual-test "$label" --only dm-audit,plain-evening
  '';
  # Apply only chrome counts from a completed full or --only run, without recapturing.
  scripts.visual-baseline-chrome.exec = ''
    if [ "$#" -ne 1 ]; then
      echo "Use: visual-baseline-chrome <label>" >&2
      exit 2
    fi
    case "$1" in
      "" | */* | .*) echo "Use a plain capture label." >&2; exit 2 ;;
    esac
    SPLAT_VISUAL_BASELINE_RUN="$PWD/artifacts/screenshots/$1" \
      cargo test --locked --package splat --lib visual_update_chrome -- --ignored --nocapture
  '';
  # A sheet is optional and can be made from existing captures without rerunning tests.
  scripts.visual-contact-sheet.exec = ''
    if [ "$#" -ne 1 ]; then
      echo "Use: visual-contact-sheet <label>" >&2
      exit 2
    fi
    case "$1" in
      "" | */* | .*) echo "Use a plain capture label." >&2; exit 2 ;;
    esac
    out="$PWD/artifacts/screenshots/$1"
    if ! command -v montage >/dev/null; then
      echo "montage not found: no contact sheet." >&2
      exit 1
    fi
    # Host ImageMagick must not load the Nix libraries on the shell's library path.
    find "$out" -maxdepth 1 -name '*__*.png' ! -name '*@2x.png' | sort > "$out/.contact-sheet-files"
    trap 'rm -f "$out/.contact-sheet-files"' EXIT
    mkdir -p "$PWD/target/imagemagick"
    env -u LD_LIBRARY_PATH MAGICK_TEMPORARY_PATH="$PWD/target/imagemagick" \
      montage @"$out/.contact-sheet-files" \
      -label '%t' -pointsize 9 -tile 8x -geometry 300x225+4+4 -background '#222222' -fill white \
      "$out/contact-sheet.png"
  '';
  # Which captures of two visual-test runs differ, above the rendering noise.
  scripts.visual-compare.exec = ''
    if [ "$#" -ne 2 ]; then
      echo "Use two labels, for example: visual-compare iter-2 iter-3" >&2
      exit 2
    fi
    VISUAL_COMPARE_A="$PWD/artifacts/screenshots/$1" VISUAL_COMPARE_B="$PWD/artifacts/screenshots/$2" \
      cargo test --locked --package splat --lib visual_compare -- --ignored --nocapture
  '';
  enterTest = "cargo test --locked";
}
