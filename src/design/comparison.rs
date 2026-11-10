//! Align equal source lines before counting, marking or navigating changes.

pub(super) struct Comparison<'a> {
    pub columns: [Vec<Option<&'a str>>; 2],
    pub changed: usize,
    pub groups: Vec<usize>,
}

/// The aligned rows as line indexes, so the layout can be kept between frames.
#[derive(Clone, Default)]
struct Layout {
    rows: Vec<[Option<usize>; 2]>,
    changed: usize,
    groups: Vec<usize>,
}

impl Layout {
    fn of(lines: &[Vec<&str>; 2]) -> Self {
        let mut anchors = Vec::new();
        matching_lines(&lines[0], &lines[1], [0, 0], &mut anchors);
        anchors.push((lines[0].len(), lines[1].len()));
        let mut rows = Vec::new();
        let mut start = [0, 0];
        for (left, right) in anchors {
            let end = [left, right];
            for offset in 0..(left - start[0]).max(right - start[1]) {
                rows.push([0, 1].map(|side| {
                    (start[side] + offset < end[side]).then_some(start[side] + offset)
                }));
            }
            if left < lines[0].len() {
                rows.push([Some(left), Some(right)]);
            }
            start = [left + 1, right + 1];
        }
        let mut changed = 0;
        let mut groups = Vec::new();
        let mut in_change = false;
        for (row, [left, right]) in rows.iter().enumerate() {
            let differs = left.map(|at| lines[0][at]) != right.map(|at| lines[1][at]);
            if differs {
                changed += 1;
                if !in_change {
                    groups.push(row);
                }
            }
            in_change = differs;
        }
        Self {
            rows,
            changed,
            groups,
        }
    }
}

