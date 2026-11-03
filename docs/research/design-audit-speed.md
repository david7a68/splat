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

## Third scan: one full gate per final source

2026-11-03. Base: `c7af1f328cb48f286ba8b83b894a099bb1aaec7f` (main,
after PRs 49–52). Same 16-core, 31 GiB machine, Rust 1.95.0 and devenv
2.2.2; pinned inputs and lockfiles remain unchanged. The suite has 51 scenarios
and 1,063 captures.

### What the iteration-3 timings show

The [PR 49](https://github.com/david7a68/splat/pull/49),
[PR 50](https://github.com/david7a68/splat/pull/50),
[PR 51](https://github.com/david7a68/splat/pull/51), and
[PR 52](https://github.com/david7a68/splat/pull/52) bodies were read with
`gh pr view <n> --json number,url,body`. Their complete JSON is saved in the ignored scan
logs. These are reported historical wall times, not controlled benchmarks:

| PR | Evidence | Implication |
| --- | --- | --- |
| 49 | Full capture 238 s; one-scenario refresh 120 s; full tests after the PR-50 merge 101 s. The body identifies shared-fixture lock waits. | A small selection does not avoid the suite-wide lock. |
| 50 | Full refresh 268 s; later full tests 142 s, then 57 s after the assertion fix. The body identifies a shared-lock queue. | Waiting and failed expectations dominate some loops, not shell entry. |
| 51 | After main merges: full captures 107 s and 62 s; full tests 73 s. Final tests 67 s plus final captures 91 s. One targeted 191 s run had an accessibility expectation error as well. | Do not label every long run a lock wait. Full tests and captures also repeat the same scenario scripts. |
| 52 | First full capture 243 s, with the global lock owner checked. Later full captures 59–106 s; tests after successive merges 70, 145 and 62 s. Some lint runs waited on the local Cargo lock. | Private targets help Cargo isolation but do not isolate visual fixtures. |

Each incoming main merge changes the source under test. A previous gate cannot
certify that new source. No gate is skipped after a merge, and this change adds
no result cache. The cheap opportunity is to perform the required checks once
in each gate, rather than run the visual scenario scripts twice.

### Change retained

`devenv shell -- audit-gate <label>` runs the exact `check` command, then
`cargo test --locked -- --nocapture` with the same capture directory and video
settings as `visual-test`. The full Cargo test invocation includes library,
binary and documentation tests. Its existing `visual_suite` test renders every
capture while running every scenario and ergonomics check. There is no test-name
filter. The separate full-test, check and visual-test commands remain available.
Human review of all captures remains required before merging.

The command clears inherited `SPLAT_VISUAL_SCENARIOS` and
`SPLAT_VISUAL_UPDATE_BASELINE`, and forces baseline enforcement. It validates
labels before running checks. A lint failure stops before tests; a test or capture
failure retains its nonzero exit status. Video still defaults to on. The existing
PNG-only option and optional contact-sheet behavior remain available. No Rust
source, baseline, scenario, frame count, size plan, encoder, renderer, tolerance,
profile or dependency changes.

### Measurements and fidelity

| Workflow | Before median | After median | Difference |
| --- | ---: | ---: | ---: |
| Full merge gate, warm artifacts, PNG-only | 118.54 s | 69.23 s | −49.30 s (41.6%) |
| Complete visual scenario executions per gate | 2 | 1 | All 51 scenarios still run with captures. |
| Shared fixture lock acquisitions per gate | 2 | 1 | Lock and fixture paths are unchanged. |

Before: `check && cargo test --locked -- --nocapture && visual-test <label>`.
After: `audit-gate <label>`. Both ran through a fresh `devenv shell --`, with
`SPLAT_VISUAL_VIDEO=0` and `SPLAT_VISUAL_TIMINGS=1`. Video remains on by default;
this fidelity contract covers PNGs and ergonomics. Contact sheets remain optional.
The table measures a complete warm merge gate, not a faster standalone capture
command. The removed work is the redundant no-capture scenario pass.

Order was before, after, before, after; medians use two runs per state. A host
process monitor sampled Cargo, compilers, linkers and Splat every 250 ms, excluding
our descendants. No competing build/test process was detected during these four
retained measurements. Seven reviewers were reading PNGs. Loads below are literal
1/5/15-minute values; high end loads include our own 51 scenario threads. Times
include shell entry and completion polling, but exclude seed preparation and
compilation warmup. The original pinned toolchain and private target were warm.
These are local results, not a promise under several competing implementers.

| Run, in pair order | Seconds | Start (Toronto) | Start load 1/5/15 | End load 1/5/15 | Other build/test | Exit |
| --- | ---: | --- | --- | --- | --- | ---: |
| before-1 | 119.341 | 06:43:21 | 2.25, 4.73, 9.54 | 31.56, 13.83, 12.22 | No | 0 |
| after-1 | 69.750 | 06:45:20 | 31.56, 13.83, 12.22 | 36.39, 18.52, 13.93 | No | 0 |
| before-2 | 117.729 | 06:46:30 | 36.39, 18.52, 13.93 | 34.91, 22.73, 15.94 | No | 0 |
| after-2 | 68.718 | 06:48:28 | 34.91, 22.73, 15.94 | 34.29, 24.89, 17.16 | No | 0 |

All four runs passed the same 695 tests (187 library + 508 binary), with the same
12 opt-in tests ignored; binary/doc targets with no tests also completed. The
before gate additionally ran the same `visual_suite` test again through
`visual-test`. A comparison of every named test/status confirms that only this
duplicate execution is removed. Every run captured all 51 scenarios and the
same 1,063 names and dimensions: 431 at 960×640, 429 at 1200×800, 186 at
1920×1080, and 17 at 2400×1600 (the unchanged 1200×800 HiDPI plan). Both ergonomics
reports and `chrome-observation.json` are byte-identical across all four runs.
The last after run inherited an invalid scenario selection, report enforcement
and baseline-update mode; the gate cleared them and still passed the full suite
without changing baseline bytes.

The app still does not produce byte-identical screenshots from unchanged code.
The comparison uses the existing channel tolerance of 2, without masks or new
allowances:

| Capture pair | Changed PNG hashes | Above existing tolerance |
| --- | ---: | ---: |
| before-1 / after-1 | 122 | 13 |
| before-2 / after-2 | 29 | 12 |
| before-1 / before-2 | 54 | 7 |
| after-1 / after-2 | 95 | 18 |

Every above-tolerance before/after file also differs in at least one of the two
same-version repeat comparisons. Variations include copied-content IDs and
session-note/recording states; the remaining causes were not diagnosed in this
quick scan. Thus tests, check results, reports and capture coverage match exactly;
byte-identical pixels cannot be promised even for the unmodified baseline. No
Rust source, renderer, scenario, tolerance or capture scheduling change is made.

Guard probes confirmed invalid labels/extra arguments exit 2, a lint error stops
before tests, test errors propagate, and inherited feedback settings are cleared.
The generated command still defaults video to 1. Nix formatting and
`git diff --check` passed. Raw commands, all output, process samples, run metadata,
file-by-file image comparisons and probes are under ignored
`artifacts/screenshots/audit-speed-3/`; captures are in the four sibling
`audit-speed-3-{before,after}-{1,2}` label folders. All build output stayed under this
worktree's `target/`. No build output or new large artifacts went into `/tmp`.

A later alternative wrapper, which deferred only the no-capture suite and then
called standalone `visual-test`, was not retained. Its completed trials overlapped
other builds (122.748 s before / 78.781 s after; **other build: Yes for both**).
The next trial queued behind an iteration-4 diagnostic run for over three
minutes. `/proc/locks` confirmed another worktree as owner and this worktree plus
a third as waiters. Only our extra benchmark processes were stopped; no other
worktree was interrupted. These incomplete/contended alternative trials are
excluded from the retained medians; their logs and metadata remain under
`standalone-trial/`, with live lock evidence alongside them.

### Shared fixture lock: keep it shared for now

[`world_root`](../../src/visual_tests/mod.rs) locks
`/tmp/splat-visual-world.lock` before deleting/recreating the shared
`/tmp/splat-visual-world`, and holds it until the suite returns. The shell leaves
`TMPDIR` unset. All worktrees therefore contend on the same lock, including the
visual run inside ordinary `cargo test`. PR 52 checked the blocking owner;
PRs 49 and 50 also report queue waits. This is an observed cross-worktree problem.

Making only the lock per-worktree would allow a second process to delete files
that the first process is using. Moving the root per-worktree, or setting a
worktree-specific `TMPDIR`, would also change the campaign/module paths shown
in the UI. The ergonomics elision allowance names the existing absolute root
in [`ergonomics.rs`](../../src/visual_tests/ergonomics.rs). Such a change is not
safe under the requirement to retain the same captures and checks.

A two-process diagnostic used the same linked test executable with separate
output folders and the `party` scenario: both passed with 32 PNGs and the same
checks. The first held the fixture for 8.86 s; the second reported an 8.85 s lock
wait. The diagnostic overlapped the image-comparison task, not a build, and is
not included in the before/after benchmark. It confirms serialization rather
than estimating concurrent throughput. The retained command instead acquires the shared fixture lock once per full gate, rather than
twice. It does not make capture runs concurrent.

### Larger ideas not implemented

These are estimates, not additional measured savings.

| Idea | Expected opportunity | Effort estimate |
| --- | --- | --- |
| Isolate fixtures at the same visible absolute paths with Linux mount namespaces | Remove suite-sized waits when worktrees capture at once. Must prove namespace permissions, path identity, cleanup, character-cache isolation and non-Linux behavior; concurrent rasterizers can still compete for CPU/RAM. | 0.5–1 day, then full paired capture validation. |
| Fix remaining visible random IDs, then repeat the bounded-scheduling trial | Earlier limit-16 experiment saved about 6 s; current copy paths still use `Uuid::new_v4()`. No scheduling change belongs in this small gate wrapper. | 2–4 h for determinism, then 1–2 h for alternating full trials. |
| Integrate packages in an agreed order before each final gate | Reduce successive merge-and-retest rounds, while still running the full gate on each PR's final merged source. Cannot reuse old-source results after another main merge. | 30–60 min of workflow planning per audit; savings depend on landing order. |
| Cache more immutable fixture/catalogue work | Fixture timers include substantial work across 51 overlapping scenarios. SRD parsing and generated fixture PNG bytes are already cached; measure the remaining work before adding caches. | 0.5–1 day; possible 1–10 s per suite, unmeasured. |
| Tune rasterizer threads or Cargo jobs under several implementers | Could reduce oversubscription, but idle-build runs cannot select a useful cap. Preserve every capture and check. | 1–2 h of controlled concurrent trials and full image comparisons. |

Sources: the four PR bodies above; the existing primary Cargo documentation linked
in the first scan; [`devenv.nix`](../../devenv.nix),
[`visual_suite`](../../src/visual_tests/mod.rs),
[`Driver::state_prepared` and `capture`](../../src/visual_tests/driver.rs), and
[`hermetic.rs`](../../src/hermetic.rs). The driver checks all size plans regardless
of capture mode, then renders when output is enabled; the combined command uses
that unchanged capture path.

## Fourth scan: compare captures in parallel

2026-11-03. Base: `5c176b4551e9a73a7a23277578b69703fd1f4232` (main,
after PRs 54–56). Same 16-core, 31 GiB machine, Rust 1.95.0 and devenv
2.2.2. The suite now has 54 scenarios and 1,110 captures. Pinned inputs,
lockfiles, profiles and gate commands are unchanged.

### What iteration 4 spent time on

The bodies of [PR 54](https://github.com/david7a68/splat/pull/54),
[PR 55](https://github.com/david7a68/splat/pull/55) and
[PR 56](https://github.com/david7a68/splat/pull/56) were read with
`gh pr view <n> --json body,url,headRefOid`. These are historical observations,
not controlled measurements:

| PR | Reported time | Cause or implication |
| --- | --- | --- |
| 54 | Final gates 86 and 314 s; partial run 263 s; three later diagnostic runs 60–61 s | The final gate queued on the shared fixture lock. The early partial run also had a wrong-case painted-text wait that reached its 180 s timeout. |
| 55 | Targeted runs 203, 46, 68 and 267 s; later gates 74–82 s | The long targeted runs queued on the fixture lock. Combining tests and captures already removed the duplicate scenario pass. |
| 56 | Gate 263 s, then 149 s; passing gates 113, 77 and 84 s | The 263 s failure waited for an accessible name as painted text; the 149 s gate included a fixture-lock wait. Fixes and main merges still required complete final gates. |

Most remaining large costs need the larger work listed below. This scan found one
small saving in capture comparison, outside rendering and the merge gate.

### Change retained

[`compare`](../../src/visual_tests/compare.rs) now uses the existing Rayon
dependency to decode and compare independent image pairs in parallel. It collects
small results in indexed filename order, then emits the same ordered report. It
still decodes both images even when their PNG bytes match, checks dimensions,
and uses the original per-channel tolerance of 2. Decode errors are retained as
results until collection completes, so the first error still names the first
invalid file in sorted order. Workers release each decoded pair before taking
another; decoded images for the whole suite are not collected in memory.

No app, renderer, PNG encoder, scenario, fixture, capture scheduling, size plan,
ergonomics baseline or merge-gate code changes. The full tests and every capture
remain required before merge. This is a faster optional `visual-compare` step,
not a faster `visual-test` or `audit-gate`, and not a replacement for human review.

### Alternating measurements

| Stage | Before median | After median | Saving |
| --- | ---: | ---: | ---: |
| Compare all 1,110 fixed capture pairs | 7.48 s | 2.61 s | 4.87 s (65.1%) |
| Pairs decoded per run | 1,110 | 1,110 | None skipped |
| Reported changed captures / exit status | 3 / 101 | 3 / 101 | Identical |

The fixed inputs are copies of PR 56's final `iter-4-B` capture set. The second
copy has one red-channel change of 64 in each of three files; all other bytes
are held fixed. Thus the benchmark checks preserved failure reporting as well as
the common equal-image path. Each version reports exactly the same three names,
one differing pixel per name, maximum gap 64, and failure status 101.

Every run decodes all 447 captures at 960×640, 445 at 1200×800, 201 at
1920×1080, and 17 at 2400×1600 (the existing HiDPI plan). The before executable
was saved before editing; the after executable was built with the unchanged
pinned toolchain and profiles. Both live under this worktree's `target/audit-speed-4/`.
The timer covers the linked comparison test and a 250 ms completion poll;
compilation and devenv entry are excluded. Each process still starts through a
fresh `devenv shell --`. These warm-filesystem measurements do not estimate
concurrent-implementer throughput or end-to-end audit savings.

| Run, in pair order | Seconds | Start (Toronto) | Start load 1/5/15 | End load 1/5/15 | Other build/test | Exit |
| --- | ---: | --- | --- | --- | --- | ---: |
| before-1 | 7.610 | 07:28:15 | 2.26, 12.74, 17.60 | 2.07, 12.35, 17.42 | No | 101 |
| after-1 | 2.636 | 07:29:16 | 2.25, 10.67, 16.55 | 2.25, 10.67, 16.55 | No | 101 |
| before-2 | 7.356 | 07:29:31 | 1.90, 10.32, 16.38 | 2.08, 10.08, 16.23 | No | 101 |
| after-2 | 2.582 | 07:29:42 | 2.07, 9.94, 16.15 | 2.07, 9.94, 16.15 | No | 101 |

Only one of our benchmarks/builds ran at a time. A host process monitor sampled
Cargo, rustc, Clippy, linkers, Splat and ffmpeg every 250 ms, excluding this run's
descendants. No competing build/test was detected in the four measurements;
seven reviewers continued reading PNGs. Load values are literal 1/5/15-minute
averages and include earlier machine activity.

Raw commands, stdout/stderr, process samples, run metadata, exact normalized
comparison reports and input dimensions are under ignored
`artifacts/screenshots/audit-speed-4/`. Fixed input copies also live there;
all build output stays in `target/`. No build output or large files were put in
`/tmp`. The small comparison unit-test fixtures use the existing `tempfile`
convention.

### Verification

The comparison tests cover equal pixels, within-tolerance noise, changed pixels,
missing files on either side, mismatched dimensions, sorted report order and
identical corrupt PNGs. The corrupt-file test confirms that matching file bytes
do not bypass PNG validation and that the first invalid filename remains stable.

Final validation passed: `SPLAT_VISUAL_VIDEO=0 SPLAT_VISUAL_TIMINGS=1 devenv
shell -- audit-gate audit-speed-4-final` ran formatting, warnings-denied Clippy,
all 706 tests (187 library + 519 binary; 12 existing ignored), documentation and
other binary targets, all 54 scenarios, and all 1,110 captures. The visual suite
reported 56.61 s; this is validation, not a paired capture-speed measurement.
Capture names and dimensions match the final PR-56 set. Both ergonomics reports
and `chrome-observation.json` are byte-identical to that set, and every manifest
SHA-256 was checked against its PNG. A separate complete equal-set comparison
passes with 1,110 pairs and no differences. `git diff --check` passed.

The comparison benchmark uses fixed existing images, so its equality claim is
exact and does not depend on the app's pre-existing screenshot nondeterminism
recorded in earlier scans. It does not claim that independent new app runs
produce identical PNG bytes.

### Larger ideas still requiring separate work

These are estimates, not measured gains from this scan.

| Idea | Opportunity | Effort estimate |
| --- | --- | --- |
| Isolate fixtures at identical visible paths with mount namespaces | Remove suite-sized lock queues seen in all three iteration-4 PRs; retain cache isolation, cleanup and non-Linux behavior. | 0.5–1 day plus full paired capture validation |
| Diagnose remaining visible nondeterminism, then bound scenario concurrency | The first scan's limit-16 trial saved about 6 s; it remains reverted because pixel fidelity was not proved. | 2–4 h for determinism, then 1–2 h for paired full trials |
| Cache more immutable catalogue/fixture work | Existing SRD parsing and generated fixture PNG caches already remove duplicate work. Remaining possible gain 1–10 s per suite, unmeasured. | 0.5–1 day |
| Tune rasterizer threads or Cargo jobs under concurrent implementers | Possible contention savings; idle CPU during review cannot establish useful limits. | 1–2 h plus complete image comparisons |
| Agree landing order before final gates | Reduce repeated merge-and-retest rounds while still gating every final source. | 30–60 min per audit; savings depend on merge order |

Sources: PR bodies linked above; the repository's
[comparer](../../src/visual_tests/compare.rs),
[full suite](../../src/visual_tests/mod.rs),
[gate commands](../../devenv.nix), and
[existing dependencies/profiles](../../Cargo.toml); Rayon's
[`IndexedParallelIterator`](https://docs.rs/rayon/1.12.0/rayon/iter/trait.IndexedParallelIterator.html)
and [indexed collection implementation](https://docs.rs/rayon/1.12.0/src/rayon/iter/collect/mod.rs.html).
The installed Rayon 1.12.0 source was also inspected; no dependency update is made.

## Fifth scan: isolate visual worlds and bound diagnostic text waits

2026-11-03. Base: `b8232ef` (main, merged with `git fetch origin && git merge
origin/main`). Same machine and pinned toolchain. The lead supplied a reflinked
warm `target/`; `CARGO_TARGET_DIR` was never set. The suite now has 54 scenarios
and 1,119 PNGs.

### Changes retained

- On Linux, the visual-suite test probes an unprivileged user/mount namespace,
  then re-executes **only that test** inside it. A private temporary folder is
  bind-mounted over the original temporary directory. The app still opens and
  paints `/tmp/splat-visual-world/...`; no app/UI code or path allowance changes.
  The existing world lock now lives in that private folder. A separate lock in
  each worktree's `target/` still serializes commands from that worktree, including
  commands that would write the same capture label.
- The backing folder stays on the **original temporary filesystem**. Only the
  synthetic fixtures go there, as before; a full tree measured about 40–43 MiB.
  Builds, copied executables, captures and videos stay in the worktree. The
  parent's temporary-directory guard removes the private folder after the child,
  including failed runs. Mounts end with the child. Unsupported Linux hosts,
  non-Linux hosts and builds/captures inside the temporary directory retain the
  original shared lock. `SPLAT_VISUAL_ISOLATE=0` explicitly selects that old route.
  Linux devenv shells now include the `unshare`/`mount` utilities.
- `wait_for_text` retains 180 seconds in normal mode. Under the existing
  `SPLAT_VISUAL_DIAGNOSE=1`, missing painted text fails after five seconds and
  includes the current screen's accessible text. It still panics: it cannot turn
  a missing phrase into a successful scenario or an eligible observation. A
  genuinely slow diagnostic run can therefore fail sooner; use normal mode for
  its full allowance. Idle-based failure was rejected: the existing busy/repaint
  signal does not prove that a delayed poll or deferred action cannot paint text.
- `audit-gate` clears diagnostic mode along with its other feedback-only settings.
  Every normal gate retains the original text timeout, checks and capture plan.

### Measurements

Times use `date +%s`, in wall seconds. The two-folder trials launch saved linked
executables together through one `devenv shell --` from the worktree root, with
separate working/output folders. Both select `party,settings`, run every check
and capture for those scenarios, and save all 60 PNGs each. They are a focused
lock-contention measurement, not a claim about two full suites' memory or CPU
scaling. The original missing-text probe overlapped the first baseline repeat
and first two-folder trial; one short rebuild overlapped that first pair. The
lock-wait durations directly identify serialization despite those overlaps.
A memory sample during the full gate found the isolated suite at 21.6 GiB RSS
and its parent at 1.7 GiB. Running several full suites concurrently on this
31 GiB host can therefore be limited by memory; this change removes fixture
serialization, not that resource limit. Scheduling and scenario concurrency
remain unchanged. Use `SPLAT_VISUAL_ISOLATE=0` to retain machine-wide serialization.

| Measurement | Before | After |
| --- | ---: | ---: |
| Two simultaneous two-scenario runs, total wall | 31 s | 17 s |
| Individual runs in that pair | 16 s / 31 s | 17 s / 17 s |
| Longest world-lock wait in that pair | 15.744 s | under 0.00001 s |
| Controlled absent painted text in diagnostic mode | 181 s | 6 s |

The text probe builds an actual empty-world `Driver`, waits for a deliberately
absent phrase, catches its panic and asserts that diagnostic feedback returns
within 15 seconds. The original implementation fails that regression after
180.85 seconds; the changed implementation passes while still catching the
missing-text panic. The six-second wall measurement includes driver setup.
The normal timeout and all successful text checks remain unchanged.
A repeat of the retained pair before the final per-worktree lock took 18 s;
the final-source pair took 17 s, with world-lock waits of 3.40 and 2.25 µs
and worktree-lock waits of 3.42 and 4.32 µs.

The first baseline `visual-test speed-5-before` took 76 s including a 19.14 s
Splat rebuild; its suite took 55.79 s. A second baseline capture using the saved
executable took 61 s wall / 60.22 s in the suite. Compilation and seed preparation
are excluded from the focused two-folder and text rows. These are separate
measurements and must not be added as an end-to-end saving.

### Fidelity and rejected trial

The first namespace trial bound a folder under `target/` over `/tmp`. It passed
all checks but **was rejected**: disk directory order made the party scenario's
existing `read_dir(...).find(...)` edit Brannoch's sheet instead of Sable's.
Six captures changed. Disk fsyncs also made the full suite take 102.99 s and its
gate 161 s. Keeping the private fixtures on the same temporary filesystem fixes
both without changing that scenario or adding any baseline allowance. The
retained two-folder captures are byte-identical to both original 60-PNG sets;
`ergonomics.json`, `ergonomics.md` and `chrome-observation.json` also match exactly.

The two unchanged full baseline runs have 111 changed PNG hashes and 29 captures
above the existing channel tolerance of 2. Those differences are in dialogs,
session notes and recording states already covered by the earlier noise caveat.
Full capture inventories have 451 images at 960×640, 449 at 1200×800, 202 at
1920×1080 and 17 at 2400×1600. No masks or tolerance changes were made.

Final validation: `SPLAT_VISUAL_VIDEO=0 devenv shell -- audit-gate speed-5`
passed in 110 s, including a 12.54 s Clippy rebuild and 22.12 s test build.
All 712 tests passed (187 library + 525 binary), with the same 12 opt-in tests
ignored; other binary and documentation targets also passed. The isolated
visual suite took 57.98 s, ran all 54 scenarios and wrote all 1,119 captures.
A retained full repeat passed in 73 s wall / 71.91 s in the suite; it overlapped
file-by-file image analysis and the repository comparer, so this is not an
isolated standalone-speed estimate.

Full reports and `chrome-observation.json` are byte-identical across the four
baseline and two retained full runs. Capture names and dimensions match, and
every manifest hash was checked against its PNG. The two retained focused
60-PNG sets are also byte-identical to their original sets. The namespace
regression verifies that a file at the same absolute path is private and the
host file remains intact. Controlled child failure and unavailable-namespace
probes both retain failure status 101; a child scenario failure is never retried
through the fallback. Rust/Nix formatting and `git diff --check` pass.

Full-image comparison uses the unchanged tolerance of 2. A/B/C are the original
standalone captures; D runs every original binary test with captures (64 s wall,
523 passed, 12 ignored). It excludes only the temporary failing text probe added
for this measurement, not any original test. E is the retained full gate, F its
full capture repeat.

| Capture pair | Changed PNG hashes | Above existing tolerance |
| --- | ---: | ---: |
| Original A / B | 111 | 29 |
| Original A / C | 100 | 43 |
| Original A / D | 74 | 28 |
| Original A / retained E | 196 | 13 |
| Original B / retained E | 181 | 33 |
| Original C / retained E | 156 | 47 |
| Original D / retained E | 234 | 32 |
| Retained E / F | 155 | 24 |

Every above-tolerance file in the cross-version comparisons also differs in
a same-version comparison, including an original `session-recording` focused
control. That control passed in 7 s, with its full scenario/check/capture plan;
its two `recording-delete` images match E within the existing tolerance. They
differ from original A by the same 2,884 pixels each. Inspection shows only the
existing “Recording saved” notice and the resulting rail layout. Its
[eight-second lifetime](../../src/sessions/page.rs) uses real `Instant` time, so
scenario load changes whether it is still visible. No timeout, notice, scenario
or allowance was changed to conceal these pixels. This proves the existing
transient variation, not byte-identical full runs.

The repository `visual-compare speed-5-before speed-5` agrees with the independent
image analysis: 1,119 pairs, 183 differences within tolerance, 13 above it, and
expected failure status 101. The latter files are included in the demonstrated
noise above; the comparator and its failure behavior remain unchanged.

Raw logs, second-resolution timestamps,
benchmark harnesses and file-by-file comparisons live under ignored
`artifacts/screenshots/audit-speed-5/`; captures use sibling `speed-5-*` labels.
No `artifacts/design-audit/` files are committed.

### PR 62 review: measure memory and restore a machine-wide admission limit

Merged `origin/main` at `1c5472e28dea5c1529cdcfce1bfedcca2261cbae` before this
revision. The measured merged source has 57 scenarios and 1,176 captures. This
revision supersedes the unrestricted namespace admission described above.

#### Memory measurements

Each run used a separate `systemd-run --user --scope -p MemoryAccounting=yes`
scope, through `devenv shell --` from the worktree root. No memory limit was set.
The linked merged-source executable was built and saved before adding slots;
compilation is excluded. PNG capture was on, video off. The scope contains the
suite's parent, namespace child and a small Python monitor. The monitor reads
`memory.peak`, `memory.swap.peak`, process-tree `/proc/<pid>/smaps_rollup` PSS and
SwapPss, and file sizes/allocated blocks under the private fixture folder. It
pauses 250 ms between samples; PSS and fixture peaks are sampled, while the
cgroup peaks are kernel counters. Only exited short-lived processes vanished
during `/proc` reads; there were no permission failures. The old shared-world
lock was held externally during these pre-slot measurements to avoid overlapping
another main-branch full suite. Only one measured suite ran at a time.

| Measurement | Full suite | `--only party,settings` |
| --- | ---: | ---: |
| Wall time, `date +%s` | 70 s | 16 s |
| Cgroup RAM peak, `memory.peak` | 24.275 GiB | 1.592 GiB |
| Cgroup swap peak, `memory.swap.peak` | 2.085 GiB | 0 GiB |
| Sampled cgroup RAM + swap peak | 25.650 GiB | 1.590 GiB |
| Sampled process-tree resident PSS peak | 23.788 GiB | 1.621 GiB |
| Sampled process-tree PSS + SwapPss peak | 25.159 GiB | 1.621 GiB |
| Fixture file bytes, sampled peak | 33.367 MiB | 1.057 MiB |
| Fixture allocated blocks in `/tmp`, sampled peak | 44.289 MiB | 1.449 MiB |
| Captures | 1,176 | 60 |

Both runs passed all selected checks. Cgroup accounting includes charged tmpfs
and file-cache pages; PSS describes mappings, including proportional shares of
pages charged to other cgroups. These are different accounting views and their
peaks need not coincide. Do not add PSS to the cgroup figures or add the fixtures
a second time to cgroup memory. RAM-peak plus swap-peak is a conservative
26.360 GiB upper bound for this full run, leaving about 4.9 GiB on the measured
31.26 GiB host for other work. Two such full suites cannot fit with headroom.

**Default: `SPLAT_VISUAL_SLOTS=1`.** This retains the old maximum of one active
full visual suite. Focused runs use the same limit: a slot must be safe for any
suite that enters it. `SPLAT_VISUAL_SLOTS` accepts a positive integer override;
all worktrees should use the same value. Raising it to 2 on this host is not
safe when full gates can run. The two-slot override test below used only the
measured-small focused runs, with no full suite running.

#### Admission and tests

Before namespace setup, a process scans all configured slot lock files in the
original temporary directory. It takes any free slot with `File::try_lock`,
or waits 100 ms and scans again. Its permit stays held through the suite and
report generation. The namespace child inherits the admission decision and
must not acquire locks inside its private temporary directory.

Slot zero reuses `$TMPDIR/splat-visual-world.lock` (normally `/tmp/`); higher
slots use `splat-visual-slot-1.lock`, `splat-visual-slot-2.lock`, and so on.
Thus an older main-branch worktree holding the original lock also occupies
slot zero, including during rollout of this change. Slot files are never
unlinked on release. Non-isolated/fallback runs take ownership of the already
held slot-zero world lock rather than trying to lock it twice. They retain the
old serial shared-world behavior. Different admitted namespace runs still
have independent worlds at the same visible paths.

`SPLAT_VISUAL_TIMINGS=1` now prints `Timing visual slot wait = ... (slot i/N)`
before the existing worktree/world/setup timings. Scenario scripts, scheduling,
checks, capture sizes, renderer, baseline and allowances are unchanged.

| Two focused runs started together from different folders | Default one slot | Two-slot override |
| --- | ---: | ---: |
| Total wall seconds | 29 s | 16 s |
| Individual wall seconds | 29 s / 15 s | 16 s / 16 s |
| Slot wait | 14.615 s / 0.000257 s | 0.000052 s / 0.000085 s |
| Slots used | 1 then 1 | 1 and 2 |

All four focused runs passed and wrote every one of their 60 captures. Their
namespace backing folders were distinct. A separate `SPLAT_VISUAL_ISOLATE=0`
focused capture run passed in 14.93 s inside the suite, confirming that the
legacy-lock transfer does not deadlock. Unit tests cover invalid/zero counts,
exhaustion, choosing a free higher slot, release/reuse, the legacy-lock handoff
and a waiting caller proceeding after release. Formatting and Clippy passed.

The single final gate passed: `env -u SPLAT_VISUAL_SLOTS SPLAT_VISUAL_VIDEO=0
SPLAT_VISUAL_TIMINGS=1 devenv shell -- audit-gate speed-5-slots`, 91 s wall.
Formatting, warnings-denied Clippy, all 719 tests (187 library + 532 binary),
the same 12 ignored opt-in tests, other binary/documentation targets, all 57
scenarios and all 1,176 captures passed. Slot admission reported 8.19 µs for
slot 1/1. The visual suite took 74.58 s. The full ergonomics JSON/Markdown and
chrome observation are byte-identical to the merged-source pre-slot run; capture
names match and every final manifest hash was verified. All six focused capture
sets (pre-slot, both one-slot runs, both two-slot runs, and fallback) are
byte-identical across their 60 PNGs. `git diff --check` passed.
Raw memory samples, cgroup paths, commands, logs and focused-pair timings are in
ignored `artifacts/screenshots/audit-speed-5-slots/`. All captures and the saved
measurement executable remain on disk in the worktree; only the original small
synthetic fixture trees use `/tmp`. No design-audit artifacts are committed.

## Sixth scan: share parsed fields instead of retaining YAML clones

2026-11-03. Merged `origin/main` first; baseline commit
`d91ce7038a22cd7965f881a455a8c6e36dabdf2c`. Same 31.26 GiB host. This main
revision has 58 scenarios and 1,201 PNGs. The warm `target/` was copied before
building; `CARGO_TARGET_DIR` was not set. Builds, saved executables, captures and
profiling output stayed in the worktree, not `/tmp`.

### Memory holders and rejected trials

Each full measurement used a separate `systemd-run --user --scope
-p MemoryAccounting=yes` scope, without a memory limit. The monitor read
`memory.peak`, `memory.swap.peak` and each scope process's `smaps_rollup`, with
250 ms sleeps between samples (reading mappings adds time). Compilation is
excluded from the standalone suite measurements. Captures are on and video is
explicitly off. Only one of our suites ran at a time except the stated pair.

The baseline passed in 68.743 s, with 26.555 GiB cgroup RAM peak, 0.283 GiB swap
peak and 25.936 GiB sampled resident PSS peak. PSS was 16.652 GiB after 104
captures, 20.627 GiB after 431 and 25.936 GiB after 926. Thus the growth is real
process memory, not fixture storage or just cgroup file cache.

A diagnostic `malloc_info` probe sampled glibc's arenas during another full run.
Its snapshot near maximum RSS contained 128 arenas, 25.929 GiB of system storage,
19.810 GiB free inside those arenas and 6.119 GiB in allocated chunks/overhead.
After the suite, 22.390 of 22.472 GiB was free: only 0.082 GiB remained allocated.
The largest individual arena held 545.2 MiB, of which 544.1 MiB was free. Arena
statistics and PSS are different views; do not add their figures. The probe's
extra thread prevented `unshare` setup, so this diagnostic used the existing
shared-world fallback. It passed all 58 scenarios and all 1,201 captures; its
reports match the isolated baseline.

Neither heaptrack nor valgrind was available in the devenv shell. A temporary
preload probe recorded allocation stacks: all allocations at least 64 KiB and
every 1,024th smaller allocation, weighted by 1,024. These are diagnostic
estimates, not an exact heap census. Sixteen-frame stacks and executable symbols
identify the dominant live holders:

| Allocation stack | Estimated per-stack live high-water mark |
| --- | ---: |
| Nested YAML mapping clone: indexmap entry storage | 4,589.8 MiB |
| YAML mapping clone through a sequence | 1,031.5 MiB |
| Nested YAML mapping clone: reserve-exact variant | 725.4 MiB |
| YAML sequence clone | 569.2 MiB |
| YAML mapping clone: hash table storage | 529.0 MiB |

These stack peaks occur at different times and must not be summed. Shallower
clones trace to `Entry::clone`, `Catalogue::load`, `encounters::Snapshot::load`
and `lookup::ContentIndex::load`. The same built-in nested mappings are copied
into many independent catalogues/views. Their allocation churn also fills the
free glibc arenas. Captures already stream to disk, repeated-capture checks keep
hashes, and reports keep small summaries; those are not the dominant holders.

Two alternatives were measured and removed:

- Restricting glibc to eight arenas on the saved main executable was still
  incomplete after 82.039 s, with only a small part of the capture plan finished.
  It had reached 15.638 GiB cgroup RAM; the trial was stopped. A short rebuild
  overlapped this diagnostic trial, so it is not used for speed comparisons.
  It did not meet the memory or feedback-time goal.
- Calling `malloc_trim(0)` every 250 ms, without changing any scenario scheduling,
  passed the full suite in 69.774 s. RAM peak remained 15.746 GiB and PSS peak
  13.452 GiB. Reclaiming free pages alone misses the live clone cost and fails
  the requested headroom and timing goals. No trimming worker or allocator
  setting remains in the implementation.

### Retained change

`Entry.fields` now holds `Fields`, a small `Arc<Mapping>` wrapper. Cloning an
entry shares its parsed mapping. A mutable borrow uses `Arc::make_mut`, so an
edit obtains a private mapping before changing even a nested value. There is
no global interning, mutable shared state or new cache lifetime. The last owner
releases the mapping normally. Serde remains transparent: existing JSON/YAML
field shapes and ordering are unchanged. The regression test edits a nested
field and removes a key from a clone, checks two unchanged peers, and checks
serialization and an independently editable deserialized entry.

The scenario list, scripts, driver, renderer, checks, all capture sizes, report
contents, video default and scenario concurrency are unchanged.

| Full standalone suite, no video | Main | Shared fields |
| --- | ---: | ---: |
| Wall seconds | 68.743 | 51.886 |
| Cgroup RAM peak | 26.555 GiB | 6.252 GiB |
| Cgroup swap peak | 0.283 GiB | 0 GiB |
| Sampled resident PSS peak | 25.936 GiB | 5.709 GiB |
| Scenarios / PNGs | 58 / 1,201 | 58 / 1,201 |

This pair reduces RAM peak by 76.5% and wall time by 24.5%. The commands execute
saved/warm linked tests through devenv; the monitor's wall clock starts after
shell entry and includes its completion polling. Diagnostic profiler timings
are not used as speed evidence.

The fresh 6.252 GiB full-suite measurement supports **two default visual slots**:
two such peaks total 12.504 GiB, leaving 18.76 GiB of the measured host for builds,
app processes and cache. The positive `SPLAT_VISUAL_SLOTS` override, persistent
lock files, legacy slot-zero lock and namespace behavior remain unchanged.
Use the same slot setting and updated implementation in all worktrees; keep one
slot while mixing old high-memory full suites with new ones.

### Fidelity and full gate

The first shared-field run has the same 1,201 filenames as main. Of those PNGs,
1,111 are byte-identical, 72 differ only within the unchanged tolerance of two,
and 18 differ above it. Every one of those 18 files also differs in a main/main
comparison. The affected scenarios are the already variable `book-to-package`,
`dialogs`, `session-notes` and `session-recording`; no tolerance, masking, timeout,
allowance or capture was changed. Main/main repeat comparisons had 30 and 55
above-tolerance files. Full ergonomics JSON/Markdown and chrome observations
match main byte for byte. The comparison also verifies every manifest hash.

`SPLAT_VISUAL_VIDEO=0 devenv shell -- audit-gate speed-6` passed on the final
implementation: formatting, warnings-denied Clippy, 188 library plus 540 binary
tests, the same 12 ignored opt-in tests, other binary/documentation targets,
58 scenarios and 1,201 captures. The new mutation/serialization test passed.
The complete command took 111.369 s, including shell entry, checks and all tests;
the isolated visual child took 82.26 s. This gate encountered a host load average
of 53.93 on 16 cores and is not an isolated-machine timing or a substitute for
the standalone before/after pair. Its scope peaked at 5.983 GiB RAM and 0.757 GiB
swap (a conservative sum-of-peaks bound of 6.740 GiB); sampled PSS peaked at
5.443 GiB, or 6.089 GiB including SwapPss.

Two full saved/warm executables then ran together, launched through devenv from
the worktree root into separate runner folders. Each used its own worktree lock
and namespace, with the original visible fixture paths. Neither set a slot
override: slot 2/2 admitted one in 44.44 microseconds and slot 1/2 admitted the
other in 35.60 microseconds. Both passed all 58 scenarios and saved all 1,201
captures each. Combined cgroup RAM peaked at **11.866 GiB**, swap at 0.364 GiB,
and sampled PSS at 10.930 GiB. Even adding the separate RAM/swap peaks gives
12.230 GiB, leaving about 19.03 GiB on this host. This directly confirms the
new default's memory headroom. Total pair wall time was 107.952 s, with individual
completion times 106.885/107.387 s; host load averages of 31.29–45.69 were
observed during this trial. This is a concurrent admission/memory check, not
an isolated throughput benchmark.

The final gate's image comparison against main has 1,147 byte-identical captures,
30 within tolerance and 24 above tolerance. Every above-tolerance file also
varies between main runs, including a third unchanged-main profiled repeat.
The unchanged `visual-compare speed-6-main speed-6` comparator agrees exactly:
1,201 pairs, 30 within tolerance and 24 differences, expected exit status 101.
The extra `library-to-encounter` variation is 578 pixels per affected file:
two equal-name/equal-initiative rows exchange their source labels. It also
occurs on unchanged main. It is unrelated to the shared field representation.
Final gate and both simultaneous runs have byte-identical full ergonomics
JSON/Markdown and chrome observations. Every capture name and manifest hash
was verified for every full run.

The simultaneous runs contain further transient notice variants. An unchanged
main control selected only `session-notes,session-recording`, retaining all their
scripts/checks and all 183 captures, under `CPUQuota=25%` and `MemoryMax=3G`.
It used the two-slot override only for this bounded focused control, passed in
134.66 s and reproduced every remaining variant. The nine additionally affected
files in the pair match that control within the existing pixel tolerance.
Inspection locates the changes in the existing two-second “Copied” button and
eight-second “Recording saved” notice, including the rail shift after expiry.
Their clocks and lifetimes are unchanged. No allowance or mask was added.

| Main versus retained run | Byte-identical PNGs | Within tolerance | Above tolerance |
| --- | ---: | ---: | ---: |
| Standalone shared fields | 1,111 | 72 | 18 |
| Final `speed-6` gate | 1,147 | 30 | 24 |
| Simultaneous A | 1,107 | 43 | 51 |
| Simultaneous B | 1,128 | 16 | 57 |

Every above-tolerance file in this table also varies in an unchanged-main
comparison/control. Raw scopes, logs, allocation probes, symbolized stack
summaries, pixel counts and per-file comparisons are under ignored
`artifacts/screenshots/audit-speed-6/`; captures use sibling `speed-6-*` labels.
Saved executables and profiler libraries are under `target/`. No instrumentation
remains in source, and no `artifacts/design-audit/` files are committed.
`git diff --check` passed.
