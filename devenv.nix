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
    unset SPLAT_DM_AUDIT_WRITE SPLAT_PLAIN_EVENING_WRITE SPLAT_FROZEN_AT SPLAT_PLAIN_EVENING_OUT SPLAT_FREEZE_STAGE
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
  # G6's counts and G7's plain reference (README.md, Visual tests).
  #   audit-freeze counts [label]     G6: write every row of the audit's counts from this source.
  #   audit-freeze journeys [label]   G6: write every row of journeys J2, J3, J4 and J6 likewise.
  #   audit-freeze reference [label]  G7: write the plain reference on an unchanged commit of main.
  #   audit-freeze add [label]        Add new tasks, views and acts; keep every frozen record.
  #   audit-freeze verify [label]     Write the reference to a scratch folder and compare bytes.
  scripts.audit-freeze.exec = ''
    what="''${1:-}"
    label="''${2:-freeze}"
    if [ "$#" -lt 1 ] || [ "$#" -gt 2 ]; then
      echo "Use: audit-freeze counts|journeys|reference|add|verify [label]" >&2
      exit 2
    fi
    case "$label" in
      "" | */* | .*) echo "Use a plain capture label." >&2; exit 2 ;;
    esac
    export SPLAT_VISUAL_ENFORCE=report
    # The reference is main's: only its own files may differ from the commit it names.
    unchanged() {
      if [ -n "$(git status --porcelain --untracked-files=no -- . \
        ':(exclude)src/visual_tests/plain_evening/' ':(exclude)artifacts/dm-audit/titles.tsv')" ]; then
        echo "audit-freeze $what runs on an unchanged commit of main. These files differ:" >&2
        git status --short --untracked-files=no -- . \
          ':(exclude)src/visual_tests/plain_evening/' ':(exclude)artifacts/dm-audit/titles.tsv' >&2
        exit 1
      fi
    }
    # Each frozen file under artifacts/dm-audit/ and src/visual_tests/plain_evening/, with its hash.
    frozen_sums() {
      find artifacts/dm-audit src/visual_tests/plain_evening -type f -print0 | sort -z | xargs -0 sha256sum
    }
    # A freeze of several scenarios writes all its files or none. Each scenario writes into a
    # staging folder; only when every one passed are they copied into the tree. A refusal must
    # leave every frozen file byte-equal, and this checks it.
    staged() {
      stage="$PWD/artifacts/screenshots/$label/freeze-stage"
      rm -rf "$stage"
      before="$(frozen_sums)"
      if env SPLAT_FREEZE_STAGE="$stage" "$@"; then
        if [ -d "$stage" ]; then cp -a "$stage"/. "$PWD"/; fi
        rm -rf "$stage"
        return 0
      fi
      if [ "$(frozen_sums)" != "$before" ]; then
        echo "audit-freeze $what refused, and yet changed frozen files:" >&2
        diff <(echo "$before") <(frozen_sums) >&2
        exit 3
      fi
      echo "audit-freeze $what refused: no frozen file changed." >&2
      exit 1
    }
    journeys="journey-j2,journey-j3,journey-j4,journey-j6"
    case "$what" in
      counts)
        staged env SPLAT_DM_AUDIT_WRITE=all visual-test "$label" --only dm-audit ;;
      journeys)
        staged env SPLAT_DM_AUDIT_WRITE=all visual-test "$label" --only "$journeys" ;;
      reference)
        unchanged
        SPLAT_FROZEN_AT="$(git rev-parse HEAD)" SPLAT_PLAIN_EVENING_WRITE=all \
          visual-test "$label" --only plain-evening ;;
      add)
        staged env SPLAT_DM_AUDIT_WRITE=add SPLAT_PLAIN_EVENING_WRITE=add \
          visual-test "$label" --only "dm-audit,plain-evening,$journeys" ;;
      verify)
        unchanged
        out="$PWD/artifacts/screenshots/$label/verify"
        rm -rf "$out"
        SPLAT_FROZEN_AT="$(git rev-parse HEAD)" SPLAT_PLAIN_EVENING_WRITE=all SPLAT_PLAIN_EVENING_OUT="$out" \
          visual-test "$label" --only plain-evening || exit 1
        tree="$PWD/src/visual_tests/plain_evening"
        if diff -q "$out/reference.json" "$tree/reference.json" >/dev/null \
          && diff -rq "$out/files" "$tree/files" >/dev/null \
          && diff -q "$out/titles.tsv" "$PWD/artifacts/dm-audit/titles.tsv" >/dev/null; then
          echo "audit-freeze verify: the reference is the one main $(git rev-parse HEAD) writes."
        else
          echo "audit-freeze verify: the reference differs from the one main $(git rev-parse HEAD) writes:" >&2
          diff -rq "$out/files" "$tree/files" >&2
          diff -q "$out/reference.json" "$tree/reference.json" >&2
          diff -q "$out/titles.tsv" "$PWD/artifacts/dm-audit/titles.tsv" >&2
          exit 1
        fi ;;
      *)
        echo "Use: audit-freeze counts|journeys|reference|add|verify [label]" >&2
        exit 2 ;;
    esac
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