impl<'a> Comparison<'a> {
    /// Uncached, for tests; the app uses `cached`.
    #[cfg(test)]
    pub fn new(text: [&'a str; 2]) -> Self {
        let lines = text.map(|text| text.lines().collect::<Vec<_>>());
        Self::from_layout(&lines, Layout::of(&lines))
    }

    /// The same comparison, aligned only when either text changes. Alignment costs about
    /// n × m line comparisons, too much to repeat every frame for a long note.
    pub fn cached(ctx: &eframe::egui::Context, slot: eframe::egui::Id, text: [&'a str; 2]) -> Self {
        let lines = text.map(|text| text.lines().collect::<Vec<_>>());
        let layout = Self::cached_layout(ctx, slot, text, &lines);
        Self::from_layout(&lines, (*layout).clone())
    }

    pub fn cached_rows(
        ctx: &eframe::egui::Context,
        slot: eframe::egui::Id,
        text: [&str; 2],
    ) -> usize {
        let lines = text.map(|text| text.lines().collect::<Vec<_>>());
        Self::cached_layout(ctx, slot, text, &lines).rows.len()
    }

    fn cached_layout(
        ctx: &eframe::egui::Context,
        slot: eframe::egui::Id,
        text: [&str; 2],
        lines: &[Vec<&str>; 2],
    ) -> std::sync::Arc<Layout> {
        let texts = eframe::egui::Id::new((text[0], text[1])).value();
        let kept = ctx.data(|data| data.get_temp::<(u64, std::sync::Arc<Layout>)>(slot));
        match kept {
            Some((seen, layout)) if seen == texts => layout,
            _ => {
                let layout = std::sync::Arc::new(Layout::of(lines));
                ctx.data_mut(|data| data.insert_temp(slot, (texts, layout.clone())));
                layout
            }
        }
    }

    fn from_layout(lines: &[Vec<&'a str>; 2], layout: Layout) -> Self {
        let columns = [0, 1].map(|side| {
            layout
                .rows
                .iter()
                .map(|row| row[side].map(|at| lines[side][at]))
                .collect()
        });
        Self {
            columns,
            changed: layout.changed,
            groups: layout.groups,
        }
    }
}

/// Hirschberg's longest common subsequence uses linear working memory. Trim shared
/// ends first so a small edit in a long note does not compare the whole note.
fn matching_lines(
    left: &[&str],
    right: &[&str],
    mut base: [usize; 2],
    out: &mut Vec<(usize, usize)>,
) {
    let prefix = left.iter().zip(right).take_while(|(a, b)| a == b).count();
    out.extend((0..prefix).map(|i| (base[0] + i, base[1] + i)));
    let (left, right) = (&left[prefix..], &right[prefix..]);
    base = [base[0] + prefix, base[1] + prefix];
    let suffix = left
        .iter()
        .rev()
        .zip(right.iter().rev())
        .take_while(|(a, b)| a == b)
        .count();
    let (a, b) = (&left[..left.len() - suffix], &right[..right.len() - suffix]);
    if a.len() == 1 {
        if let Some(at) = b.iter().position(|line| line == &a[0]) {
            out.push((base[0], base[1] + at));
        }
    } else if !a.is_empty() && !b.is_empty() {
        let middle = a.len() / 2;
        let forward = lengths(a[..middle].iter().copied(), b.iter().copied());
        let backward = lengths(a[middle..].iter().rev().copied(), b.iter().rev().copied());
        let split = (0..=b.len())
            .max_by_key(|&i| forward[i] + backward[b.len() - i])
            .unwrap();
        drop((forward, backward));
        matching_lines(&a[..middle], &b[..split], base, out);
        matching_lines(
            &a[middle..],
            &b[split..],
            [base[0] + middle, base[1] + split],
            out,
        );
    }
    out.extend((0..suffix).map(|i| (base[0] + a.len() + i, base[1] + b.len() + i)));
}

fn lengths<'a>(
    left: impl Iterator<Item = &'a str>,
    right: impl Iterator<Item = &'a str> + Clone,
) -> Vec<usize> {
    let mut lengths = vec![0; right.clone().count() + 1];
    for a in left {
        let mut previous = 0;
        for (j, b) in right.clone().enumerate() {
            let old = lengths[j + 1];
            lengths[j + 1] = if a == b {
                previous + 1
            } else {
                lengths[j].max(old)
            };
            previous = old;
        }
    }
    lengths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alignment_preserves_both_sources_and_marks_only_real_gaps() {
        for (left, right, expected) in [
            (
                "one\nremoved\ntwo",
                "one\ntwo",
                vec![
                    [Some("one"), Some("one")],
                    [Some("removed"), None],
                    [Some("two"), Some("two")],
                ],
            ),
            (
                "one\ntwo",
                "one\nadded\ntwo",
                vec![
                    [Some("one"), Some("one")],
                    [None, Some("added")],
                    [Some("two"), Some("two")],
                ],
            ),
            (
                "old\nsame\nold",
                "new\nsame\nnew",
                vec![
                    [Some("old"), Some("new")],
                    [Some("same"), Some("same")],
                    [Some("old"), Some("new")],
                ],
            ),
        ] {
            let comparison = Comparison::new([left, right]);
            let rows: Vec<_> = comparison.columns[0]
                .iter()
                .zip(&comparison.columns[1])
                .map(|(&a, &b)| [a, b])
                .collect();
            assert_eq!(rows, expected);
        }
    }

    #[test]
    fn repeated_and_empty_lines_keep_the_longest_unchanged_sequence() {
        // Exhaust all short notes over two words and blank lines. This also covers
        // empty files, insertions at either end, unequal replacements and repeats.
        let mut notes = vec![String::new()];
        for len in 1..=4 {
            for mut code in 0..3_usize.pow(len) {
                let mut note = String::new();
                for _ in 0..len {
                    note.push_str(["a\n", "b\n", "\n"][code % 3]);
                    code /= 3;
                }
                notes.push(note);
            }
        }
        for left in &notes {
            for right in &notes {
                let comparison = Comparison::new([left, right]);
                for (side, source) in [left, right].iter().enumerate() {
                    assert_eq!(
                        comparison.columns[side]
                            .iter()
                            .copied()
                            .flatten()
                            .collect::<Vec<_>>(),
                        source.lines().collect::<Vec<_>>()
                    );
                }
                let equal = comparison.columns[0]
                    .iter()
                    .zip(&comparison.columns[1])
                    .filter(|(a, b)| a == b)
                    .count();
                // Independent full-table LCS oracle for these small inputs.
                let a: Vec<_> = left.lines().collect();
                let b: Vec<_> = right.lines().collect();
                let mut table = vec![vec![0; b.len() + 1]; a.len() + 1];
                for (i, a) in a.iter().enumerate() {
                    for (j, b) in b.iter().enumerate() {
                        table[i + 1][j + 1] = if a == b {
                            table[i][j] + 1
                        } else {
                            table[i][j + 1].max(table[i + 1][j])
                        };
                    }
                }
                assert_eq!(equal, table[a.len()][b.len()], "{left:?} vs {right:?}");
                assert!(
                    comparison.columns[0]
                        .iter()
                        .zip(&comparison.columns[1])
                        .all(|(a, b)| a.is_some() || b.is_some())
                );
                assert_eq!(comparison.changed + equal, comparison.columns[0].len());
            }
        }
    }
    #[test]
    fn aligned_row_count_reuses_the_layout_and_invalidates_on_either_source() {
        let ctx = eframe::egui::Context::default();
        let slot = eframe::egui::Id::new("source rows");
        for (text, expected) in [
            (["removed\nanchor", "anchor\ninserted"], 3),
            (["anchor", "anchor\ninserted"], 2),
            (["anchor", "anchor"], 1),
            (["", ""], 0),
        ] {
            assert_eq!(Comparison::cached_rows(&ctx, slot, text), expected);
            let first = ctx
                .data(|data| data.get_temp::<(u64, std::sync::Arc<super::Layout>)>(slot))
                .unwrap()
                .1;
            let painted = Comparison::cached(&ctx, slot, text);
            let second = ctx
                .data(|data| data.get_temp::<(u64, std::sync::Arc<super::Layout>)>(slot))
                .unwrap()
                .1;
            assert!(std::sync::Arc::ptr_eq(&first, &second));
            assert_eq!(painted.columns[0].len(), expected);
        }
    }
}
