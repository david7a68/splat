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


## Second scan: iteration 2 feedback and partial runs

2026-11-03. Base: `c5b8998dd744c8ebdd5b49ca71f91324044b1a26` (main,
after PRs 44–47). Same 16-core, 31 GiB machine and pinned toolchain. This scan
keeps the previous scheduling experiment reverted. The main opportunity is to
remove redundant feedback runs; all merge gates remain required.

### Changes

- **Show all measured baseline changes before failing.** Regressions and
  improvements now print before scenario, stale-allowance or regression assertions.
  Previously a failed scenario or stale allowance could hide a second class of
  failure in terminal output. Both report files already contained both lists.
- **Collect text mismatches explicitly.** `SPLAT_VISUAL_DIAGNOSE=1` makes
  `Driver::expect` record missing text and continue. The suite still fails, even
  with report enforcement or the full baseline updater enabled. Normal mode still
  stops at the first missing phrase. Missing controls, raw assertions and async
  timeouts still stop a scenario; continuing after those could drive the wrong UI.
  This finds all reached text checks in one run, not checks after an unreachable
  action. The diagnostic mode cannot produce an eligible baseline observation
  when any expected text was missing.
- **Apply saved chrome counts without another visual run.** An eligible run writes
  `chrome-observation.json` beside its reports. After reviewing the changes, run
  `devenv shell -- visual-baseline-chrome <label>`. The helper updates only the
  selected scenarios' three `/egui-chrome` checks. Zero counts remove their old
  allowances. App checks, workflow allowances, task costs and unselected scenarios
  retain their counts. It requires the baseline on disk to equal the one used by
  the run. A started run removes its previous observation before selection or
  capture cleanup. Failed scenarios, missing full-suite goals, workflow failures
  and non-chrome regressions prevent observation creation. Unchanged baseline
  bytes are not rewritten, so a no-op refresh does not invalidate `include_str!`.
  This command applies measurements; it does not certify later source edits.
  Run every full gate on the final source after an actual baseline change.
- **Cache generated fixture PNG bytes.** Module art and character portraits are
  pure functions of width, height and colour. A process-local cache uses all three
  inputs as its key. Each scenario still writes independent files into its own
  world/cache folders; only immutable encoded bytes are shared. Tests compare
  these bytes against fresh generation and check that editing one world's file
  does not alter another world. Screenshot rendering, encoding and sizes are
  unchanged.
- **Expose stage costs.** `SPLAT_VISUAL_TIMINGS=1` prints lock wait, fixture setup,
  app/fonts/renderer setup, settling, PNG rendering and PNG encoding/writing.
  These are per-scenario wall times, with overlapping work; they must not be added
  as if they were serial stages. SRD work can be inside fixture/setup/settling.

Suggested feedback sequence:

```sh
# One full feedback run exposes every reached text mismatch and baseline delta.
SPLAT_VISUAL_DIAGNOSE=1 devenv shell -- cargo test --locked
# Or capture affected scenarios while working.
SPLAT_VISUAL_DIAGNOSE=1 devenv shell -- visual-test work --only party,settings
# Review the reports and captures, fix text expectations, then apply chrome counts
# from an eligible run. App/workflow/task failures need fixes, not new allowances.
devenv shell -- visual-baseline-chrome work
# The final full merge gates are still required.
devenv shell -- cargo check --locked --tests
devenv shell -- cargo test --locked
devenv shell -- check
devenv shell -- visual-test final
```

### Measurements and limits

| Stage or workflow | Before median | After median | Interpretation |
| --- | ---: | ---: | --- |
| Refresh chrome counts from an already completed run | 38.20 s | 0.10 s | Removes a redundant full visual run; linked test execution plus devenv, build excluded. Final merge gates still run. |
| Complete 965-PNG run, no video | 42.95 s | 43.08 s | Effectively unchanged (+0.3%); no claim of a full-suite wall-time gain. |
| Two-scenario feedback, existing video-on / PNG-only option | 16.01 s | 14.64 s | Existing switch, not a new default; same 60 PNGs and checks. |
| Reached missing text checks exposed per controlled failing run | 1 | 2 | Opt-in diagnosis; all four runs still failed with status 101. |
| Generate fixture PNG bytes, 39 worlds × 6 inputs, writes excluded | 4.04 s | 0.20 s | Fresh process per run, same 1,499,472 returned bytes; this removes duplicate work, not screenshot captures. |

These rows are separate stages and workflows, not additive iteration savings.
A real baseline edit still requires a rebuild and every full final gate. The
refresh comparison reused the same valid measurements and unchanged counts to
isolate redundant remeasurement versus applying saved data. The observed chrome
baseline was unchanged in this PR. The final CLI no-op check also verifies that
applying unchanged counts preserves the baseline's modification time.

