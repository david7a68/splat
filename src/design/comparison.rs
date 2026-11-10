//! Align equal source lines before counting, marking or navigating changes.

pub(super) struct Comparison<'a> {
    pub columns: [Vec<Option<&'a str>>; 2],
    pub changed: usize,
    pub groups: Vec<usize>,
}

impl<'a> Comparison<'a> {
    pub fn new(text: [&'a str; 2]) -> Self {
        let lines = text.map(|text| text.lines().collect::<Vec<_>>());
        let mut anchors = Vec::new();
        matching_lines(&lines[0], &lines[1], [0, 0], &mut anchors);
        anchors.push((lines[0].len(), lines[1].len()));
        let mut columns = [Vec::new(), Vec::new()];
        let mut start = [0, 0];
        for (left, right) in anchors {
            let end = [left, right];
            for offset in 0..(left - start[0]).max(right - start[1]) {
                for side in 0..2 {
                    columns[side].push(
                        (start[side] + offset < end[side])
                            .then(|| lines[side][start[side] + offset]),
                    );
                }
            }
            if left < lines[0].len() {
                columns[0].push(Some(lines[0][left]));
                columns[1].push(Some(lines[1][right]));
            }
            start = [left + 1, right + 1];
        }
        let mut changed = 0;
        let mut groups = Vec::new();
        let mut in_change = false;
        for (row, (left, right)) in columns[0].iter().zip(&columns[1]).enumerate() {
            let differs = left != right;
            if differs {
                changed += 1;
                if !in_change {
                    groups.push(row);
                }
            }
            in_change = differs;
        }
        Self {
            columns,
            changed,
            groups,
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
}
