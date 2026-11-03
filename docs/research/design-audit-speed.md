# Faster design-audit iterations

2026-11-03. Base: `cff90a3e7ced12d1b17cc2beb1e49377fd12b685` (main).
Machine: 16 cores, 31 GiB RAM, btrfs; Rust 1.95.0, devenv 2.2.2.
The existing pinned devenv input is 2.2.1; no lockfiles were changed.

## Changes kept

| Change | Before median | After median | Reduction |
| --- | ---: | ---: | ---: |
| Splat test rebuild, dependencies warm | 24.21 s | 18.56 s | 23% |
| First check, test-only / test + Clippy seed | 40.11 s | 15.97 s | 60% |
| Compare 64 fixed captures, same 10 differences | 18.84 s | 6.23 s | 67% |
| Contact-sheet stage / omit optional sheet | 68.61 s | 0.65 s | 99% |

These are separate stages, not an end-to-end speedup estimate. Each category used
before, after, before, after and the median of two runs per state. Other sessions
started builds during later measurements; the run table marks every detected
overlap. The seed and comparison medians include contended runs and are not
isolated-machine estimates.

- **Test debug information:** only `[profile.test.package.splat]` changes to
  `debug = "line-tables-only"`. File/line backtraces remain. Normal app builds retain
  full debug information. Assertions, overflow checks, optimization levels and
  incremental compilation remain unchanged. Use
  `cargo test --locked --config 'profile.test.package.splat.debug=2'` for test locals.
  Maximum child compiler RSS medians fell from 1.66 to 1.39 GiB. Each measured
  rebuild first removed only Splat's artifacts; dependency artifacts stayed warm.
  Source was held constant within each debug-information pair.
- **Seed both artifact modes:** `devenv shell -- audit-warm` runs the exact test
  build and Clippy gate before the lead reflinks `target/`. It runs
  `cargo test --locked --no-run && check`. This is preparation, not a replacement
  for the full tests or capture gate. For each seed measurement pair, restore the
  same test-only `target/debug` snapshot, run `check`, remove only Splat artifacts
  with `cargo clean --package splat`, then run `check` again. Thus the after run
  rechecks Splat, as a new/edited worktree must, while retaining dependency metadata.
  The cost of warming is paid once by the lead; it is not free work.
- **Compare equal images quickly:** `differing_pixels` returns `(0, 0)` when the
  decoded RGBA images are equal. Both files are still decoded and validated.
  Unequal images use the original channel-by-channel check, with the original
  tolerance. The fixed 64-image benchmark includes every changed PNG hash in the
  chosen pair. Both versions reported the exact same ten differences, pixel
  counts, maximum gaps and failure status 101. It measures preserved failure
  reporting, not a newly passing comparison.
- **Optional contact sheet:** all source captures, reports and manifests are still
  made. `visual-test` omits montage by default. Set `SPLAT_VISUAL_CONTACT_SHEET=1`
  to request it, or run `visual-contact-sheet <label>` later without recapturing.
  The table times the legacy montage stage on a full 938-PNG run versus its omitted
  branch, including a devenv entry in each. It does not time skipping captures.
  ImageMagick scratch storage is explicitly `target/imagemagick`, not `/tmp`.

## Fidelity and limits

The scenario list, scenario scripts, driver, all window sizes, ergonomics baseline,
checks, video default and test scheduling are unchanged. The merge gate remains
`cargo test --locked`, `check`, a full `visual-test` and human capture review.
`--only` is for iteration feedback and does not replace the full merge gate.

The unmodified baseline is not pixel-identical to itself: its two full runs have
35 changed PNG hashes, including visible random UUIDs in copied content. The
existing comparison fails above its unchanged tolerance of 2. For example,
[the content-copy implementation](../../src/content_workspace.rs) calls
`Uuid::new_v4()`, and the generated ID appears in the Markdown editor capture.
No new tolerance, masking or reduced coverage is introduced here. This PR does
not claim byte-identical PNGs or fix that existing reproducibility issue.
The full ergonomics JSON and Markdown reports are byte-identical between baseline
runs, the scheduling trials, and the final retained implementation.

