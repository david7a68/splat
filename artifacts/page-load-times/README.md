# Page load times

Three trees, measured on 2026-11-01 and 2026-11-02 on an Intel i7-12700H with Iris Xe graphics. The
app ran in its real window code at 1200 × 800, in a headless nested KWin, with vsync off. Values are
medians in milliseconds, from a release build with real data unless a row says otherwise.

- **f4a464a**: the redesign when the goals were set.
- **fa54fed**: the redesign branch just before this change. It already caches the embedded content.
- **change**: this change, measured on top of `fa54fed`.

The change is responsible only for the difference between the last two columns. The first column
shows where the goals came from. Raw summaries are in `results-<tree>-<profile>.json`.

## Goals

| Goal | Limit | f4a464a | fa54fed | change |
| --- | ---: | ---: | ---: | ---: |
| Campaign Packages redraw | 8 | 31 | 23 | 1.0 |
| Library first visit | 200 | 407 | 189 | 176 |
| Campaign Packages first visit | 200 | 227 | 92 | 98 |
| Slowest page content at app start (Party) | 500 | 1168 | 875 | 406 |
| Slowest return visit (Campaign Packages) | one frame | 32 | 24 | 1.3 |

## Each page

| Page | Start here: fa54fed → change | First visit: fa54fed → change | Return: fa54fed → change |
| --- | ---: | ---: | ---: |
| Encounters | 260 → 296 | 0.5 → 0.5 | 0.2 → 0.2 |
| Party | 875 → 406 | 22 → 21 | 0.6 → 0.6 |
| Library | 362 → 346 | 189 → 176 | 0.4 → 0.4 |
| Campaign, Packages | 251 → 290 | 92 → 98 | 24 → 1.3 |
| Campaign, Rules and runs | - | 0.6 → 0.6 | 0.3 → 0.3 |
| Campaign, Link and folder | - | 0.6 → 0.6 | 0.3 → 0.3 |
| Settings | 54 → 56 | 1.1 → 1.2 | 0.3 → 0.4 |
| Run (test fixture) | 51 → 51 | 2.1 → 2.0 | 0.7 → 0.6 |
| Lookup (panel) | - | 0.5 → 0.5 | 0.3 → 0.3 |

- **Start here**: from process start until the page's own background load has ended and that frame
  is complete. For Run, until the run itself is drawn.
- **First visit**: from the switch to the page, after all startup loads have ended.
- **Return**: the same switch, after the first visit.
- **All startup loads**: these end at about 390 ms with the change, and at about 850 ms on `fa54fed`.

## What got worse

- Encounters and Campaign Packages content at app start is about 40 ms later. On every start, the
  Encounters load ends about 38 ms later and the Lookup load about 20 ms later, whichever page opens,
  while the Party load ends about 460 ms sooner. The cause is not known. One untested idea is that
  the Party thread now does its work at the same time as these loads, not spread out between disk
  flushes. All of these stay under the 500 ms goal.
- Campaign Packages first visit moved from 92 to 98 ms with real data, because the first closure is
  now calculated on the loader thread. With the fixture it moved from 83 to 63 ms. In exchange, the
  longest frame on that visit fell from 25 ms to 2 ms.

## What changed

- `definitions::entries` and `builtin::entries` return borrowed slices. `fa54fed` caches them but
  returns a clone for each caller.
- `definitions::source_file` looks up a file in an index, not with a linear search. It now finds only
  Markdown files, which is all that its callers ask for.
- Campaign Packages keeps its dependency closure until the selections change. It was calculated on
  every frame.
- Campaign Packages reads its repair notes on the loader thread, not on the UI thread. The repair
  section now appears when the load ends.
- `save_campaign_copy` does not rewrite a note that has not changed. Before, every launch rewrote each
  character note in the campaign folder, with one fsync per note.

## Debug builds

`fa54fed` builds dependencies with `opt-level = 2`, so debug builds are close to release now.
From `fa54fed` to the change, the Campaign Packages redraw moved from 73 to 4.3, Library first visit
from 372 to 329, and Party at app start from 1709 to 1224.

## Data

- A copy of the settings, campaign, packages and character cache. The Run page uses the visual test
  fixture instead (15 enemies and the party), because the real campaign has no runs.
- The fixture results for the other pages are in the JSON files.

## Limits

- Release: 10 startups and 5 tours for each page. Debug: 3 startups and 2 tours, so its values are
  rough. One example: Settings first visit in debug with the fixture has a range of 1 to 25 ms.
- There was no network and no D-Bus, so the app did not sign in. D&D Beyond time is not included.
- The time does not include GPU completion or the compositor. Add about one display refresh.
- Only the landing state of each page was measured: no open entry, no scrolling.
- The laptop was on battery at the start of the `f4a464a` run and charging at its end. Power state
  for the later runs was not recorded. The driver waited until less than 2.5 cores were busy before
  each run.

## Repeat

Use one scratch folder for each tree you compare. The harness never goes in the real source tree.
From the repository root, with `REF` set to a commit:

```sh
P=$PWD/target/perf-$REF
mkdir -p $P/src $P/harness $P/tmp
cp artifacts/page-load-times/harness/* $P/harness/
git archive $REF | tar -x -C $P/src
python3 $P/harness/apply_patch.py $P/src
devenv shell -- bash -c "printf '%s\n' \"LD_LIBRARY_PATH=\$LD_LIBRARY_PATH\" \"__EGL_VENDOR_LIBRARY_FILENAMES=\$__EGL_VENDOR_LIBRARY_FILENAMES\" > $P/app-env.txt
  cd $P/src && export CARGO_TARGET_DIR=$P/target
  cargo build --locked --bin splat && cargo build --locked --release --bin splat
  TMPDIR=$P/tmp SPLAT_PERF_WORLD=$P/tmp/fixture cargo test --locked --bin splat perf_world -- --ignored"
python3 $P/harness/driver.py $P release 10 5 11
python3 $P/harness/driver.py $P debug 3 2 6
```

Results go to `$P/results-<profile>.json`. Keep the inherited `LD_LIBRARY_PATH` when you start the
driver from T3 Code. Without it the app got software rendering here, and the driver stops when it
sees that.