All paired measurements alternate before, after, before, after. Each median uses
two runs per state. Only one of our builds/benchmarks ran at once. A 250 ms
process monitor recorded Cargo, rustc, devenv, Splat and ffmpeg processes. The
run table below marks other builds. Reviewers continued reading PNGs. These are
measurements with no detected competing build, not a reproduction of five
implementers rebuilding at once. They cannot promise the same gains under that
load. Linked before/after test executables were saved under this worktree's
`target/audit-speed/`, then run through the same devenv shell. Compilation is
excluded from those paired rows. Raw commands, timestamps, process samples,
stdout and stderr are under ignored `artifacts/screenshots/audit-speed-2/`.
Build output stayed in `target/`; no build output or large files went into `/tmp`.

For `party,settings` before fixture caching, the first PNG-only run took
14.72 s wall / 14.54 s inside the suite. Outside-test overhead was about 0.18 s,
including devenv entry, process start and process exit. Each scenario's fixture
took about 1.2 s; combined app/font/renderer setup took 41–64 ms. Party spent
6.05 s settling, 0.49 s rendering PNGs and 0.55 s encoding/writing PNGs; Settings
spent 12.23 s settling, 0.43 s rendering and 0.46 s encoding/writing. Those
per-scenario stages overlap with the other scenario. Font bytes are embedded;
there is no font-file download. SRD and definition entries are parsed once per
process, then reused. The remaining time includes interaction, checks and frame
work. These timers do not isolate SRD parsing or each individual window size.
All existing size plans, including HiDpi captures, remain in the measured totals.

Optional video added only about 1.36 s here. It does not explain a 60–137 s
partial run by itself. `world_root` instead holds `/tmp/splat-visual-world.lock`
for the entire suite. The pinned shell leaves `TMPDIR` unset, so the worktrees
share that path. No lock wait was significant in these idle-build measurements;
under five implementers, waiting behind one 38–43 s full suite is a plausible
large fixed cost. This scan measures the wait but does not remove the lock,
change fixture paths or claim a measured five-way speedup. Reducing settle
frames, repaint checks, states or sizes was not attempted.

| Run, in pair order | Seconds | Start (Toronto) | Other build | Exit |
| --- | ---: | --- | --- | ---: |
| retained-before-1 | 43.408 | 05:36:31 | No | 0 |
| retained-after-1 | 43.221 | 05:37:14 | No | 0 |
| retained-before-2 | 42.497 | 05:37:58 | No | 0 |
| retained-after-2 | 42.946 | 05:38:40 | No | 0 |
| refresh-before-1 | 38.332 | 05:28:47 | No | 0 |
| refresh-after-1 | 0.109 | 05:29:26 | No | 0 |
| refresh-before-2 | 38.072 | 05:29:26 | No | 0 |
| refresh-after-2 | 0.096 | 05:30:04 | No | 0 |
| fixture-before-1 | 4.052 | 05:42:22 | No | 0 |
| fixture-after-1 | 0.197 | 05:42:26 | No | 0 |
| fixture-before-2 | 4.036 | 05:42:26 | No | 0 |
| fixture-after-2 | 0.198 | 05:42:30 | No | 0 |
| small-video-1 | 15.983 | 05:30:13 | No | 0 |
| small-png-1 | 14.720 | 05:30:29 | No | 0 |
| small-video-2 | 16.036 | 05:30:44 | No | 0 |
| small-png-2 | 14.569 | 05:31:00 | No | 0 |
| fault-normal-1 | 8.268 | 05:35:54 | No | 101 |
| fault-diagnose-1 | 8.419 | 05:36:02 | No | 101 |
| fault-normal-2 | 8.327 | 05:36:10 | No | 101 |
| fault-diagnose-2 | 8.273 | 05:36:19 | No | 101 |

Exploratory diagnostics-only and module-art-only pairs were superseded by the
retained implementation's pairs above. Their logs remain in the same folder.
An initial isolated fixture probe had the wrong image-count assertion (five
instead of six); those failed probes are excluded, and its corrected paired
runs use two module images plus four portraits.

### Fidelity and verification

Final retained-source validation passed:

- `cargo check --locked --tests` and `check` (fmt plus warnings-denied Clippy).
- `cargo test --locked`: 187 library and 486 binary tests passed. The 11 existing
  ignored tests remain unchanged. The new ignored baseline command was exercised
  explicitly through `visual-baseline-chrome`.