Final validation passed: `audit-warm` (test compilation plus the exact fmt/Clippy
gate), `cargo test --locked` (187 lib + 472 bin tests passed, 11 existing ignored
tests unchanged), and `SPLAT_VISUAL_VIDEO=0 visual-test audit-final` (42 scenarios,
938 PNGs, all usual sizes). Both final ergonomics reports match the baseline byte
for byte. `git diff --check` passed. Final commands overlapped other sessions;
the raw run metadata records those overlaps too.

## All retained measurements

Loads are the literal 1/5/15-minute load averages from `uptime`, recorded at each
run's start and end. Toronto was UTC-5 on this date. Timings include devenv startup
and a 250 ms completion poll. One benchmark/build of ours ran at a time. Existing
builds from other sessions could not be paused. The monitor sampled Cargo,
compilers and Splat processes, excluding this run's descendants and sccache
workers whose output directory is this worktree's `target/`.

| Run, in pair order | Seconds | Start (Toronto) | Start load 1/5/15 | End load 1/5/15 | Other build |
| --- | ---: | --- | --- | --- | --- |
| build-before-1 | 24.443 | 01:44:24 | 19.29, 18.43, 17.13 | 13.59, 17.15, 16.74 | No |
| build-after-1 | 18.288 | 01:44:58 | 11.50, 16.59, 16.56 | 9.24, 15.84, 16.31 | No |
| build-before-2 | 23.983 | 01:46:51 | 6.79, 13.24, 15.33 | 5.57, 12.44, 15.01 | No |
| build-after-2 | 18.823 | 01:47:27 | 4.71, 12.03, 14.85 | 4.00, 11.39, 14.58 | No |
| check-before-5 | 37.244 | 01:58:45 | 23.26, 16.21, 14.55 | 28.44, 19.01, 15.60 | **Yes** |
| check-after-5 | 14.563 | 01:59:23 | 28.44, 19.01, 15.60 | 24.61, 18.50, 15.47 | **Yes** |
| check-before-6 | 42.967 | 01:59:58 | 16.54, 17.10, 15.08 | 27.04, 20.37, 16.31 | **Yes** |
| check-after-6 | 17.367 | 02:00:42 | 27.04, 20.37, 16.31 | 22.67, 19.77, 16.19 | **Yes** |
| compare-before-1 | 14.100 | 01:53:08 | 2.17, 9.12, 13.07 | 1.99, 8.85, 12.94 | No |
| compare-after-1 | 6.561 | 02:01:05 | 21.33, 19.54, 16.14 | 22.11, 19.73, 16.22 | **Yes** |
| compare-before-2 | 23.584 | 02:01:12 | 22.11, 19.73, 16.22 | 26.90, 21.07, 16.76 | **Yes** |
| compare-after-2 | 5.906 | 02:01:36 | 26.90, 21.07, 16.76 | 25.23, 20.82, 16.70 | **Yes** |
| sheet-before-1 | 68.824 | 02:01:58 | 20.28, 19.98, 16.51 | 29.99, 22.30, 17.51 | **Yes** |
| sheet-after-1 | 0.599 | 02:03:07 | 29.99, 22.30, 17.51 | 30.79, 22.60, 17.63 | **Yes** |
| sheet-before-2 | 68.399 | 02:03:08 | 30.79, 22.60, 17.63 | 25.62, 22.79, 18.04 | **Yes** |
| sheet-after-2 | 0.695 | 02:04:16 | 25.62, 22.79, 18.04 | 25.62, 22.79, 18.04 | **Yes** |

Raw commands, stdout/stderr, uptime and detected overlaps are in this worktree's
ignored `artifacts/screenshots/audit-speed/*.log` and `*.json`; no build output or
large files were put in `/tmp`. The original 135.6 s preparation test run overlapped
other work and is excluded. Initial Clippy warmups and monitor-development trials
are also excluded. The source images remain under `artifacts/screenshots/` and
all build binaries/snapshots remain under `target/`.

