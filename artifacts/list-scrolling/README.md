# Content list scrolling verification

Compared base commit `85e2397` with this change on the same machine, using the
same headless egui input sequence and debug build settings. The harness renders
and tessellates the complete Library view, scrolls the built-in package of 2,065
entries for 150 frames, and measures the final 120 frames after warm-up.

| CPU frame time | Before | After |
| --- | ---: | ---: |
| Median | 38.230 ms | 2.262 ms |
| 95th percentile | 41.213 ms | 2.351 ms |
| Maximum | 41.865 ms | 2.444 ms |

This is approximately 17 times less median CPU time. These are debug-build CPU
measurements, not release-build results or measured display FPS. The baseline
worktree used the identical `library_scroll_frame_times` test added by this PR.

Repeat with:

```sh
devenv shell -- cargo test --locked --bin splat library_scroll_frame_times -- --ignored --nocapture
```

## Visual checks

Ran the native app in Xvfb with isolated temporary settings and data directories.
Used pointer wheel input to scroll the list, selected Air Elemental after
scrolling, searched for `dragon` (51 results), and resized to the supported
minimum window size. Inspected the actual framebuffer captures:

- [Scrolled list, 1200 × 800](scrolled.png): aligned rows and visible subtitles.
- [Search, 1200 × 800](search.png): results and selected entry agree.
- [Search, 960 × 640](small-window.png): list remains usable at minimum size.

Full-name tooltips were also inspected on the running Library list. The virtual
display uses software rendering; this check does not measure physical display
smoothness or GPU performance. Campaign lists were covered by source review;
these screenshots cover the Library.

## Review and checks

Independent standards and correctness reviews found stale campaign ordering after
rename, missing full-name checkbox tooltips, and missing accessible row labels.
All were fixed; the follow-up correctness review found no remaining blockers.
A shared row-height constant also keeps layout and virtualization in sync.

The full Rust test suite passes (150 tests, 3 ignored). The timing test was run
explicitly. Formatting and Clippy with warnings denied pass.