- `SPLAT_VISUAL_VIDEO=0 visual-test audit-speed-final`: all 42 scenarios and 965
  PNGs, with the original 960x640 / 1200x800 / 1920x1080 and HiDpi capture plans.
- A 60-PNG `party,settings` partial run and the actual CLI chrome updater passed.
  An unchanged update preserved both baseline bytes and modification time.
- Stale-baseline, incomplete-run and invalid-selection probes were rejected.
  Diagnostic mode also remained a failure in report mode. None of these negative
  probes wrote baseline counts or left an eligible observation.
- Tests verify exact generated PNG bytes, independent world files, selected-only
  chrome changes, zero-count removal, unchanged app/workflow/task allowances,
  accumulated text failures and normal mode's existing immediate panic.
- Baseline counts, scenario scripts, capture scheduling, render/PNG encoders,
  window sizes, gate commands, Cargo profiles and lockfiles have no final diff.
  `git diff --check` passed.

All four retained paired full runs have identical capture names and dimensions
and byte-identical ergonomics JSON/Markdown reports. The final real `visual-test`
reports also match the before run byte for byte. Existing image nondeterminism
remains: before/before has 35 changed PNG hashes (8 above the existing tolerance),
after/after 30 (9 above), and before/after 16 (8 above). The larger differences
are in copied-content states and a conflict note; the other differences have a
maximum channel gap of one. The previous scan already recorded this problem;
`content_workspace::CopyPlan::prepare` still uses `Uuid::new_v4()`. No masking, tolerance
increase, scheduling change or reduced capture coverage was introduced. This
scan therefore proves exact fixture PNG inputs and matching reports/inventories,
not byte-identical screenshots from an app that already varies between runs.
Raw file-by-file comparisons are in `audit-speed-2/fidelity.json`.

The fixture microbenchmark calls the actual old generator or the new cache,
234 times in a fresh process. It includes a cold cache in each after run. Its
in-test medians are 3.93 s and 0.10 s; the table includes shell/process overhead.
It excludes file writes and overlapping scenario work, which is why its gain
must not be presented as a four-second full-suite wall-time gain. The temporary
ignored benchmark and injected failing phrases were restored before commit;
their source and logs are kept only under ignored screenshot artifacts.

### Larger ideas not implemented

These are estimates, not measured gains from this scan.

| Idea | Likely opportunity | Estimated effort |
| --- | --- | --- |
| Isolate visual worlds while retaining their visible absolute paths | `world_root` holds one exclusive lock for the complete suite. Five worktrees queue on it even with private `target/` directories. Removing a queued full run can save roughly one suite duration per waiter; changing paths directly changes captures. Linux mount namespaces could isolate the same paths, but need portability, cleanup and capture proof. | 0.5–1 day |
| Cache a verified immutable catalogue or parts of seeded worlds | SRD and definition entries already use `OnceLock`; blindly adding another parsed-SRD cache will not help. Rebuilding catalogues, fixtures and app state remains a candidate. Possible 1–10 s per full run; not isolated here. | 0.5–1 day |
| Fix remaining UUID/display nondeterminism, then revisit bounded scheduling | The prior limit-16 trial saved about 6 s, but was reverted because identical pixels could not be proved. Keep it reverted until all visible generated values repeat. | 2–4 h for reproducibility investigation, then repeat the paired scheduling study |
| Reuse software render resources between scenarios | Setup is small in a two-scenario run but grows under 42-scenario load. Shared textures/devices need isolation proof. Possible 0–3 s per run, unmeasured. | 0.5 day |
| Tune PNG filters or compression | The image crate already defaults to Fast compression with Adaptive filtering. Lossless alternatives might save a fraction of PNG encoding time, but would change file hashes and sizes. No encoder or tolerance change was made. | 1–2 h plus full decoded-image comparisons |
| Limit Cargo jobs across five implementers | May reduce compile contention; this scan ran while reviewers, rather than implementers, were active. No evidence here for a specific cap. | 30–60 min of controlled five-worktree trials |

Source evidence: [suite and shared lock](../../src/visual_tests/mod.rs),
[driver and timings](../../src/visual_tests/driver.rs),
[baseline selection and comparison](../../src/visual_tests/ergonomics.rs),
[fixtures](../../src/visual_tests/fixture.rs),
[embedded SRD cache](../../src/content/builtin.rs),
[definition cache](../../src/content/definitions.rs),
[fonts](../../src/design/theme.rs),
[video pipeline](../../src/visual_tests/video.rs), and
[shell commands](../../devenv.nix). The linked primary Cargo/Mesa documentation
above still applies; no compiler, renderer, job limit or dependency version changed.