## Ideas not kept

Effort and possible gains below are estimates unless explicitly measured.

| Idea | Evidence / possible gain | Follow-up effort |
| --- | --- | --- |
| Bound live visual scenarios | Already parallel: 42 scenario threads. Dynamic limit 16 measured 44.99 -> 39.04 s and peak child RSS 17.87 -> 13.97 GiB on full 938-capture runs. Reverted: baseline nondeterminism prevented proving identical pixels. | 1–2 hours to make visible random fixture values repeat, then repeat paired captures before changing scheduling. |
| Batches of eight scenarios | Rejected: 61.86 s wall versus original 44.31 s in the exploratory pair; batch barriers waste available work. Ergonomics reports still matched. | No further work on fixed batches. |
| Change linker | This checkout already uses wild on Linux x86_64 and mold on other Linux hosts, through `nix/rust-toolchain.nix`. No measured reason to switch to mold/lld here. | 30–60 minutes for isolated linker trials; expected local gain small/unknown. |
| Split debug information / tune codegen units | Dev/test incremental compilation and 256 codegen units are already defaults. Test-only line tables remove much of the relevant debug work. Further gain could be 0–20% of the remaining build stage. | 30–60 minutes per setting plus full gates; check debugger behavior and ARM support. |
| Tune Mesa `LP_NUM_THREADS` | Could reduce rasterizer oversubscription when several implementers capture at once; possible 0–30% capture gain, unmeasured. | 30–60 minutes plus complete image comparisons after reproducibility is fixed. |
| Share parsed immutable SRD data or render resources across scenarios | Could remove repeated allocation/parsing and several GiB of live data; possible 10–20 s per capture run, unmeasured. Cache lifetime and scenario isolation need proof. | Half a day to a day. |
| Reuse one entered devenv shell | Observed entry/command overhead is often under one second; useful across many tiny fmt commands, not the main build cost. | 5–15 minutes of workflow changes, no gate removal. |

`visual-test --only` already filters scenarios before building their worlds, runs
only the bin's visual suite and skips montage. Skipping additional states, sizes,
checks or PNGs would reduce fidelity and was not implemented. Full videos remain
on by default; benchmark captures used the existing `SPLAT_VISUAL_VIDEO=0` switch
because the requested fidelity contract covers PNGs and ergonomics, not video.

For early errors, run `check` after a group of edits, before the full tests.
`cargo check --locked --tests` is another feedback tool, but omits code generation
and cannot replace tests or the exact warnings-denied Clippy gate. A Clippy/check
metadata build is not a test executable: build-script/proc-macro artifacts can be
shared where compatible, but matching profiles alone cannot make check output
serve as a linked test binary. Retain both artifact kinds in the seed and give
all implementers private target directories to avoid lock waits. Limiting Cargo
jobs per implementer may help contention, but no global job cap was imposed.

## Primary sources

- [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html): test
  inheritance, package overrides, debug modes, incremental and codegen defaults.
- [rustc code generation options](https://doc.rust-lang.org/rustc/codegen-options/index.html):
  debug information and split debug information.
- [Cargo check](https://doc.rust-lang.org/cargo/commands/cargo-check.html): metadata
  checking omits code generation and does not find every code-generation error.
- [Clippy usage](https://doc.rust-lang.org/clippy/usage.html): warnings-denied gate;
  this repository remains pinned to Rust 1.95 rather than adopting newer Cargo options.
- [Mesa environment variables](https://docs.mesa3d.org/envvars.html): rasterizer
  threading options require measurement and complete capture validation.
- Repository sources: [toolchain](../../nix/rust-toolchain.nix),
  [scripts](../../devenv.nix), [visual suite](../../src/visual_tests/mod.rs),
  [driver](../../src/visual_tests/driver.rs),
  [comparison](../../src/visual_tests/compare.rs), [profiles](../../Cargo.toml).
