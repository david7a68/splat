//! Named usage patterns, scripted step by step as a user works. Goals carry the
//! test IDs from `docs/ux-intent.md` section 8.
//!
//! A redesign can rename or move controls. Each scenario keeps its own labels in
//! its first lines, and the labels several share are in `steps.rs`, so a script
//! says what the user does and a rename changes one line.
use super::{
    driver::{DEFAULT, Driver, LARGE, SMALL, Sizes},
    fixture::{
        CAMPAIGN, GROUP_ARENA, GROUP_GOBLINS, GROUP_LOST, GROUP_ORPHAN, GROUP_RIVALS, GROUP_SHRINE,
        GROUP_WATCH, MODULE, PAST_SESSION, QUIET_CAMPAIGN, RUN, RUN_12, RUN_20, RUN_ARENA, RUN_OLD,
        RUN_PAST, RUN_VAULT, UNLINKED_CAMPAIGN,
    },
    steps::*,
};
use eframe::egui::{self, Key, Pos2, pos2};
pub mod encounter_difficulty;
pub mod file_formats;
pub mod integrity;
pub mod list_formats;
pub mod markdown_links;
pub mod run_continue;
pub mod session_plan_scenes;
pub mod stat_formats;
pub mod travel_checks;
pub mod world_build;

/// The middle of the reading pane and of the entry list, at the minimum window.
const READER: Pos2 = pos2(720.0, 400.0);
const ENTRY_LIST: Pos2 = pos2(400.0, 400.0);

/// Message choices start at the message text, never at its bar. `choices` are in order and must
/// all show. `gap` names the choice that keeps exactly an `L` gap from the one before it (the red
/// choice, or a quiet dismissal): on a shared line the gap is `L`; on a line of its own it starts at
/// the text edge, as every wrapped choice does.
/// Iteration 21 (FD4): at each size, the Skills rows in view hold at least `least` bonuses, and
/// every one keeps the row's rule (`Driver::skills_row_faults`): one blank between each name and
/// its bonus and after each comma, a target at least `XL` square inside its own entry, and whole
/// entries on each row.
fn check_skills_row(d: &mut Driver<'_>, id: &str, least: usize, sizes: &[(f32, f32)]) {
    for &size in sizes {
        let (count, faults) = d.at_size(size, Driver::skills_row_faults);
        d.observe(
            id,
            count >= least && faults.is_empty(),
            format!("{count} skill bonuses in view at {size:?} (at least {least}): {faults:?}"),
        );
    }
}

fn check_choices_at_text(
    d: &mut Driver<'_>,
    id: &str,
    words: &str,
    choices: &[&str],
    gap: Option<&str>,
    sizes: &[(f32, f32)],
) {
    for &size in sizes {
        let (passed, detail) = d.at_size(size, |d| {
            let text = d.painted_rect_in(words, egui::Rect::EVERYTHING);
            let places: Vec<_> = choices.iter().map(|name| d.place_of(name)).collect();
            let detail = format!(
                "Choices {choices:?} start at the text of \"{words}\" at {size:?}: text {text:?}, choices {places:?}"
            );
            let (Some(text), Some(places)) = (text, places.into_iter().collect::<Option<Vec<_>>>())
            else {
                return (false, detail);
            };
            let first = (places[0].left() - text.left()).abs() < 1.0;
            let below = places
                .iter()
                .all(|place| place.left() >= text.left() - 0.5 && place.top() >= text.bottom());
            let spaced = gap.is_none_or(|gap| {
                choices
                    .iter()
                    .position(|name| *name == gap)
                    .filter(|at| *at > 0)
                    .is_some_and(|at| {
                        let (before, after) = (places[at - 1], places[at]);
                        if (before.center().y - after.center().y).abs() < 1.0 {
                            (after.left() - before.right() - crate::design::L).abs() < 1.0
                        } else {
                            (after.left() - text.left()).abs() < 1.0
                        }
                    })
            });
            (first && below && spaced, detail)
        });
        d.observe(id, passed, detail);
    }
}

/// iter-25-S FD3 family: a banner's dismiss is the close icon at the message end, on its first
/// line, with no words of its own on screen.
fn check_dismiss_at_message_end(d: &mut Driver<'_>, id: &str, words: &str) {
    let text = d.painted_rect_in(words, egui::Rect::EVERYTHING);
    let icon = d.place_of("Dismiss this error");
    d.observe(
        id,
        text.zip(icon).is_some_and(|(text, icon)| {
            icon.left() >= text.right() - 0.5
                && icon.top() <= text.top() + crate::design::S
                && icon.right() <= text.left() + crate::design::LIST_MEASURE + 0.5
        }) && !d.visible_text("Dismiss this error"),
        format!("The dismiss icon {icon:?} ends the first line of \"{words}\" {text:?}"),
    );
}

/// Where each painted copy of these words starts on screen, after stepping back `back`
/// characters (to the icon before them). Only copies inside their pane's clip count.
fn painted_starts(d: &Driver<'_>, words: &str, back: usize) -> Vec<Pos2> {
    let mut starts = Vec::new();
    d.each_painted_text(|text, clip| {
        let job = &text.galley.job.text;
        if let Some(at) = job.find(words) {
            let index = job[..at].chars().count().saturating_sub(back);
            let rect = text
                .galley
                .pos_from_cursor(egui::text::CCursor::new(index))
                .translate(text.pos.to_vec2());
            if clip.contains(rect.center()) {
                starts.push(rect.left_top());
            }
        }
    });
    starts
}

/// Every disk-conflict choice row uses the Read density, so its buttons are one height on every
/// page. Runs at the size being captured.
fn conflict_choices_read_height(d: &mut Driver<'_>, id: &str, safe: &str, red: &str) {
    let height = crate::design::Density::Read.control();
    let (safe_rect, red_rect) = (d.place_of(safe), d.place_of(red));
    let size = d.window_size();
    d.observe(
        id,
        safe_rect.zip(red_rect).is_some_and(|(safe, red)| {
            (safe.height() - height).abs() < 0.5 && (red.height() - height).abs() < 0.5
        }),
        format!("At {size:?}: {safe} {safe_rect:?} and {red} {red_rect:?} are {height} px tall"),
    );
}

/// iter-15-E: a file-conflict message's choices start at its text edge, not at its bar. The safe
/// choice comes first; the red one follows exactly `L` after it on the same line (every captured
/// size has room for both). `below`, when given, is text that must stay under both choices. The
/// check runs at the size being captured.
fn choices_at_text(
    d: &mut Driver<'_>,
    id: &str,
    message: &str,
    safe: &str,
    red: &str,
    below: Option<&str>,
) {
    let text = d.painted_rect_in(message, egui::Rect::EVERYTHING);
    let (safe_rect, red_rect) = (d.place_of(safe), d.place_of(red));
    let below_rect = below.map(|words| d.painted_rect_in(words, egui::Rect::EVERYTHING));
    let size = d.window_size();
    let (ok, detail) = match (text, safe_rect, red_rect) {
        (Some(text), Some(safe_rect), Some(red_rect)) => {
            let at_text = (safe_rect.left() - text.left()).abs() < 1.0;
            let one_line = (safe_rect.center().y - red_rect.center().y).abs() < 1.0;
            let gap = red_rect.left() - safe_rect.right();
            let red_placed = one_line && (gap - crate::design::L).abs() < 0.5;
            let stays_below = match below_rect {
                None => true,
                Some(Some(rect)) => rect.top() >= safe_rect.bottom().max(red_rect.bottom()),
                Some(None) => false,
            };
            (
                at_text && red_placed && stays_below && text.bottom() <= safe_rect.top(),
                format!(
                    "At {size:?}: text x {:.1}, {safe} x {:.1}; {red} x {:.1}, {} gap {gap:.1}; {}",
                    text.left(),
                    safe_rect.left(),
                    red_rect.left(),
                    if one_line {
                        "same line,"
                    } else {
                        "not on one line,"
                    },
                    match below {
                        Some(words) if stays_below => format!("{words} stays below"),
                        Some(words) => format!("{words} is not below"),
                        None => "nothing below to check".into(),
                    }
                ),
            )
        }
        _ => (
            false,
            format!("At {size:?}: missing message text, {safe} or {red}"),
        ),
    };
    d.observe(id, ok, detail);
}

/// FT-2: the run's Undo button always shows the word Undo, at every captured size, and keeps
/// its full action as its accessible name.
fn undo_keeps_its_word(d: &mut Driver<'_>, action: &str) {
    let target = d.place_of(action).expect("The Undo action is visible");
    let painted = d.painted_in(target);
    d.observe(
        "iter10-B-FT2-word",
        painted.contains("Undo"),
        format!("{action} shows the word Undo: {painted:?}"),
    );
}

/// iter11-B-FT1: beside a state notice, which stays whole, the run's Undo keeps a shortened
/// target while one fits. Its accessible name keeps the full action. A sheet-change notice,
/// in the header or below the roster, still shows its words and its Log link.
fn undo_keeps_a_target(d: &mut Driver<'_>, notice: &str, sheet: Option<&str>) {
    let size = d.window_size();
    let undo: Vec<_> = d
        .nodes()
        .into_iter()
        .filter(|node| node.in_view() && node.name.starts_with("Undo: "))
        .collect();
    let painted = undo
        .first()
        .map_or_else(String::new, |node| d.painted_in(node.rect));
    let action = undo.first().map_or("", |node| node.name.as_str());
    d.observe(
        "iter11-B-FT1",
        undo.len() == 1 && painted.contains("Undo: ") && d.visible_text(notice),
        format!("At {size:?}, beside a whole {notice:?}, Undo for {action:?} paints {painted:?}"),
    );
    if let Some(sheet) = sheet {
        let window = egui::Rect::from_min_size(pos2(0.0, 0.0), egui::vec2(size.0, size.1));
        let text = d.painted_rect_in(sheet, window);
        let link = text.and_then(|text| {
            d.nodes()
                .into_iter()
                .filter(|node| node.in_view() && node.name == "Log")
                .find(|node| {
                    (node.rect.center().y - text.center().y).abs() <= crate::design::S
                        && node.rect.left() >= text.right() - 1.0
                })
                .map(|node| node.rect)
        });
        d.observe(
            "iter11-B2-sheet",
            text.is_some() && link.is_some(),
            format!("At {size:?}, the sheet-change notice paints {sheet:?} at {text:?}, with its Log link at {link:?}"),
        );
    }
}

/// Recovery actions keep the shared Undo icon, inside their own target.
fn check_undo_icon(d: &mut Driver<'_>, action: &str, id: &str) {
    let target = d.place_of(action).expect("The Undo action is visible");
    d.observe(
        id,
        d.painted_in(target).contains(crate::design::icons::UNDO),
        format!("{action} uses the shared Undo icon"),
    );
}

/// A message action's shape: its border, fill, height and whether it bears the Undo icon.
fn undo_shape(d: &Driver<'_>, action: &str) -> Option<(egui::Stroke, egui::Color32, i32, bool)> {
    let target = d.place_of(action)?;
    let frame = d.control_frame(action)?;
    Some((
        frame.stroke,
        frame.fill,
        target.height().round() as i32,
        d.painted_in(target).contains(crate::design::icons::UNDO),
    ))
}

/// Whether painted text is only icon glyphs (the icon font's private-use characters).
fn is_icon(text: &str) -> bool {
    !text.is_empty()
        && text
            .chars()
            .all(|c| ('\u{E000}'..='\u{F8FF}').contains(&c) || c.is_whitespace())
}

/// Where a control's words start: the leftmost text painted inside its press area. An icon
/// before the words does not count: `line_start` hangs it in the gutter, so the words start at
/// the text edge.
fn words_left_in(d: &Driver<'_>, area: egui::Rect) -> Option<f32> {
    let mut left = None::<f32>;
    d.each_painted_text(|text, _| {
        let rect = egui::Rect::from_min_size(text.pos, text.galley.size());
        if area.contains(rect.center()) && !is_icon(&text.galley.job.text) {
            left = Some(left.map_or(rect.left(), |left| left.min(rect.left())));
        }
    });
    left
}

/// Where `words` are drawn in each visible painted paragraph that contains `paragraph`: the
/// left and right of the words and their baseline, for a match on one row.
pub(super) fn glyph_spans(d: &Driver<'_>, paragraph: &str, words: &str) -> Vec<(f32, f32, f32)> {
    let mut spans = Vec::new();
    d.each_painted_text(|text, clip| {
        let job = &text.galley.job;
        if !job.text.contains(paragraph) || !clip.intersects(text.visual_bounding_rect()) {
            return;
        }
        let Some(start) = job.text.find(words) else {
            return;
        };
        let end = start + words.len();
        let mut byte_at = 0;
        let mut span: Option<(f32, f32, f32)> = None;
        for row in &text.galley.rows {
            for glyph in &row.glyphs {
                if (start..end).contains(&byte_at) {
                    let x = text.pos.x + row.pos.x + glyph.pos.x;
                    let y = text.pos.y + row.pos.y + glyph.pos.y;
                    let right = x + glyph.advance_width;
                    span = Some(span.map_or((x, right, y), |(left, _, y)| (left, right, y)));
                }
                byte_at += glyph.chr.len_utf8();
            }
            if row.ends_with_newline {
                byte_at += 1;
            }
        }
        spans.extend(span);
    });
    spans
}

/// For each visible painted paragraph that contains `paragraph`: each row's top and its first
/// glyph's baseline.
fn paragraph_baselines(d: &Driver<'_>, paragraph: &str) -> Vec<Vec<(f32, f32)>> {
    let mut found = Vec::new();
    d.each_painted_text(|text, clip| {
        if text.galley.job.text.contains(paragraph) && clip.intersects(text.visual_bounding_rect())
        {
            found.push(
                text.galley
                    .rows
                    .iter()
                    .filter_map(|row| {
                        let top = text.pos.y + row.pos.y;
                        Some((top, top + row.glyphs.first()?.pos.y))
                    })
                    .collect(),
            );
        }
    });
    found
}

/// The row tops of each visible painted paragraph that contains `paragraph`.
pub(super) fn paragraph_row_tops(d: &Driver<'_>, paragraph: &str) -> Vec<Vec<f32>> {
    let mut found = Vec::new();
    d.each_painted_text(|text, clip| {
        if text.galley.job.text.contains(paragraph) && clip.intersects(text.visual_bounding_rect())
        {
            found.push(
                text.galley
                    .rows
                    .iter()
                    .map(|row| text.pos.y + row.pos.y)
                    .collect(),
            );
        }
    });
    found
}

/// A note under a menu choice (its blocked reason or its help) starts at the choice's words, after
/// the padding and any icon, and ends at least the choice's padding `S` before the menu content's
/// right edge (iter-22-B FT2), with the menu inside the window. Requires the choice and at least
/// one painted note below it.
fn note_starts_at_choice_words(d: &Driver<'_>, choice: &str, note: &str) -> Result<(), String> {
    let target = d
        .nodes()
        .into_iter()
        .find(|node| node.name == choice && node.in_view())
        .ok_or(format!("{choice} is not in view"))?
        .rect;
    let words = words_left_in(d, target).ok_or(format!("{choice} paints no words"))?;
    let mut notes = Vec::new();
    d.each_painted_text(|text, clip| {
        let rect = egui::Rect::from_min_size(text.pos, text.galley.size());
        if text.galley.job.text == note && clip.intersects(rect) {
            notes.push(rect);
        }
    });
    if notes.is_empty() {
        return Err(format!("{note} is not painted"));
    }
    let window = d.window_size();
    notes
        .iter()
        .all(|rect| {
            (rect.left() - words).abs() <= 1.0
                && rect.top() >= target.bottom() - 1.0
                && rect.right() <= target.right() - crate::design::S + 0.5
        })
        .then_some(())
        .filter(|()| target.right() <= window.0 && target.bottom() <= window.1)
        .ok_or(format!(
            "{choice} words at {words}, {target:?}; {note} at {notes:?}; window {window:?}"
        ))
}

/// The tones of the bars beside each visible painted paragraph that holds `words`, in paint
/// order: "error", "warning" or "none". Empty when no such paragraph is visible.
pub(super) fn tones_beside(d: &Driver<'_>, words: &str) -> Vec<&'static str> {
    tones_where(d, |text| text.contains(words))
}

/// The painted rect of the visible paragraph that starts with `words`.
pub(super) fn painted_paragraph(d: &Driver<'_>, words: &str) -> Option<egui::Rect> {
    let mut found = None;
    d.each_painted_text(|text, clip| {
        let rect = egui::Rect::from_min_size(text.pos, text.galley.size());
        if text.galley.job.text.starts_with(words) && clip.intersects(rect) {
            found = Some(rect);
        }
    });
    found
}

/// The tones of the bars beside each visible painted paragraph whose text passes `wanted`.
fn tones_where(d: &Driver<'_>, wanted: impl Fn(&str) -> bool) -> Vec<&'static str> {
    let mut paragraphs = Vec::new();
    d.each_painted_text(|text, clip| {
        let rect = egui::Rect::from_min_size(text.pos, text.galley.size());
        if wanted(&text.galley.job.text) && clip.intersects(rect) {
            paragraphs.push(rect);
        }
    });
    let result = crate::design::colors().result;
    paragraphs
        .into_iter()
        .map(|rect| {
            match (
                d.has_tone_bar(rect, result.error),
                d.has_tone_bar(rect, result.warning),
            ) {
                (true, false) => "error",
                (false, true) => "warning",
                _ => "none",
            }
        })
        .collect()
}

/// A group's `Move to…` submenu and its parent menu stay in the window. The submenu's frame and
/// every choice keep `S` clear of each scroll bar and column splitter beside them; the parent
/// menu's frame does not overlap one.
/// Requires every choice of both menus. Returns the submenu's frame.
fn submenu_clear_of_chrome(
    d: &Driver<'_>,
    choices: &[&str],
    beside_a_lane: bool,
) -> Result<egui::Rect, String> {
    let nodes = d.nodes();
    let opener = d
        .place_of("Move to…")
        .ok_or("Move to… is not in view".to_owned())?;
    let margin = egui::Frame::menu(&d.app_style()).total_margin();
    // A page can show the same words elsewhere: each choice is the one level with the opener's
    // row or below it, nearest to it.
    let targets_of = |names: &[&str]| -> Result<egui::Rect, String> {
        let mut frame = egui::Rect::NOTHING;
        for name in names {
            frame = frame.union(
                nodes
                    .iter()
                    .filter(|node| {
                        node.name == *name
                            && node.in_view()
                            && node.rect.top() >= opener.top() - 1.0
                    })
                    .min_by(|a, b| {
                        (a.rect.top() - opener.top()).total_cmp(&(b.rect.top() - opener.top()))
                    })
                    .ok_or(format!("{name} is not in view beside Move to…"))?
                    .rect,
            );
        }
        Ok(frame)
    };
    let submenu = targets_of(choices)?;
    let menu = targets_of(&[
        "Move to…",
        "Make available to any encounter",
        "Delete group…",
    ])?;
    let window = d.window_size();
    let lanes = d.chrome_lanes();
    for (what, targets) in [("submenu", submenu), ("menu", menu)] {
        let frame = targets + margin;
        if frame.left() < 0.0
            || frame.right() > window.0
            || frame.top() < 0.0
            || frame.bottom() > window.1
        {
            return Err(format!("{what} {frame:?} leaves the window {window:?}"));
        }
        let beside: Vec<_> = lanes
            .iter()
            .filter(|lane| lane.bottom() > frame.top() && lane.top() < frame.bottom())
            .collect();
        if beside_a_lane && beside.is_empty() {
            return Err(format!(
                "no scroll bar or splitter beside the {what} {frame:?}: {lanes:?}"
            ));
        }
        for lane in beside {
            let clear =
                |rect: egui::Rect| (lane.left() - rect.right()).max(rect.left() - lane.right());
            // The submenu's whole frame keeps `S`. A parent menu's frame may sit at its pane's
            // content edge (the gallery's starts 7 px after the rail splitter, as all page
            // content does); it must not overlap, and its choices keep `S`.
            let frame_needs = if what == "submenu" {
                crate::design::S - 0.5
            } else {
                0.0
            };
            if clear(frame) < frame_needs || clear(targets) < crate::design::S - 0.5 {
                return Err(format!(
                    "{what} frame {frame:?} is {} and choices {targets:?} are {} from {lane:?}; lanes {lanes:?}",
                    clear(frame),
                    clear(targets)
                ));
            }
        }
    }
    Ok(submenu + margin)
}

/// A warning's file line starts at the message words: the path, whole, then `Edit in Obsidian`
/// and `Show file`. A link that wraps starts its row at the message words too.
fn file_links_at_message_words(
    d: &Driver<'_>,
    message: &str,
    path: &std::path::Path,
    relative: &str,
) -> Result<(), String> {
    let nodes = d.nodes();
    let words = nodes
        .iter()
        .find(|node| node.name.starts_with(message) && node.in_view())
        .ok_or(format!("{message}… is not in view"))?
        .rect;
    let shown = nodes
        .iter()
        .find(|node| node.name == path.display().to_string() && node.in_view())
        .ok_or(format!("{} is not in view", path.display()))?
        .rect;
    let painted = d.painted_in(shown);
    let path_left = words_left_in(d, shown);
    if !painted.contains(relative)
        || painted.contains('…')
        || shown.top() < words.bottom() - 1.0
        || path_left.is_none_or(|left| (left - words.left()).abs() > 1.0)
    {
        return Err(format!(
            "message words {words:?}; path {shown:?} paints {painted:?} from {path_left:?}"
        ));
    }
    for link in ["Edit in Obsidian", "Show file"] {
        let target = nodes
            .iter()
            .filter(|node| {
                node.name == link && node.in_view() && node.rect.center().y >= shown.top()
            })
            .min_by(|a, b| a.rect.top().total_cmp(&b.rect.top()))
            .ok_or(format!("{link} is not in view below the message"))?
            .rect;
        let left = words_left_in(d, target).ok_or(format!("{link} paints no words"))?;
        let wrapped = target.center().y > shown.bottom();
        if left < words.left() - 1.0 || (wrapped && (left - words.left()).abs() > 1.0) {
            return Err(format!(
                "{link} words at {left}, message words at {}",
                words.left()
            ));
        }
    }
    Ok(())
}

/// The colours of `⚠ HP not set` in each visible painted paragraph that holds it, icon and words.
fn hp_not_set_colours(d: &Driver<'_>) -> Vec<Vec<egui::Color32>> {
    let fault = format!("{} HP not set", crate::design::icons::WARN);
    let kept = fault.replace(' ', "\u{a0}");
    let mut found = Vec::new();
    d.each_painted_text(|text, clip| {
        let job = &text.galley.job;
        if !clip.intersects(text.visual_bounding_rect()) {
            return;
        }
        let Some(range) = [&fault, &kept]
            .into_iter()
            .find_map(|words| job.text.find(words.as_str()).map(|at| at..at + words.len()))
        else {
            return;
        };
        found.push(
            job.sections
                .iter()
                .filter(|section| {
                    section.byte_range.start < range.end && range.start < section.byte_range.end
                })
                .map(|section| section.format.color)
                .collect(),
        );
    });
    found
}

fn words_left(d: &Driver<'_>, name: &str) -> Option<f32> {
    let area = d
        .nodes()
        .into_iter()
        .find(|node| node.name == name && node.in_view())?
        .rect;
    words_left_in(d, area)
}

/// The painted line that holds these words, and the named control nearest below it. A name
/// can be on two controls (a header link and a dialog's action); this is the one under the text.
fn line_and_action_below(
    d: &Driver<'_>,
    above: &str,
    action: &str,
) -> Option<(egui::Rect, crate::visual_tests::driver::Found)> {
    let mut line = None;
    d.each_painted_text(|text, _| {
        if text.galley.job.text.contains(above) {
            line = Some(egui::Rect::from_min_size(text.pos, text.galley.size()));
        }
    });
    let line = line?;
    let node = d
        .nodes()
        .into_iter()
        .filter(|node| {
            node.name == action && node.in_view() && node.rect.top() >= line.bottom() - 1.0
        })
        .min_by(|a, b| a.rect.top().total_cmp(&b.rect.top()))?;
    Some((line, node))
}

/// A control's press area overlaps no splitter and no other control. `among` limits the others
/// to these names, for a modal dialog whose page stays in the tree behind it. On success, says
/// which splitter is nearest and how far.
fn press_area_clear(
    d: &Driver<'_>,
    target: egui::Rect,
    action: &str,
    among: Option<&[&str]>,
) -> Result<String, String> {
    use egui::accesskit::Role;
    const PRESSABLE: [Role; 12] = [
        Role::Button,
        Role::DefaultButton,
        Role::Link,
        Role::CheckBox,
        Role::ComboBox,
        Role::TextInput,
        Role::MultilineTextInput,
        Role::Splitter,
        Role::Slider,
        Role::SpinButton,
        Role::Tab,
        Role::RadioButton,
    ];
    let mut nearest = None::<(f32, String)>;
    for node in d.nodes() {
        if (node.name == action && (node.shown.center() - target.center()).length() < 1.0)
            || !node.in_view()
            || !PRESSABLE.contains(&node.role)
            || among.is_some_and(|names| !names.contains(&node.name.as_str()))
        {
            continue;
        }
        // A row or panel that holds the action is not a second target.
        if node.rect.contains_rect(target) && node.rect.area() > 4.0 * target.area() {
            continue;
        }
        // What a press can reach: the part of each control in view.
        let overlap = node.shown.intersect(target);
        if overlap.is_positive() && overlap.area() > 0.5 {
            return Err(format!(
                "{action} {target:?} overlaps {} {:?}",
                node.name, node.shown
            ));
        }
        if node.role == Role::Splitter {
            let gap = (target.left() - node.shown.right()).max(node.shown.left() - target.right());
            if nearest.as_ref().is_none_or(|(near, _)| gap < *near) {
                nearest = Some((gap, node.name.clone()));
            }
        }
    }
    Ok(nearest.map_or_else(
        || "no splitter compared".to_owned(),
        |(gap, name)| format!("{gap:.1} px from {name}"),
    ))
}

/// A quiet action that starts a line: its words start at the text edge of the line above
/// (`above`), its press area clears every splitter, and no other press area overlaps it.
fn check_line_start(
    d: &mut Driver<'_>,
    id: &str,
    action: &str,
    above: &str,
    sizes: &[(f32, f32)],
    among: Option<&[&str]>,
) {
    check_start(d, id, action, above, sizes, among, false);
}

/// A bordered action that starts a line: its frame starts at the text edge of the line above
/// and its words sit inside the frame, so it is a button and not a label. Its press area clears
/// every splitter, and no other press area overlaps it.
fn check_frame_start(
    d: &mut Driver<'_>,
    id: &str,
    action: &str,
    above: &str,
    sizes: &[(f32, f32)],
    among: Option<&[&str]>,
) {
    check_start(d, id, action, above, sizes, among, true);
}

fn check_start(
    d: &mut Driver<'_>,
    id: &str,
    action: &str,
    above: &str,
    sizes: &[(f32, f32)],
    among: Option<&[&str]>,
    frame: bool,
) {
    for &size in sizes {
        let (words, edge, clear) = d.at_size(size, |d| {
            let Some((line, node)) = line_and_action_below(d, above, action) else {
                return (None, None, Err(format!("no {action} below {above:?}")));
            };
            let words = words_left_in(d, node.rect);
            // A frame starts at the text, and its words start inside it. It has a border.
            let start = if frame {
                words
                    .filter(|words| *words > node.rect.left() + 1.0)
                    .filter(|_| {
                        d.control_frame_at(node.rect)
                            .is_some_and(|frame| frame.stroke.width > 0.0)
                    })
                    .map(|_| node.rect.left())
            } else {
                words
            };
            (
                start,
                Some(line.left()),
                press_area_clear(d, node.shown, action, among),
            )
        });
        let part = if frame { "frame" } else { "words" };
        d.observe(
            id,
            words
                .zip(edge)
                .is_some_and(|(words, edge)| (words - edge).abs() <= 1.0),
            format!(
                "At {size:?}, {action}'s {part} start at {words:?}, the text above at {edge:?}"
            ),
        );
        d.observe(
            &format!("{id}-clear"),
            clear.is_ok(),
            format!("At {size:?}, {action}'s press area: {clear:?}"),
        );
    }
}

/// Each affected encounter remains a separate link with a word space on both sides of `and`.
fn check_link_spaces(d: &mut Driver<'_>) {
    let first = d
        .place_of("Bridge Ambush")
        .expect("First affected encounter link");
    let second = d
        .place_of("Shrine of Cinders")
        .expect("Second affected encounter link");
    let area = first.union(second).expand(crate::design::XS);
    let conjunction = d
        .painted_word_rect("and", area)
        .expect("The conjunction is painted");
    let first = d
        .painted_word_rect("Bridge Ambush", area)
        .expect("First link is painted");
    let second = d
        .painted_word_rect("Shrine of Cinders", area)
        .expect("Second link is painted");
    let before = conjunction.left() - first.right();
    let after = second.left() - conjunction.right();
    d.observe(
        "iter-6-C-SD4",
        before > 1.0 && after > 1.0 && (before - after).abs() <= 1.0,
        format!("Affected encounter links have word spaces: {before}, {after}"),
    );
}

fn escape_closes(d: &mut Driver<'_>, view: &str, still_open: &str, close_with: &str) {
    d.press(Key::Escape);
    let closed = !d.has_containing(still_open);
    d.keyboard_result(
        "escape-closes",
        view,
        closed,
        format!(
            "Escape {} it",
            if closed { "closes" } else { "does not close" }
        ),
    );
    if !closed {
        d.click(close_with);
    }
}

/// W6: find content in the built-in library, read it, and return to the list.
pub mod library_browse {
    use super::*;
    const ENTRY_COUNT: &str = "2,065 entries";
    const SEARCH_RESULTS: &str = "SEARCH RESULTS";
    const CREATURE: &str = "Adult Black Dragon";
    const SPELL_ROW: &str = "Fireball · ";
    const CLASS_ROW: &str = "Wizard · Class";
    const CREATURES_SECTION: &str = "Creatures";
    const CREATURES_COUNT: &str = "CREATURES 336";
    const NEXT_CREATURE: &str = "Aboleth";
    const LINKED_QUERY: &str = "bard subclass";
    const LINKED_ROW: &str = "Level 3: Bard Subclass";
    const LINKED_TITLE: &str = "Bard Subclass";
    const LINK: &str = "College of Lore";

    pub fn run(d: &mut Driver<'_>) {
        d.expect(ENTRY_COUNT);
        d.observe_both(
            "i20 C4 header space",
            "Without notices, the message and columns keep their audit-18 tops",
            |d| {
                d.place_of("Add campaign")
                    .is_some_and(|rect| (rect.top() - 67.0).abs() < 1.0)
                    && d.place_of("What the Library shows: Everything")
                        .is_some_and(|rect| (rect.top() - 105.0).abs() < 1.0)
            },
        );
        d.state("first-launch", Sizes::AllAndHiDpi);
        d.check_tab_order("library, first launch");
        d.begin_task();

        search_library(d, "dragon");
        d.expect(SEARCH_RESULTS);
        d.state("search-dragon", Sizes::All);
        d.goal("search for dragons");

        d.click_containing(CREATURE);
        d.expect(CREATURE);
        d.state("creature-stat-block", Sizes::AllAndHiDpi);
        d.goal("open a creature stat block");
        d.review(
            "T6.4",
            "the Library reader, to compare with a campaign entry and a run",
        );

        search_library(d, "fireball");
        d.click_starting(SPELL_ROW);
        let reader_at_left = |d: &Driver<'_>| {
            d.painted_left("Fireball").is_some_and(|left| {
                (left - d.column_edge("library_entries") - crate::design::L).abs() < 2.0
            })
        };
        let large_anchor = d.at_size(LARGE, reader_at_left);
        d.observe_both(
            "iter-1-A-reader-anchor",
            "The Library reader starts beside the list at all three window sizes",
            |d| large_anchor && reader_at_left(d),
        );
        d.state("spell", Sizes::All);
        d.goal("open a spell");

        search_library(d, "wizard");
        d.scroll_until(ENTRY_LIST, CLASS_ROW, 80);
        d.click_containing(CLASS_ROW);
        d.state("class-traits-and-features", Sizes::All);
        d.goal("open a class");

        let default_width = d.at_size(DEFAULT, Driver::longest_visible_line);
        let large_width = d.at_size(LARGE, Driver::longest_visible_line);
        // Zero would mean no text was found, which is no pass.
        let long_entry = (20..=75).contains(&default_width) && (20..=75).contains(&large_width);
        // The same measure beside a page, in the panel's smaller prose.
        d.begin_task();
        look_up(d, "fireball");
        let panel_default = d.at_size(DEFAULT, Driver::longest_visible_line);
        let panel_large = d.at_size(LARGE, Driver::longest_visible_line);
        d.press(Key::Escape);
        d.begin_task();
        d.observe(
            "T6.1",
            long_entry && (20..=75).contains(&panel_default) && (20..=75).contains(&panel_large),
            format!(
                "Longest line, class: {default_width} at 1200x800, {large_width} at 1920x1080. Lookup: {panel_default}, {panel_large}"
            ),
        );

        d.scroll_at(READER, 12);
        d.state("class-scrolled", Sizes::Default);
        d.goal("scroll down a long entry");

        // Follow a link in the reader, then come back in one click.
        d.begin_task();
        search_library(d, LINKED_QUERY);
        d.click_containing(LINKED_ROW);
        d.click(LINK);
        d.state("followed-link", Sizes::Default);
        let wide_toolbar = d.has("Back") && d.visible_text("Alt+Left");
        let away = !d.in_view(LINKED_TITLE);
        let before = d.interaction_count();
        d.click("Back");
        let cost = d.interaction_count() - before;
        d.observe(
            "T6.2",
            wide_toolbar
                && away
                && cost == 1
                && d.in_view(LINKED_TITLE)
                && d.has_containing(SEARCH_RESULTS),
            "Back returns to the prior entry and its search in one click, with Alt+Left shown",
        );
        d.state("back-after-link", Sizes::Default);
        d.begin_task();
        d.not_measured(
            "T6.3",
            "frame times are not stable while scenarios share the CPU",
        );

        d.click("Section");
        d.click(&format!("Library section: {CREATURES_SECTION}"));
        d.expect(CREATURES_COUNT);
        d.state("creature-list", Sizes::Default);
        d.click_containing(NEXT_CREATURE);
        d.expect(NEXT_CREATURE);
        d.goal("return through the list and open another entry");
    }
}

/// Find creatures by habitat, type and other facts, with Filter and with search terms.
pub mod creature_filter {
    use super::*;
    const CREATURES_COUNT: &str = "CREATURES 336";
    const FILTER: &str = "Filter creatures";
    const UNDERDARK: &str = "Remove filter Habitat: Underdark";
    const ABERRATION: &str = "Remove filter Type: Aberration";
    const SEARCH_RESULTS: &str = "SEARCH RESULTS";

    pub fn run(d: &mut Driver<'_>) {
        d.click("Section");
        d.click("Library section: Creatures");
        d.expect(CREATURES_COUNT);
        d.begin_task();

        d.click(FILTER);
        // egui names a submenu button with its arrow, `Habitat ⏵`.
        d.click_starting("Habitat");
        d.click("Filter Habitat: Underdark");
        d.click_starting("Type");
        d.click("Filter Type: Aberration");
        d.press(Key::Escape);
        d.expect(UNDERDARK);
        d.expect(ABERRATION);
        d.expect("MATCHES 9");
        d.state("underdark-aberrations", Sizes::All);
        // The Aboleth's Skills row: `History +12, Perception +10`.
        check_skills_row(
            d,
            "iter-21-S-fd4-library-skills",
            2,
            &[SMALL, DEFAULT, LARGE],
        );
        d.goal("find the aberrations of the Underdark");
        d.observe(
            "creature filter",
            d.has_containing("Aboleth") && !d.has_containing("Adult Black Dragon"),
            "Habitat and Type keep the Aboleth and drop a swamp dragon",
        );

        d.begin_task();
        d.click(ABERRATION);
        d.expect(UNDERDARK);
        d.observe(
            "creature filter chip",
            !d.has(ABERRATION) && d.has_containing("Archmage") && d.has_containing("Basilisk"),
            "Removing the Type chip keeps the habitat; Archmage (Any) and Basilisk return",
        );
        d.click(UNDERDARK);
        d.expect(CREATURES_COUNT);
        d.goal("remove the filters");

        d.begin_task();
        search_library(d, "immune:fire cr:17+");
        d.expect(SEARCH_RESULTS);
        d.state("search-terms", Sizes::Default);
        check_skills_row(
            d,
            "iter-21-S-fd4-dragon-skills",
            2,
            &[SMALL, DEFAULT, LARGE],
        );
        d.goal("search by immunity and challenge");
        d.observe(
            "creature search terms",
            d.has_containing("Ancient Red Dragon") && !d.has_containing("Aboleth"),
            "immune:fire cr:17+ finds an ancient red dragon only among fire-immune creatures",
        );
    }
}

/// W8: edit a note in a local package, then manage packages and add an entry.
pub mod library_author {
    use super::*;
    const LOCAL_PACKAGE: &str = "Ashen Marches · Chosen for this campaign";
    const NOTE: &str = "Ash Exposure";
    const NOTE_ROW: &str = "Ash Exposure · ";
    const OTHER_NOTE: &str = "Ashfall";
    const NOTE_FILE: &str = "modules/ashen-marches/Rules/Ash Exposure.md";
    const SAVE: &str = "Save";
    const DISCARD: &str = "Discard";
    const DRAFT_WORDS: &str = "Rangers of the ash line are exempt. ";
    const SECOND_WORDS: &str = "Rangers carry spare cloth. ";
    const THIRD_WORDS: &str = "Cloth wears out in a week. ";
    const OTHER_NOTE_TEXT: &str = "Hot ash falls";
    const OVERWRITE: &str = "Overwrite with my draft";
    const RELOAD_DISK: &str = "Reload and lose my draft";
    const SHOW_DISK: &str = "Next difference";
    const UNDO_DISCARD: &str = "Undo discard of Ash Exposure";
    const CHANGED_ON_DISK: &str = "This entry changed on disk";
    const PACKAGE_ACTIONS: &str = "Packages";
    const MANAGE_PACKAGES: &str = "Manage packages…";
    const MANAGE_TITLE: &str = "Manage packages";
    const CLOSE_WINDOW: &str = "Close the package form";
    const NEW_ENTRY: &str = "New entry";
    const CREATE_ENTRY: &str = "Create entry";
    const NEW_NAME: &str = "Cinder Hound";
    const NEW_NAME_FIELD: &str = "Entry name";

    pub fn run(d: &mut Driver<'_>) {
        // The app opens on Encounters when a campaign is active.
        go_library(d);
        d.begin_task();
        d.open_combo_at(LIBRARY_PACKAGE_DROPDOWN);
        d.click(MODULE);
        d.goal("open a local package");
        d.click_starting(NOTE_ROW);
        d.expect(LOCAL_PACKAGE);
        d.state("local-package", Sizes::All);
        d.check_tab_order("library, local package");

        d.expect(NOTE);
        d.observe(
            "Package scope",
            d.has_containing(NOTE) && !d.has_containing("Adult Red Dragon"),
            "Choosing a package limits the catalogue to it. The active campaign stays",
        );
        d.click(EDIT_MARKDOWN);
        d.state("edit-markdown", Sizes::All);
        let edges_match = |d: &Driver<'_>| {
            let (frame, clip) = d.input_frame("Markdown editor").unwrap();
            let save = d.place_of("Save").unwrap();
            (frame.right() - save.right()).abs() <= 1.0 && clip.expand(1.0).contains_rect(frame)
        };
        d.observe_both(
            "C3",
            "The editor and save row share a right edge",
            edges_match,
        );
        let wide = d.at_size((1920.0, 1080.0), edges_match);
        d.observe(
            "C3",
            wide,
            "The editor keeps the save row edge at 1920x1080",
        );
        d.observe(
            "Clean word",
            d.visible_text("No changes"),
            "A clean document says so, in one word, as Encounters and Campaign do",
        );
        d.goal("open a note for editing");

        d.type_into_editor(DRAFT_WORDS);
        d.expect(UNSAVED_NOTE);
        d.state("dirty", Sizes::All);
        d.goal("change the note");

        let other_row_locked = d.is_disabled(OTHER_NOTE);
        d.click_containing(OTHER_NOTE);
        let draft_here =
            |d: &Driver<'_>, words: &str| d.editor_text().is_some_and(|text| text.contains(words));
        d.observe(
            "T8.2",
            !other_row_locked
                && d.visible_text(OTHER_NOTE_TEXT)
                && d.has_containing(UNSAVED_NOTE)
                && draft_here(d, DRAFT_WORDS),
            "Read another entry while a draft is open. Its text opens in Lookup, and the draft's words stay in the editor",
        );
        {
            let size = DEFAULT;
            let ordered = d.at_size(size, |d| {
                for name in [
                    "What the Library shows: Ashen Marches",
                    "Section",
                    "Search the Library",
                    "About Ashen Marches",
                ] {
                    d.expect(name);
                }
                let scope = d.place_of("What the Library shows: Ashen Marches").unwrap();
                let category = d.place_of("Section").unwrap();
                let search = d.place_of("Search the Library").unwrap();
                let about = d.place_of("About Ashen Marches").unwrap();
                scope.bottom().max(category.bottom()) <= search.top()
                    && search.bottom() <= about.top()
            });
            d.observe(
                "iter-7-D-FD10",
                ordered,
                format!("Scope and category precede search, then About at {size:?}"),
            );
        }
        d.state("read-beside-draft", Sizes::All);
        d.press(Key::Escape);
        d.begin_task();

        d.press_ctrl(Key::S);
        let saved = !d.has_containing(UNSAVED_NOTE);
        d.keyboard_result(
            "ctrl-s",
            "library, dirty note",
            saved,
            "Ctrl+S saves a dirty note, as artifacts/ui-review.md says".into(),
        );
        d.expect("Saved");
        assert!(
            std::fs::read_to_string(d.root.join(NOTE_FILE))
                .unwrap()
                .contains(DRAFT_WORDS)
        );
        d.expect_same("A normal save keeps the editor layout; the save word and typed sentence are the only changes.");
        d.state("saved-with-ctrl-s", Sizes::All);
        d.goal("save with the keyboard");

        // External change: the draft stays, and each way out is a control that works.
        d.begin_task();
        d.type_into_editor(SECOND_WORDS);
        let on_disk = d.root.join(NOTE_FILE);
        let edited =
            std::fs::read_to_string(&on_disk).expect("read note") + "\nEdited elsewhere.\n";
        std::fs::write(&on_disk, edited).expect("edit note on disk");
        d.click(SAVE);
        d.observe(
            "T8.3",
            d.has_containing(CHANGED_ON_DISK)
                && d.has(OVERWRITE)
                && d.has(RELOAD_DISK)
                && !d.has(SHOW_DISK)
                && d.visible_text("YOUR DRAFT"),
            "An external change keeps the draft's words, and offers overwrite, reload and the disk version",
        );
        d.state_prepared("save-conflict", Sizes::All, |d| {
            choices_at_text(
                d,
                "iter-15-E-AD3-library",
                CHANGED_ON_DISK,
                OVERWRITE,
                RELOAD_DISK,
                Some("YOUR DRAFT"),
            );
            conflict_choices_read_height(d, "iter-15-B2-density-library", OVERWRITE, RELOAD_DISK);
            // Each column is (width - gap) / 2 wide, so the second starts (width + gap) / 2 after
            // the first; the theme's gap is S. At 1920x1080 the side-by-side comparison takes the
            // full pane, wider than the source editor (a 720 px cap measures exactly 720).
            if d.window_size() == (1920.0, 1080.0) {
                let draft = d.painted_rect_in("YOUR DRAFT", egui::Rect::EVERYTHING);
                let disk = d.painted_rect_in("IN THE FILE NOW", egui::Rect::EVERYTHING);
                let width = draft.zip(disk).map_or(0.0, |(draft, disk)| {
                    (disk.left() - draft.left()) * 2.0 - crate::design::S
                });
                d.observe(
                    "iter-13-E2-AD3-comparison",
                    width > crate::library::EDITOR_MAX_WIDTH + 1.0,
                    format!("The draft and disk comparison is {width:.0} px wide"),
                );
            }
        });
        d.goal("handle a failed save");
        d.expect("IN THE FILE NOW");
        d.observe(
            "C2 grouped changes",
            !d.has(SHOW_DISK),
            "Adjacent differences need no Next difference action",
        );
        d.observe(
            "T8.3",
            d.has_containing("Edited elsewhere."),
            "The disk version can be read before choosing",
        );
        d.click(RELOAD_DISK);
        d.observe(
            "T8.3",
            draft_here(d, "Edited elsewhere.")
                && !draft_here(d, SECOND_WORDS)
                && d.has(UNDO_DISCARD),
            "Reloading takes the disk version, and the draft is one undo away",
        );
        d.click(UNDO_DISCARD);
        d.observe(
            "Discard undo",
            draft_here(d, SECOND_WORDS),
            "Undo brings the dropped draft back",
        );
        d.click(SAVE);
        d.observe(
            "T8.3",
            !d.has_containing(CHANGED_ON_DISK)
                && std::fs::read_to_string(&on_disk).is_ok_and(|text| text.contains(SECOND_WORDS)),
            "After the reload, the same draft saves",
        );

        // The same way out when the file changes again: overwrite.
        d.type_into_editor(THIRD_WORDS);
        let edited = std::fs::read_to_string(&on_disk).expect("read note") + "\nEdited again.\n";
        std::fs::write(&on_disk, edited).expect("edit note on disk");
        d.click(SAVE);
        d.click(OVERWRITE);
        let text = std::fs::read_to_string(&on_disk).expect("read note");
        d.observe(
            "T8.3",
            text.contains(THIRD_WORDS) && !text.contains("Edited again."),
            "Overwrite puts the draft on disk in place of the other change",
        );

        // Invalid note: Save would refuse it, so the state says Invalid with Save's reason and
        // Save is blocked before any write; the words stay. (Round 23: a ready state means Save
        // would accept the draft.)
        d.type_into_editor(THIRD_WORDS);
        d.replace_editor_text("no front matter at all");
        let reason = crate::campaign_content::repair_problem(
            "Rules/Ash Exposure.md",
            "no front matter at all",
        )
        .0
        .or_else(|| {
            splat::content::Entry::parse("Rules/Ash Exposure.md", "no front matter at all").err()
        })
        .expect("the draft is refused");
        d.observe(
            "T8.3",
            d.is_disabled(SAVE)
                && d.visible_text("Invalid")
                && !d.visible_text("Unsaved")
                && d.has_containing(&reason)
                && draft_here(d, "no front matter at all"),
            format!("An invalid note blocks Save, keeps the draft and states why: {reason}"),
        );
        d.click(DISCARD);
        d.observe(
            "Discard undo",
            d.has(UNDO_DISCARD) && !draft_here(d, "no front matter at all"),
            "Discard takes one press, and offers undo in place of a dialog",
        );
        d.click(UNDO_DISCARD);
        d.observe(
            "Discard undo",
            draft_here(d, "no front matter at all") && !d.has(UNDO_DISCARD),
            "Undo brings back the discarded draft, and the discard message goes",
        );
        let editor_top = d
            .nodes()
            .into_iter()
            .find(|node| node.name == "Markdown editor")
            .expect("editor before discard")
            .rect
            .top();
        let editor_at = |d: &Driver<'_>| d.place_of("Markdown editor").map(|rect| rect.top());
        let dirty_tops: Vec<_> = [SMALL, DEFAULT, LARGE]
            .into_iter()
            .map(|size| (size, d.at_size(size, editor_at)))
            .collect();
        d.click(DISCARD);
        let discarded_top = d
            .nodes()
            .into_iter()
            .find(|node| node.name == "Markdown editor")
            .expect("editor after discard")
            .rect
            .top();
        d.observe(
            "C5",
            editor_top == discarded_top,
            format!("Discard keeps the editor at the same height: {editor_top} to {discarded_top}"),
        );
        d.state_prepared("library-draft-discarded", Sizes::All, |d| {
            // iter-12-E-FD3: the Library uses the shared discard message in its reason slot.
            let size = d.window_size();
            let words = "Ash Exposure draft discarded.";
            let window = egui::Rect::from_min_size(pos2(0.0, 0.0), egui::vec2(size.0, size.1));
            let editor = d.place_of("Markdown editor").expect("The editor stays open");
            let Some(text) = d.painted_rect_in(words, window) else {
                panic!("At {size:?}, the discard message is painted");
            };
            let icon = d
                .painted_rect_in(
                    crate::design::icons::INFO,
                    egui::Rect::from_min_max(
                        pos2(editor.left(), text.top() - 2.0),
                        pos2(text.left(), text.bottom() + 2.0),
                    ),
                )
                .is_some();
            let painted = d.painted_size(text, words);
            let colour = d.painted_color(words);
            d.observe(
                "iter-12-E-FD3",
                icon && painted == Some(crate::design::Type::Body.size())
                    && colour == Some(crate::design::colors().text),
                format!(
                    "At {size:?}, the discard line has the info icon, Body text and the text colour: {painted:?}, {colour:?}"
                ),
            );
            // Round 26 lead ruling: the message sits in the pane's footer, below the editor.
            d.observe(
                "iter-12-E-FD3-slot",
                text.top() > editor.bottom() && text.left() > editor.left(),
                format!("At {size:?}, the message sits in the footer below the editor: {text:?}, {editor:?}"),
            );
            check_undo_icon(d, UNDO_DISCARD, "iter-12-E-FD3-undo");
            // Must stay true: the editor does not move when the reason becomes the message.
            let before = dirty_tops
                .iter()
                .find(|(at, _)| *at == size)
                .and_then(|(_, top)| *top);
            let after = editor_at(d);
            d.observe(
                "iter-12-E-FD3-steady",
                before.is_some() && before == after,
                format!("At {size:?}, Discard keeps the editor top: {before:?} to {after:?}"),
            );
        });
        d.goal("try each way out of a failed save");

        d.click(PACKAGE_ACTIONS);
        d.state("package-actions", Sizes::Default);
        d.click(MANAGE_PACKAGES);
        d.expect(MANAGE_TITLE);
        d.state("manage-packages", Sizes::All);
        escape_closes(d, "manage packages form", MANAGE_TITLE, CLOSE_WINDOW);
        d.goal("open and close the package form");

        d.click(NEW_ENTRY);
        d.observe_both("W7", "A new creature starts in Creatures", |d| {
            d.field_text("Folder").as_deref() == Some("Creatures")
        });
        d.expect("Type a new folder, or choose one. A new folder is made with the file.");
        for size in [SMALL, DEFAULT, LARGE] {
            let labels = d.at_size(size, |d| {
                ["Kind", "Destination"].into_iter().all(|word| {
                    d.expect(word);
                    d.place_of(word).is_some_and(|rect| {
                        d.painted_size(rect, word) == Some(crate::design::Type::Small.size())
                    }) && !d.visible_text(&word.to_uppercase())
                })
            });
            d.observe(
                "iter-7-D-SD1",
                labels,
                format!("Peer field labels use sentence-case Small at {size:?}"),
            );
        }
        d.state("new-entry-form", Sizes::All);
        d.type_into(NEW_NAME_FIELD, NEW_NAME);
        d.click(CREATE_ENTRY);
        d.expect(NEW_NAME);
        d.state("new-entry-created", Sizes::All);
        d.goal("create a new entry");

        // Creation closes the form. Open it again to check a name clash.
        d.observe(
            "New entry form",
            !d.has(NEW_NAME_FIELD),
            "The creation form closes after the entry is made",
        );
        d.click(NEW_ENTRY);
        d.type_into(NEW_NAME_FIELD, NEW_NAME);
        d.click(CREATE_ENTRY);
        d.observe(
            "T8.3",
            d.has_containing("already exists")
                && d.field_text(NEW_NAME_FIELD).as_deref() == Some(NEW_NAME),
            "A name clash states the fix, and the typed name stays",
        );
    }
}

/// Follow a homebrew reference across owners, then edit without losing the draft to Back.
pub mod library_navigation {
    use super::*;

    pub fn run(d: &mut Driver<'_>) {
        let path = d.root.join("modules/ashen-marches/Rules/Ash Exposure.md");
        let text = std::fs::read_to_string(&path).unwrap();
        let target_path = "Notes/Captain Varra.md";
        let target_text =
            std::fs::read_to_string(d.root.join("campaigns/emberfall").join(target_path)).unwrap();
        let target = splat::content::Entry::parse(target_path, &target_text).unwrap();
        let reading = serde_yaml_ng::to_value(serde_json::json!({
            "title": "Ash Exposure",
            "summary": "Captain Varra knows the road through the ash fields.",
            "related": {target.id.to_string(): "Read Captain Varra"}
        }))
        .unwrap();
        let text =
            crate::content_workspace::patch_fields(&text, &[("splat_reading", Some(reading))])
                .unwrap();
        let text = format!("{text}\n\n[[Ashfall|Wiki watch spell]]\n\n[[Missing place]]\n");
        std::fs::write(&path, text).unwrap();

        go_library(d);
        d.open_combo_at(LIBRARY_PACKAGE_DROPDOWN);
        d.click(MODULE);
        d.click_starting("Ash Exposure · ");
        // iter-12-E-FD2: the broken link counts in the header, as World counts its notes.
        let rows: Vec<_> = d
            .nodes()
            .into_iter()
            .filter(|node| node.name.starts_with("Ash Exposure · "))
            .collect();
        d.observe(
            "iter-12-E-FD2",
            !rows.is_empty()
                && rows.iter().all(|row| row.name.contains("1 broken link"))
                && d.in_view("1 file needs fixing"),
            "The row's broken link also counts in the Library header",
        );
        d.state("homebrew-reference", Sizes::All);
        // From another entry, the header count's menu opens the entry with the broken link.
        d.click_starting("Ashfall · ");
        d.wait_for_text("Hot ash falls");
        assert!(
            !d.visible_text("Captain Varra knows the road through the ash fields."),
            "fixture: another entry is open"
        );
        d.click("1 file needs fixing");
        d.expect("Rules/Ash Exposure.md: 1 broken link");
        d.state_prepared("broken-link-menu", Sizes::All, |d| {
            if !d.has("Rules/Ash Exposure.md: 1 broken link") {
                d.click("1 file needs fixing");
            }
            // Round 26 FB1: the menu is as wide as its content and ends at the count's right
            // edge, so its frame and its link keep `S` from every splitter and scroll bar.
            // Must stay true: the whole link label fits on one line in the menu.
            let count = d.place_of("1 file needs fixing");
            let link = d.place_of("Rules/Ash Exposure.md: 1 broken link");
            let words = d.painted_rect_in("Rules/Ash Exposure.md: 1 broken link", egui::Rect::EVERYTHING);
            let frame = link.and_then(|link| d.painted_frame_around(link));
            let lanes = d.chrome_lanes();
            let clear = |rect: egui::Rect| {
                lanes.iter().all(|lane| {
                    !lane.y_range().intersects(rect.y_range())
                        || !lane.x_range().intersects(rect.expand(crate::design::S).x_range())
                })
            };
            d.observe(
                "iter-26-L-FB1",
                count.zip(frame).zip(words).is_some_and(|((count, frame), words)| {
                    clear(frame)
                        && (frame.right() - count.right()).abs() <= crate::design::S
                        && frame.width() <= crate::design::READER_MEASURE + 2.0 * crate::design::M
                        && words.height() <= crate::design::Type::Small.line() + 0.5
                }),
                format!(
                    "The menu {frame:?} ends at the count {count:?}, its words {words:?} keep one line, and it is clear of the lanes {lanes:?}"
                ),
            );
        });
        d.click("Rules/Ash Exposure.md: 1 broken link");
        d.wait_for_text("Captain Varra knows the road through the ash fields.");
        d.observe(
            "iter-12-E-FD2-menu",
            !d.visible_text("Hot ash falls") && !d.has("Rules/Ash Exposure.md: 1 broken link"),
            "The header count's menu opens the entry with the broken link, and closes",
        );
        d.click("Copy Captain Varra to Ashen Marches");
        d.wait_for_text("Captain Varra copied to Ashen Marches and linked.");
        d.observe(
            "W1",
            !d.visible_text("Captain Varra is in this campaign only."),
            "Copying the dependency also reconnects the package entry",
        );
        for size in [SMALL, DEFAULT, LARGE] {
            let quiet_undo = d.at_size(size, |d| {
                d.expect("Undo copy");
                let rect = d.place_of("Undo copy").unwrap();
                d.control_frame("Undo copy")
                    .is_some_and(|frame| frame.stroke == egui::Stroke::NONE)
                    && d.painted_in(rect).contains(crate::design::icons::UNDO)
            });
            d.observe(
                "iter-7-D-SD2",
                quiet_undo,
                format!("Dependency Undo is quiet and bears the Undo icon at {size:?}"),
            );
        }
        d.state("dependency-copy-reconnected", Sizes::All);
        d.click("Undo copy");
        d.wait_for_text("Captain Varra is in this campaign only.");
        // A link in a paragraph is named by its words and where it goes (iter-16 AD1).
        d.click("Wiki watch spell (opens Ashfall)");
        d.wait_for_text("Hot ash falls");
        d.observe(
            "C4",
            !d.has("Close Lookup"),
            "A wiki link opens in the Library reader",
        );
        d.observe_both(
            "Library toolbar anchor",
            "Read stays before Back after following a link",
            |d| d.place_of("Read").unwrap().left() < d.place_of("Back").unwrap().left(),
        );
        d.state("wiki-link-in-library", Sizes::All);
        // Round 26 SF1, SB1: a list click records the trail as a link does. Link Ash Exposure →
        // Ashfall, list click About Ashen Marches, Back → Ashfall, Back → Ash Exposure.
        d.click("About Ashen Marches");
        d.observe(
            "iter-26-L-SB1-about",
            !d.visible_text("Hot ash falls") && d.has("Back"),
            "The About row opens the package's About page, with Back",
        );
        d.click("Back");
        d.observe(
            "iter-26-L-SB1-list",
            d.visible_text("Hot ash falls"),
            "Back after a list click returns to the entry the click left",
        );
        d.click("Back");
        d.observe(
            "iter-26-L-SB1-link",
            d.has("Read Captain Varra") && !d.visible_text("Hot ash falls"),
            "The next Back returns to the entry the link left (Ash Exposure, with its Read Captain Varra link)",
        );
        // Round 26 review: Lookup's Open in Library records the trail too. Link Ash Exposure →
        // Ashfall, open Veilglass Lantern from Lookup, Back → Ashfall, Back → Ash Exposure.
        d.click("Wiki watch spell (opens Ashfall)");
        d.wait_for_text("Hot ash falls");
        look_up(d, "Veilglass Lantern");
        d.wait_for_text("This lantern of smoked glass");
        d.click("Open in Library");
        if d.has("Close Lookup") {
            d.click("Close Lookup");
        }
        d.wait_for_text("This lantern of smoked glass");
        let lantern_open = !d.visible_text("Hot ash falls");
        d.click("Back");
        let back_to_link = d.visible_text("Hot ash falls");
        d.click("Back");
        d.observe(
            "iter-26-L-SB1-lookup",
            lantern_open
                && back_to_link
                && d.has("Read Captain Varra")
                && !d.visible_text("Hot ash falls"),
            format!(
                "Open in Library from Lookup, then Back returns to Ashfall ({back_to_link}) and Back again to Ash Exposure"
            ),
        );
        // A fault that loses the trail fails the observation above, not the steps below.
        if !d.has("Read Captain Varra") {
            d.open_combo_at(LIBRARY_PACKAGE_DROPDOWN);
            d.click(MODULE);
            d.click_starting("Ash Exposure · ");
        }
        d.click("Read Captain Varra");
        d.wait_for_text("The last commander of the Emberfall watch.");
        d.state("reference-outside-package", Sizes::All);
        let toolbar = d
            .nodes()
            .into_iter()
            .filter(|node| {
                [
                    "Back",
                    "Read",
                    "Edit Markdown",
                    "Copy to…",
                    "Make NPC note",
                    "More",
                ]
                .contains(&node.name.as_str())
            })
            .collect::<Vec<_>>();
        d.observe(
            "Library toolbar",
            d.has("Back")
                && toolbar.iter().all(|node| {
                    (node.rect.center().y - toolbar[0].rect.center().y).abs() <= crate::design::XS
                }),
            "The toolbar keeps Back and every visible action on one line",
        );
        d.click(EDIT_MARKDOWN);
        d.type_into_editor("\nVarra now carries a silver sword. ");
        d.state("draft-with-history", Sizes::All);
        d.observe(
            "Draft safety",
            d.has("Back")
                && !d.is_disabled("Back")
                && d.editor_text()
                    .is_some_and(|text| text.contains("silver sword")),
            "The editor keeps its draft beside the list",
        );
        d.click("Back");
        d.expect("Close Lookup");
        d.observe(
            "Draft Back",
            d.editor_text()
                .is_some_and(|text| text.contains("silver sword")),
            "Back opens Lookup and keeps the draft",
        );
        d.press(Key::Escape);
        d.click("Save");
        d.click("Read");
        d.observe(
            "Saved homebrew",
            d.has_containing("silver sword"),
            "The saved homebrew words appear in Read view",
        );

        // Round 26 review: a copy's result and Undo belong to the file it linked, by package and
        // path. Package B (Ember Reach) has its own `Rules/Ash Exposure.md` linking the same
        // campaign-only note. Copy in A, copy in B, Undo A: each reader shows its own result,
        // A's files come back, and B's copy, link and result stay.
        let b_folder = d.root.join("modules/ember-reach");
        std::fs::create_dir_all(b_folder.join("Rules")).unwrap();
        std::fs::write(
            b_folder.join("Package.md"),
            crate::library::note_text("Ember Reach", "package", Some("ember_reach")),
        )
        .unwrap();
        let b_reading = serde_yaml_ng::to_value(serde_json::json!({
            "title": "Ash Exposure",
            "summary": "Ember Reach keeps its own rule.",
            "related": {target.id.to_string(): "Read Captain Varra"}
        }))
        .unwrap();
        let b_path = b_folder.join("Rules/Ash Exposure.md");
        let b_text = crate::content_workspace::patch_fields(
            &crate::library::note_text("Ash Exposure", "rule", None),
            &[("splat_reading", Some(b_reading))],
        )
        .unwrap();
        std::fs::write(&b_path, &b_text).unwrap();
        let b_module = crate::library::local_module(&b_folder).unwrap();
        d.app_mut().campaigns.register_module(b_module).unwrap();
        d.settle();
        let a_text = std::fs::read_to_string(&path).unwrap();
        let copies_in = |folder: &std::path::Path| {
            splat::content::read_directory(folder)
                .map(|(entries, _)| entries.iter().filter(|e| e.name == "Captain Varra").count())
                .unwrap_or(0)
        };
        let a_folder = d.root.join("modules/ashen-marches");
        let open_in = |d: &mut Driver<'_>, package: &str| {
            d.open_combo_at(LIBRARY_PACKAGE_DROPDOWN);
            d.click(package);
            d.click_starting("Ash Exposure · ");
        };
        open_in(d, MODULE);
        d.click("Copy Captain Varra to Ashen Marches");
        d.wait_for_text("Captain Varra copied to Ashen Marches and linked.");
        open_in(d, "Ember Reach");
        d.wait_for_text("Ember Reach keeps its own rule.");
        let a_hidden_in_b = !d.has_containing("copied to Ashen Marches");
        d.click("Copy Captain Varra to Ember Reach");
        d.wait_for_text("Captain Varra copied to Ember Reach and linked.");
        let b_alone = !d.has_containing("copied to Ashen Marches");
        let b_linked = std::fs::read_to_string(&b_path).unwrap();
        open_in(d, MODULE);
        d.settle();
        // A fault that loses A's result fails the observation below, not these steps.
        let a_alone = d.has_containing("Captain Varra copied to Ashen Marches and linked.")
            && !d.has_containing("copied to Ember Reach");
        if a_alone {
            d.click("Undo copy");
            d.wait_for_text("Captain Varra is in this campaign only.");
        }
        let a_back = std::fs::read_to_string(&path).unwrap() == a_text && copies_in(&a_folder) == 0;
        let b_kept = std::fs::read_to_string(&b_path).unwrap() == b_linked
            && b_linked != b_text
            && copies_in(&b_folder) == 1;
        open_in(d, "Ember Reach");
        d.wait_for_text("Ember Reach keeps its own rule.");
        let b_result_kept = d.has_containing("Captain Varra copied to Ember Reach and linked.")
            && d.has("Undo copy");
        d.observe(
            "iter-26-L-copy-owner",
            a_hidden_in_b && b_alone && a_alone && a_back && b_kept && b_result_kept,
            format!(
                "Each copy result shows in its own file's reader: A hidden in B {a_hidden_in_b}, B alone {b_alone}, A alone {a_alone}; Undo A restores A {a_back} and keeps B's files {b_kept} and result {b_result_kept}"
            ),
        );
    }
}

/// A homebrewer reads a wide table in the Library, then beside a live fight.
pub mod reader_tables {
    use super::*;

    pub fn run(d: &mut Driver<'_>) {
        go_library(d);
        d.click("New entry");
        d.type_into("Entry name", "Spell Progression");
        d.click("Kind of entry");
        d.click("Rule");
        d.click("Create entry");
        assert!(
            d.nodes()
                .iter()
                .any(|n| n.name.starts_with("Spell Progression · ") && n.fully_in_view()),
            "the new entry row is in view"
        );
        assert!(d.app_mut().library.selected_name() == Some("Spell Progression"));
        d.type_into_editor(
            "| Level | Proficiency Bonus | Features | Cantrips | Prepared Spells | 1st | 2nd | 3rd | 4th | 5th | 6th | 7th | 8th | 9th |\n\
             | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n\
             | 5 | +3 | Memorize Spell | 4 | 9 | 4 | 3 | 2 | 0 | 0 | 0 | 0 | 0 | Ninth-slot |\n\n\
             The table leaves this note readable.\n\n\
             Read the row for your class level. A slot is spent when you cast a spell of that \
             level or higher, and every slot comes back when you finish a Long Rest, so plan the \
             hardest fights of the day around the slots you still have.\n",
        );
        d.click("Save");
        d.click("Read");
        d.state("wide-table-in-library", Sizes::All);
        // Round 23 AE1: at 1920 the table takes the pane past the prose measure and needs no
        // sideways scroll; the prose after it keeps the measure.
        // The pane's inner edge is the page's right edge, where `New entry` ends.
        for size in [SMALL, DEFAULT, LARGE] {
            let seen = d.at_size(size, |d| {
                let read = d.place_of("Read")?;
                let edge = d.place_of("New entry")?.right();
                let row = d.place_of("Memorize Spell")?;
                let last = d.place_of("Ninth-slot")?;
                // The prose keeps the measure: a paragraph that wraps fills it, and no more.
                let prose = d.painted_rect_in(
                    "Read the row for your class level.",
                    egui::Rect::EVERYTHING,
                )?;
                // Where the table scrolls, its visible part is its scroll surface.
                let surface = d.horizontal_scroll_regions().into_iter().find(|region| {
                    region.top() <= row.center().y && region.bottom() >= row.center().y
                });
                Some((
                    read.left(),
                    edge,
                    last.right(),
                    prose.width(),
                    surface,
                    d.has_containing("More columns"),
                ))
            });
            let measure = crate::design::READER_MEASURE;
            let ok = seen.is_some_and(|(left, edge, last, prose, surface, hint)| {
                prose <= measure + 0.5
                    && prose >= measure - crate::design::XXL
                    && match surface {
                        // It scrolls: the surface ends at or before the pane's inner edge.
                        // At 1920 the table must not scroll at all.
                        Some(surface) => size != LARGE && hint && surface.right() <= edge + 0.5,
                        // It fits: past the prose measure, inside the pane, with no scroll hint.
                        None => {
                            !hint && size == LARGE && last > left + measure && last <= edge + 0.5
                        }
                    }
            });
            d.observe(
                "iter-23-L-AE1",
                ok,
                format!(
                    "At {size:?} the prose wraps at the measure and the table stays inside the pane: reader left, pane edge, last cell right, prose width, scroll surface, hint = {seen:?}"
                ),
            );
        }
        // Round 24 FA8: the first column's rule shows only while the columns scroll under it, and
        // then it runs through the header as well as the rows.
        for size in [SMALL, DEFAULT, LARGE] {
            let seen = d.at_size(size, |d| {
                let level = d.painted_rect_in("Level", egui::Rect::EVERYTHING)?;
                let row = d.painted_rect_in("Memorize Spell", egui::Rect::EVERYTHING)?;
                let area = egui::Rect::from_min_max(
                    egui::pos2(level.left() - 1.0, level.top() - crate::design::XS),
                    egui::pos2(row.right(), row.bottom() + crate::design::XS),
                );
                Some((
                    level,
                    row,
                    d.vertical_segments_in(area),
                    d.has_containing("More columns"),
                ))
            });
            let ok = seen.as_ref().is_some_and(|(level, row, rules, scrolls)| {
                if *scrolls {
                    rules
                        .iter()
                        .any(|rule| rule.top() <= level.top() && rule.bottom() >= row.bottom())
                } else {
                    rules.is_empty()
                }
            });
            d.observe(
                "iter-24-L-FA8",
                ok,
                format!(
                    "At {size:?} a table that fits has no column rule, and a scrolling one has a rule through its header: header, row, rules, scrolls = {seen:?}"
                ),
            );
        }
        let table = d.place_of("Memorize Spell").expect("feature cell");
        d.scroll_at(table.center(), 1);
        d.observe(
            "Vertical scrolling",
            d.visible_text("Level"),
            "An ordinary wheel gesture does not scroll the table sideways",
        );
        d.scroll_sideways_at(table.center(), 20);
        d.observe(
            "Wide tables",
            d.visible_text("Ninth-slot"),
            "The last column is readable after scrolling across the Library table",
        );
        d.set_base(DEFAULT);
        d.observe(
            "Table resize",
            d.visible_text("Ninth-slot"),
            "The table keeps its sideways position when resized to 1200x800",
        );
        // At 1920 the whole table shows without a sideways scroll (round 23 AE1), so only the
        // sizes where it scrolls have a last-column state.
        d.state("last-column-in-library", Sizes::Standard);
        d.set_base(SMALL);

        go_encounters(d);
        resume_run_from_cold_start(d);
        look_up(d, "Spell Progression");
        d.expect("Memorize Spell");
        d.state("wide-table-beside-run", Sizes::All);
        let table = d.place_of("Memorize Spell").expect("feature cell");
        d.scroll_sideways_at(table.center(), 20);
        d.observe(
            "Wide tables",
            d.visible_text("Ninth-slot") && d.in_view(NEXT_TURN),
            "The last column is readable in Lookup while the current fight stays visible",
        );
        d.state("last-column-beside-run", Sizes::All);
    }
}

/// W9: choose what a campaign uses, then edit a campaign-owned entry.
pub mod campaign_content {
    use super::*;
    const PAGE_TITLE: &str = "entries chosen";
    const MODULE_ROW: &str = "Ashen Marches · ";
    // Round 25 FA2: `chosen` counts the DM's own choices; the entry the other choices need is
    // counted once, as needed.
    const PARTIAL: &str = "4 of 6 chosen · 1 needed";
    const CHANGED: &str = "5 of 6 chosen · 1 needed";
    const UNSELECTED_ENTRY: &str = "Ash Wraith";
    const DISCARD_SELECTION: &str = "Discard";
    const CAMPAIGN_ENTRY: &str = "Captain Varra";
    const SIBLING_ENTRY: &str = "Bridge Ambush";
    const SAVE_ENTRY: &str = "Save";
    const SAVED: &str = "Saved";
    const REQUIRED: &str = "Needed by your entries";
    const REQUIRED_BY: &str = "needed by";
    const OWN_ENTRY_SEARCH: &str = "varra";
    const RELOAD: &str = "Reload from disk";
    const REPAIR: &str = "Repair";
    const SAVE_SELECTIONS: &str = "Save selections";
    const BROKEN_NOTE: &str = "campaigns/emberfall/Notes/Broken.md";

    pub fn run(d: &mut Driver<'_>) {
        go_campaign(d);
        d.expect(PAGE_TITLE);
        d.expect("2,069 entries chosen · 1 needed");
        d.click_starting("D&D 2024 · SRD");
        d.expect("Whole package, with future additions");
        d.state("packages-tab", Sizes::Default);
        d.check_tab_order("campaign, packages");
        // No click: the brief asks for these on screen, with the built-in package open.
        d.click_starting(REQUIRED);
        d.wait_for_text("NEEDED BY YOUR ENTRIES");
        let reasons = d.visible_text(REQUIRED_BY);
        let counted = d.has_containing(REQUIRED);
        // The list fills its pane, rather than stopping short of the end.
        d.state("required-dependencies", Sizes::All);
        d.observe(
            "T9.1",
            reasons && counted,
            "Required dependencies and their reasons are on screen, with the built-in package open",
        );
        d.begin_task();
        d.click_starting(MODULE_ROW);
        d.expect(PARTIAL);
        d.observe_both(
            "U1",
            "Package mode comes before the selection actions",
            |d| {
                d.place_of("Chosen entries only")
                    .zip(d.place_of("Use all"))
                    .is_some_and(|(mode, action)| {
                        mode.left() < action.left() && mode.center().y < action.bottom()
                    })
            },
        );
        d.state("packages-partial-selection", Sizes::All);
        d.goal("see a partial package selection");

        let positions = |d: &Driver<'_>| {
            ["D&D 2024", "Use Ash Wraith"].map(|label| {
                d.nodes()
                    .iter()
                    .find(|n| n.name.starts_with(label))
                    .unwrap()
                    .rect
                    .top()
            })
        };
        let before = [(960.0, 640.0), (1200.0, 800.0)].map(|size| d.at_size(size, positions));
        d.click(&format!("Use {UNSELECTED_ENTRY}"));
        for (size, expected) in [(960.0, 640.0), (1200.0, 800.0)].into_iter().zip(before) {
            let after = d.at_size(size, positions);
            d.observe(
                "i19-U1",
                after == expected,
                "The first tick keeps the package and entry rows at the same y.",
            );
        }
        d.expect(CHANGED);
        d.observe(
            "iter-25-C-FA2",
            !d.has_containing("6 of 6 chosen"),
            "A needed entry is not counted as chosen as well",
        );
        d.expect("Include future additions");
        d.expect("2,070 entries chosen · 1 needed");
        d.observe_both(
            "iter-18-A-lock",
            "A package draft states the campaign switch lock",
            |d| d.visible_text(crate::encounters::draft_switch_reason()),
        );
        d.state("selection-changed", Sizes::All);
        // Round 25 review: the page's results and errors sit in the message space below
        // `Save selections`, so the package rows keep their y through a draft, its discard, Undo
        // discard, a failed Save and a saved one.
        let package_tops = |d: &mut Driver<'_>| {
            [SMALL, DEFAULT, LARGE].map(|size| {
                d.at_size(size, |d| {
                    d.nodes()
                        .into_iter()
                        .find(|node| node.name.starts_with("D&D 2024"))
                        .map(|node| node.rect.top())
                })
            })
        };
        let draft_tops = package_tops(d);
        d.click(DISCARD_SELECTION);
        d.goal("change and discard a selection");
        let discarded_tops = package_tops(d);
        let discarded = d.in_view("Undo discard of Campaign packages");
        d.click("Undo discard of Campaign packages");
        d.expect(CHANGED);
        let undone_tops = package_tops(d);
        let manifest = d.root.join("campaigns/emberfall/Campaign.md");
        let on_disk = std::fs::read(&manifest).unwrap();
        let set_read_only = |read_only: bool| {
            let mut permissions = std::fs::metadata(&manifest).unwrap().permissions();
            permissions.set_readonly(read_only);
            std::fs::set_permissions(&manifest, permissions).unwrap();
        };
        set_read_only(true);
        d.click(SAVE_SELECTIONS);
        d.settle();
        let failed_tops = package_tops(d);
        let failed = std::fs::read(&manifest).unwrap() == on_disk && d.visible_text("read only");
        set_read_only(false);
        d.click(SAVE_SELECTIONS);
        d.settle();
        let saved_tops = package_tops(d);
        let saved = d.visible_text("Campaign selections saved.");
        d.observe(
            "iter-25-C-page-results",
            draft_tops.iter().all(Option::is_some)
                && [discarded_tops, undone_tops, failed_tops, saved_tops] == [draft_tops; 4]
                && discarded
                && failed
                && saved,
            format!(
                "Package row tops: draft {draft_tops:?}, discarded {discarded_tops:?}, undone {undone_tops:?}, failed Save {failed_tops:?}, saved {saved_tops:?}; results shown {discarded}, {failed}, {saved}"
            ),
        );
        // Back to the fixture's choices, then a draft and its discard, as before.
        d.click(&format!("Use {UNSELECTED_ENTRY}"));
        d.click(SAVE_SELECTIONS);
        d.settle();
        d.click(&format!("Use {UNSELECTED_ENTRY}"));
        d.expect(CHANGED);
        d.click(DISCARD_SELECTION);

        d.click(TAB_ENTRIES);
        d.expect(CAMPAIGN_ENTRY);
        assert!(d.visible_text("Campaign entries"));
        d.state("own-entries-in-library", Sizes::All);
        d.begin_task();
        let in_list = |d: &Driver<'_>, name: &str| {
            d.nodes()
                .into_iter()
                .any(|node| node.name.starts_with(name) && node.in_view() && !node.disabled)
        };
        let sibling_before = in_list(d, SIBLING_ENTRY);
        d.type_into(SEARCH_LIBRARY, OWN_ENTRY_SEARCH);
        let found = in_list(d, CAMPAIGN_ENTRY);
        let sibling_after = in_list(d, SIBLING_ENTRY);
        d.observe(
            "T8.4",
            sibling_before && found && !sibling_after,
            format!("Typing finds a campaign-owned entry, and {SIBLING_ENTRY} leaves the list: before {sibling_before}, after {sibling_after}"),
        );
        d.click(CLEAR_SEARCH);
        d.begin_task();
        d.click_containing(&format!("{CAMPAIGN_ENTRY} · Creature"));
        d.state("entry-read", Sizes::All);
        d.review("T6.4", "the campaign entry reader");
        // Read from the reader's own place: the list beside it has numbers of its own.
        let reader = d.painted_from_label(CAMPAIGN_ENTRY);
        let stats = ["ARMOR", "16", "POINTS", "52", "INITIATIVE", "+2", "Medium"]
            .iter()
            .all(|word| reader.split_whitespace().any(|shown| shown == *word));
        d.observe(
            "T6.4",
            stats && !reader.contains("—"),
            format!("A creature the DM wrote reads with its armor class, hit points and initiative, and no empty cells: {reader}"),
        );
        d.click(EDIT_MARKDOWN);
        d.state("entry-edit", Sizes::Default);
        d.goal("open a campaign entry for editing");

        let editor_top = |d: &Driver<'_>| {
            let node = d
                .nodes()
                .into_iter()
                .find(|node| node.role == egui::accesskit::Role::MultilineTextInput)
                .unwrap();
            // A long source scrolls inside the editor. Its visible viewport must stay put.
            node.shown.top()
        };
        let clean_top = editor_top(d);
        d.type_into_editor("Scarred by the fire at the bridge. ");
        assert_eq!(
            editor_top(d),
            clean_top,
            "Starting a draft must keep the editor's top edge"
        );
        d.state("entry-dirty", Sizes::Default);
        d.click(SAVE_ENTRY);
        d.expect(SAVED);
        assert_eq!(
            editor_top(d),
            clean_top,
            "Saving must keep the editor's top edge"
        );
        d.state("entry-saved", Sizes::Default);
        d.goal("save the campaign entry");

        // A picture opens at full size, over the page, and Escape closes it.
        d.click("Read");
        d.click(CAMPAIGN_ENTRY);
        d.state("picture-full-size", Sizes::Default);
        d.press(Key::Escape);
        d.settle();
        d.expect(EDIT_MARKDOWN);

        std::fs::write(
            d.root.join(BROKEN_NOTE),
            "---\nsplat_schema: 2\nsplat_id: [\n---\n",
        )
        .expect("write an invalid note");
        go_campaign(d);
        d.click(RELOAD);
        d.expect(REPAIR);
        d.click(TAB_PACKAGES);
        d.state("repair-needed", Sizes::Default);
        d.observe(
            "T9.2",
            d.is_disabled(SAVE_SELECTIONS) && d.visible_text("Fix Notes/Broken.md first."),
            "Save disabled by a repair states its reason",
        );
        // Round 25 FB6: a package change while a repair blocks Save selections names Discard
        // only in its switch lock.
        d.click(&format!("Use {UNSELECTED_ENTRY}"));
        d.expect(CHANGED);
        d.observe(
            "iter-25-C-FB6-selections",
            d.visible_text(crate::encounters::blocked_draft_switch_reason())
                && !d.visible_text(crate::encounters::draft_switch_reason())
                && d.is_disabled(SAVE_SELECTIONS),
            "A package draft that a repair blocks states the switch lock without Save",
        );
        d.click(DISCARD_SELECTION);
        // The repair opens here, with the note's text in it.
        d.click(REPAIR);
        d.expect("Note text");
        d.observe_both(
            "U5",
            "The package save row stays above repairs, with one right edge",
            |d| {
                d.place_of("Save repaired note")
                    .zip(d.place_of(SAVE_SELECTIONS))
                    .is_some_and(|(repair, selections)| {
                        (repair.right() - selections.right()).abs() < 1.0
                            && repair.top() - selections.bottom() >= crate::design::L
                    })
            },
        );
        // FD2: one rule separates the open repair from the package list, an `L` gap below the
        // repair. Round 25 FD1: the repair ends with its message space below Save repaired note,
        // so the gap is below that space. The rule must sit above the first thing drawn after
        // the repair.
        d.observe_both(
            "iter-14-E-FD2",
            "One rule, an L gap below the repair's message space, separates the repair from the list",
            |d| {
                let Some(save) = d.place_of("Save repaired note") else {
                    return false;
                };
                let end = save.bottom() + crate::design::message_space_height();
                let Some(next) = d
                    .nodes()
                    .into_iter()
                    .filter(|node| node.in_view() && node.rect.top() > end + 1.0)
                    .map(|node| node.rect.top())
                    .min_by(f32::total_cmp)
                else {
                    return false;
                };
                let rules = d.horizontal_rules(save.left(), end, next);
                rules.len() == 1 && rules[0] - end >= crate::design::L
            },
        );
        // Round 22 FD3: an unchanged repair has no primary action on the whole screen.
        d.observe(
            "iter-22-C-FD3-campaign-clean",
            super::session_recording::accent_filled(d).is_empty()
                && d.in_view("Save repaired note"),
            format!(
                "Accent-filled: {:?}",
                super::session_recording::accent_filled(d)
            ),
        );
        // Round 23 FA9: the message names the file; the REPAIR heading does not repeat it. The
        // editor's accessible name holds the path.
        let mut named = 0;
        d.each_painted_text(|text, clip| {
            if text.galley.job.text.contains("Notes/Broken.md")
                && clip.intersects(text.visual_bounding_rect())
            {
                named += 1;
            }
        });
        d.observe(
            "iter-23-C-FA9",
            named == 2
                && d.has(&crate::world::repair::editor_name("Notes/Broken.md"))
                && d.visible_text("REPAIR"),
            format!("Notes/Broken.md is painted {named} times: the Save reason and the message"),
        );
        // Round 23 FA3: the unchanged draft is the invalid file, so Save would refuse it. Only
        // the package save row still says No changes.
        let mut clean = 0;
        d.each_painted_text(|text, clip| {
            if text.galley.job.text.contains("No changes")
                && clip.intersects(text.visual_bounding_rect())
            {
                clean += 1;
            }
        });
        d.observe(
            "iter-23-C-FA3-clean",
            d.visible_text("Invalid") && clean == 1 && d.is_disabled("Save repaired note"),
            format!("An unchanged invalid repair says Invalid, and Save is blocked; No changes is painted {clean} times"),
        );
        d.state("repair-open", Sizes::Default);
        d.observe(
            "Repair says why",
            d.visible_text("Line 3 opens a list"),
            "The repair says what is wrong before the DM edits anything",
        );
        d.observe(
            "T9.2",
            d.editor_text()
                .is_some_and(|text| text.contains("splat_id")),
            "The repair is reachable without leaving the page",
        );
        let editor_top = |d: &Driver<'_>| {
            d.nodes()
                .into_iter()
                .find(|node| node.name == crate::world::repair::editor_name("Notes/Broken.md"))
                // A long draft scrolls inside the editor; its visible viewport must stay put.
                // The page scrolls too, so the top is measured from the page's save row.
                .zip(d.place_of(SAVE_SELECTIONS))
                .map(|(node, save)| node.shown.top() - save.top())
        };
        const REPAIR_SIZES: [(f32, f32); 3] = [SMALL, DEFAULT, LARGE];
        let before = REPAIR_SIZES.map(|size| {
            d.at_size(size, |d| {
                (d.place_of(SAVE_SELECTIONS).unwrap().top(), editor_top(d))
            })
        });
        // Round 25 FD1: the editor's top at each size, in every later state of this repair.
        let editor_tops = |d: &mut Driver<'_>| REPAIR_SIZES.map(|size| d.at_size(size, editor_top));
        let clean_tops = before.map(|(_, editor)| editor);
        let draft = d
            .editor_text()
            .unwrap()
            .replacen("splat_id: [", "splat_id: [ ", 1);
        d.replace_text_in("Note text", &draft);
        for (size, (y, editor)) in REPAIR_SIZES.into_iter().zip(before) {
            let (after, editor_after) = d.at_size(size, |d| {
                (d.place_of(SAVE_SELECTIONS).unwrap().top(), editor_top(d))
            });
            d.observe(
                "i19-U1-repair",
                after == y,
                "The first repair edit does not move the selection save row.",
            );
            // Round 23 FA1: `Fix Notes/Broken.md first.` keeps its own line below Save, so the
            // first key moves neither the error nor the editor.
            d.observe(
                "iter-23-C-FA1",
                editor.is_some() && editor_after == editor,
                format!("At {size:?}, the repair editor top {editor:?} → {editor_after:?}"),
            );
        }
        // Round 25 FB6: Save refuses this draft, so the lock names Discard only.
        d.observe_both(
            "iter-18-A-repair-lock",
            "A repair draft that Save refuses states the campaign switch lock without Save",
            |d| {
                d.visible_text(crate::encounters::blocked_draft_switch_reason())
                    && !d.visible_text(crate::encounters::draft_switch_reason())
            },
        );
        // Round 23 FA3: the changed draft still fails as Save would fail it. It says Invalid with
        // its cause, Save is blocked, and nothing on the screen is primary.
        d.observe_both(
            "iter-23-C-FA3-dirty",
            "A changed draft that Save refuses says Invalid, blocks Save and has no primary",
            |d| {
                d.visible_text("Invalid")
                    && !d.visible_text("Unsaved")
                    && d.visible_text("Line 3 opens a list")
                    && d.is_disabled("Save repaired note")
                    && super::session_recording::accent_filled(d).is_empty()
            },
        );
        d.state("repair-dirty", Sizes::All);
        // Lead ruling on round 23 FA3: a draft that parses but that Save refuses (a package
        // note, which settings own) is Invalid with Save's reason, and a Save attempt writes
        // nothing.
        let on_disk = std::fs::read(d.root.join(BROKEN_NOTE)).unwrap();
        let owned = crate::library::note_text("Broken", "package", Some("broken"));
        assert!(splat::content::Entry::parse("Notes/Broken.md", &owned).is_ok());
        d.replace_text_in("Note text", &owned);
        d.press_ctrl(Key::S);
        d.settle();
        const OWNERSHIP: &str = "Keep campaign ownership consistent with settings.";
        d.observe(
            "iter-23-C-FA3-refused",
            d.visible_text("Invalid")
                && !d.visible_text("Unsaved")
                && d.visible_text(OWNERSHIP)
                && d.is_disabled("Save repaired note")
                && super::session_recording::accent_filled(d).is_empty()
                && std::fs::read(d.root.join(BROKEN_NOTE)).unwrap() == on_disk,
            format!(
                "A parsed draft that Save refuses: Invalid {}, reason {}, Save blocked {}, file unchanged {}",
                d.visible_text("Invalid"),
                d.visible_text(OWNERSHIP),
                d.is_disabled("Save repaired note"),
                std::fs::read(d.root.join(BROKEN_NOTE)).unwrap() == on_disk
            ),
        );
        // Round 25 FD1: the long refusal sits in the message space, not in the message above
        // the editor, so it moves the editor at no size.
        let refused_tops = editor_tops(d);
        d.observe(
            "iter-25-C-FD1-refused",
            refused_tops == clean_tops,
            format!("Editor tops with a refused draft {refused_tops:?}, clean {clean_tops:?}"),
        );
        // A draft that Save would write says Unsaved, and its Save is the one primary action.
        let valid = crate::library::note_text("Broken", "rule", None);
        d.replace_text_in("Note text", &valid);
        d.observe(
            "iter-23-C-FA3-valid",
            d.visible_text("Unsaved")
                && !d.visible_text("Invalid")
                && !d.is_disabled("Save repaired note"),
            "A draft that Save would write says Unsaved, and Save is open",
        );
        // Round 25 FB6: with Save open, the lock names Save again.
        d.observe(
            "iter-25-C-FB6-valid",
            d.visible_text(crate::encounters::draft_switch_reason())
                && !d.visible_text(crate::encounters::blocked_draft_switch_reason()),
            "A draft that Save would write states the lock with Save",
        );
        // Round 22 FD3: a changed repair's save is the one primary action, as `Save changes` is.
        d.observe_both(
            "iter-22-C-FD3-campaign",
            "Save repaired note is the only accent-filled action while the draft has changes",
            |d| super::session_recording::accent_filled(d) == ["Save repaired note"],
        );
        // Lead ruling on round 23 FA3: Save that fails on the disk (here a read-only file) is
        // an action-time failure. The state slot names Save's reason, not Unsaved, until the
        // next edit of that draft.
        let file = d.root.join(BROKEN_NOTE);
        let set_read_only = |read_only: bool| {
            let mut permissions = std::fs::metadata(&file).unwrap().permissions();
            permissions.set_readonly(read_only);
            std::fs::set_permissions(&file, permissions).unwrap();
        };
        set_read_only(true);
        d.click("Save repaired note");
        d.settle();
        const READ_ONLY: &str = "Not saved. This file is read only.";
        const NOT_SAVED_TWICE: &str = "The repair has not been saved";
        let unchanged = std::fs::read(&file).unwrap() == on_disk;
        d.observe(
            "iter-23-C-FA3-disk",
            d.visible_text(READ_ONLY) && !d.visible_text("Unsaved") && unchanged,
            format!(
                "After a read-only Save: reason shown {}, Unsaved shown {}, file unchanged {unchanged}",
                d.visible_text(READ_ONLY),
                d.visible_text("Unsaved")
            ),
        );
        // Round 24 FD1, AD3, SD1: the failed Save stays open for a retry, but does not own the
        // primary. FD9: the failure shows once, in the state slot; the message above stops at
        // the way to fix the file.
        d.observe_both(
            "iter-24-C-FD1",
            "After a read-only Save, Save repaired note is open, secondary and the failure shows once",
            |d| {
                d.visible_text(READ_ONLY)
                    && !d.is_disabled("Save repaired note")
                    && super::session_recording::accent_filled(d).is_empty()
                    && d.visible_text("Notes/Broken.md cannot be read. Fix it here or in World.")
                    && !d.visible_text(NOT_SAVED_TWICE)
            },
        );
        d.state("repair-save-failed", Sizes::Default);
        // The next edit ends it: the draft is a new one that Save has not tried, and its Save is
        // the one primary action again.
        d.replace_text_in("Note text", &format!("{valid}\nOne more line.\n"));
        d.observe(
            "iter-23-C-FA3-disk-ends",
            d.visible_text("Unsaved") && !d.visible_text(READ_ONLY),
            "An edit of the draft ends the failed Save's reason",
        );
        d.observe(
            "iter-24-C-FD1-ends",
            super::session_recording::accent_filled(d) == ["Save repaired note"],
            format!(
                "After the next edit the primary is {:?}",
                super::session_recording::accent_filled(d)
            ),
        );
        set_read_only(false);
        d.click("Discard");
        // Round 25 FD1: the discard result sits in the message space below the save row. Its
        // Undo is live, so it comes first, on the space's first line, and the invalid file's
        // reason follows it in the same part. The editor keeps its top through the result, Undo
        // discard, an invalid draft and a valid one, at every size.
        const DISCARDED: &str = "Broken.md draft discarded.";
        const UNDO_DISCARD: &str = "Undo discard of Broken.md";
        let discarded_tops = editor_tops(d);
        let first_line = REPAIR_SIZES.map(|size| {
            d.at_size(size, |d| {
                let save = d
                    .nodes()
                    .into_iter()
                    .filter(|node| node.name == "Save repaired note")
                    .map(|node| node.rect)
                    .next();
                let undo = d.place_of(UNDO_DISCARD);
                let reason = d.painted_rect_in("Line 3 opens a list", egui::Rect::EVERYTHING);
                // The space starts below the save row; its first line is one Body line deep.
                save.zip(undo)
                    .zip(reason)
                    .is_some_and(|((save, undo), reason)| {
                        undo.top() >= save.bottom()
                            && undo.center().y
                                < save.bottom()
                                    + crate::design::Type::Body.line()
                                    + crate::design::S
                            && reason.top() > undo.center().y
                    })
            })
        });
        d.observe(
            "iter-25-C-FD1-discard",
            clean_tops.iter().all(Option::is_some)
                && discarded_tops == clean_tops
                && first_line.iter().all(|ok| *ok)
                && d.visible_text(DISCARDED)
                && d.visible_text("Invalid"),
            format!(
                "Editor tops: clean {clean_tops:?}, after Discard {discarded_tops:?}; Undo on the first line, reason after it, at 960/1200/1920: {first_line:?}"
            ),
        );
        d.click(UNDO_DISCARD);
        let undone_tops = editor_tops(d);
        d.observe(
            "iter-25-C-FD1-undo",
            undone_tops == clean_tops
                && d.editor_text()
                    .is_some_and(|text| text.contains("One more line."))
                && d.visible_text("Unsaved")
                && !d.visible_text(DISCARDED),
            format!("Editor tops after Undo discard {undone_tops:?}"),
        );
        // An edit ends the result's owner; Save's reason takes the space, and no edit moves the
        // editor.
        d.replace_text_in("Note text", &draft);
        let invalid_tops = editor_tops(d);
        d.observe(
            "iter-25-C-FD1-invalid",
            invalid_tops == clean_tops
                && d.visible_text("Line 3 opens a list")
                && !d.visible_text(DISCARDED),
            format!("Editor tops with an invalid draft {invalid_tops:?}"),
        );
        d.replace_text_in("Note text", &valid);
        let valid_tops = editor_tops(d);
        d.observe(
            "iter-25-C-FD1-valid",
            valid_tops == clean_tops
                && d.visible_text("Unsaved")
                && !d.visible_text("Line 3 opens a list")
                && !d.visible_text(DISCARDED),
            format!("Editor tops with a valid draft {valid_tops:?}"),
        );
        d.click("Discard");
        // Round 25 review: Reload re-reads the repairs; the open one keeps its editor's place,
        // and its discard result lives on with the repair.
        d.click(RELOAD);
        d.settle();
        d.expect("Note text");
        let reloaded_tops = editor_tops(d);
        d.observe(
            "iter-25-C-FD1-reload",
            reloaded_tops == clean_tops && d.has(UNDO_DISCARD),
            format!("Editor tops after Reload {reloaded_tops:?}, clean {clean_tops:?}"),
        );
        two_changed_repairs_have_one_primary(d);
        message_space_overflows_and_scrolls(d);
        std::fs::remove_file(d.root.join(BROKEN_NOTE)).unwrap();
        let manifest = d.root.join("campaigns/emberfall/Campaign.md");
        let before = std::fs::read_to_string(&manifest).unwrap();
        const PACKAGES_DISCARDED: &str = "Campaign packages draft discarded.";
        let discarded_before = d.visible_text(PACKAGES_DISCARDED);
        std::fs::remove_file(&manifest).unwrap();
        d.click(RELOAD);
        d.wait_for_text("Write Campaign.md again");
        d.expect("Campaign.md is missing. Splat still has the copy it last read: 2 packages.");
        assert!(!manifest.exists());
        d.state("missing-campaign-manifest-needs-explicit-write", Sizes::All);
        d.click("Write Campaign.md again");
        d.wait_for_text("Undo write Campaign.md");
        assert_eq!(std::fs::read_to_string(&manifest).unwrap(), before);
        // Round 23 FA8: the discarded package draft belonged to the file that went missing. Its
        // Undo does not come back with the written file.
        d.observe(
            "iter-23-C-FA8",
            discarded_before && !d.visible_text(PACKAGES_DISCARDED) && !d.has("Undo discard"),
            format!(
                "Undo discard shown before the file went: {discarded_before}; after the write: {}",
                d.visible_text(PACKAGES_DISCARDED)
            ),
        );
        d.state_prepared("campaign-manifest-written-with-undo", Sizes::All, |d| {
            check_undo_icon(d, "Undo write Campaign.md", "iter-6-C-SD3-Campaign");
        });
        // Round 25 review: a failed Undo keeps the write and its Undo; once the fault is gone,
        // the same Undo works. The test namespace runs as root, so permission bits do not stop
        // a delete there; the fault is a folder where Campaign.md was, so the Undo cannot read it.
        let written = std::fs::read(&manifest).unwrap();
        std::fs::remove_file(&manifest).unwrap();
        std::fs::create_dir(&manifest).unwrap();
        d.click("Undo write Campaign.md");
        d.settle();
        let kept =
            d.has("Undo write Campaign.md") && d.visible_text("Campaign.md was written again.");
        std::fs::remove_dir(&manifest).unwrap();
        std::fs::write(&manifest, &written).unwrap();
        d.observe(
            "iter-25-C-undo-failed",
            kept,
            format!(
                "After a failed Undo: Undo kept {}, result kept {}",
                d.has("Undo write Campaign.md"),
                d.visible_text("Campaign.md was written again.")
            ),
        );
        d.click("Undo write Campaign.md");
        d.wait_for_text("Write Campaign.md again");
        assert!(!manifest.exists());
        d.expect_same(
            "Undo removes only the restored manifest and returns to the same missing-file view.",
        );
        d.assert_state_matches(
            "campaign-manifest-write-undone",
            "missing-campaign-manifest-needs-explicit-write",
        );
        d.restart();
        go_campaign(d);
        d.wait_for_text("Write a new Campaign.md");
        // Round 23 FB5: the sentence offers the write; it does not say Splat is writing now.
        d.expect("Campaign.md is missing, and Splat has no copy. You can write a new one with the default packages.");
        d.state("missing-campaign-manifest-without-copy", Sizes::All);
        d.click("Write a new Campaign.md");
        d.wait_for_text("Undo write Campaign.md");
        assert!(manifest.exists());
    }

    /// Round 25 review: a repair whose discard result and cause need more than the message
    /// space's two lines. The part stays two lines high; its words scroll inside it by keyboard
    /// and wheel; Tab reaches it by its whole message, and its focus ring stays clear of its
    /// scroll bar. At 960, 1200 and 1920. Starts and ends with only Broken.md's repair.
    fn message_space_overflows_and_scrolls(d: &mut Driver<'_>) {
        const NAME: &str = "The ledger of the toll keeper at the old Ashford bridge and every name he wrote down.md";
        let path = format!("Notes/{NAME}");
        let file = d.root.join("campaigns/emberfall").join(&path);
        let broken = "---\nsplat_schema: 2\nsplat_id: [\n---\n";
        std::fs::write(&file, broken).unwrap();
        d.click(RELOAD);
        d.wait_for_text(NAME);
        let message = format!("{path} cannot be read");
        d.click_below(&message, REPAIR);
        let editor = crate::world::repair::editor_name(&path);
        d.replace_text_in(&editor, &format!("{broken}extra\n"));
        d.click_below(&message, "Discard");
        let words =
            format!("{NAME} draft discarded. Line 3 opens a list with [ that never closes.");
        let base = d.window_size();
        // The part's own scroll bar lane, as `design::fixed_part` keeps it.
        let lane = {
            let style = d.app_style();
            style.spacing.scroll.bar_width + style.spacing.scroll.bar_outer_margin
        };
        let mut results = Vec::new();
        for size in [SMALL, DEFAULT, LARGE] {
            d.set_base(size);
            // Tab first: focus scrolls the page to the part. Then back to the part's start.
            let focused = d.tab_to(&words);
            for _ in 0..4 {
                d.press(Key::ArrowUp);
            }
            // The part's texts, by their clip inside it: the result's top and the content's
            // bottom, measured from the part's top.
            let inside = |d: &Driver<'_>| {
                let part = d.nodes().into_iter().find(|node| node.name == words)?.rect;
                let (mut result, mut bottom) = (None, f32::MIN);
                d.each_painted_text(|text, clip| {
                    // The part's scroll area clips its texts to the part's lines (its clip is
                    // wider than the part).
                    if (clip.top() - part.top()).abs() <= 4.0
                        && (clip.bottom() - part.bottom()).abs() <= 4.0
                        && part.x_range().contains(text.pos.x)
                    {
                        let rect = egui::Rect::from_min_size(text.pos, text.galley.size());
                        bottom = bottom.max(rect.bottom() - part.top());
                        if text.galley.job.text.contains("draft discarded.") {
                            result = Some(rect.top() - part.top());
                        }
                    }
                });
                Some((part, result, bottom))
            };
            // Where the result's first line sits in the part: `Some(None)` once it has scrolled
            // out of the part's view (a line out of view is not painted); `None` without a part.
            let top = |d: &Driver<'_>| inside(d).map(|(_, top, _)| top);
            let Some((part, Some(before), bottom)) = inside(d) else {
                results.push((size, false, focused, None, None, None, None, None));
                continue;
            };
            // The words need more than two lines: they end below the part's end.
            let overflows = (part.height() - crate::design::message_space_height()).abs() < 1.0
                && bottom > part.height() + 1.0;
            let ring = d
                .focus_rings()
                .into_iter()
                .find(|(ring, _)| part.expand(2.0).contains_rect(*ring))
                .map(|(ring, _)| ring);
            let ring_clear = ring.is_some_and(|ring| ring.right() <= part.right() - lane + 1.0);
            d.press(Key::ArrowDown);
            let keyboard = top(d);
            for _ in 0..4 {
                d.press(Key::ArrowUp);
            }
            d.press(Key::Escape);
            let wheel_start = top(d);
            d.scroll_at(part.center(), 1);
            let wheel = top(d);
            results.push((
                size,
                overflows,
                focused,
                ring.filter(|_| ring_clear),
                Some(Some(before)),
                keyboard,
                wheel_start,
                wheel,
            ));
        }
        d.set_base(base);
        // Moved up inside the part, or out of its view; the part itself is still there.
        let moved = |from: Option<Option<f32>>, to: Option<Option<f32>>| match (from, to) {
            (Some(Some(from)), Some(Some(to))) => to < from,
            (Some(Some(_)), Some(None)) => true,
            _ => false,
        };
        d.observe(
            "iter-25-C-space-overflow",
            results.iter().all(|(_, overflows, focused, ring, before, keyboard, start, wheel)| {
                *overflows
                    && *focused
                    && ring.is_some()
                    && moved(*before, *keyboard)
                    && moved(*start, *wheel)
            }),
            format!("Per size (overflows, Tab, ring clear of the bar, result top before, after ArrowDown, before wheel, after wheel): {results:?}"),
        );
        std::fs::remove_file(&file).unwrap();
        d.click(RELOAD);
        d.settle();
        assert!(!d.has_containing(NAME));
    }

    /// Every `Save repaired note` in view, top to bottom, and whether it is accent-filled.
    fn repair_saves(d: &Driver<'_>) -> Vec<(egui::Rect, bool)> {
        let accent = crate::design::colors().accent_fill;
        let mut saves: Vec<_> = d
            .nodes()
            .into_iter()
            .filter(|node| node.name == "Save repaired note" && node.in_view())
            .map(|node| {
                let filled = d
                    .control_frame_at(node.rect)
                    .is_some_and(|frame| frame.fill == accent);
                (node.rect, filled)
            })
            .collect();
        saves.sort_by(|a, b| a.0.top().total_cmp(&b.0.top()));
        saves
    }

    /// Round 22 review: two changed repairs give one primary Save, the active repair's; an
    /// open run takes the primary from both. Both drafts stay. Starts and ends with the first
    /// repair open and unchanged.
    fn two_changed_repairs_have_one_primary(d: &mut Driver<'_>) {
        const LARGE: (f32, f32) = (1920.0, 1080.0);
        const SECOND: &str = "campaigns/emberfall/Notes/Cracked.md";
        let broken = "---\nsplat_schema: 2\nsplat_id: [\n---\n";
        std::fs::write(d.root.join(SECOND), broken).unwrap();
        d.click(RELOAD);
        d.wait_for_text("Notes/Cracked.md cannot be read");
        d.click(REPAIR);
        assert_eq!(
            d.nodes()
                .iter()
                .filter(|node| node.name == "Close repair")
                .count(),
            2,
            "Both repairs are open"
        );
        // Only a draft that Save would write holds the primary (round 23 FA3), so both drafts
        // are valid notes. Their names, near the top of each editor, tell them apart.
        let first = crate::library::note_text("First draft.", "rule", None);
        let second = crate::library::note_text("Second draft.", "rule", None);
        // Edit the upper repair, close it, edit the lower one, then open the upper one again.
        d.replace_text_in("Note text", &first);
        d.click("Close repair");
        d.replace_text_in("Note text", &second);
        d.click(REPAIR);
        let (lower_active, kept) = d.at_size(LARGE, |d| {
            (
                repair_saves(d),
                d.visible_text("First draft.") && d.visible_text("Second draft."),
            )
        });
        d.observe(
            "iter-22-C-FD3-two-repairs",
            lower_active.len() == 2
                && lower_active.iter().filter(|(_, filled)| *filled).count() == 1
                && lower_active[1].1
                && kept,
            format!("Lower repair edited last: saves {lower_active:?}; both drafts shown: {kept}"),
        );
        // Focusing the upper repair makes it the active one.
        d.type_into("Note text", "");
        let upper_active = d.at_size(LARGE, repair_saves);
        d.observe(
            "iter-22-C-FD3-two-repairs-focus",
            upper_active.len() == 2
                && upper_active.iter().filter(|(_, filled)| *filled).count() == 1
                && upper_active[0].1,
            format!("Upper repair focused: saves {upper_active:?}"),
        );
        // Round 25 review: two open repairs are taller than 960x640, so the page scrolls. Tab
        // reaches each Save and brings it into view; so does the wheel.
        let b_message_top = |d: &Driver<'_>| {
            d.nodes()
                .into_iter()
                .find(|node| {
                    node.name.contains("Notes/Cracked.md cannot be read")
                        && node.role != egui::accesskit::Role::TextRun
                })
                .map(|node| node.rect)
        };
        let ring_in_view = |d: &Driver<'_>| {
            let (width, height) = d.window_size();
            let window = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(width, height));
            d.focus_rings()
                .into_iter()
                .any(|(ring, _)| window.contains_rect(ring))
        };
        d.press(Key::Escape);
        let b_hidden = !d
            .nodes()
            .into_iter()
            .filter(|node| node.name == "Save repaired note")
            .all(|node| node.in_view());
        let tab_save = |d: &mut Driver<'_>| {
            let reached = d.tab_to("Save repaired note");
            d.settle();
            reached && ring_in_view(d)
        };
        let a_tab = tab_save(d);
        let b_tab = tab_save(d);
        let lower = d
            .focus_rings()
            .into_iter()
            .any(|(ring, _)| b_message_top(d).is_some_and(|message| ring.top() > message.top()));
        d.observe(
            "iter-25-C-page-tab",
            b_hidden && a_tab && b_tab && lower,
            format!(
                "B's Save hidden at first {b_hidden}; Tab to A's Save in view {a_tab}; Tab to B's Save in view {b_tab}, below B's message {lower}"
            ),
        );
        d.press(Key::Escape);
        let page = b_message_top(d).map(|rect| rect.center()).unwrap();
        d.scroll_up_at(page, 20);
        let a_wheel = d.nodes().into_iter().any(|node| {
            node.name == "Save repaired note"
                && node.in_view()
                && b_message_top(d).is_some_and(|message| node.rect.top() < message.top())
        });
        d.scroll_at(page, 20);
        let b_wheel = d.nodes().into_iter().any(|node| {
            node.name == "Save repaired note"
                && node.in_view()
                && b_message_top(d).is_some_and(|message| node.rect.top() > message.top())
        });
        // Tab into B's editor from the top of the page, where the page cuts it: the editor
        // scrolls the page until the whole editor is in view.
        let b_editor_name = crate::world::repair::editor_name("Notes/Cracked.md");
        let whole = |d: &Driver<'_>| {
            d.nodes()
                .into_iter()
                .find(|node| node.name == b_editor_name)
                .is_some_and(|node| {
                    // The editor's own viewport: its text, up to the editor's maximum height.
                    let viewport = node.rect.height().min(
                        crate::design::XXL * 6.0
                            + crate::design::L
                            + crate::design::S
                            + crate::design::XS,
                    );
                    node.in_view() && node.shown.height() >= viewport - 2.0
                })
        };
        d.scroll_up_at(page, 20);
        let rects = |d: &Driver<'_>| {
            d.nodes()
                .into_iter()
                .find(|node| node.name == b_editor_name)
                .map(|node| (node.rect, node.shown))
        };
        let before_rects = rects(d);
        let editor_cut = !whole(d);
        let editor_tab = d.tab_to(&b_editor_name);
        // The page scrolls with egui's animation.
        d.settle();
        let editor_whole = whole(d);
        let after_rects = rects(d);
        d.press(Key::Escape);
        d.observe(
            "iter-25-C-page-tab-editor",
            editor_cut && editor_tab && editor_whole,
            format!("B's editor cut at the top {editor_cut}; Tab reaches it {editor_tab}; then whole in view {editor_whole}; (rect, shown) before {before_rects:?}, after {after_rects:?}"),
        );
        d.observe(
            "iter-25-C-page-wheel",
            a_wheel && b_wheel,
            format!("The wheel brings A's Save into view {a_wheel}, then B's Save {b_wheel}"),
        );
        // Round 25 FD1: A's discard result belongs to A. B's edit leaves it, and Undo discard
        // brings back A's draft only; B's edit stays.
        let b_name = crate::world::repair::editor_name("Notes/Cracked.md");
        let draft = |d: &mut Driver<'_>, path: &str| {
            d.app_mut()
                .campaign_content
                .repair_draft(path)
                .map(str::to_owned)
                .unwrap_or_default()
        };
        d.click_below("Notes/Broken.md cannot be read", "Discard");
        let a_discarded = !draft(d, "Notes/Broken.md").contains("First draft.");
        // B's edit keeps its line count, so its editor shows the same lines.
        let b_draft = second.replacen("Second draft.", "Second draft. Kept B line.", 1);
        d.replace_text_in(&b_name, &b_draft);
        let b_changed = draft(d, "Notes/Cracked.md").contains("Kept B line.");
        let a_result = d.has("Undo discard of Broken.md");
        if a_result {
            d.click("Undo discard of Broken.md");
        } else {
            // A fault lost A's Undo: put A's words back by hand, so the observation below fails
            // and the later steps still start from both drafts.
            d.replace_text_in(
                &crate::world::repair::editor_name("Notes/Broken.md"),
                &first,
            );
        }
        let a_back = draft(d, "Notes/Broken.md").contains("First draft.")
            && !d.has("Undo discard of Broken.md");
        let b_kept = draft(d, "Notes/Cracked.md").contains("Kept B line.");
        // The displayed words too: each editor scrolled into view shows its own draft.
        let shown = |d: &mut Driver<'_>, editor: &str, words: &str| {
            d.type_into(editor, "");
            d.settle();
            let shown = d.visible_passage(words);
            d.press(Key::Escape);
            shown
        };
        let a_shown = shown(
            d,
            &crate::world::repair::editor_name("Notes/Broken.md"),
            "First draft.",
        );
        let b_shown = shown(d, &b_name, "Kept B line.");
        d.observe(
            "iter-25-C-FD1-owner-shown",
            a_shown && b_shown,
            format!(
                "A's editor shows First draft. {a_shown}; B's editor shows Kept B line. {b_shown}"
            ),
        );
        d.observe(
            "iter-25-C-FD1-owner",
            a_discarded && b_changed && a_result && a_back && b_kept,
            format!(
                "A discarded {a_discarded}, B changed {b_changed}, A's Undo kept through B {a_result}, A back {a_back}, B kept {b_kept}"
            ),
        );
        // An open run owns `Next turn`: no repair Save is primary, and both drafts stay.
        go_encounters(d);
        run_saved_encounter(d, SIBLING_ENTRY);
        go_campaign(d);
        let (in_run, kept) = d.at_size(LARGE, |d| {
            (
                repair_saves(d),
                d.visible_text("First draft.") && d.visible_text("Second draft."),
            )
        });
        d.observe(
            "iter-22-C-FD3-two-repairs-run",
            in_run.len() == 2 && in_run.iter().all(|(_, filled)| !filled) && kept,
            format!("With a run open: saves {in_run:?}; both drafts shown: {kept}"),
        );
        d.click_starting("Run · ");
        close_run(d);
        go_campaign(d);

        // Round 23 review: each repair editor names its file, so a DM who tabs between the
        // two hears which is which. No repair editor is named only `Note text`.
        let (a_editor, b_editor) = (
            crate::world::repair::editor_name("Notes/Broken.md"),
            crate::world::repair::editor_name("Notes/Cracked.md"),
        );
        let bare = d
            .nodes()
            .iter()
            .filter(|node| node.name == "Note text")
            .count();
        d.observe(
            "iter-23-C-FA9-names",
            d.has(&a_editor) && d.has(&b_editor) && bare == 0,
            format!(
                "Editors named {a_editor:?}: {}, {b_editor:?}: {}; named only Note text: {bare}",
                d.has(&a_editor),
                d.has(&b_editor)
            ),
        );

        // Round 23 review: a failed Save belongs to its own repair. A's cause stays through B's
        // edit, B's Save and B's own failed Save; B's state is its own.
        const READ_ONLY: &str = "Not saved. This file is read only.";
        const CHANGED_ON_DISK: &str = "Not saved. This entry changed on disk.";
        const B_MESSAGE: &str = "Notes/Cracked.md cannot be read";
        let top_of = |d: &Driver<'_>, words: &str| {
            d.nodes()
                .into_iter()
                .filter(|node| {
                    node.name.contains(words) && node.role != egui::accesskit::Role::TextRun
                })
                .map(|node| node.rect.top())
                .collect::<Vec<_>>()
        };
        // A's cause sits above B's message; B's state sits below it.
        let a_owns = |d: &Driver<'_>| {
            let message = top_of(d, B_MESSAGE);
            let cause = top_of(d, READ_ONLY);
            message.len() == 1 && cause.len() == 1 && cause[0] < message[0]
        };
        let b_state = |d: &Driver<'_>, word: &str| {
            let message = top_of(d, B_MESSAGE);
            message.len() == 1 && top_of(d, word).iter().any(|top| *top > message[0])
        };
        let a_file = d.root.join(BROKEN_NOTE);
        let b_file = d.root.join(SECOND);
        let set_read_only = |file: &std::path::Path, read_only: bool| {
            let mut permissions = std::fs::metadata(file).unwrap().permissions();
            permissions.set_readonly(read_only);
            std::fs::set_permissions(file, permissions).unwrap();
        };
        set_read_only(&a_file, true);
        d.click_first("Save repaired note");
        d.settle();
        let a_before = a_owns(d);
        // Round 23 review: an unchanged frame does not check the drafts again.
        let checks = d.app_mut().campaign_content.repair_checks();
        d.at_size(LARGE, |_| ());
        d.settle();
        let unchanged = d.app_mut().campaign_content.repair_checks();
        d.replace_text_in(&b_editor, &format!("{second}\nB edit.\n"));
        let edited = d.app_mut().campaign_content.repair_checks();
        d.observe(
            "iter-23-C-check-once",
            unchanged == checks && edited > unchanged,
            format!(
                "Draft checks: {checks} before, {unchanged} after unchanged frames at two sizes, {edited} after an edit"
            ),
        );
        d.observe(
            "iter-23-C-own-failure-edit",
            a_before && a_owns(d) && b_state(d, "Unsaved"),
            format!(
                "A failed first: {a_before}; after B's edit, A's cause {} and B Unsaved {}",
                a_owns(d),
                b_state(d, "Unsaved")
            ),
        );
        d.click_below(B_MESSAGE, "Save repaired note");
        d.settle();
        let b_written = std::fs::read_to_string(&b_file)
            .unwrap()
            .contains("B edit.");
        d.observe(
            "iter-23-C-own-failure-save",
            b_written && a_owns(d) && b_state(d, "No changes"),
            format!(
                "After B's Save: B written {b_written}, A's cause {}, B No changes {}",
                a_owns(d),
                b_state(d, "No changes")
            ),
        );
        d.replace_text_in(&b_editor, &format!("{second}\nB again.\n"));
        std::fs::remove_file(&b_file).unwrap();
        d.click_below(B_MESSAGE, "Save repaired note");
        d.settle();
        d.observe(
            "iter-23-C-own-failure-fail",
            a_owns(d) && b_state(d, CHANGED_ON_DISK) && !b_file.exists(),
            format!(
                "After B's Save fails: A's cause {}, B's own cause {}",
                a_owns(d),
                b_state(d, CHANGED_ON_DISK)
            ),
        );
        set_read_only(&a_file, false);
        d.click("Discard");
        d.click("Discard");
        let _ = std::fs::remove_file(d.root.join(SECOND));
        d.click(RELOAD);
        d.settle();
        assert!(!d.visible_text("Notes/Cracked.md"));
    }
}

/// W7: read the party's cached sheets, and the states with nothing to show.
pub mod party {
    use super::*;
    const FIRST_MEMBER: &str = "Brannoch Vale";
    const SECOND_MEMBER: &str = "Iselle Marrowind";
    const OFFLINE: &str = "Not connected";
    const FRESHNESS: &str = "Fetched";
    const PAGES: [(&str, &str); 5] = [
        ("Actions", "page-actions"),
        ("Spells", "page-spells"),
        ("Equipment", "page-equipment"),
        ("Features", "page-features"),
        ("Background", "page-background"),
    ];
    const OVERVIEW: &str = "Overview";
    const SOURCE: &str = "Technical details";
    const COMPARE: [&str; 3] = ["AC 19", "HP 58/62", "Level 6 Fighter"];
    const NO_CHARACTERS: &str = "No characters yet";
    const NOT_LINKED: &str = "Connect D&D Beyond to see the party";

    pub fn run(d: &mut Driver<'_>) {
        go_party(d);
        d.expect(FIRST_MEMBER);
        d.expect(OFFLINE);
        for size in [SMALL, DEFAULT, (1920.0, 1080.0)] {
            let one_line = d.at_size(size, |d| {
                let first = d.place_of("Overview").unwrap();
                let tabs: Vec<_> = [
                    "Overview",
                    "Actions",
                    "Spells",
                    "Equipment",
                    "Features",
                    "Background",
                    "More sheet pages",
                ]
                .iter()
                .filter_map(|name| d.place_of(name))
                .collect();
                tabs.len() >= 2
                    && tabs
                        .iter()
                        .all(|tab| (tab.center().y - first.center().y).abs() < 1.0)
                    && (d.in_view("Background") || d.in_view("More sheet pages"))
            });
            d.observe(
                "iter6-B-SD2",
                one_line,
                format!("Sheet tabs stay on one line at {size:?}"),
            );
        }
        d.state("list-and-sheet", Sizes::AllAndHiDpi);
        d.check_tab_order("party, character sheet");
        d.goal("see the party and a sheet");
        d.observe(
            "T7.2",
            d.in_view_containing(FRESHNESS),
            "Freshness text is visible",
        );
        let row = d
            .nodes()
            .into_iter()
            .find(|node| node.name.starts_with(FIRST_MEMBER) && node.in_view())
            .map_or_else(String::new, |node| node.name);
        d.observe(
            "Party compare",
            COMPARE.iter().all(|word| row.contains(word)),
            format!("A member's row shows AC, HP and class with nothing opened: {row}"),
        );

        d.click_containing(SECOND_MEMBER);
        d.state("second-character", Sizes::Default);
        d.goal("change character");
        d.scroll_at(pos2(720.0, 500.0), 4);
        d.state("sheet-scrolled", Sizes::Default);
        for (page, state) in PAGES {
            if !d.in_view(page) {
                assert!(d.in_view("More sheet pages"));
                d.click("More sheet pages");
            }
            assert!(d.in_view(page));
            d.click(page);
            d.state(
                state,
                if matches!(page, "Features" | "Background") {
                    Sizes::All
                } else {
                    Sizes::Default
                },
            );
            if page == "Actions" {
                // A Party tile shows where its attack lands, as Library and run tiles do.
                d.observe(
                    "iter-20-R-FD8",
                    d.visible_text("Reach 5 ft") && d.visible_text("Range 120 ft"),
                    "Party's Dagger and Fire Bolt tiles show their reach and range",
                );
            }
            if page == "Equipment" {
                d.observe(
                    "P29",
                    !d.has("Filter this page"),
                    "A short list needs no filter",
                );
            }
            d.observe(
                "T7.2",
                d.in_view_containing(FRESHNESS),
                format!("Freshness text on {page}"),
            );
        }
        d.click(OVERVIEW);
        d.click(SOURCE);
        d.state("source-and-parity", Sizes::Default);
        d.goal("read every sheet page");

        switch_campaign(d, QUIET_CAMPAIGN);
        d.expect(NO_CHARACTERS);
        d.observe(
            "P20",
            !d.has("Connect to refresh") && d.has("Connect D&D Beyond"),
            "An empty Party has one Open Settings action",
        );
        d.state("no-characters-signed-out", Sizes::All);
        d.goal("see a linked campaign with no characters");

        switch_campaign(d, UNLINKED_CAMPAIGN);
        d.expect(NOT_LINKED);
        d.expect("First connect your D&D Beyond account. Then choose its campaign under Campaign → Link and folder.");
        d.state("campaign-not-linked", Sizes::All);
        d.goal("see a campaign with no link");

        // A note edited outside Splat stops its save. Force read needs a session, then replaces
        // the note and keeps the edited version in an archive.
        let copies = d.root.join("campaigns/emberfall/Characters/D&D Beyond");
        let note = std::fs::read_dir(&copies)
            .expect("list character copies")
            .map(|file| file.expect("read character copy").path())
            .find(|path| {
                path.is_file()
                    && path.extension().is_some_and(|x| x == "md")
                    && !path.ends_with(crate::sheet_writer::FILE)
            })
            .expect("a character copy");
        std::fs::write(&note, EDITED_NOTE).expect("edit the copy on disk");
        switch_campaign(d, CAMPAIGN);
        d.expect(CONNECT);
        d.observe(
            "Force read needs a session",
            !d.has(FORCE_READ) && std::fs::read_to_string(&note).is_ok_and(|t| t == EDITED_NOTE),
            "Signed out, the warning says to connect, and the edited note stays",
        );
        d.observe(
            "P18",
            d.has_containing("Local edits"),
            "The affected party row marks its local edits",
        );
        // The warning asks the DM to connect, so its own line has the way to connect, first.
        for size in [SMALL, DEFAULT, LARGE] {
            let (placed, detail) = d.at_size(size, |d| {
                let words = d.painted_rect_in("Connect, then replace it.", egui::Rect::EVERYTHING);
                let connect = d.place_of("Connect D&D Beyond");
                let file = d.place_of("Show file");
                let placed = match (words, connect, file) {
                    (Some(words), Some(connect), Some(file)) => {
                        // One line: the bordered button is taller than the link beside it.
                        (connect.top()..=connect.bottom()).contains(&file.center().y)
                            && file.left() - connect.right() >= crate::design::L - 0.5
                            && connect.left() >= words.left() - 0.5
                    }
                    _ => false,
                };
                (
                    placed,
                    format!("words {words:?}, connect {connect:?}, file {file:?}"),
                )
            });
            d.observe(
                "iter-14-B-FD1",
                placed,
                format!(
                    "Connect D&D Beyond comes before Show file on the warning at {size:?}: {detail}"
                ),
            );
        }
        // One action, one name: the warning's Connect D&D Beyond is the only way to connect on
        // the page. The header keeps the state.
        d.observe_both(
            "iter-15-B-FD8",
            "With the warning's Connect D&D Beyond, the header shows only Not connected and when it fetched",
            |d| {
                !d.has(CONNECT_FIRST)
                    && d.nodes().iter().filter(|node| node.name == CONNECT).count() == 1
                    && d.visible_text("Not connected · Fetched")
            },
        );
        // Round 25 FG7: an `L` gap ends the notice; the list and the sheet start below it.
        for size in [SMALL, DEFAULT, LARGE] {
            let (gap, detail) = d.at_size(size, |d| {
                let words =
                    d.painted_rect_in("so Splat stopped updating it.", egui::Rect::EVERYTHING);
                let actions = [CONNECT, "Show file"].map(|name| d.place_of(name));
                let Some(bottom) = words
                    .into_iter()
                    .chain(actions.into_iter().flatten())
                    .map(|rect| rect.bottom())
                    .reduce(f32::max)
                else {
                    return (None, "no notice".to_owned());
                };
                let next = d
                    .nodes()
                    .into_iter()
                    .filter(|node| node.in_view() && node.rect.top() > bottom)
                    .min_by(|a, b| a.rect.top().total_cmp(&b.rect.top()));
                let heading = d
                    .painted_rect_in("NAME", egui::Rect::EVERYTHING)
                    .map(|rect| rect.top());
                let gap = next
                    .as_ref()
                    .map(|node| node.rect.top())
                    .into_iter()
                    .chain(heading)
                    .map(|top| top - bottom)
                    .reduce(f32::min);
                (
                    gap.filter(|_| words.is_some() && actions.iter().all(Option::is_some)),
                    format!(
                        "notice bottom {bottom}, next {:?}, NAME {heading:?}",
                        next.map(|node| (node.name, node.rect.top()))
                    ),
                )
            });
            d.observe(
                "iter-25-C-FG7",
                gap.is_some_and(|gap| gap >= crate::design::L - 0.5),
                format!("At {size:?}, the gap below the local-edit notice is {gap:?}: {detail}"),
            );
        }
        // Round 25 family: the member list keeps the scroll bar's lane with no bar, so its rows
        // and header columns do not move when the list starts to scroll.
        for size in [SMALL, DEFAULT, LARGE] {
            let (kept, detail) = d.at_size(size, |d| {
                let lane = d.app_style().spacing.scroll.allocated_width();
                let edge = d.column_edge("party_members");
                // The member's row in the list: its name holds AC and HP, left of the splitter.
                let row = d
                    .nodes()
                    .into_iter()
                    .find(|node| {
                        node.name.starts_with(FIRST_MEMBER)
                            && COMPARE.iter().all(|word| node.name.contains(word))
                            && node.rect.right() < edge
                    })
                    .map(|node| node.rect);
                (
                    row.is_some_and(|row| row.right() <= edge - crate::design::M - lane + 1.0),
                    format!("row {row:?}, column edge {edge}, lane {lane}"),
                )
            });
            d.observe(
                "iter-25-C-party-lane",
                kept,
                format!("At {size:?}, the member rows keep the scroll bar lane: {detail}"),
            );
        }
        d.state("copy-has-local-edits-signed-out", Sizes::All);
        d.click(CONNECT);
        d.observe(
            "iter-14-B-FD1-opens",
            d.page_title().as_deref() == Some("Settings")
                && d.focused_name().as_deref() == Some("Paste session cookie"),
            "Connect D&D Beyond on the warning opens Settings with the cookie field focused",
        );
        d.click(NAV_PARTY);
        d.expect(CONNECT);
        // Must stay true: the edited note is still on disk, untouched, after the detour.
        d.observe(
            "iter-14-B-FD1-safe",
            std::fs::read_to_string(&note).is_ok_and(|t| t == EDITED_NOTE),
            "Opening Settings from the warning leaves the edited note as it was",
        );
        d.observe(
            "iter-15-B-FD8-back",
            !d.has(CONNECT_FIRST)
                && d.nodes().iter().filter(|node| node.name == CONNECT).count() == 1,
            "Back from Settings, the warning still owns the one Connect D&D Beyond",
        );

        d.app_mut()
            .settings
            .simulate(crate::settings::Account::Connected);
        d.settle();
        d.expect(FORCE_READ);
        d.state("copy-has-local-edits", Sizes::All);
        d.click(FORCE_READ);
        d.settle();
        d.expect(REPLACED);
        // Round 25 FD3: one dismiss form on the page. The replaced-sheet notice ends its first
        // line with the close icon, as the stopped-sync notice does; no word `Dismiss` is left.
        d.observe_both(
            "iter-25-C-FD3",
            "The replaced-sheet notice has the close icon at the end of its first line",
            |d| {
                let words = d.painted_rect_in(REPLACED, egui::Rect::EVERYTHING);
                let file = d.place_of("Show file");
                let close = d.place_of(DISMISS);
                let word = d.nodes().iter().any(|node| node.name == "Dismiss");
                match (words, file, close) {
                    (Some(words), Some(file), Some(close)) => {
                        !word
                            && close.left() > file.right()
                            && close.left() > words.right()
                            && close.top() < words.top() + crate::design::L
                    }
                    _ => false,
                }
            },
        );
        d.state("force-read-replaced", Sizes::Default);
        let archive = note.with_extension("md.splat-archive");
        d.observe(
            "Force read replaces and keeps",
            std::fs::read_to_string(&note).is_ok_and(|t| t != EDITED_NOTE && t.starts_with("---"))
                && std::fs::read_to_string(&archive).is_ok_and(|t| t == EDITED_NOTE)
                && !d.has(FORCE_READ),
            "The note holds the sheet, the archive holds the edited words, and the warning is gone",
        );
        d.click(DISMISS);
        d.settle();
        d.observe(
            "Force read report dismisses",
            !d.has_containing(REPLACED),
            "Dismiss removes the report of where the edits went",
        );
        d.app_mut()
            .settings
            .simulate(crate::settings::Account::NotConnected);
        d.settle();
        d.click("Connect to refresh");
        d.observe(
            "P26",
            d.page_title().as_deref() == Some("Settings")
                && d.focused_name().as_deref() == Some("Paste session cookie"),
            "Connect to refresh opens Settings with the cookie field focused",
        );
        d.click("Paste session cookie");
        d.wait_for_text("Paste session cookie");
        d.observe(
            "Coverage",
            d.focused_name().as_deref() == Some("Paste session cookie"),
            "The cookie field is focused before capture",
        );
        d.state_prepared("connect-to-refresh", Sizes::Default, |d| {
            d.click("Paste session cookie");
            d.settle();
            d.observe(
                "iter-5-P13",
                d.focused_name().as_deref() == Some("Paste session cookie"),
                "The cookie field is focused after each capture resize",
            );
        });
    }
    const FORCE_READ: &str = "Replace with D&D Beyond sheet";
    const CONNECT_FIRST: &str = "Connect to refresh";
    const CONNECT: &str = "Connect D&D Beyond";
    const REPLACED: &str = "sheet now matches D&D Beyond. Your edited version is kept.";
    const DISMISS: &str = "Dismiss this message";
    const EDITED_NOTE: &str = "My own words";
}

/// W5: build an encounter from a cold start, then inspect a saved one.
pub mod encounter_build {
    use super::*;
    const NEW_NAME: &str = "Ashfall Crossing";
    const MONSTER: &str = "wolf";
    const BOSS: &str = "cultist";
    const SAVED_NOTE: &str = "Saved";
    const SAVED_ENCOUNTER: &str = "Bridge Ambush";
    const SAVED_FIRST_SLOT: &str = "1. Brannoch Vale";
    const SYNC_OPTION: &str = "Sync party with D&D Beyond";
    const PARTY_NOW: &str = "Party now: HP";
    const LIVE_ROW: &str = "HP\u{a0}11/41";
    const LOCAL_ONLY: &str = "Nothing in this encounter reaches D&D Beyond";
    const VIRTUALISED_LIMIT: usize = 100;
    // One disk-change form, as World and Library: it names the encounter, never its file.
    const CHANGED_ON_DISK: &str =
        "This encounter changed on disk. Your draft is safe until you choose.";
    const OVERWRITE: &str = "Overwrite with my draft";
    const RELOAD: &str = "Reload and lose my draft";
    const OUTSIDE_WORDS: &str = "\nWritten outside Splat.\n";
    const DRAFT_WORDS: &str = " Ash on the water.";
    const RESULT_NAME: &str = "Wolf";
    const OWN_CREATURE: &str = "Cinder Wolf";
    const EDITOR_NAME: &str = "Encounter name";
    const BACK_TO_LIST: &str = "Back to encounters";
    const NO_SUCH_CREATURE: &str = "zzz";
    const SAVED_ROWS: [&str; 8] = [
        "1. Brannoch Vale",
        "2. Iselle Marrowind",
        "3. Kestrel Dunmoor",
        "4. Oren Thistlewick",
        "5. Sable Harrowgate",
        "6. Wolf",
        "7. Wolf",
        "8. Cultist",
    ];

    /// A page's title: the painted text with exactly these words in the largest type.
    fn page_title_left(d: &Driver<'_>, title: &str) -> Option<f32> {
        let mut found: Option<(f32, f32)> = None;
        d.each_painted_text(|text, clip| {
            let height = text.galley.size().y;
            if text.galley.job.text == title
                && clip.intersects(text.visual_bounding_rect())
                && found.is_none_or(|(tallest, _)| height > tallest)
            {
                found = Some((height, text.pos.x));
            }
        });
        found.map(|(_, left)| left)
    }

    pub fn run(d: &mut Driver<'_>) {
        // iter-24-B FE4: the Encounters title starts at the shared page edge, as Party's does,
        // at every size. Measured before T5.3's task begins, so its cost is unchanged.
        d.expect(READY_TO_RUN);
        let sizes = [SMALL, DEFAULT, LARGE];
        let encounters = sizes.map(|size| d.at_size(size, |d| page_title_left(d, NAV_ENCOUNTERS)));
        // FA5 (round 25 replaces round 24's gutter): the first column starts at the title's x.
        let content = sizes.map(|size| d.at_size(size, |d| d.painted_left(READY_TO_RUN)));
        d.click(NAV_PARTY);
        d.expect("Party");
        let party = sizes.map(|size| d.at_size(size, |d| page_title_left(d, NAV_PARTY)));
        go_encounters(d);
        d.observe(
            "iter-24-B-FE4",
            encounters.iter().zip(&party).all(|(encounters, party)| {
                encounters
                    .zip(*party)
                    .is_some_and(|(encounters, party)| (encounters - party).abs() < 0.5)
            }),
            format!(
                "Encounters title x {encounters:?}, Party title x {party:?} at 960, 1200, 1920"
            ),
        );
        d.observe(
            "iter-25-B-FA5",
            encounters.iter().zip(&content).all(|(title, content)| {
                title
                    .zip(*content)
                    .is_some_and(|(title, content)| (content - title).abs() < 0.5)
            }),
            format!(
                "Title x {encounters:?}, list heading x {content:?}: one edge at 960, 1200, 1920"
            ),
        );
        // T5.3 measures the whole build, so it starts from the cold start.
        d.begin_task();
        d.expect(READY_TO_RUN);
        d.state("encounter-list", Sizes::All);
        d.check_tab_order("encounters, list");

        // A new draft selects its name, and adding moves focus to the search.
        d.click(NEW_ENCOUNTER);
        d.type_text(NEW_NAME);
        d.expect("5. ");
        d.state("new-encounter", Sizes::Default);

        // The party is already there, and Enter goes on to the search.
        d.press(Key::Enter);
        d.type_text(MONSTER);
        d.state("find-creature", Sizes::Default);
        // The result row for the plain Wolf, not a word anywhere on the page.
        let nodes = d.nodes();
        // The line under its name, read as the DM sees it: in columns.
        let detail = nodes
            .iter()
            .find(|node| node.name == RESULT_NAME && node.in_view())
            .map_or_else(String::new, |name| {
                d.painted_in(eframe::egui::Rect::from_min_max(
                    pos2(name.rect.min.x - 1.0, name.rect.max.y),
                    pos2(name.rect.min.x + 400.0, name.rect.max.y + 18.0),
                ))
            });
        d.observe(
            "T5.1",
            ["CR ", "HP ", "AC "]
                .iter()
                .all(|word| detail.contains(word)),
            format!("The Wolf result shows CR, HP and AC: {detail}"),
        );
        // A campaign's own creature has its challenge rating in its text, not in a field.
        let own = d
            .nodes()
            .iter()
            .find(|node| node.name == OWN_CREATURE && node.in_view())
            .map_or_else(String::new, |name| {
                d.painted_in(eframe::egui::Rect::from_min_max(
                    pos2(name.rect.min.x - 1.0, name.rect.max.y),
                    pos2(name.rect.min.x + 400.0, name.rect.max.y + 18.0),
                ))
            });
        d.observe(
            "T5.1",
            own.contains("CR "),
            format!("A campaign creature's result shows its CR too: {own}"),
        );
        d.click_first(ADD);
        d.expect("6. ");
        d.click_first(ADD);
        d.expect("7. ");
        d.type_text(BOSS);
        d.click_first(ADD);
        d.expect("8. ");
        d.state("creatures-added", Sizes::All);
        d.click(SAVE_ENCOUNTER);
        d.expect(SAVED_NOTE);
        d.state("saved", Sizes::Default);
        d.goal_within(
            "T5.3",
            "save a roster of party and monsters from a cold start",
            10,
        );
        d.observe(
            "T5.3",
            SAVED_ROWS.iter().all(|row| d.has_label(row)),
            format!("The saved roster has the 8 rows asked for: {SAVED_ROWS:?}"),
        );

        // T5.7: the roster's line ends with its XP and difficulty: five characters at levels 5
        // and 6 have Low 2,800, and the three creatures are worth 200 XP each. The heading in a
        // wide editor, the tab row in a narrow one.
        for size in [SMALL, DEFAULT, LARGE] {
            let words = d.at_size(size, |d| d.painted_in(eframe::egui::Rect::EVERYTHING));
            d.observe(
                "T5.7",
                words.contains("ROSTER 8 · XP 125 of 2,800 · Low")
                    || words.contains("Low · XP 125 of 2,800"),
                format!("At {size:?} the saved roster says its XP and word: {words}"),
            );
        }
        // Read a full stat block in Lookup, with the editor still open.
        let before = d.interaction_count();
        d.click("Cultist");
        let cost = d.interaction_count() - before;
        let read = d.has_containing("ARMOR CLASS");
        // Round 23 SB1: an exact match shows no creation action, so no empty row stands between
        // `Show 3 results` and the rule above the entry.
        for size in [SMALL, DEFAULT, LARGE] {
            let (close, detail) = d.at_size(size, |d| {
                let Some(show) = d.place_of("Show the results") else {
                    return (false, "no Show the results".to_owned());
                };
                // The rule spans the Lookup pane, so its centre lies right of the action.
                let below = eframe::egui::Rect::from_min_max(
                    pos2(show.left(), show.bottom() - 0.5),
                    pos2(f32::INFINITY, show.bottom() + crate::design::XXL * 2.0),
                );
                let rule = d
                    .hairlines_in(below)
                    .into_iter()
                    .map(|rule| rule.center().y)
                    .min_by(f32::total_cmp);
                (
                    rule.is_some_and(|rule| rule - show.bottom() <= crate::design::S + 1.0)
                        && !d.has_containing("New NPC")
                        && !d.has("Generate NPC"),
                    format!("Show the results {show:?}, rule at {rule:?}"),
                )
            });
            d.observe(
                "iter-23-D-SB1",
                close,
                format!("The rule follows Show 3 results with no empty row at {size:?}: {detail}"),
            );
        }
        d.state("stat-block-beside-editor", Sizes::All);
        // The Cultist's Skills row in Lookup: `Deception +2, Religion +2`.
        check_skills_row(
            d,
            "iter-21-S-fd4-lookup-skills",
            2,
            &[SMALL, DEFAULT, LARGE],
        );
        let still_editing = d.has_containing(EDITOR_NAME);
        // The real loop: read, decide, add. The results stay beside what was just read.
        let add_in_view = d.nodes().iter().any(|n| n.name == ADD && n.in_view());
        d.press(Key::Escape);
        d.observe(
            "T5.2",
            cost <= 2 && read && still_editing && add_in_view,
            format!(
                "A full stat block opens in {cost} beside the editor. The draft stays open, and an Add button is in view: {add_in_view}"
            ),
        );
        d.begin_task();

        // A search with no match says so, and what to try.
        d.replace_text_in(FIND_CREATURE, NO_SUCH_CREATURE);
        d.expect("Campaign → Packages");
        d.state("no-search-results", Sizes::All);
        // iter-24-B FB4: one sentence in one size: the words, the link and its full stop.
        let mut sizes = Vec::new();
        let mut stops = Vec::new();
        let mut sentence = None;
        d.each_painted_text(|text, _| {
            let words = &text.galley.job.text;
            let size = text
                .galley
                .job
                .sections
                .first()
                .map(|s| s.format.font_id.size);
            if words == "Check the spelling, or check" || words == "Campaign → Packages" {
                sizes.push((words.clone(), size));
                if words.starts_with("Check") {
                    sentence = Some(text.pos.y);
                }
            }
            if words == "." {
                stops.push((text.pos, size));
            }
        });
        let link = d.place_of("Campaign → Packages");
        // The full stop on the link's line, after its last word.
        let stop = link.and_then(|link| {
            stops
                .iter()
                .find(|(pos, _)| {
                    pos.x >= link.left() && pos.y >= link.top() - 1.0 && pos.y < link.bottom()
                })
                .map(|(pos, size)| (pos.x, *size))
        });
        d.observe(
            "iter-24-B-FB4",
            sizes.len() == 2
                && sizes
                    .iter()
                    .all(|(_, size)| size.is_some() && *size == sizes[0].1)
                && stop.is_some_and(|(x, size)| {
                    size == sizes[0].1 && link.is_some_and(|link| x >= link.left())
                })
                && sentence.is_some(),
            format!("Sizes {sizes:?}, full stop {stop:?}, link {link:?}"),
        );
        d.observe(
            "T5.1",
            d.visible_text("No creatures match"),
            "A search with no match says so",
        );
        d.begin_task();

        d.clear_text_in(FIND_CREATURE);
        let rows = d.nodes().iter().filter(|node| node.name == ADD).count();
        d.observe(
            "T5.4",
            rows > 0 && rows <= VIRTUALISED_LIMIT,
            format!("{rows} result rows are built each frame with no search text, from about 340"),
        );

        d.click(BACK_TO_LIST);
        d.expect(READY_TO_RUN);
        let nodes = d.nodes();
        let dated = nodes.iter().any(|node| {
            node.name.contains(RUN) && node.name.contains("Paused") && node.name.contains("Round 2")
        });
        let first = nodes
            .iter()
            .position(|node| node.name.starts_with("IN PROGRESS "))
            .zip(
                nodes
                    .iter()
                    .position(|node| node.name.contains(READY_TO_RUN)),
            )
            .is_some_and(|(unfinished, ready)| unfinished < ready);
        d.observe(
            "T4.4",
            dated && first,
            format!("Unfinished runs show their encounter, round, and paused status: {dated}. They come first: {first}"),
        );
        d.state("saved-runs", Sizes::Default);

        d.expect(SAVED_ENCOUNTER);
        open_saved_encounter(d, SAVED_ENCOUNTER);
        d.expect(SAVED_FIRST_SLOT);
        d.observe(
            "P1",
            d.has(RESUME) && !d.has(RUN_ENCOUNTER) && d.has_containing("Round 2 · Paused"),
            "The editor offers Resume for its paused run and shows its state",
        );
        d.state("saved-roster", Sizes::All);
        d.goal("open a saved encounter");
        d.begin_task();

        // T5.6: what the party has left is in view while the DM decides what to put against it.
        let (option, together, row) = (
            d.has(SYNC_OPTION),
            d.visible_text(PARTY_NOW),
            d.visible_text(LIVE_ROW)
                && d.visible_text("AC\u{a0}15")
                && d.visible_text("Bloodied")
                && d.visible_text("Poisoned"),
        );
        d.observe(
            "T5.6",
            option && together && row,
            format!(
                "With sync on, the roster has the option: {option}. It says what the party has now, together: {together}. And for each row: {row}"
            ),
        );
        d.click(SYNC_OPTION);
        d.state("saved-roster-sync-off", Sizes::Default);
        d.observe(
            "T5.6",
            d.visible_text(LOCAL_ONLY) && !d.visible_text(PARTY_NOW),
            "With sync off, the roster says that nothing reaches D&D Beyond, and the live line goes",
        );
        d.click(SYNC_OPTION);
        d.expect(PARTY_NOW);
        // AD1: the help under Sync party and Keep in Ready wraps at the reader measure at 1920.
        let widths = d.at_size(LARGE, |d| {
            [
                "The party starts with the HP",
                "Lists this encounter under Ready",
            ]
            .map(|words| {
                let mut width = None;
                d.each_painted_text(|text, _| {
                    if text.galley.job.text.starts_with(words) {
                        width = Some(text.galley.size().x);
                    }
                });
                width
            })
        });
        d.observe(
            "iter-25-B-AD1",
            widths.iter().all(|width| {
                width.is_some_and(|width| width <= crate::design::READER_MEASURE + 0.5)
            }),
            format!("At 1920, the Sync party and Keep in Ready help widths: {widths:?}"),
        );

        // Another program changes the encounter while the DM edits it. The choices start at the
        // message text: the safe one first, then exactly `L` before the red one.
        let file = d
            .root
            .join("campaigns/emberfall/Encounters/Bridge Ambush.md");
        let outside = std::fs::read_to_string(&file).expect("read the encounter") + OUTSIDE_WORDS;
        std::fs::write(&file, &outside).expect("change the encounter on disk");
        d.type_into("Read-aloud and notes", DRAFT_WORDS);
        d.click(SAVE_ENCOUNTER);
        d.expect(CHANGED_ON_DISK);
        d.observe(
            "iter-25-B-FE3",
            {
                let named: Vec<String> = d
                    .nodes()
                    .into_iter()
                    .filter(|node| node.name.contains("changed on disk"))
                    .map(|node| node.name)
                    .collect();
                !named.is_empty() && named.iter().all(|name| name == CHANGED_ON_DISK)
            },
            "The disk-change warning names the encounter, not its file",
        );
        d.observe(
            "iter-15-B-EC-safe",
            std::fs::read_to_string(&file).is_ok_and(|text| text == outside)
                && d.has_containing(OUTSIDE_WORDS.trim()),
            "The changed file stays as it is, and the comparison shows its words, until the DM chooses",
        );
        for size in [SMALL, DEFAULT, LARGE] {
            let (placed, detail) = d.at_size(size, |d| {
                let text = d.painted_rect_in(CHANGED_ON_DISK, egui::Rect::EVERYTHING);
                let safe = d.place_of(OVERWRITE);
                let red = d.place_of(RELOAD);
                let placed = match (text, safe, red) {
                    (Some(text), Some(safe), Some(red)) => {
                        (safe.left() - text.left()).abs() < 1.0
                            && safe.top() >= text.bottom() - 0.5
                            && (safe.center().y - red.center().y).abs() < 1.0
                            && (red.left() - safe.right() - crate::design::L).abs() < 0.5
                    }
                    _ => false,
                };
                (placed, format!("text {text:?}, safe {safe:?}, red {red:?}"))
            });
            d.observe(
                "iter-15-B-EC",
                placed,
                format!(
                    "Overwrite with my draft starts at the message text, and the red choice is L after it at {size:?}: {detail}"
                ),
            );
        }
        d.state_prepared("encounter-changed-on-disk", Sizes::All, |d| {
            conflict_choices_read_height(d, "iter-15-B2-density", OVERWRITE, RELOAD);
            // Each part that differs has a line in the DM's words, above the raw lines.
            let size = d.window_size();
            d.observe(
                "iter-23-B-FB1",
                d.visible_text("your draft adds “Ash on the water.”")
                    && d.visible_text("the file adds “Written outside Splat.”")
                    && d.visible_text("the file names the same 8 rows in another form."),
                format!("At {size:?}, the conflict names what differs"),
            );
        });
        d.click(OVERWRITE);
        d.expect(SAVED_NOTE);
        d.observe(
            "iter-15-B-EC-overwrite",
            !d.has(OVERWRITE)
                && std::fs::read_to_string(&file).is_ok_and(|text| text.contains(DRAFT_WORDS)),
            "Overwrite with my draft still saves the draft over the changed file",
        );
        d.click(RESUME);
        d.expect(&format!("{RUN_HEADING}Bridge Ambush"));
        d.observe(
            "P1",
            d.has_containing("Bridge Ambush · Round 2") && !d.has("Begin combat"),
            "Resume from the editor opens the paused second round",
        );
        d.state("resumed-from-editor", Sizes::All);

        // T12.12: Remove… on a Ready to run row, only for an encounter with no saved run, with
        // the words and the Undo of a saved group's removal. An unrelated act between them
        // stays.
        go_encounters(d);
        if d.has(BACK_TO_LIST) {
            d.click(BACK_TO_LIST);
        }
        d.expect(READY_TO_RUN);
        let file = d
            .root
            .join(format!("campaigns/emberfall/Encounters/{NEW_NAME}.md"));
        let text = std::fs::read_to_string(&file).expect("the new encounter");
        d.click_first(&format!("More actions for {NEW_NAME}"));
        d.expect("Remove…");
        d.state("encounter-menu-remove", Sizes::All);
        d.click("Remove…");
        d.expect(&format!("Removed encounter {NEW_NAME}."));
        d.observe(
            "T12.12",
            !file.exists() && !d.has(&format!("More actions for {NEW_NAME}")),
            "Remove… takes the encounter's file away, and its row goes",
        );
        d.state("encounter-removed", Sizes::All);
        d.click(BACK_TO_RUN);
        d.click(NEXT_TURN);
        let turn = d.app_mut().encounters.run().unwrap().strip().current;
        go_encounters(d);
        d.click("Undo remove encounter");
        d.expect(&format!("More actions for {NEW_NAME}"));
        let same_turn = d.app_mut().encounters.run().unwrap().strip().current == turn;
        d.observe(
            "T12.12",
            std::fs::read_to_string(&file).is_ok_and(|back| back == text) && same_turn,
            "Undo remove encounter writes the same file back, and the turn taken between stays",
        );
    }
}

/// W1: run combat at the table: turn order, hit points and conditions.
pub mod encounter_run {
    use super::*;
    const TARGET: &str = "Ember Cultist";
    const DAMAGE: &str = "7";
    const AFTER_DAMAGE: &str = "HP 26/33";
    const CONDITION: &str = "Frightened";
    const UNDO: &str = "Undo: ";
    const SECOND_ENCOUNTER: &str = "Shrine of Cinders";
    const ROLL_NPCS: &str = "Roll NPC initiative";
    const KEYBOARD_LOOKUP: &str = "bless";
    const ANCHOR_ROW: &str = "Ember Cultist";

    pub fn run(d: &mut Driver<'_>) {
        d.expect(READY_TO_RUN);
        // Image already covered by encounter-build / encounter-list. Keep every size check.
        d.check_state("saved-runs", Sizes::Default);
        resume_run(d, RUN);
        d.expect(CURRENT_TURN);
        d.app_mut()
            .encounters
            .run_mut()
            .unwrap()
            .simulate_fresh_live_history();
        d.settle();
        d.expect("Nothing to undo");
        d.state("live-with-no-history", Sizes::AllAndHiDpi);
        d.check_tab_order("encounter run");
        d.goal("resume a run in progress");
        d.review("T1.8", "the initiative list");
        d.review("T4.5", "the live view, to compare with run-setup");

        let anchor = d.row_node(ANCHOR_ROW).map(|row| row.rect.min.y);
        let before = current_participant(d);
        d.click(NEXT_TURN);
        let after = current_participant(d);
        d.observe(
            "T1.1",
            before.is_some() && after.is_some() && before != after,
            format!("Next turn moves the current turn: {before:?} then {after:?}"),
        );
        d.state("next-turn", Sizes::Default);
        d.goal_within("T1.1", "advance the turn", 1);
        let after = d.row_node(ANCHOR_ROW).map(|row| row.rect.min.y);
        d.observe(
            "Stable rows",
            anchor.is_some() && anchor == after,
            format!("Rows hold their places on Next turn: {anchor:?} then {after:?}"),
        );

        d.click_row(TARGET);
        d.state("participant-selected", Sizes::Default);
        d.type_text(DAMAGE);
        d.press(Key::Enter);
        d.observe(
            "T1.2",
            d.row_node(TARGET)
                .is_some_and(|row| row.name.contains(AFTER_DAMAGE))
                && d.row_words(TARGET).contains("26"),
            format!("The row paints the new HP: {}", d.row_words(TARGET)),
        );
        d.observe_both("T1.2", "The damaged row stays in view", |d| {
            d.row_node(TARGET).is_some_and(|row| row.fully_in_view())
        });
        d.state("hp-changed", Sizes::All);
        d.observe_both(
            "iter-12-F2",
            "The run card shows current and maximum HP after damage",
            |d| d.visible_text("26 / 33"),
        );
        d.goal_within("T1.2", "damage a named participant who is not current", 3);

        d.click_row(TARGET);
        d.click(CONDITIONS);
        d.state("condition-tab", Sizes::All);
        // AI1: the condition form is a form, not the pane: three aligned columns, each as wide as
        // its longest choice and at most `L` from the next, with Add inside the form width.
        let columns = d.at_size(LARGE, |d| {
            let field = d.place_of("Other effect").unwrap_or(egui::Rect::NOTHING);
            condition_columns(d, field.left(), 0.0)
        });
        d.observe(
            "iter-23-R-AI1",
            condition_columns_close(&columns),
            format!("At 1920 the run's condition columns (left, right, count): {columns:?}"),
        );
        let form = d.at_size(LARGE, |d| {
            let field = d.place_of("Other effect").unwrap_or(egui::Rect::NOTHING);
            let nodes = d.nodes();
            let toggles: Vec<_> = crate::combat_sync::CONDITIONS
                .iter()
                .filter_map(|(_, name)| {
                    nodes
                        .iter()
                        .find(|node| node.name == *name && node.rect.left() >= field.left() - 1.0)
                        .map(|node| node.rect)
                })
                .collect();
            let left = toggles.iter().map(|r| r.left()).fold(f32::MAX, f32::min);
            let right = toggles.iter().map(|r| r.right()).fold(0.0, f32::max);
            let mut columns: Vec<f32> = toggles.iter().map(|r| r.left().round()).collect();
            columns.sort_by(f32::total_cmp);
            columns.dedup();
            let add = nodes
                .iter()
                .find(|node| {
                    node.name == "Add" && (node.rect.center().y - field.center().y).abs() < 4.0
                })
                .map(|node| node.rect.right());
            (toggles.len(), columns.len(), left, right, add)
        });
        d.observe(
            "iter-23-R-AI1",
            matches!(form, (count, 3, left, right, Some(add))
                if count == crate::combat_sync::CONDITIONS.len()
                    && right - left <= crate::design::FORM_WIDTH + 1.0
                    && add - left <= crate::design::FORM_WIDTH + 1.0),
            format!("At 1920 the conditions keep three columns inside the form width: {form:?}"),
        );
        let clean = !d.row_words(TARGET).contains(CONDITION);
        let lefts = |d: &Driver<'_>| {
            let field = d.place_of("Other effect").unwrap_or(egui::Rect::NOTHING);
            condition_columns(d, field.left(), 0.0)
                .iter()
                .map(|column| column.0)
                .collect::<Vec<_>>()
        };
        let before = lefts(d);
        d.click(CONDITION);
        // FI1: the columns are fixed, so turning on Frightened moves no other toggle.
        let after = lefts(d);
        d.observe(
            "iter-24-R-FI1",
            before.len() >= 2 && before == after,
            format!("Condition columns before and after Frightened: {before:?} then {after:?}"),
        );
        d.state("condition-added", Sizes::All);
        d.goal_within("T1.5", "add an official condition", 3);
        let row = d.row_words(TARGET);
        d.observe(
            "T1.5",
            clean && row.contains(CONDITION) && row.contains("26"),
            format!("The row gains the condition and keeps its HP. Nothing was typed: {row}"),
        );

        // The label keeps the action and direction; accessibility keeps the full target.
        let undo_text = format!("{UNDO}{CONDITION} on {TARGET}");
        d.observe(
            "T1.6",
            d.in_view(&undo_text) && d.visible_passage(&format!("{CONDITION} on")),
            format!("At 960x640, Undo shows its action and keeps the full name: {undo_text}"),
        );
        d.click_containing(UNDO);
        d.expect_same("Undo returns to the Conditions panel before the condition was added.");
        d.state("undo", Sizes::Default);
        d.observe(
            "T1.6",
            !d.row_words(TARGET).contains(CONDITION),
            "Undo removes what it named",
        );
        d.goal("undo the last action");

        // Shift+Enter heals, and Enter damages. The Keys tab says so.
        d.click_row(TARGET);
        let hp = d.row_hp(TARGET);
        d.type_text("4");
        d.press(Key::Enter);
        let hurt = d.row_hp(TARGET);
        d.type_text("3");
        d.press_shift(Key::Enter);
        let healed = d.row_hp(TARGET);
        d.observe(
            "T1.7",
            hurt == hp.map(|hp| hp - 4) && healed == hp.map(|hp| hp - 1),
            format!("Enter takes 4, then Shift+Enter gives back 3: {hp:?}, {hurt:?}, {healed:?}"),
        );
        d.press(Key::U);
        d.press(Key::U);

        // A target by key: the dock names the row the amount will act on.
        let selected_before = d.focused_row_name();
        d.press(Key::ArrowUp);
        let target = selected_before.is_some() && selected_before != d.focused_row_name();

        d.begin_task();
        let before = current_participant(d);
        d.press(Key::N);
        let next = current_participant(d) != before;
        d.state("next-turn-by-keyboard", Sizes::Default);
        let hp_before = d
            .nodes()
            .iter()
            .filter(|n| n.name.contains("HP "))
            .map(|n| n.name.clone())
            .collect::<Vec<_>>();
        d.type_text("3");
        d.press(Key::Enter);
        let hp_after = d
            .nodes()
            .iter()
            .filter(|n| n.name.contains("HP "))
            .map(|n| n.name.clone())
            .collect::<Vec<_>>();
        let damage = hp_before != hp_after;
        d.press(Key::U);
        let undone = d
            .nodes()
            .iter()
            .filter(|n| n.name.contains("HP "))
            .map(|n| n.name.clone())
            .collect::<Vec<_>>();
        let undo = undone == hp_before;
        d.press_ctrl(Key::K);
        d.type_text(KEYBOARD_LOOKUP);
        let search = d.has("Bless");
        d.observe(
            "i19 written spell",
            !d.visible_text("%% splat") && !d.visible_text("Field"),
            "Lookup hides machine lines and empty field rows",
        );
        d.observe_both(
            "AT3/FT-6",
            "Lookup omits the reserved Splat data section",
            |d| !d.visible_text("Splat data"),
        );
        d.state("lookup-by-keyboard", Sizes::All);
        // FT1: Lookup's Show all is the quiet action Arrivals uses. It lists every match in
        // place, and the list follows keyboard focus. These proof steps end where they began.
        d.aside(|d| lookup_show_all(d, KEYBOARD_LOOKUP));
        d.press(Key::Escape);
        let back = !d.has(SEARCH_LOOKUP);
        let keys = d.goal("run a turn, damage, undo and a lookup by keyboard alone");
        d.observe(
            "T1.7",
            next && target && damage && undo && search && back,
            format!(
                "Direct key commands, {keys} key presses in all: next turn {next}, target {target}, damage {damage}, undo {undo}, lookup {search}, close {back}"
            ),
        );
        d.expect(RUN_HEADING);
        // Tab and Enter also reach the same control.
        let tabbed = d.activate_by_keyboard(NEXT_TURN);
        d.keyboard_result(
            "tab-to-next-turn",
            "encounter run",
            tabbed,
            "Tab and Enter reach Next turn".into(),
        );
        d.begin_task();

        close_run(d);
        d.expect(NAV_ENCOUNTERS);
        run_saved_encounter(d, SECOND_ENCOUNTER);
        d.expect(&format!("{RUN_HEADING}{SECOND_ENCOUNTER}"));
        d.state("run-setup", Sizes::All);
        // FT3: this roster has identical NPCs, so the default roll gives them one shared
        // roll and tie arrows. Their slot is there from the start: the roll moves nothing.
        let columns = |d: &Driver<'_>| {
            let mut fields: Vec<(String, f32)> = d
                .nodes()
                .into_iter()
                .filter(|node| node.name.starts_with("Initiative for ") && node.in_view())
                .map(|node| (node.name, node.rect.left()))
                .collect();
            fields.sort_by(|a, b| a.0.cmp(&b.0));
            (d.painted_left("INITIATIVE"), fields)
        };
        let tie_arrows = |d: &Driver<'_>| {
            d.nodes()
                .iter()
                .filter(|node| {
                    node.name.starts_with("Move ")
                        && (node.name.contains(" before ") || node.name.contains(" after "))
                        && node.in_view()
                })
                .count()
        };
        let before = columns(d);
        let arrows_before = tie_arrows(d);
        d.click(ROLL_NPCS);
        let after = columns(d);
        let arrows_after = tie_arrows(d);
        d.observe(
            "iter-18-B-FT3",
            before.0.is_some()
                && before.1.len() >= 3
                && arrows_before == 0
                && arrows_after > 0
                && before.0 == after.0
                && before.1 == after.1,
            format!(
                "Roll NPCs adds {arrows_after} tie arrows and moves no field: before {before:?}, after {after:?}"
            ),
        );
        d.observe(
            "iter-8-F10",
            !d.visible_text("1st") && !d.visible_text("2nd"),
            "Places wait until every present participant has initiative",
        );
        d.state("npc-initiative-rolled", Sizes::Default);
        d.goal("start a new run and roll initiative");

        d.click("Roll NPC initiative another way");
        d.state("npc-roll-menu", Sizes::Default);
        d.check_tab_order("NPC roll menu");
        d.click("Individually");
        d.expect_same(
            "Rolling another way changes only the initiative numbers in the same Setup rows.",
        );
        d.state("npc-initiative-rolled-individually", Sizes::Default);
        d.goal_within(
            "Roll once another way",
            "roll NPC initiative another way",
            2,
        );
    }
}

/// W10: connect to D&D Beyond and manage campaigns.
pub mod settings {
    use super::*;
    use crate::settings::Account;
    const ACCOUNT_WORDS: [(Account, &str); 5] = [
        (Account::Busy, "Connecting"),
        (Account::Connected, "Connected"),
        (Account::NotConnected, "Not connected"),
        (Account::Failed, "Connect failed"),
        (Account::KeyringWarning, "Connected · not remembered"),
    ];
    const REMOVE_CAMPAIGN: &str = "More actions for Quiet Harbor";
    const REMOVE_TITLE: &str = "Remove Quiet Harbor?";
    const NOT_CONNECTED: &str = "Not connected";
    const COOKIE_HELP: &str = "How to get your session cookie";
    const COOKIE_HELP_TEXT: &str = "Sign in to D&D Beyond in your browser";
    const COOKIE_FIELD: &str = "Paste session cookie";
    const SYNTHETIC_COOKIE: &str = "synthetic-not-a-real-cookie";
    const CONNECT: &str = "Connect";
    const DISCONNECT: &str = "Disconnect";
    const OFFLINE_ERROR: &str = "Could not reach D&D Beyond";
    const CAMPAIGNS_SECTION: &str = "CAMPAIGNS";
    const EDIT_DETAILS: &str = "Edit details";
    const EDIT_FORM: &str = "Campaign name";
    const CANCEL: &str = "Cancel";

    pub fn run(d: &mut Driver<'_>) {
        go_settings(d);
        d.expect(NOT_CONNECTED);
        d.state("connect-form", Sizes::AllAndHiDpi);
        d.check_tab_order("settings");
        d.goal("open settings");

        assert!(d.in_view("Connect…"));
        assert!(!d.in_view(COOKIE_FIELD));
        d.click("Connect…");
        d.observe_both(
            "iter-18-A-blocked",
            "Connect keeps its reason beside the button",
            |d| {
                let button = d.place_of(CONNECT).unwrap();
                let reason = d.place_of("Paste your session cookie first.").unwrap();
                (button.left() - reason.right() - crate::design::S).abs() < 1.0
            },
        );
        d.expect(COOKIE_FIELD);
        assert!(d.in_view(COOKIE_FIELD));
        assert!(!d.has("Connect…"));
        d.state_changed_at("cookie-placeholder", "connect-form", Sizes::Small);
        d.type_into(COOKIE_FIELD, SYNTHETIC_COOKIE);
        d.wait_for_text("Remember my D&D Beyond sign-in on this computer");
        d.observe_both("iter-13-U2", "Connect ends at the cookie field edge", |d| {
            d.place_of(CONNECT)
                .zip(d.place_of(COOKIE_FIELD))
                .is_some_and(|(button, field)| (button.right() - field.right()).abs() < 1.0)
        });
        d.state("cookie-entered", Sizes::Default);
        d.click(CONNECT);
        d.expect(OFFLINE_ERROR);
        // Round 23 SF1: the failure scrolls the form so its cause, the kept cookie and the retry
        // show together, at the 960 size where the error pushed Connect out of view.
        let fully = |d: &Driver<'_>, name: &str, exact: bool| {
            d.nodes().iter().any(|node| {
                (if exact {
                    node.name == name
                } else {
                    node.name.contains(name)
                }) && node.fully_in_view()
            })
        };
        d.observe(
            "iter-23-C-SF1",
            fully(d, OFFLINE_ERROR, false)
                && fully(d, COOKIE_FIELD, true)
                && fully(d, CONNECT, true),
            format!(
                "After a failed Connect at 960x640: error {}, cookie {}, Connect {}",
                fully(d, OFFLINE_ERROR, false),
                fully(d, COOKIE_FIELD, true),
                fully(d, CONNECT, true)
            ),
        );
        // Round 23 FM4: the trust text uses the DM's words, not the code's.
        d.observe(
            "iter-23-C-FM4-form",
            d.has_containing("With the box ticked, Splat signs in again when it starts.")
                && !d.has_containing("access token")
                && !d.has_containing("keyring"),
            "The connect form says what the remember box does, without token or keyring words",
        );
        // Round 24 FA6: after a real failed attempt, the account status and the rail name the
        // failure, so the error colour never sits on the idle words.
        let rail = d.rail_words();
        d.observe(
            "iter-24-C-FA6",
            d.visible_text("Connect failed") && rail.contains("Connect failed"),
            format!("After a failed Connect, the page and the rail say Connect failed: {rail}"),
        );
        d.state("connect-failed", Sizes::All);
        d.goal("try to connect without a network");
        // The cookie stays for a retry. There is nothing saved, so nothing to disconnect.
        let retry = !d.is_disabled(CONNECT) && !d.has(DISCONNECT);
        d.observe(
            "T10.3",
            retry,
            "After a failed attempt, Connect can retry at once",
        );
        // Each state through the same fields a real attempt sets.
        let mut words = true;
        let mut targets = Vec::new();
        // iter-25-A FG6: the painted one-line Connected status in the rail: where its icon
        // starts, and how wide it is with its word.
        let rail = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(100.0, 10_000.0));
        let mut one_line = None;
        for (state, word) in ACCOUNT_WORDS {
            d.app_mut().settings.simulate(state);
            d.settle();
            words &= d.in_view_containing(word);
            targets.push((
                word,
                [SMALL, DEFAULT, LARGE]
                    .map(|size| d.at_size(size, session_recording::rail_targets)),
            ));
            // Not connected and failed already have their captures, from real attempts.
            match state {
                Account::Busy => d.state("account-connecting", Sizes::Default),
                // Also supplies signed-in's shared Settings image at the wide size.
                Account::Connected => {
                    one_line = d.painted_rect_in(" Connected", rail);
                    d.observe(
                        "iter-23-C-FM4-connected",
                        d.visible_text("Splat stays signed in while it is open.")
                            && d.visible_text("Disconnect removes Splat’s saved D&D Beyond sign-in. It does not sign your browser out of D&D Beyond.")
                            && !d.has_containing("access token")
                            && !d.has_containing("local credentials"),
                        "Connected account help says what the DM keeps, once, in plain words",
                    );
                    d.state("account-connected", Sizes::All);
                }
                Account::KeyringWarning => {
                    d.state("account-keyring-warning", Sizes::Default);
                    d.observe_both(
                        "iter-12-U3",
                        "The rail paints the full keyring warning",
                        |d| {
                            let nodes = d.nodes();
                            let connected = nodes
                                .iter()
                                .find(|node| node.name == "Connected" && node.rect.left() < 100.0);
                            let detail = nodes.iter().find(|node| {
                                node.name == "Not remembered" && node.rect.left() < 100.0
                            });
                            connected.zip(detail).is_some_and(|(first, second)| {
                                first.fully_in_view()
                                    && second.fully_in_view()
                                    && (first.rect.left() - second.rect.left()).abs() < 1.0
                                    && second.rect.top() >= first.rect.bottom()
                            })
                        },
                    );
                    // iter-25-A FG6: the two-line status paints its icon and its word where the
                    // one-line status paints them. The one-line word starts where its whole
                    // text ends less the word's own width.
                    let icon = d.painted_rect_in(crate::design::icons::WARN, rail);
                    let word = d.painted_rect_in("Connected", rail);
                    d.observe(
                        "iter-25-A-FG6",
                        one_line
                            .zip(icon)
                            .zip(word)
                            .is_some_and(|((one, icon), word)| {
                                (one.left() - icon.left()).abs() <= 0.5
                                    && (one.right() - word.width() - word.left()).abs() <= 1.5
                            }),
                        format!(
                            "The two-line rail status paints its icon at {icon:?} and its word at \
                             {word:?}; the one-line status paints {one_line:?}"
                        ),
                    );
                    // AD2: this state really takes two lines, so the target check below compares
                    // a one-line account with a two-line one at every size.
                    let line = crate::design::Type::Small.line();
                    let heights = [SMALL, DEFAULT, LARGE].map(|size| {
                        d.at_size(size, |d| {
                            session_recording::account_rect(d).map(|rect| rect.height())
                        })
                    });
                    d.observe(
                        "iter-11-C-AD2-two-lines",
                        heights
                            .iter()
                            .all(|height| height.is_some_and(|h| h >= 2.0 * line - 0.5)),
                        format!(
                            "The keyring warning's account status is at least {} tall at each \
                             size: {heights:?}",
                            2.0 * line
                        ),
                    );
                    session_recording::check_rail_anchor(d, "iter-11-C-AD2-anchor");
                }
                Account::NotConnected | Account::Failed | Account::RefreshFailing => {}
            }
        }
        d.observe(
            "T10.3",
            words,
            "Busy, connected, not connected, failed and keyring warning each have words, from a test session",
        );
        // AD2: the account slot holds two lines, so Lookup, Notes and Settings stay where they are
        // when the account status takes one line or two.
        let first = targets[0].1;
        d.observe(
            "iter-11-C-AD2",
            targets
                .iter()
                .any(|(word, _)| word.contains("not remembered"))
                && targets.iter().all(|(_, at)| {
                    at.iter().all(Option::is_some)
                        && at.iter().zip(first.iter()).all(|(now, was)| now == was)
                }),
            format!(
                "Lookup, Notes and Settings keep their tops in every account state: {targets:?}"
            ),
        );
        d.app_mut().settings.simulate(Account::NotConnected);
        d.settle();
        d.expect(NOT_CONNECTED);

        d.row_of(CAMPAIGNS_SECTION);
        // FT-6: the rail's status slot gained Sync stopped above Lookup. Its targets stay still,
        // so the rest of the page is the cookie step's.
        let sync_stopped = d.rail_words().contains("Sync stopped");
        d.observe(
            "iter-10-C-FT6-settings",
            sync_stopped,
            "The rail shows Sync stopped in its status slot",
        );
        d.expect_same(
            "The rail's status slot gains Sync stopped; Lookup, Notes, Settings and the account line stay where they were.",
        );
        d.state("campaign-list", Sizes::All);
        d.goal("find campaign management");

        d.click(EDIT_DETAILS);
        d.expect(EDIT_FORM);
        d.observe_both(
            "iter-18-A-edges",
            "Campaign fields and actions share their right edge",
            |d| {
                let name = d
                    .nodes()
                    .iter()
                    .find(|node| {
                        node.name == "Campaign name"
                            && node.role == egui::accesskit::Role::TextInput
                    })
                    .unwrap()
                    .rect;
                let browse = d.place_of("Browse…").unwrap();
                let save = d.place_of("Save details").unwrap();
                (name.right() - browse.right()).abs() < 1.0
                    && (name.right() - save.right()).abs() < 1.0
            },
        );
        d.state("edit-campaign", Sizes::Default);
        d.click(CANCEL);
        d.click(REMOVE_CAMPAIGN);
        d.click("Remove…");
        d.expect(REMOVE_TITLE);
        // Round 24 FC5: Remove campaign uses the shared dialog, anchored at the top like
        // Unregister, so the DM meets the same question in one place.
        d.observe_both(
            "iter-24-C-FC5",
            "The Remove campaign dialog's title sits at the shared top anchor",
            |d| {
                d.painted_rect_in(REMOVE_TITLE, egui::Rect::EVERYTHING)
                    .is_some_and(|title| {
                        (title.top() - (crate::design::XXL + crate::design::L)).abs() <= 2.0
                    })
            },
        );
        d.state("confirm-remove-campaign", Sizes::Standard);
        d.press(Key::Escape);
        d.goal("edit a campaign's details");
        d.click(EDIT_DETAILS);
        let saved_name = d.field_text(EDIT_FORM).unwrap();
        d.replace_text_in(EDIT_FORM, "Quiet Harbor revised");
        // Round 25 FG5: the discard result sits in the message space below the save row, so the
        // campaign rows below the form keep their y through Discard and Undo discard.
        let rows = |d: &mut Driver<'_>| {
            [SMALL, DEFAULT].map(|size| {
                d.at_size(size, |d| {
                    d.nodes()
                        .into_iter()
                        .filter(|node| node.name.contains(UNLINKED_CAMPAIGN) && node.in_view())
                        .map(|node| node.rect.top())
                        .reduce(f32::min)
                })
            })
        };
        let changed = rows(d);
        d.click("Discard");
        assert_eq!(
            d.field_text(EDIT_FORM).as_deref(),
            Some(saved_name.as_str())
        );
        let discarded = rows(d);
        d.observe(
            "iter-25-C-FG5",
            changed.iter().all(Option::is_some)
                && changed == discarded
                && d.visible_text("Campaign details draft discarded."),
            format!("{UNLINKED_CAMPAIGN} row y: changed {changed:?}, discarded {discarded:?}"),
        );
        d.state("campaign-details-discarded", Sizes::Default);
        d.click("Undo discard of Campaign details");
        assert_eq!(
            d.field_text(EDIT_FORM).as_deref(),
            Some("Quiet Harbor revised")
        );
        let undone = rows(d);
        d.observe(
            "iter-25-C-FG5-undo",
            undone == changed && !d.visible_text("Campaign details draft discarded."),
            format!("{UNLINKED_CAMPAIGN} row y after Undo discard: {undone:?}"),
        );
        d.click("Discard");
        d.click(CANCEL);
        d.begin_task();
        d.app_mut().settings.simulate(Account::Connected);
        d.settle();
        d.click("Add from D&D Beyond");
        d.wait_for_text("Salt Road");
        d.click("Add Salt Road from D&D Beyond");
        d.state_prepared("add-from-remote", Sizes::Default, |d| {
            if !d.has("Campaign name") {
                d.click("Add from D&D Beyond");
            }
            d.row_of("Campaign name");
            d.wait_for_text("Campaign name");
            assert_eq!(d.field_text("Campaign name").as_deref(), Some("Salt Road"));
            for _ in 0..4 {
                if d.nodes()
                    .iter()
                    .any(|node| node.name == "Add campaign" && node.fully_in_view())
                {
                    break;
                }
                let folder = d.row_of("Folder");
                d.scroll_at(folder.center(), 1);
            }
            let controls = d.nodes();
            let cancel = controls.iter().find(|node| node.name == "Cancel").unwrap();
            let add = controls
                .iter()
                .find(|node| node.name == "Add campaign")
                .unwrap();
            assert!(cancel.rect.right() < add.rect.left());
            assert!((cancel.rect.center().y - add.rect.center().y).abs() <= 1.0);
            assert!(cancel.fully_in_view() && add.fully_in_view());
        });
        d.click(CANCEL);
        d.app_mut().settings.simulate(Account::NotConnected);
        d.settle();

        d.state_prepared("campaign-row-menu", Sizes::Default, |d| {
            if !d.in_view("Remove…") {
                d.click(REMOVE_CAMPAIGN);
            }
            d.wait_for_text("Remove…");
        });
        d.press(Key::Escape);
        d.click(COOKIE_HELP);
        d.expect(COOKIE_HELP_TEXT);
        d.state_prepared("cookie-help", Sizes::Default, |d| {
            if !d.has_containing(COOKIE_HELP_TEXT) {
                d.click(COOKIE_HELP);
            }
            d.row_of(COOKIE_HELP_TEXT);
            d.wait_for_text(COOKIE_HELP_TEXT);
        });
        d.click(
            "Party sync stopped when D&D Beyond disconnected. Connect, then turn sync on in Party.",
        );
        d.expect("Party");
        for size in [SMALL, DEFAULT, (1920.0, 1080.0)] {
            let aligned = d.at_size(size, |d| {
                let header = d.nodes().into_iter().filter(|node| node.name == "Party")
                    .max_by(|a, b| a.rect.left().total_cmp(&b.rect.left())).unwrap().rect;
                let notice = d.place_of("Combat sync stopped when the connection to D&D Beyond ended. Changes not yet sent did not reach D&D Beyond. Connect in Settings, and sync starts again.").unwrap();
                let dismiss = d.place_of("Dismiss this message").unwrap();
                notice.left() < header.left() + crate::design::L * 3.0
                    && dismiss.left() > notice.left()
                    && dismiss.top() < notice.top() + crate::design::L
            });
            d.observe(
                "iter6-B-FD9",
                aligned,
                format!("Party notice starts at the page edge with a trailing dismiss at {size:?}"),
            );
        }
        d.state("sync-stopped-recovery", Sizes::All);
        assert!(d.in_view("Dismiss this message"));
        d.click("Dismiss this message");
        assert!(!d.has_containing("Combat sync stopped when the connection"));

        go_settings(d);
        d.click("Touchpad");
        d.expect("Scrolling speed");
        d.expect("Mouse wheels keep their speed.");
        d.state("touchpad-settings", Sizes::Default);
        d.click("8×");
        let chosen = d.app_mut().touchpad_speed == crate::touchpad::TouchpadSpeed::Eight;
        d.observe(
            "touchpad-speed",
            chosen,
            "Choosing a touchpad speed changes how far a touchpad scroll moves",
        );
        d.click("Account and campaigns");
    }
}

/// The copy dialog, and the unsaved-changes prompt when closing.
pub mod dialogs {
    use super::*;
    const CREATURES_SECTION: &str = "Creatures";
    const CREATURE: &str = "Aboleth";
    const COPY_TITLE: &str = "Copy entry";
    const CLOSE_WINDOW: &str = "Cancel";
    const NOTE: &str = "Ash Exposure";
    const CLOSE_PROMPT: &str = "has unsaved changes.";
    const KEEP_EDITING: &str = "Keep editing";

    pub fn run(d: &mut Driver<'_>) {
        go_library(d);
        d.begin_task();
        d.click("Section");
        d.click(&format!("Library section: {CREATURES_SECTION}"));
        d.expect(CREATURE);
        d.click(COPY_TO);
        d.expect(COPY_TITLE);
        d.expect("Type a new folder, or choose one. A new folder is made with the file.");
        d.state("copy-dialog", Sizes::All);
        d.goal("open the copy dialog");

        escape_closes(d, "copy dialog", COPY_TITLE, CLOSE_WINDOW);

        d.click(COPY_TO);
        d.observe(
            "Copy default",
            d.visible_text(&format!("Campaign · {CAMPAIGN}")),
            "The dialog starts on the active campaign",
        );
        d.click("Destination");
        d.state("copy-destination-menu", Sizes::Default);
        d.click(&format!("Campaign · {CAMPAIGN}"));
        // The copy dialog has fixed bounds; its wide layout is captured before typing.
        d.state_prepared("copy-renamed", Sizes::Standard, |d| {
            if d.field_text("Copy name").as_deref() != Some("Watch Aboleth") {
                d.replace_text_in("Copy name", "Watch Aboleth");
            }
            d.wait_for_text("Watch Aboleth.md");
        });
        d.observe(
            "Copy review actions",
            d.in_view("Create copy") && d.in_view("Cancel") && !d.has("Review copy"),
            "The review ends in one row of actions",
        );
        d.click(CREATE_COPY);
        d.expect("Copied to ");
        d.state("copy-complete", Sizes::Default);
        d.click("Edit the copy");
        d.expect("Markdown editor");
        d.state("edit-the-copy", Sizes::All);
        d.observe_both(
            "W1/C1",
            "The copied Watch Aboleth row has no broken-link warning",
            |d| {
                d.has_containing("Watch Aboleth · ")
                    && !d.nodes().iter().any(|node| {
                        node.name.starts_with("Watch Aboleth · ")
                            && node.name.contains("broken link")
                    })
            },
        );
        d.goal("copy an entry to a campaign");

        open_local_package(d, MODULE);
        d.click_containing(NOTE);
        d.click(EDIT_MARKDOWN);
        d.type_into_editor("Draft ruling. ");
        d.expect(UNSAVED_NOTE);
        d.request_close();
        d.observe(
            "Close prompt",
            d.has_containing(&format!("\u{201c}{NOTE}\u{201d}"))
                && d.has_containing(MODULE)
                && d.has_containing(CLOSE_PROMPT),
            "The prompt names the entry, and the package it is in",
        );
        d.observe_both(
            "FD-12",
            "The destructive close action stays left of the safe actions on one line",
            |d| {
                let nodes = d.nodes();
                let rect = |name: &str| {
                    nodes
                        .iter()
                        .find(|node| {
                            node.name == name && node.role == egui::accesskit::Role::Button
                        })
                        .unwrap()
                        .rect
                };
                let (lose, keep, save) = (
                    rect("Discard and close"),
                    rect("Keep editing"),
                    rect("Save and close"),
                );
                lose.right() < keep.left()
                    && keep.right() < save.left()
                    && (lose.center().y - keep.center().y).abs() < 1.0
                    && (save.center().y - keep.center().y).abs() < 1.0
            },
        );
        // Round 24 FE6: the help names the three choices in the order the DM reads them.
        d.expect("Save and close, keep editing, or close and lose these changes.");
        d.state("unsaved-changes-on-close", Sizes::All);
        d.goal("ask to close with unsaved changes");

        escape_closes(d, "unsaved changes prompt", CLOSE_PROMPT, KEEP_EDITING);
        d.expect(UNSAVED_NOTE);
        d.observe(
            "C1 focus",
            d.focused_name().as_deref() == Some("Markdown editor"),
            "Keep editing restores focus to the draft",
        );
        d.state("kept-editing", Sizes::All);
        d.goal("return to the draft");
    }
}

/// W10: nothing set up yet, then a new campaign's placeholder tabs.
pub mod empty {
    use super::*;
    const NO_CAMPAIGN: &str = "No campaign yet";
    const ADD_CAMPAIGN: &str = "Add campaign";
    const NAME_FIELD: &str = "Campaign name";
    const FOLDER_FIELD: &str = "Folder";
    const OPEN_CAMPAIGN: &str = "Add campaign";
    const NEW_CAMPAIGN: &str = "Ashes of Emberfall";
    const NEW_FOLDER: &str = "new-campaign";
    const NO_ENTRIES: &str = "entry you can use";
    const OPEN_SETTINGS: &str = "Add campaign";
    const NO_CAMPAIGN_ENCOUNTERS: &str = "No campaign yet";
    const NO_CAMPAIGN_PARTY: &str = "No campaign yet";
    const NOT_LINKED: &str = "Connect D&D Beyond to see the party";
    const CONTENT_TITLE: &str = "entries chosen";
    const NEXT_STEP: &str = "Add campaign";

    pub fn run(d: &mut Driver<'_>) {
        d.observe(
            "T10.1",
            d.has(NAV_LIBRARY) && !d.has_containing("Error") && d.has_containing(NEXT_STEP),
            "The Library works with no campaign, and one next step is stated",
        );
        d.review("T10.1", "first launch shows one next step");
        // Image already covered by library-browse / first-launch. Keep every size check.
        d.check_state("first-launch", Sizes::Default);
        d.click("Active campaign: No campaign");
        d.expect("Add campaign…");
        // Round 25 FE4: the popup's one command is one line, inside the popup.
        d.observe_both(
            "iter-25-C-FE4",
            "The switcher's Add campaign… is one line, with S on each side, inside the column",
            |d| {
                d.painted_rect_in("Add campaign…", egui::Rect::EVERYTHING)
                    .zip(d.place_of("Add campaign…"))
                    .is_some_and(|(words, row)| {
                        words.height() < 1.5 * crate::design::Type::Body.line()
                            && words.right() <= row.right() - crate::design::S + 0.5
                            && words.left() >= row.left() + crate::design::S - 0.5
                            && d.place_of("Active campaign: No campaign")
                                .is_some_and(|control| row.right() <= control.right() + 0.5)
                    })
            },
        );
        d.state("empty-campaign-switcher", Sizes::Default);
        d.click("Add campaign…");
        assert_eq!(d.current_page(), "Settings");
        d.expect(NAME_FIELD);
        go_library(d);
        d.begin_task();
        go_campaign(d);
        d.expect(NO_CAMPAIGN);
        d.state("no-campaign", Sizes::All);
        d.check_tab_order("campaign, no campaign");
        d.goal("see the no-campaign state");
        d.begin_task();
        let campaign_says =
            d.has_containing(NO_CAMPAIGN) && d.has(NAME_FIELD) && d.has(ADD_CAMPAIGN);
        go_encounters(d);
        let encounters_says = d.has_containing(NO_CAMPAIGN_ENCOUNTERS) && d.has(OPEN_SETTINGS);
        d.state("encounters-no-campaign", Sizes::Default);
        go_party(d);
        let party_says = d.has_containing(NO_CAMPAIGN_PARTY) && d.has(OPEN_SETTINGS);
        d.state("party-no-campaign", Sizes::Default);
        d.observe(
            "T10.2",
            campaign_says && encounters_says && party_says,
            format!("With no campaign, each page says what to do. Campaign {campaign_says}, Encounters {encounters_says}, Party {party_says}"),
        );
        d.begin_task();

        go_settings(d);
        d.expect(ADD_CAMPAIGN);
        d.state("settings-no-campaigns", Sizes::All);
        let folder = d.root.join(NEW_FOLDER);
        std::fs::create_dir_all(&folder).expect("create campaign folder");
        d.type_into(NAME_FIELD, NEW_CAMPAIGN);
        d.type_into(FOLDER_FIELD, &folder.display().to_string());
        d.click_below(FOLDER_FIELD, OPEN_CAMPAIGN);
        d.expect(NEW_CAMPAIGN);
        d.state_prepared("campaign-added", Sizes::Default, |d| {
            assert!(
                !d.has(NAME_FIELD),
                "A successful add closes the form at every window size"
            );
        });
        d.goal("add a first campaign");
        d.observe(
            "Campaign added",
            d.has("Build an encounter"),
            "After adding a campaign, the next step is a button",
        );

        go_campaign(d);
        d.expect(CONTENT_TITLE);
        d.click(TAB_ENTRIES);
        d.expect(NO_ENTRIES);
        d.observe_both("U4", "The empty scope menu keeps the category width", |d| {
            d.nodes()
                .into_iter()
                .find(|node| node.name.starts_with("What the Library shows:"))
                .is_some_and(|node| node.rect.width() <= 166.0)
        });
        d.state("no-campaign-entries", Sizes::Default);
        // Round 26 review: in the empty campaign scope, a create error shows in the pane's
        // footer, and nothing is written. Cancel ends it.
        let files = |d: &Driver<'_>| {
            fn count(dir: &std::path::Path) -> usize {
                std::fs::read_dir(dir).map_or(0, |entries| {
                    entries
                        .flatten()
                        .map(|entry| {
                            let path = entry.path();
                            if path.is_dir() { count(&path) } else { 1 }
                        })
                        .sum()
                })
            }
            count(&d.root.join(NEW_FOLDER))
        };
        let before = files(d);
        d.click("New entry");
        d.type_into("Entry name", "A/B");
        d.click("Create entry");
        let error = "Use a name without path separators or line breaks.";
        let shown = d.visible_text(error);
        let written = files(d) != before;
        d.click("Cancel");
        d.observe(
            "iter-26-L-empty-scope-error",
            shown && !written && !d.visible_text(error) && d.has(NO_ENTRIES),
            format!(
                "The create error shows in the empty scope ({shown}), nothing is written ({}), and Cancel ends it",
                !written
            ),
        );
        d.begin_task();
        d.click("entry you can use");
        d.expect("Can use");
        d.state("entries-you-can-copy", Sizes::Default);
        d.audit_goal_within(
            "iter-11-U2",
            "find an entry to copy from the empty state",
            1,
        );
        go_encounters(d);
        d.observe_both("U6", "The empty encounter actions share a line", |d| {
            d.place_of("Create a first encounter")
                .zip(d.place_of("New group"))
                .is_some_and(|(encounter, group)| {
                    (encounter.center().y - group.center().y).abs() < 1.0
                        && group.left() - encounter.right() >= crate::design::L
                })
        });
        d.state("no-encounters", Sizes::Default);
        go_party(d);
        d.expect(NOT_LINKED);
        d.state("party-without-link", Sizes::Default);
        d.observe(
            "Party next step",
            d.has("Connect D&D Beyond") && !d.has("Link this campaign"),
            "With no session, the next step is to connect, because linking needs one",
        );
        d.goal("see the other empty tabs");
    }
}

/// W2: look something up during a run, then return. Lookup opens beside the roster,
/// so the run stays on screen and Next turn stays clickable.
pub mod run_lookup {
    use super::*;
    const SECOND_RESULT: &str = "Delayed Blast Fireball";
    const PARTICIPANT: &str = "Ember Cultist";
    const SPELL_QUERY: &str = "fireball";
    const SPELL: &str = "Fireball";
    const PACKAGE_ENTRY: &str = "Ash Exposure";
    const CAMPAIGN_ENTRY: &str = "Captain Varra";
    const SPELL_IN_STAT_BLOCK: &str = "Ashfall";
    const SPELL_BODY: &str = "Hot ash falls";
    const OTHER_NOTE: &str = "Ashfall";
    const NOTE: &str = "Ash Exposure";
    const DRAFT_WORDS: &str = "Unfinished ruling. ";

    pub fn run(d: &mut Driver<'_>) {
        resume_run_from_cold_start(d);
        d.click_row(PARTICIPANT);
        d.state("in-the-run", Sizes::All);
        d.goal("be in a run, with a participant selected");

        look_up(d, SPELL_QUERY);
        d.expect(SPELL);
        d.observe(
            "Keys",
            d.visible_text("Keys"),
            "Keys keeps its word beside Lookup at 960x640",
        );
        d.observe(
            "i19 character-copy spell",
            !d.visible_text("%% splat") && !d.visible_text("Field"),
            "The character-copy spell reads without machine lines or an empty field table",
        );
        d.state("reading-spell-beside-run", Sizes::All);
        d.goal_within("T2.1", "read a spell from a run", 3);
        // The top match opens with the other results still visible.
        d.observe(
            "T2.1",
            d.has_containing(SECOND_RESULT),
            "At 960x640 the best match opens with the other results visible",
        );
        let builtin_found = d.in_view(SPELL);
        d.observe_both("T2.1", "Roster and Next turn stay visible", |d| {
            d.in_view(NEXT_TURN)
                && d.row_node(PARTICIPANT)
                    .is_some_and(|row| row.fully_in_view())
        });
        // SI1: at 960 the strip's hidden action is in a menu, by name and number, and the mouse
        // arms it while Lookup stays open.
        const HIDDEN_ACTION: &str = "Cinder Dart +3";
        let more = d
            .nodes()
            .into_iter()
            .find(|node| node.name.ends_with(" more") && node.role == egui::accesskit::Role::Button)
            .map(|node| node.name);
        d.observe(
            "iter-24-R-SI1-menu",
            more.as_deref() == Some("1 more") && !d.visible_text("+1"),
            format!("The strip ends with a menu of its hidden actions: {more:?}"),
        );
        if let Some(more) = &more {
            d.click(more);
            d.expect(HIDDEN_ACTION);
            d.observe(
                "iter-24-R-SI1-number",
                d.visible_text("Ctrl+3"),
                "The menu gives the hidden action's number",
            );
            d.state("more-actions-menu", Sizes::Small);
            let armed_before = d.has_containing("% to hit");
            d.click(HIDDEN_ACTION);
            d.observe(
                "iter-24-R-SI1",
                !armed_before && d.has_containing("% to hit") && d.in_view(SPELL),
                "Choosing Cinder Dart arms it, and Lookup stays open",
            );
            // The keys: Enter opens the menu on its first choice, Escape closes it, and the
            // focus is back on the button. Lookup and the armed action stay.
            let reached = d.tab_to(more);
            d.press(Key::Enter);
            let first = d.focused_name();
            d.press(Key::Escape);
            let back = d.focused_name();
            d.observe(
                "iter-24-R-SI1-escape",
                reached
                    && first.as_deref() == Some(HIDDEN_ACTION)
                    && back.as_deref() == Some(more.as_str())
                    && !d.has(HIDDEN_ACTION)
                    && d.has_containing("% to hit")
                    && d.in_view(SPELL),
                format!("Enter focuses {first:?}; Escape gives the focus to {back:?} and keeps the action armed"),
            );
            // A choice made with the keys gives the focus back to the button too.
            d.press(Key::Enter);
            d.press(Key::Enter);
            let chosen = d.focused_name();
            d.observe(
                "iter-24-R-SI1-choice",
                chosen.as_deref() == Some(more.as_str())
                    && !d.has(HIDDEN_ACTION)
                    && d.in_view(SPELL),
                format!("After a choice by the keys, the focus is on {chosen:?}"),
            );
            // The mouse: a click opens the menu, and Escape or a choice gives the focus to
            // the button too. Lookup and the armed action stay.
            d.click(more);
            d.expect(HIDDEN_ACTION);
            // Only the keys move the focus to the first choice.
            let opened = d.focused_name();
            d.press(Key::Escape);
            let back = d.focused_name();
            d.observe(
                "iter-24-R-SI1-mouse-escape",
                opened.as_deref() != Some(HIDDEN_ACTION)
                    && back.as_deref() == Some(more.as_str())
                    && !d.has(HIDDEN_ACTION)
                    && d.has_containing("% to hit")
                    && d.in_view(SPELL),
                format!("A click opens the menu with the focus on {opened:?}; Escape gives it to {back:?}"),
            );
            d.click(more);
            d.click(HIDDEN_ACTION);
            let chosen = d.focused_name();
            d.observe(
                "iter-24-R-SI1-mouse-choice",
                chosen.as_deref() == Some(more.as_str())
                    && !d.has(HIDDEN_ACTION)
                    && d.has_containing("% to hit")
                    && d.in_view(SPELL),
                format!("After a choice by the mouse, the focus is on {chosen:?}"),
            );
        }
        // A click on a result leaves no field in focus. Typing must not run commands.
        let turn = current_participant(d);
        d.harness_focus_nothing();
        d.press(Key::N);
        d.observe(
            "T2.1",
            current_participant(d) == turn,
            "With Lookup open, the letter N does not advance the turn",
        );
        // Pressed, with Lookup open, and it acts. Then undone, so the script goes on.
        let before = current_participant(d);
        d.click(NEXT_TURN);
        let after = current_participant(d);
        d.observe(
            "T2.1",
            before != after && d.in_view(SPELL),
            format!("Next turn acts with Lookup open: {before:?} then {after:?}"),
        );
        // Must stay true (SI1): Next turn ends the action armed from the menu.
        d.observe(
            "iter-24-R-SI1-ends",
            !d.has_containing("% to hit"),
            "Next turn ends the action armed from the strip's menu",
        );
        // Next turn put the focus in Amount. A search word typed now is not a command.
        d.press(Key::N);
        let held = current_participant(d);
        d.observe(
            "T2.1",
            held == after,
            format!("With Lookup open, N in Amount does not advance: {after:?} then {held:?}"),
        );
        d.click_containing("Undo: next turn");
        d.click_row(PARTICIPANT);
        d.begin_task();

        look_up_again(d, &PACKAGE_ENTRY.to_lowercase());
        let package_found = d.in_view(PACKAGE_ENTRY);
        look_up_again(d, &CAMPAIGN_ENTRY.to_lowercase());
        let campaign_found = d.in_view(CAMPAIGN_ENTRY);
        d.state("found-package-and-campaign-entries", Sizes::Default);
        d.observe(
            "T2.3",
            builtin_found && package_found && campaign_found,
            format!("Lookup with no package switch: built-in {builtin_found}, package entry {package_found}, campaign entry {campaign_found}"),
        );

        d.begin_task();
        d.press(Key::Escape);
        d.expect(RUN_HEADING);
        d.expect_same("Closing Lookup returns to the preserved run and selected participant.");
        d.state("back-in-the-run", Sizes::All);
        d.goal_within("T2.2", "return to the run", 1);
        d.observe(
            "T2.2",
            participant_is_selected(d, PARTICIPANT),
            "The selected participant is unchanged on return",
        );

        d.review("T6.4", "the run's creature reader");
        let linked = d
            .nodes()
            .iter()
            .any(|node| node.is_link() && node.name.contains(SPELL_IN_STAT_BLOCK));
        d.begin_task();
        let unread = !d.visible_text(SPELL_BODY);
        d.click(SPELL_IN_STAT_BLOCK);
        let opened = d.visible_text(SPELL_BODY) && d.in_view(NEXT_TURN);
        d.state("spell-from-stat-block", Sizes::Default);
        d.observe(
            "T2.4",
            linked && unread && opened,
            "A spell named in a run stat block opens its text beside the roster",
        );
        let cost = d.goal("read the creature's details in the run");
        d.observe(
            "T2.4",
            cost <= 1,
            format!("It took {cost}, against a target of 1"),
        );
        let with_back = (d.has("Back"), d.place_of(SEARCH_LOOKUP));
        d.press_alt(Key::ArrowLeft);
        d.expect("Captain Varra");
        // Round 23 FJ5: the search field keeps its place whether Back shows or not.
        let without_back = (d.has("Back"), d.place_of(SEARCH_LOOKUP));
        d.observe(
            "iter-23-D-FJ5",
            with_back.0
                && !without_back.0
                && with_back
                    .1
                    .zip(without_back.1)
                    .is_some_and(|(with, without)| (with.top() - without.top()).abs() < 0.5),
            format!("Search field with Back {with_back:?}, without {without_back:?}"),
        );
        d.state("lookup-back-by-key", Sizes::All);
        d.press(Key::Escape);

        // A draft elsewhere does not block Lookup, and Lookup does not touch the draft.
        go_library(d);
        open_local_package(d, MODULE);
        d.click_containing(NOTE);
        d.click(EDIT_MARKDOWN);
        d.type_into_editor(DRAFT_WORDS);
        d.expect(UNSAVED_NOTE);
        d.begin_task();
        look_up(d, &OTHER_NOTE.to_lowercase());
        d.state("lookup-with-draft-open", Sizes::All);
        d.goal("try a lookup with a draft open");
        d.press(Key::Escape);
        let kept = d
            .editor_text()
            .is_some_and(|text| text.contains(DRAFT_WORDS));
        d.observe(
            "T2.5",
            kept && d.has_containing(UNSAVED_NOTE) && d.in_view(NEXT_TURN),
            "Lookup works while a Library draft is unsaved. The draft's own words are still in the editor, and the run strip stays",
        );
    }
}

/// No fixture has a long name. A long name is never cut: the `clipped` check reads every
/// painted word in these states.
pub mod run_long_names {
    use super::*;
    const RENAMED: &str = "Iselle Marrowind";
    const BACK_TO_RUN: &str = "Back to run";
    const LONG_NAME: &str = "Seraphina Thornwood-Ashgrave of the Ninth Bell";
    /// Past three lines of the name column at 960x640.
    const LONGEST_NAME: &str = "Seraphina Thornwood-Ashgrave of the Ninth Bell, Warden of the Cinder Road and the Ashen Gate";
    const FIRST_NAME: &str = "Seraphina";
    const ENEMY: &str = "Cinder Wolf 1";

    /// A row's AC right edge and HP slash, from what it paints. The AC number ends at the AC
    /// column; the HP readout's `/max` starts at its slash.
    fn numeric_columns(d: &Driver<'_>, name: &str) -> Option<(f32, f32)> {
        let row = d.row_node(name)?.shown;
        let mut texts = Vec::new();
        d.each_painted_text(|text, _| {
            let rect = egui::Rect::from_min_size(text.pos, text.galley.size());
            if row.contains(rect.center()) {
                texts.push((text.galley.job.text.clone(), rect));
            }
        });
        let label = texts.iter().find(|(text, _)| text == "AC")?.1;
        let ac = texts
            .iter()
            .filter(|(_, rect)| {
                rect.left() >= label.right() && (rect.center().y - label.center().y).abs() < 4.0
            })
            .min_by(|a, b| a.1.left().total_cmp(&b.1.left()))?
            .1
            .right();
        let hp = texts
            .iter()
            .find(|(text, _)| text.starts_with('/'))?
            .1
            .left();
        Some((ac, hp))
    }

    /// FT3: the numeric columns never move for a long name, at every captured size.
    fn check_columns_stay(d: &mut Driver<'_>, when: &str) {
        check_columns_stay_for(d, LONG_NAME, when);
    }

    fn check_columns_stay_for(d: &mut Driver<'_>, name: &str, when: &str) {
        for size in [SMALL, DEFAULT, LARGE] {
            let (long, short) = d.at_size(size, |d| {
                (numeric_columns(d, name), numeric_columns(d, ENEMY))
            });
            d.observe(
                "iter-17-B-FT3",
                long.zip(short).is_some_and(|(long, short)| {
                    (long.0 - short.0).abs() <= 1.0 && (long.1 - short.1).abs() <= 1.0
                }),
                format!(
                    "{when} at {size:?}: AC and HP x on the long-name row {long:?} match a short row {short:?}"
                ),
            );
        }
    }

    pub fn run(d: &mut Driver<'_>) {
        resume_run_from_cold_start(d);
        // A Bloodied enemy has its mark and its word side by side: they must not read as one.
        d.click_row(ENEMY);
        d.type_text("5");
        d.press(Key::Enter);
        d.app_mut().encounters.simulate_rename(RENAMED, LONG_NAME);
        d.settle();
        d.click_row(LONG_NAME);
        // iter-26-R FK1: the dock's name slot has one width for the fight, so Conditions keeps
        // one x whichever row is selected: a long name, a short one, a party name.
        for size in [SMALL, DEFAULT, LARGE] {
            let mut lefts = Vec::new();
            for row in [LONG_NAME, ENEMY, "Kestrel Dunmoor"] {
                d.click_row(row);
                lefts.push(d.at_size(size, |d| {
                    d.place_of("Conditions for this participant")
                        .map(|rect| rect.left().round())
                }));
            }
            d.observe(
                "iter-26-R-FK1",
                lefts.iter().all(Option::is_some) && lefts.windows(2).all(|w| w[0] == w[1]),
                format!(
                    "At {size:?}, Conditions x for a long, a short and a party name: {lefts:?}"
                ),
            );
        }
        d.click_row(LONG_NAME);
        d.state("long-name", Sizes::All);
        check_columns_stay(d, "Selected");
        let row = d.row_words(LONG_NAME);
        d.observe(
            "Long names",
            row.contains(LONG_NAME),
            format!("A long name is whole on its row at 960x640: {row}"),
        );
        // The strip names whose turn it is, from any page.
        go_library(d);
        d.state("long-name-on-the-strip", Sizes::Default);
        d.click(BACK_TO_RUN);
        d.begin_task();
        // The header names the current participant first. Two turns on, that is this name.
        d.click(NEXT_TURN);
        d.state("long-name-has-the-turn", Sizes::All);
        check_columns_stay(d, "With the turn");
        let turn = current_participant(d);
        let header = d.header_rect();
        let words = d.painted_in(header);
        let size = d.painted_size(header, FIRST_NAME);
        let row_size = d
            .row_node(LONG_NAME)
            .and_then(|row| d.painted_size(row.shown, LONG_NAME));
        d.observe(
            "Long names",
            turn.as_deref() == Some(LONG_NAME)
                && words.contains(&format!("{FIRST_NAME}…"))
                && !words.contains("’s turn")
                && size > row_size
                && row_size.is_some(),
            format!(
                "At 960x640 the header says \"{words}\" at {size:?} px, larger than the row's name at {row_size:?} px"
            ),
        );
        go_library(d);
        d.state("long-name-has-the-turn-on-the-strip", Sizes::Default);
        // FT3, edge content: a name past three lines ends its third line in `…` and keeps the
        // columns. Hover and the row's accessible name give it whole.
        d.click(BACK_TO_RUN);
        d.app_mut()
            .encounters
            .simulate_rename(LONG_NAME, LONGEST_NAME);
        d.settle();
        // The row paints the name's first whole words in three lines, then `…`.
        let row = d.row_node(LONGEST_NAME).map(|row| row.shown);
        let mut shown = None;
        d.each_painted_text(|text, _| {
            let words = &text.galley.job.text;
            if words.starts_with(FIRST_NAME)
                && row.is_some_and(|row| row.contains(text.visual_bounding_rect().center()))
            {
                shown = Some((words.clone(), text.galley.rows.len(), text.galley.elided));
            }
        });
        d.observe(
            "iter-17-B-FT3-cut",
            shown.as_ref().is_some_and(|(words, lines, elided)| {
                *lines == 3
                    && !elided
                    && words.ends_with('…')
                    && LONGEST_NAME.starts_with(words.trim_end_matches('…'))
                    && words.len() < LONGEST_NAME.len()
            }),
            format!("At 960x640 the row paints (words, lines, cut by egui) = {shown:?}"),
        );
        d.expect_same(
            "Only the long name's third line differs: it ends in `…` after the longest name's \
             first whole words",
        );
        d.state("longest-name", Sizes::All);
        check_columns_stay_for(d, LONGEST_NAME, "Longest name");
        let cut = shown.map(|(words, _, _)| words).unwrap_or_default();
        let whole = !cut.is_empty() && d.painted_tooltip_shows(&cut, LONGEST_NAME);
        d.observe(
            "iter-17-B-FT3-hover",
            whole,
            "Hover over the cut name paints the whole name",
        );
        // iter-17-B: a creature file changed during the run. The stat block says so, and its
        // Read the new version starts its line at the notice's text edge.
        let file = d
            .app_mut()
            .campaigns
            .modules()
            .iter()
            .map(|module| module.path.join("Creatures/Cinder Wolf.md"))
            .find(|path| path.is_file())
            .expect("a package holds the Cinder Wolf");
        let text = std::fs::read_to_string(&file).expect("read the creature");
        std::fs::write(&file, text + "\nThe pack howls at the ash moon.\n")
            .expect("change the creature on disk");
        d.focus_window(false);
        d.focus_window(true);
        d.click_row(ENEMY);
        if d.has("Stat block") && !d.has_containing("Cinder Wolf.md has changed.") {
            d.click("Stat block");
        }
        d.wait_for_text("Read the new version");
        // iter-25-R AJ1: the notice's action is a bordered secondary button, and its border
        // starts at the notice's words.
        for size in [SMALL, DEFAULT, LARGE] {
            let (button, frame, words) = d.at_size(size, |d| {
                let button = d.place_of("Read the new version");
                (
                    button,
                    button.and_then(|button| d.control_frame_at(button)),
                    d.painted_rect_in("Cinder Wolf.md has changed.", egui::Rect::EVERYTHING),
                )
            });
            d.observe(
                "iter-25-R-AJ1",
                matches!((button, &frame, words), (Some(button), Some(frame), Some(words))
                    if frame.stroke.width > 0.0
                        && (button.left() - words.left()).abs() <= 1.0
                        && button.top() >= words.bottom() - 0.5),
                format!("At {size:?}, Read the new version {button:?} is bordered ({:?}) under the words {words:?}", frame.map(|f| f.stroke)),
            );
        }
        // iter-20-R-ST1: the notice says what the card shows, subject first.
        d.observe(
            "iter-20-R-ST1",
            d.visible_text(
                "This stat block uses the copy from the start of the fight. Cinder Wolf.md has changed.",
            ) && !d.visible_text("As at the start of this fight"),
            "The changed-file notice says the stat block is the copy from the start of the fight",
        );
        // FJ8: the changed-on-disk state has its icon and colour; the words keep the text colour.
        let sentence = "This stat block uses the copy from the start of the fight.";
        let mut icon = None;
        let mut words = None;
        d.each_painted_text(|text, _| {
            let rect = text.visual_bounding_rect();
            if text.galley.job.text == crate::design::icons::WARN {
                let color = text.galley.job.sections.first().map(|s| s.format.color);
                icon = Some((rect, color));
            } else if text.galley.job.text.starts_with(sentence) {
                let color = text.galley.job.sections.first().map(|s| s.format.color);
                words = Some((rect, color));
            }
        });
        d.observe(
            "iter-23-R-FJ8",
            matches!((icon, words), (Some((icon, color)), Some((words, text)))
                if icon.right() <= words.left()
                    && words.left() - icon.right() <= crate::design::XS + 2.0
                    && (icon.top() - words.top()).abs() < 4.0
                    && color == Some(crate::design::colors().document.external)
                    && text == Some(crate::design::colors().text)),
            format!("The notice starts with the changed-on-disk icon: {icon:?} before {words:?}"),
        );
        d.state("creature-changed-since", Sizes::All);
        // Must stay true: the action still opens the new version.
        d.click("Read the new version");
        // The new line ends the entry. Stat block detail rows have the link pitch (iteration 20,
        // FD4), so at 960x640 it can start just below Lookup's fold: scroll Lookup, as a DM would.
        let (width, _) = d.window_size();
        for _ in 0..6 {
            if d.visible_text("The pack howls at the ash moon.") {
                break;
            }
            d.scroll_at(pos2(width - 160.0, 400.0), 1);
        }
        d.observe(
            "iter-17-B-read-new-opens",
            d.has("Close Lookup") && d.visible_text("The pack howls at the ash moon."),
            "Read the new version opens the changed creature in Lookup",
        );
        d.not_measured(
            "Long names",
            "outside a run: only the run's rows, header, dock, panel and strip have long-name data",
        );
    }
}

/// A player who did not arrive: combat begins without them, and they can come back.
pub mod run_absent_player {
    use super::*;
    const ENCOUNTER: &str = "Shrine of Cinders";
    const PLAYER: &str = "Brannoch Vale";
    const OTHERS: [(&str, &str); 4] = [
        ("Iselle Marrowind", "19"),
        ("Kestrel Dunmoor", "21"),
        ("Oren Thistlewick", "11"),
        ("Sable Harrowgate", "13"),
    ];
    const ROLL_NPCS: &str = "Roll NPC initiative";
    const BEGIN: &str = "Begin combat";
    const NO_INITIATIVE: &str = "No initiative";
    const MIDDLE: &str = "Oren Thistlewick";
    const LAST: &str = "Sable Harrowgate";
    const TURN_ORDER_FIRST: &str = "Kestrel Dunmoor";

    pub fn run(d: &mut Driver<'_>) {
        run_saved_encounter(d, ENCOUNTER);
        d.observe_both(
            "F2/P8",
            "The blocked reason ends at Begin combat's right edge",
            |d| {
                d.painted_right("need initiative.")
                    .zip(d.place_of(BEGIN))
                    .is_some_and(|(right, button)| (right - button.right()).abs() < 2.0)
            },
        );
        d.click(&format!("Absent: {PLAYER}"));
        d.click(ROLL_NPCS);
        // The keyboard path skips the absent: after the roll, and after each Enter.
        let field = |name: &str| format!("Initiative for {name}");
        let after_roll = d.focused_name();
        d.observe(
            "Absent player",
            after_roll.as_deref() == Some(field("Iselle Marrowind").as_str()),
            format!("After the roll, the first present player's field has focus: {after_roll:?}"),
        );
        d.type_text(OTHERS[0].1);
        d.press(Key::Enter);
        d.type_text(OTHERS[1].1);
        // One more absent, in the middle: Enter goes past that one too.
        d.click(&format!("Absent: {MIDDLE}"));
        d.click(&field(OTHERS[1].0));
        d.press(Key::Enter);
        let past_middle = d.focused_name();
        d.observe(
            "Absent player",
            past_middle.as_deref() == Some(field(LAST).as_str()),
            format!("Enter goes past an absent player in the middle: {past_middle:?}"),
        );
        d.click(&format!("Absent: {MIDDLE}"));
        for (name, total) in OTHERS {
            if d.field_text(&field(name)).as_deref() != Some(total) {
                d.type_into(&field(name), total);
            }
        }
        // iter-26-R FN1: the on Absent toggle keeps the off toggles' padding, so its icon and
        // word start at their x in the column.
        for size in [SMALL, DEFAULT, LARGE] {
            let lefts = d.at_size(size, |d| {
                (
                    words_left(d, &format!("Absent: {PLAYER}")),
                    words_left(d, &format!("Absent: {LAST}")),
                )
            });
            d.observe(
                "iter-26-R-FN1",
                matches!(lefts, (Some(on), Some(off)) if (on - off).abs() < 1.0),
                format!("At {size:?}, the on and off Absent words start at {lefts:?}"),
            );
        }
        d.state("ready-with-one-absent", Sizes::All);
        d.observe_both(
            "F2/P8",
            "The ready line ends at Begin combat's right edge",
            |d| {
                d.painted_right("Everyone is ready")
                    .zip(d.place_of(BEGIN))
                    .is_some_and(|(right, button)| (right - button.right()).abs() < 2.0)
            },
        );
        d.observe(
            "Absent player",
            !d.has("Turn order") && d.has("Technical details"),
            "Setup opens Prep without a second roster; the first turn is checked after Begin combat",
        );
        d.observe(
            "Absent player",
            !d.is_disabled(BEGIN) && d.visible_text("Everyone is ready"),
            "Combat can begin with a player absent, and with no initiative for them",
        );
        d.click(BEGIN);
        d.state("began-without-them", Sizes::Standard);
        let first = current_participant(d);
        d.observe(
            "Absent player",
            first.as_deref() == Some(TURN_ORDER_FIRST) && d.row_words(PLAYER).contains("Absent"),
            format!("The first turn is {first:?}, and their row says Absent"),
        );
        d.begin_task();

        // Back in the second turn: last in the order, and the DM says when it is.
        d.click(NEXT_TURN);
        let turn = current_participant(d);
        d.click_row(PLAYER);
        d.click("Absent");
        d.state("back-with-no-initiative", Sizes::All);
        let last = roster_places(d).last().map(|row| row.0.clone());
        d.observe(
            "Absent player",
            d.row_words(PLAYER).contains(NO_INITIATIVE)
                && last.as_deref() == Some(PLAYER)
                && current_participant(d) == turn,
            format!(
                "Back with none: the row says so, they are last ({last:?}), and the turn stays"
            ),
        );
        d.click(&format!("No initiative for {PLAYER}"));
        d.expect_same("Opening the inline initiative field changes only this small row control.");
        d.state("initiative-field-in-row", Sizes::All);
        let top = d.row_node(TURN_ORDER_FIRST).map(|row| row.rect.top());
        d.type_text("20");
        d.press(Key::Enter);
        d.observe(
            "F3",
            top == d.row_node(TURN_ORDER_FIRST).map(|row| row.rect.top()),
            "A long initiative Undo does not move the roster down",
        );
        let shown = d.at_size(SMALL, |d| d.visible_text("Undo: initiative"));
        d.observe(
            "iter-23-R-SH1",
            shown && d.has("Undo: initiative for Brannoch Vale"),
            "At 960 Undo says it takes back an initiative, and its accessible name keeps the row",
        );
        d.state("initiative-set-in-combat", Sizes::Default);
        let joined = "Brannoch Vale joined at\u{a0}20 ·\u{a0}acts next round.";
        d.expect(joined);
        let words = d.place_of(joined);
        d.observe(
            "iter-23-R-FH5",
            words.is_some_and(|words| d.has_tone_bar(words, crate::design::colors().result.success))
                && d.painted_color(joined) == Some(crate::design::colors().text),
            "The result below the roster is a success message: bar and icon in the tone, words in text colour, as Arrivals says it",
        );
        let order: Vec<String> = roster_places(d).into_iter().map(|row| row.0).collect();
        d.observe(
            "Absent player",
            order.first().map(String::as_str) == Some(TURN_ORDER_FIRST)
                && order.get(1).map(String::as_str) == Some(PLAYER)
                && current_participant(d) == turn
                && !d.row_words(PLAYER).contains(NO_INITIATIVE),
            format!("Their place follows the total, and the turn stays: {order:?}"),
        );
        d.begin_task();
    }
}

/// W4: cold start to the first turn of a new run.
/// The official condition toggles at or right of `left` and below `top`, as columns: each
/// column's left edge, the right edge of its widest toggle, and how many toggles it holds.
fn condition_columns(d: &Driver<'_>, left: f32, top: f32) -> Vec<(f32, f32, usize)> {
    let nodes = d.nodes();
    let mut toggles: Vec<egui::Rect> = crate::combat_sync::CONDITIONS
        .iter()
        .filter_map(|(_, name)| {
            nodes
                .iter()
                .find(|node| {
                    node.name == *name && node.rect.left() >= left - 1.0 && node.rect.top() >= top
                })
                .map(|node| node.rect)
        })
        .collect();
    toggles.sort_by(|a, b| a.left().total_cmp(&b.left()));
    let mut columns: Vec<(f32, f32, usize)> = Vec::new();
    for toggle in toggles {
        match columns.last_mut() {
            Some(column) if (toggle.left() - column.0).abs() < 0.5 => {
                column.1 = column.1.max(toggle.right());
                column.2 += 1;
            }
            _ => columns.push((toggle.left(), toggle.right(), 1)),
        }
    }
    columns
}

/// Columns of condition toggles that hold every condition, three of them, close together.
/// A column is as wide as its widest name in the selected (bordered) form, so the gap after
/// its widest drawn toggle is `S`, and at most `S` and that toggle's frame padding when it is
/// off (FI1).
fn condition_columns_close(columns: &[(f32, f32, usize)]) -> bool {
    /// The theme's button padding, on both sides of a selected toggle's words.
    const FRAME_PADDING: f32 = 2.0 * 10.0;
    columns.len() == 3
        && columns.iter().map(|c| c.2).sum::<usize>() == crate::combat_sync::CONDITIONS.len()
        && columns.windows(2).all(|pair| {
            pair[1].0 > pair[0].1 && pair[1].0 - pair[0].1 <= crate::design::S + FRAME_PADDING + 1.0
        })
}

/// Each painted glyph of a text: its character, then its left and right, in screen points.
fn text_glyphs(text: &egui::epaint::TextShape) -> Vec<(char, f32, f32)> {
    text.galley
        .rows
        .iter()
        .flat_map(|row| row.glyphs.iter().map(move |glyph| (row, glyph)))
        .map(|(row, glyph)| {
            let left = text.pos.x + row.pos.x + glyph.pos.x;
            (glyph.chr, left, left + glyph.advance_width)
        })
        .collect()
}

/// The blank before and after the first `·` from `from` in a run of glyphs, from where each
/// blank starts to the next solid glyph. Glyphs sit on whole pixels, so a blank is measured
/// from the glyph that starts it, not from the end of the glyph before.
fn separator_gaps(glyphs: &[(char, f32, f32)], from: usize) -> Option<(f32, f32)> {
    let dot = from + glyphs[from..].iter().position(|g| g.0 == '·')?;
    let solid_before = glyphs[..dot].iter().rposition(|g| !g.0.is_whitespace())?;
    let after = glyphs[dot + 1..].iter().find(|g| !g.0.is_whitespace())?;
    let blank_after = glyphs.get(dot + 1)?;
    Some((
        glyphs[dot].1 - glyphs[solid_before + 1].1,
        after.1 - blank_after.1,
    ))
}

pub mod prep_to_table {
    use super::*;
    const ENCOUNTER: &str = "Shrine of Cinders";
    const ROLL_NPCS: &str = "Roll NPC initiative";
    const BEGIN: &str = "Begin combat";
    const ROUND: &str = "Round 1";
    const PLAYERS: [(&str, &str); 5] = [
        ("Brannoch Vale", "15"),
        ("Iselle Marrowind", "19"),
        ("Kestrel Dunmoor", "21"),
        ("Oren Thistlewick", "11"),
        ("Sable Harrowgate", "13"),
    ];

    pub fn run(d: &mut Driver<'_>) {
        run_saved_encounter(d, ENCOUNTER);
        d.expect(&format!("{RUN_HEADING}{ENCOUNTER}"));
        d.state("run-setup", Sizes::All);
        d.goal_within("T4.1", "cold start to run setup", 3);
        d.review("T4.5", "the setup view, to compare with the live view");
        d.click("More actions for Brannoch Vale");
        d.state("setup-row-menu", Sizes::Default);
        d.observe(
            "iter-23-R-FH8",
            d.in_view("Change NPC and roll mode") && !d.has("NPC and roll mode"),
            "The setup row menu's second choice says what it does, with a verb",
        );
        d.click("Correct…");
        d.state("correct-in-setup", Sizes::All);
        // Labels share one column: each value, and its Mark unknown, starts at one x.
        for size in [SMALL, DEFAULT, LARGE] {
            let lefts = d.at_size(size, |d| {
                ["Current HP", "Maximum HP", "Initiative bonus"].map(|label| {
                    d.place_of(&format!("Mark {label} unknown"))
                        .map(|r| r.left())
                })
            });
            d.observe(
                "iter-23-R-FI5",
                lefts.iter().all(Option::is_some)
                    && lefts
                        .iter()
                        .flatten()
                        .all(|left| (left - lefts[0].unwrap()).abs() < 0.5),
                format!("At {size:?}, the Correct tab's values start at one x: {lefts:?}"),
            );
        }
        d.observe(
            "Setup corrections",
            d.has("Current HP") && !d.has("Resolve ties"),
            "The row menu opens that participant's correction fields; ties stay on the roster",
        );
        fight_tab(d, "Prep");

        // A player who did not arrive can be marked, in Setup, and marked back.
        d.begin_task();
        let (player, _) = PLAYERS[0];
        let inits = |d: &Driver<'_>| -> Vec<f32> {
            d.nodes()
                .iter()
                .filter(|node| node.name.starts_with("Initiative for "))
                .map(|node| node.rect.min.x)
                .collect()
        };
        let before = inits(d);
        let add_places = |d: &mut Driver<'_>| {
            [SMALL, DEFAULT, LARGE]
                .map(|size| d.at_size(size, |d| d.place_of("Add to fight").map(|r| r.left())))
        };
        let add_before = add_places(d);
        d.click(&format!("Absent: {player}"));
        let marked = d.has(&format!("Absent: {player}"));
        let after = inits(d);
        let add_after = add_places(d);
        d.observe(
            "iter-23-R-FI4",
            add_before.iter().all(Option::is_some)
                && add_before == add_after
                && d.has_containing(&format!("Undo: {player} absent")),
            format!(
                "A longer Undo label does not move Add to fight: {add_before:?} then {add_after:?}"
            ),
        );
        d.state("absent-at-start", Sizes::All);
        d.observe(
            "Absent player",
            d.visible_text("6 need initiative"),
            "Someone absent is not counted among those who still need initiative",
        );
        d.observe(
            "T4.3",
            !before.is_empty() && before == after && d.visible_text("Absent ·\u{a0}"),
            format!("Marking a player absent moves no field, and the row says so: {before:?} then {after:?}"),
        );
        d.click(&format!("Absent: {player}"));
        d.observe(
            "Absent player",
            marked && d.has(&format!("Absent: {player}")),
            "Setup has a word for a player who did not arrive, and undoing it takes one press",
        );
        d.begin_task();

        d.click(ROLL_NPCS);
        d.state("npcs-rolled", Sizes::All);
        d.goal("roll NPC initiative");
        let add_to_fight = d.place_of("Add to fight");

        let mut row_moved = false;
        let mut focus_is_here = false;
        // Every field's place, so that a move anywhere in the list is seen.
        let places = |d: &mut Driver<'_>| -> Vec<f32> {
            PLAYERS
                .iter()
                .map(|(player, _)| d.row_of(&format!("Initiative for {player}")).min.y)
                .collect()
        };
        for (index, (player, total)) in PLAYERS.iter().enumerate() {
            let field = format!("Initiative for {player}");
            let before = places(d);
            if std::mem::take(&mut focus_is_here) {
                d.type_text(total);
            } else {
                d.type_into(&field, total);
            }
            let after = places(d);
            row_moved |= before
                .iter()
                .zip(&after)
                .any(|(before, after)| (before - after).abs() > 1.0);
            if index == 0 {
                // SI2: Add to fight keeps its slot while the DM types initiative; Undo, which
                // now names a player, takes only the rest of the line.
                let shown = d.place_of("Add to fight");
                d.observe(
                    "iter-24-R-SI2",
                    d.in_view("Add to fight") && add_to_fight.is_some() && shown == add_to_fight,
                    format!("Add to fight stays in place: {add_to_fight:?} then {shown:?}"),
                );
                d.state("first-player-initiative", Sizes::Default);
                d.goal_within("T4.3", "enter one player's initiative", 2);
            }
            // Enter goes to the next player who has no initiative.
            if index + 1 == PLAYERS.len() / 2 {
                d.press(Key::Enter);
                let next = format!("Initiative for {}", PLAYERS[index + 1].0);
                let focused = d.focused_name();
                d.observe(
                    "T4.3",
                    focused.as_deref() == Some(next.as_str()),
                    format!("Enter moves to {next}. Focus is on {focused:?}"),
                );
                focus_is_here = true;
            }
        }
        d.observe(
            "T4.3",
            !row_moved,
            "Rows do not move during initiative entry",
        );
        d.state("all-initiative-entered", Sizes::All);
        for size in [SMALL, DEFAULT, (1920.0, 1080.0)] {
            let (aligned, compact) = d.at_size(size, |d| {
                let nodes = d.nodes();
                let fields: Vec<_> = nodes
                    .iter()
                    .filter(|node| node.name.starts_with("Initiative for "))
                    .collect();
                let rolls: Vec<_> = nodes
                    .iter()
                    .filter(|node| node.name.starts_with("Roll for "))
                    .collect();
                let aligned = fields.len() == 7
                    && rolls.len() == 2
                    && fields
                        .windows(2)
                        .all(|pair| (pair[0].rect.left() - pair[1].rect.left()).abs() <= 1.0)
                    && fields.iter().all(|field| {
                        let name = field.name.trim_start_matches("Initiative for ");
                        [
                            format!("Absent: {name}"),
                            format!("More actions for {name}"),
                        ]
                        .iter()
                        .all(|name| {
                            nodes
                                .iter()
                                .find(|node| node.name == *name)
                                .is_some_and(|node| {
                                    (node.rect.center().y - field.rect.center().y).abs() <= 1.0
                                        && node.rect.left() >= field.rect.right()
                                })
                        })
                    })
                    && rolls.iter().all(|roll| {
                        let name = roll.name.trim_start_matches("Roll for ");
                        fields
                            .iter()
                            .find(|field| field.name == format!("Initiative for {name}"))
                            .is_some_and(|field| {
                                (roll.rect.center().y - field.rect.center().y).abs() <= 1.0
                                    && roll.rect.right() <= field.rect.left()
                            })
                    });
                let compact = nodes.iter().any(|node| {
                    node.name.starts_with("Brannoch Vale · ")
                        && node.rect.height() <= crate::design::RUN_REGULAR.row
                });
                (aligned, compact)
            });
            d.observe(
                "Setup row controls",
                aligned && compact,
                format!(
                    "At {size:?}, all seven totals align, both Roll controls precede them, Absent and menus follow, and an untied row fits RUN_REGULAR: aligned {aligned}, compact {compact}"
                ),
            );
        }
        d.goal("enter the other players' initiative");
        let tie = d
            .nodes()
            .iter()
            .find(|node| node.name.starts_with("Move ") && node.name.contains(" before "))
            .map(|node| node.name.clone());
        let places = |d: &Driver<'_>| -> Vec<String> {
            d.nodes()
                .iter()
                .filter(|node| node.name.starts_with("Place for "))
                .map(|node| node.name.clone())
                .collect()
        };
        let before = places(d);
        if let Some(tie) = tie {
            d.click(&tie);
        }
        d.expect("Undo: tie order");
        let after = places(d);
        let first = after
            .iter()
            .find_map(|place| place.strip_prefix("Place for ")?.strip_suffix(": 1st"))
            .unwrap_or_default()
            .to_owned();
        d.observe(
            "P1",
            before != after && !first.is_empty() && d.visible_text(&format!("{first} goes first")),
            "Tie arrows update the fixed places, and the ready line names the first participant",
        );
        d.state("shared-roll-order", Sizes::Default);
        // Tied rows keep their arrows in two fixed places, so that a second click on one lands
        // on the same arrow.
        let arrows = |d: &Driver<'_>, word: &str| -> Vec<f32> {
            d.nodes()
                .iter()
                .filter(|n| n.name.starts_with("Move ") && n.name.contains(word))
                .map(|n| n.rect.min.x)
                .collect()
        };
        let (ups, downs) = (arrows(d, " before "), arrows(d, " after "));
        d.observe(
            "T4.3",
            !ups.is_empty()
                && ups.windows(2).all(|x| x[0] == x[1])
                && downs.windows(2).all(|x| x[0] == x[1]),
            format!("Tie arrows have fixed places: up {ups:?}, down {downs:?}"),
        );
        d.begin_task();

        d.click(BEGIN);
        d.expect(ROUND);
        d.expect(CURRENT_TURN);
        d.state("first-turn", Sizes::Standard);
        d.observe(
            "P1",
            current_participant(d).as_deref() == Some(first.as_str()),
            "Combat starts with the participant named in setup",
        );
        d.goal("begin combat");
    }
}

/// W4: the app restarted mid-session. Return to the interrupted turn.
pub mod run_resume {
    use super::*;
    const INTERRUPTED: &str = "Cinder Wolf 2";

    pub fn run(d: &mut Driver<'_>) {
        // Image already covered by encounter-build / encounter-list. Keep every size check.
        d.check_state("cold-start", Sizes::Default);
        resume_run_from_cold_start(d);
        d.expect(CURRENT_TURN);
        d.state("resumed", Sizes::All);
        d.goal_within("T4.2", "cold start to the interrupted turn", 2);
        // The fixture's saved turn, not whatever the screen says now.
        let current = current_participant(d).unwrap_or_default();
        d.observe(
            "T4.2",
            current == INTERRUPTED && participant_is_selected(d, INTERRUPTED),
            format!("The saved turn was {INTERRUPTED}. Now {current} is current, and selected"),
        );
    }
}

/// W1 at scale: rosters of 12 and 20, at the minimum window size.
pub mod run_large_roster {
    use super::*;
    pub const ROSTER_12: [&str; 12] = [
        "Brannoch Vale",
        "Iselle Marrowind",
        "Kestrel Dunmoor",
        "Oren Thistlewick",
        "Sable Harrowgate",
        "Cinder Wolf 1",
        "Cinder Wolf 2",
        "Cinder Wolf 3",
        "Cinder Wolf 4",
        "Cinder Wolf 5",
        "Ember Cultist 1",
        "Ember Cultist 2",
    ];
    const ROUND_LENGTH: usize = 20;

    pub fn run(d: &mut Driver<'_>) {
        resume_run(d, RUN_12);
        d.expect("Sync off for the party");
        d.observe_both(
            "FT3",
            "All linked rows have sync off; the header says it once",
            |d| {
                let linked = &ROSTER_12[..5];
                let rows: Vec<_> = linked.iter().filter_map(|name| d.row_node(name)).collect();
                rows.len() == linked.len()
                    && rows.iter().all(|row| !row.name.contains("Sync off"))
                    && linked
                        .iter()
                        .all(|name| !d.row_words(name).contains("Sync off"))
                    && d.in_view_containing("Sync off for the party")
            },
        );
        for size in [SMALL, DEFAULT, (1920.0, 1080.0)] {
            let (heading, add) = d.at_size(size, |d| {
                let words = d.painted_in(d.header_rect());
                (
                    words.contains("Wolf 1") || words.contains("… 1"),
                    words.contains("Add"),
                )
            });
            d.observe(
                "iter6-B-FT1",
                heading,
                format!("The current Cinder Wolf copy number stays in the heading at {size:?}"),
            );
            d.observe("iter6-B-AT2", add, format!("Add keeps a word at {size:?}"));
        }
        d.state("roster-of-12", Sizes::All);
        d.goal("open a run with 12 participants");

        let whole_roster = |d: &Driver<'_>| {
            let nodes = d.nodes();
            let roster = ROSTER_12
                .iter()
                .filter(|name| {
                    nodes
                        .iter()
                        .any(|n| n.name.starts_with(&format!("{name},")) && n.fully_in_view())
                })
                .count();
            roster == ROSTER_12.len() && d.in_view(APPLY_DAMAGE)
        };
        d.observe_both("T1.1", "Next turn is visible with 12 participants", |d| {
            d.in_view(NEXT_TURN)
        });
        d.observe_both(
            "T1.3",
            "All 12 rows are fully in view, with the damage entry",
            whole_roster,
        );
        // At the minimum, 12 rows hold their places, use the room, and share a pitch.
        let places = roster_places(d);
        let dock = d.place_of("Amount").map_or(0.0, |rect| rect.min.y);
        let bottom = places
            .iter()
            .fold(0.0_f32, |low, place| low.max(place.1.max.y));
        let (short, tall) = places
            .iter()
            .fold((f32::MAX, 0.0_f32), |(short, tall), place| {
                (short.min(place.1.height()), tall.max(place.1.height()))
            });
        d.observe(
            "Large roster",
            places.len() == 12 && dock - bottom < 12.0 && tall - short <= 3.0,
            format!(
                "12 rows at 960x640 leave {:.0} px above the dock, and are {short:.1} to {tall:.1} px tall",
                dock - bottom
            ),
        );
        d.click(NEXT_TURN);
        d.goal_within("T1.1", "advance the turn with 12 participants", 1);
        let after = roster_places(d);
        d.observe(
            "Stable rows",
            places.len() == 12 && places == after,
            format!(
                "With 12 rows at 960x640, no row moves on Next turn: {places:?} then {after:?}"
            ),
        );

        assert!(d.in_view("Add to fight"));
        d.click("Add to fight");
        d.expect("This fight · Arrivals");
        d.click("Stat block");
        d.begin_task();

        close_run(d);
        d.begin_task();
        resume_run(d, RUN_20);
        // iter-25-R FJ3: on every row, two-line or not, the picture centres on the first name
        // line, where the initiative number is. The name's first painted row stands for it.
        let off_line = d.at_size(DEFAULT, |d| {
            let rows = roster_places(d);
            let mut checked = 0;
            let mut two_line = 0;
            let mut off = Vec::new();
            for (name, rect) in &rows {
                let mut line = None;
                let mut lines = 0;
                d.each_painted_text(|text, _| {
                    let first = text.galley.rows.first();
                    if let Some(first) = first.map(|row| row.rect().translate(text.pos.to_vec2()))
                        && text.galley.job.text.starts_with(name.as_str())
                        && rect.contains(first.center())
                    {
                        line = Some(first);
                    }
                });
                let Some(line) = line else { continue };
                // A second painted line under the name: conditions, sync or state words.
                d.each_painted_text(|text, _| {
                    let at = text.pos + text.galley.rect.center().to_vec2();
                    if rect.contains(at) && at.y > line.bottom() && at.x >= line.left() - 1.0 {
                        lines += 1;
                    }
                });
                let left =
                    egui::Rect::from_min_max(rect.min, egui::pos2(line.left(), rect.bottom()));
                let Some(picture) = d
                    .pictures_in(left)
                    .into_iter()
                    .max_by(|a, b| a.width().total_cmp(&b.width()))
                else {
                    continue;
                };
                checked += 1;
                two_line += usize::from(lines > 0);
                if (picture.center().y - line.center().y).abs() > 2.0 {
                    off.push((name.clone(), picture.center().y, line.center().y));
                }
            }
            (rows.len(), checked, two_line, off)
        });
        d.observe(
            "iter-25-R-FJ3",
            off_line.1 >= 10 && off_line.2 >= 2 && off_line.3.is_empty(),
            format!(
                "Pictures centre on the first name line (rows, checked, two-line, off): {off_line:?}"
            ),
        );
        d.state("roster-of-20", Sizes::All);
        d.goal("open a run with 20 participants");

        // A full round, working at the minimum window, then at the default size. Each
        // click must leave the whole current row in view, with no resize to help it.
        let current_row_in_view = |d: &Driver<'_>| {
            d.nodes()
                .iter()
                .any(|node| node.name.contains(CURRENT_TURN) && node.fully_in_view())
        };
        let mut missed_small = 0;
        let mut seen = std::collections::BTreeSet::new();
        let mut stuck = 0;
        let mut resized = Vec::new();
        for turn in 0..ROUND_LENGTH {
            let before = current_participant(d);
            let rows_before = roster_places(d);
            d.click(NEXT_TURN);
            let after = current_participant(d);
            // The list scrolls, so rows may move together. None may change its height.
            let rows_after = roster_places(d);
            let same_height = rows_before.len() == 20
                && rows_before
                    .iter()
                    .zip(&rows_after)
                    .all(|(a, b)| a.0 == b.0 && (a.1.height() - b.1.height()).abs() < 0.5);
            if !same_height {
                resized.push(turn);
            }
            stuck += usize::from(before == after);
            seen.extend(after);
            if !current_row_in_view(d) {
                missed_small += 1;
            }
            if turn == ROUND_LENGTH / 2 {
                d.state("mid-round", Sizes::Default);
            }
        }
        d.goal("advance through a full round of 20");
        d.observe(
            "Stable rows",
            resized.is_empty(),
            format!("With 20 rows at 960x640, no row changes height on Next turn: {resized:?}"),
        );
        d.begin_task();
        d.set_base(DEFAULT);
        let mut missed_default = 0;
        for _ in 0..ROUND_LENGTH {
            d.click(NEXT_TURN);
            if !current_row_in_view(d) {
                missed_default += 1;
            }
        }
        d.set_base(SMALL);
        d.begin_task();
        d.observe(
            "T1.4",
            missed_small == 0 && missed_default == 0 && stuck == 0 && seen.len() == ROUND_LENGTH,
            format!("Over 20 turns the current row was not fully in view {missed_small} times at 960x640, {missed_default} at 1200x800. The turn stayed put {stuck} times, and {} of {ROUND_LENGTH} participants had a turn", seen.len()),
        );

        // T2.2 at scale: Lookup and back leave the list's scroll and selection alone.
        d.scroll_at(pos2(300.0, 300.0), 3);
        let row = d
            .nodes()
            .into_iter()
            .find(|node| node.name.contains(",") && node.name.contains("HP ") && node.in_view())
            .expect("a row in view");
        let (name, before) = (row.name.clone(), row.rect.min.y);
        look_up(d, "bless");
        d.press(Key::Escape);
        let after = d
            .nodes()
            .into_iter()
            .find(|node| node.name == name)
            .map(|row| row.rect.min.y);
        d.observe(
            "T2.2",
            after == Some(before),
            format!("The roster's scroll is unchanged after Lookup: {before} then {after:?}"),
        );
        d.begin_task();
    }
}

/// W3: a D&D Beyond conflict during a run. The conflict and the sheet update are
/// simulated, because test builds cannot read a live sheet. Every other step is real.
pub mod run_sync_conflict {
    use super::*;
    const AMOUNT_FIELD: &str = "Amount";
    const CLOSE_RUN: &str = "Close run";
    const CLOSE_WHY: &str = "Close after sync";
    const SYNCED: &str = "Brannoch Vale";
    const USE_REMOTE: &str = "Use D&D Beyond values";
    const CONFLICT_TEXT: &str = "changed";
    const PARTY: [&str; 5] = [
        "Brannoch Vale",
        "Iselle Marrowind",
        "Kestrel Dunmoor",
        "Oren Thistlewick",
        "Sable Harrowgate",
    ];
    const ROW_SYNC_WORDS: [&str; 2] = ["Reading sheet…", "Synced"];
    const DISMISS_NOTICE: &str = "Dismiss this message";
    const SYNC_PARTY: &str = "Sync party";
    const STOP_SYNC: &str = "Stop party sync";
    const UNDO_CLEARED: &str = "Undo for Brannoch Vale stops here";
    const CLOSE_PROMPT: &str = "has a sync conflict";
    const DISABLED_REASON: &str = "Wait for pending sync";

    pub fn run(d: &mut Driver<'_>) {
        resume_run_from_cold_start(d);
        // Sync needs a session. The rail must not say "Not connected" beside synced rows.
        d.app_mut()
            .settings
            .simulate(crate::settings::Account::Connected);
        d.settle();
        d.state("run-before-conflict", Sizes::Default);
        d.goal("resume the run");

        d.app_mut().encounters.simulate_sync_conflict(SYNCED);
        d.settle();
        d.state("conflict-with-nothing-expanded", Sizes::All);
        // The footer's main way out says why it is off, with no hover.
        d.observe(
            "G1",
            d.is_disabled(CLOSE_RUN) && d.visible_text(CLOSE_WHY),
            "While sync has work to finish, Close run is off, and its own label says why",
        );
        let on_row = d.row_node(SYNCED).is_some_and(|row| row.fully_in_view())
            && d.row_words(SYNCED).contains("Conflict");
        d.observe(
            "T3.2",
            on_row,
            "The conflict is on that participant's row, with nothing expanded",
        );
        let reason = d.tooltip_contains(CLOSE_RUN, DISABLED_REASON);
        d.observe(
            "T3.5",
            d.is_disabled(CLOSE_RUN) && reason,
            "A disabled Close run states its reason on hover",
        );

        d.request_close();
        d.observe(
            "T3.5",
            d.has_containing(&format!("{SYNCED} {CLOSE_PROMPT}")) && d.has("Review conflict"),
            "Closing the app with pending sync warns, and the warning names the character",
        );
        // FT4: the app-close dialog draws Review conflict through the run-close dialog's
        // function, so it starts at the dialog text as it does there (signed_in, iter-17-B).
        // iter-26-R FK2: it is a bordered button, its frame at the dialog text.
        check_frame_start(
            d,
            "iter-18-B-FT4",
            "Review conflict",
            CLOSE_PROMPT,
            &[SMALL, DEFAULT, LARGE],
            Some(&[
                "Review conflict",
                "Connect to sync",
                "Keep running",
                "Close and lose changes",
            ]),
        );
        d.state("close-with-pending-sync", Sizes::Default);
        d.click("Review conflict");
        d.observe(
            "R2",
            participant_is_selected(d, SYNCED) && !d.has("Keep running"),
            "Review conflict closes the app-close dialog and selects the character",
        );
        d.state("review-conflict-from-app-close", Sizes::Default);

        // From another page: on the strip, and Next turn still works.
        go_library(d);
        let strip = format!("Sync conflict: {SYNCED}");
        let shown = d.in_view_containing(&strip)
            && d.nodes()
                .iter()
                .any(|node| node.name == strip && node.is_link())
            && !d.has("Review sync conflict");
        let before = strip_turn(d);
        d.click(NEXT_TURN);
        let after = strip_turn(d);
        d.observe(
            "T3.2",
            shown && before.is_some() && before != after,
            format!("The strip shows the conflict: {shown}. Next turn still acts: {before:?} then {after:?}"),
        );
        d.click(BACK_TO_RUN);
        d.begin_task();

        d.click_row(SYNCED);
        d.row_of(USE_REMOTE);
        d.expect(CONFLICT_TEXT);
        d.state("conflict-panel", Sizes::All);
        d.observe(
            "Conflict panel words",
            d.visible_text("HP conflict") && d.visible_text("HP from D&D Beyond"),
            "The conflict panel names the conflict, and its buttons name the values",
        );
        d.click(USE_REMOTE);
        d.state("conflict-resolved", Sizes::All);
        d.goal_within("T3.2", "resolve a conflict", 2);
        let resolved = d
            .row_node(SYNCED)
            .is_some_and(|row| !row.name.contains("Conflict"));
        d.observe(
            "T3.2",
            resolved,
            "After resolving, the row no longer says Conflict",
        );

        d.app_mut().encounters.simulate_connection();
        d.settle();
        let before = d.interaction_count();
        d.click(SYNC_PARTY);
        let cost = d.interaction_count() - before;
        d.state("party-sync-on", Sizes::Default);
        let all_rows = PARTY.iter().all(|name| {
            ROW_SYNC_WORDS
                .iter()
                .any(|word| d.row_words(name).contains(word))
        });
        d.observe(
            "T3.3",
            cost <= 2 && all_rows && d.has(STOP_SYNC),
            format!("Simulated connection: sync turned on for the whole party in {cost}. Every party row paints one of {ROW_SYNC_WORDS:?}: {all_rows}"),
        );

        // Clear what an earlier update left, so this update has to make the message.
        if d.has(DISMISS_NOTICE) {
            d.click(DISMISS_NOTICE);
        }
        let clear_before = !d.has_containing(UNDO_CLEARED);
        let rows_before = roster_places(d);
        let hp_before = d.row_hp(SYNCED).unwrap_or(0);
        let amount_before = d.place_of(AMOUNT_FIELD);
        d.app_mut().encounters.simulate_read(SYNCED, 3);
        d.settle();
        // iter-25-R FJ1: the update took the character's Undo step away, and its notice says
        // so where the DM sees the Undo label change.
        let removed = [SMALL, DEFAULT, LARGE]
            .map(|size| d.at_size(size, |d| d.visible_text("1 Undo step removed")));
        d.observe(
            "iter-25-R-FJ1",
            clear_before && removed.iter().all(|shown| *shown),
            format!("The sheet notice names the removed Undo step at each size: {removed:?}"),
        );
        d.state("undo-boundary-for-one-character", Sizes::All);
        let changed = format!("{SYNCED} HP {hp_before} → {}", hp_before - 3);
        // A notice goes under the list, and no row moves from where it was.
        let rows_after = roster_places(d);
        d.observe(
            "Stable rows",
            rows_before.len() == 8
                && rows_before == rows_after
                && amount_before == d.place_of(AMOUNT_FIELD),
            format!("A notice moves no row and not the dock at 960x640: {rows_before:?} then {rows_after:?}"),
        );
        d.observe(
            "T3.4",
            clear_before
                && d.has("Log")
                && d.has_containing(UNDO_CLEARED)
                && d.has_containing(&changed),
            format!("Simulated sheet read: a message says, only after it, what changed from what to what: {changed}"),
        );
        // A new run action clears the notice. Its details remain in Log.
        d.click(NEXT_TURN);
        d.observe(
            "T3.4",
            !d.has("Dismiss this message") && !d.has_containing(UNDO_CLEARED),
            "The next turn clears the old notice",
        );
        fight_tab(d, "Log");
        d.observe(
            "T3.4",
            d.visible_text(&changed),
            "The Log keeps the full sheet change",
        );
        d.click(STOP_SYNC);
        d.begin_task();
        d.unreached(
            "wrong campaign, a duplicate character, and a live read after a reconnect (a simulated read goes through the real comparison)",
            "the error state on the row, because these need a live session to arise",
        );
    }

    /// The turn the strip names, on a page other than the run.
    fn strip_turn(d: &Driver<'_>) -> Option<String> {
        d.nodes()
            .into_iter()
            .find(|node| node.name.ends_with("’s turn"))
            .map(|node| node.name)
    }
}

/// Go through every top-level page and back. Each page should keep its state.
pub mod nav_state {
    use super::*;
    const QUERY: &str = "dragon";
    const CREATURE: &str = "Adult Black Dragon";
    const SEARCH_RESULTS: &str = "SEARCH RESULTS";
    const MEMBER: &str = "Iselle Marrowind";
    const SHEET_PAGE: &str = "Spells";
    const SHEET_PAGE_TEXT: &str = "SPELLCASTING";
    const ENCOUNTER: &str = "Bridge Ambush";
    const ROSTER_TEXT: &str = "1. Brannoch Vale";
    const NOT_IMPLEMENTED: &str = "not implemented";
    const CAMPAIGN_TAB_TEXT: &str = "Run folder";
    const REMOVED_PAGES: [&str; 2] = ["Creatures", "Simulations"];

    pub fn run(d: &mut Driver<'_>) {
        go_library(d);
        d.begin_task();
        search_library(d, QUERY);
        d.click_containing(CREATURE);
        d.observe_both(
            "FD-9",
            "The reader starts with its title, without a repeated Library breadcrumb",
            |d| !d.visible_text("Library / Creatures"),
        );
        d.state("library-set-up", Sizes::DefaultAndHiDpi);

        go_party(d);
        d.click_containing(MEMBER);
        d.click(SHEET_PAGE);
        d.state("party-set-up", Sizes::Default);

        go_encounters(d);
        open_saved_encounter(d, ENCOUNTER);
        d.expect(ROSTER_TEXT);
        d.state("encounters-set-up", Sizes::Default);

        go_campaign(d);
        d.click(TAB_RUN_SETTINGS);
        d.goal("set up state on three pages");

        go_settings(d);
        d.state("settings", Sizes::Default);

        go_library(d);
        let library = d.has_containing(SEARCH_RESULTS) && d.in_view(CREATURE);
        assert!(library);
        d.expect_same("Library keeps its search and reader when returning from another page.");
        d.assert_state_matches("library-returned", "library-set-up");

        go_campaign(d);
        let campaign = d.in_view_containing(CAMPAIGN_TAB_TEXT);
        d.state("campaign-returned", Sizes::All);
        // AF1: the save state and Save run settings share one row, with Save at its end, at
        // 960, 1200 and 1920.
        for size in [SMALL, DEFAULT, LARGE] {
            let (state, save) = d.at_size(size, |d| {
                (
                    d.painted_rect_in("No changes", egui::Rect::EVERYTHING),
                    d.place_of("Save run settings"),
                )
            });
            d.observe(
                "iter-25-B-AF1",
                state.zip(save).is_some_and(|(state, save)| {
                    (state.center().y - save.center().y).abs() <= 1.0 && state.right() < save.left()
                }),
                format!("At {size:?}: No changes {state:?}, Save run settings {save:?}"),
            );
        }

        go_party(d);
        let party = d.in_view_containing(MEMBER) && d.in_view_containing(SHEET_PAGE_TEXT);
        assert!(party);
        d.expect_same("Party keeps the selected member and sheet page on return.");
        d.assert_state_matches("party-returned", "party-set-up");

        go_encounters(d);
        let encounters = d.in_view_containing(ROSTER_TEXT);
        assert!(encounters);
        d.expect_same("Encounters keeps its open editor on return.");
        d.assert_state_matches("encounters-returned", "encounters-set-up");
        d.goal("return to every page");
        d.observe(
            "G2",
            library && campaign && party && encounters,
            format!("State kept: Library {library}, Campaign reopens its last tab {campaign}, Party {party}, Encounters {encounters}"),
        );

        let removed = REMOVED_PAGES.iter().all(|page| !d.has(page));
        d.observe(
            "G3",
            removed && !d.has_containing(NOT_IMPLEMENTED),
            format!(
                "Creatures and Simulations are gone: {removed}. No page says it is not implemented"
            ),
        );
        d.not_measured(
            "G5",
            "the app ships one theme, so there is nothing to compare yet",
        );
        // Any violation shows in this run's totals. This says that the checks did run.
        let (minimum, high) = d.checks_run();
        d.observe(
            "G4",
            minimum >= d.states_shown() && high >= 1,
            format!("This scenario's checks ran {minimum} times at 960x640 and {high} at 2x"),
        );
    }
}

/// W7 from a run: open a participant's sheet during combat, then return.
pub mod party_from_run {
    use super::*;
    const MEMBER: &str = "Kestrel Dunmoor";
    const SHEET_TITLE: &str = "Kestrel Dunmoor";
    const SHEET_PAGE: &str = "Correct";
    const SHEET_SUBPAGE: &str = "Spells";
    const CASTER: &str = "Iselle Marrowind";
    const SHEET_ONLY_TEXT: &str = "ABILITY SCORES";
    const OTHER_ROW: &str = "Oren Thistlewick";
    const FRESHNESS: &str = "Fetched";
    const SHEET_TAB: &str = "Sheet";
    const ZERO_DAMAGE: &str = "60";
    const ABSENT_ROW: &str = "Oren Thistlewick";
    const ABSENT: &str = "Absent";
    const MANY_CONDITIONS: [&str; 5] =
        ["Blinded", "Charmed", "Grappled", "Paralyzed", "Restrained"];
    const END_ENCOUNTER: &str = "End encounter…";
    const END_NOW: &str = "End encounter";
    const ENDED: &str = "Outcome: ";
    const UNDO_ENDING: &str = "Undo: end encounter";

    pub fn run(d: &mut Driver<'_>) {
        resume_run_from_cold_start(d);
        // Image already covered by run-resume / resumed. Keep every size check.
        d.check_state("in-the-run", Sizes::Default);
        d.goal("resume the run");

        d.click_row(MEMBER);
        d.expect(SHEET_TITLE);
        d.state("sheet-beside-run", Sizes::All);
        d.goal_within("T7.1", "open a participant's sheet from the run", 1);
        d.observe_both(
            "T7.1",
            "The roster, Next turn, and the sheet are all in view",
            |d| {
                d.in_view(NEXT_TURN)
                    && d.row_node(MEMBER).is_some_and(|row| row.fully_in_view())
                    && d.row_node(OTHER_ROW).is_some_and(|row| row.fully_in_view())
                    && participant_is_selected(d, MEMBER)
                    && d.in_view_containing(SHEET_ONLY_TEXT)
            },
        );
        d.observe_both("T7.2", "The sheet in the run says how old it is", |d| {
            d.in_view_containing(FRESHNESS)
        });

        // A caster's page, so that the capture shows the state it names.
        d.click_row(CASTER);
        d.click(SHEET_SUBPAGE);
        d.expect("Spell text shows the printed numbers. Splat does not work out upcasting or this character’s own bonuses.");
        // Round 25 AI2: the spell help keeps the reader measure at every size.
        for size in [SMALL, DEFAULT, LARGE] {
            let help = d.at_size(size, |d| {
                d.painted_rect_in(
                    "Spell text shows the printed numbers.",
                    egui::Rect::EVERYTHING,
                )
            });
            d.observe(
                "iter-25-C-AI2",
                help.is_some_and(|help| help.width() <= crate::design::READER_MEASURE + 0.5),
                format!("At {size:?}, the spell help is {help:?}"),
            );
        }
        d.state("sheet-spells-beside-run", Sizes::All);
        d.observe(
            "Spells beside run",
            d.visible_text("Fire Bolt"),
            "The spells page beside the run lists a caster's spells",
        );
        d.click_row("Brannoch Vale");
        d.observe(
            "Spells beside run",
            !d.has(SHEET_SUBPAGE) && d.visible_text(SHEET_ONLY_TEXT),
            "A character with no spells opens Overview and has no Spells tab",
        );
        d.state("sheet-with-no-spells", Sizes::Default);
        d.begin_task();
        d.click(SHEET_PAGE);
        d.wait_for_text("This row moves. The current turn stays.");
        d.state("correct-tab", Sizes::Default);
        // FI3, FJ2: every reason sits on its own line below its field, at the field's left, and
        // the note field takes the form's width up to Add note.
        let correct = |d: &Driver<'_>| {
            let nodes = d.nodes();
            let find = |wanted: &dyn Fn(&str) -> bool| {
                nodes
                    .iter()
                    .find(|node| wanted(&node.name))
                    .map(|node| node.rect)
            };
            [
                find(&|name| name.starts_with("Initiative for ")),
                find(&|name| name == "Type a number first."),
                find(&|name| name == "DM note"),
                find(&|name| name == "Type a note first."),
                find(&|name| name == "This row moves. The current turn stays."),
            ]
        };
        let below = |field: Option<egui::Rect>, reason: Option<egui::Rect>| {
            field.zip(reason).is_some_and(|(field, reason)| {
                reason.top() >= field.bottom() && (reason.left() - field.left()).abs() <= 1.0
            })
        };
        for size in [SMALL, DEFAULT] {
            let [total, number, note, words, _] = d.at_size(size, |d| correct(d));
            d.observe(
                "iter-24-R-FI3",
                below(total, number) && below(note, words),
                format!("At {size:?} each reason is below its field: {total:?} {number:?}, {note:?} {words:?}"),
            );
            // FJ2: the note field takes the form's line; Add note is below it, at its left.
            let add = d.at_size(size, |d| d.place_of("Add note"));
            d.observe(
                "iter-24-R-FJ2",
                note.zip(add).is_some_and(|(note, add)| {
                    add.top() >= note.bottom()
                        && (add.left() - note.left()).abs() <= 1.0
                        && note.right() >= add.right()
                }),
                format!(
                    "At {size:?} Add note is below a field that takes the line: {note:?} {add:?}"
                ),
            );
        }
        // At 1200 and 1920 a sentence fits: the field is at least 240 px wide. At 960 the
        // Correct label column leaves the line about 216 px.
        for size in [DEFAULT, LARGE] {
            let note = d.at_size(size, |d| d.place_of("DM note"));
            d.observe(
                "iter-24-R-FJ2-width",
                note.is_some_and(|note| note.width() >= 240.0),
                format!("At {size:?} the note field is at least 240 px wide: {note:?}"),
            );
        }
        d.click(SHEET_TAB);
        d.expect(SHEET_ONLY_TEXT);
        d.goal("read a page and return to the run");
        // Outside the measured task: Correct again, to type a total.
        d.click(SHEET_PAGE);
        d.wait_for_text("This row moves. The current turn stays.");
        // The reason's line stays when it goes: the words below keep their place.
        let [total, _, _, _, moves] = correct(d);
        let field = d
            .nodes()
            .into_iter()
            .find(|node| node.name.starts_with("Initiative for "))
            .map(|node| node.name);
        if let Some(field) = &field {
            d.type_into(field, "7");
        }
        let [_, gone, _, _, moved] = correct(d);
        d.observe(
            "iter-24-R-FI3-reserved",
            total.is_some() && gone.is_none() && moves.is_some() && moves == moved,
            format!("A typed total removes the reason, and nothing below moves: {moves:?} then {moved:?}"),
        );
        if let Some(field) = &field {
            d.clear_text_in(field);
        }
        d.click(SHEET_TAB);
        d.expect(SHEET_ONLY_TEXT);

        // The hardest rows: 0 HP, absent, and many conditions. Words, not only colour.
        d.click_row(MEMBER);
        assert!(participant_is_selected(d, MEMBER));
        d.expect(SHEET_ONLY_TEXT);
        d.expect_same("Returning to Kestrel restores the same sheet beside the unchanged run.");
        d.assert_state_matches("sheet-returned-to-kestrel", "sheet-beside-run");
        let correct_y = d.place_of("Correct").map(|rect| rect.min.y);
        d.type_text(ZERO_DAMAGE);
        d.press(Key::Enter);
        d.observe(
            "Stable death saves",
            correct_y == d.place_of("Correct").map(|rect| rect.min.y),
            "The panel tabs stay in place when HP reaches zero",
        );
        d.expect("0 HP");
        d.click_row(ABSENT_ROW);
        d.click(ABSENT);
        d.click(CONDITIONS);
        for condition in MANY_CONDITIONS {
            d.click(condition);
        }
        d.state("zero-absent-many-conditions", Sizes::All);
        d.review("T1.8", "0 HP and absent are distinct without colour");
        // A crowded row keeps its first conditions and a count. The count opens all of them.
        d.click(SHEET_TAB);
        let row = d.row_words(ABSENT_ROW).replace('\u{a0}', " ");
        let full = d.row_node(ABSENT_ROW).unwrap().name;
        let unread: Vec<&str> = MANY_CONDITIONS
            .into_iter()
            .filter(|condition| !full.contains(condition))
            .collect();
        let zero = d.row_words(MEMBER);
        let shown = MANY_CONDITIONS
            .iter()
            .filter(|condition| row.contains(**condition))
            .count();
        let hidden = MANY_CONDITIONS.len() - shown;
        d.observe(
            "T1.8",
            unread.is_empty() && row.contains("Absent") && row.contains("Blinded")
                && hidden > 0 && (row.contains(&format!("{hidden} more")) || row.contains(&format!("+{hidden}"))) && zero.contains("0 HP"),
            format!("At 960x640, {ABSENT_ROW} paints: {row}. {MEMBER} paints: {zero}. Missing: {unread:?}"),
        );
        d.click("Read all conditions");
        d.observe(
            "T1.8",
            MANY_CONDITIONS.iter().all(|condition| d.has(condition)),
            "The row's count opens the full list of conditions",
        );
        d.click_row(MEMBER);
        d.state("zero-hp-selected", Sizes::Default);
        d.click("Correct");
        d.expect("DEATH SAVES");
        d.click("Add a death save success");
        d.expect("1 success · 0 failures");
        d.observe(
            "Death saves",
            !d.has("DEATH SAVES (MANUAL)"),
            "Editable counts below the tabs replace the duplicate manual fields",
        );
        d.state("death-saves-under-correct", Sizes::Default);
        // iter-26-R FJ5: the death-save line has one place, directly below the tabs, so its
        // actions keep their y when the DM changes tab.
        let success_y = |d: &mut Driver<'_>| {
            [SMALL, DEFAULT, LARGE].map(|size| {
                d.at_size(size, |d| {
                    d.place_of("Add a death save success")
                        .map(|rect| rect.top().round())
                })
            })
        };
        let on_correct = success_y(d);
        d.click(SHEET_TAB);
        d.expect("1 success · 0 failures");
        let on_sheet = success_y(d);
        d.observe(
            "iter-26-R-FJ5",
            on_correct.iter().all(Option::is_some) && on_correct == on_sheet,
            format!("+ Success y on Correct {on_correct:?}, on Sheet {on_sheet:?}"),
        );
        d.observe_both(
            "C1",
            "The Sheet shows the run's death saves once and omits stored counts",
            |d| {
                let words = d.painted_in(egui::Rect::EVERYTHING);
                words.matches("1 success · 0 failures").count() == 1
                    && !words.contains("0 successes")
            },
        );
        d.observe_both(
            "iter7-B-FT1",
            "A player at zero HP keeps visible editable death saves",
            |d| d.in_view("Add a death save success") && d.in_view("Add a death save failure"),
        );
        d.state("live-death-saves-once", Sizes::All);

        // The third view of a run. It reads differently, and undo can reopen it.
        d.click(END_ENCOUNTER);
        d.click(END_NOW);
        d.expect(ENDED);
        d.state("ended", Sizes::Default);
        d.review("T4.5", "the ended view, to compare with live and setup");
        d.click_containing(UNDO_ENDING);
        d.expect(CURRENT_TURN);
    }
}

/// W8 across pages: copy a Library entry, then add it to an unsaved encounter draft.
pub mod library_to_encounter {
    use super::*;
    const DRAFT_NAME: &str = "Deep Ford";
    const QUERY: &str = "aboleth";
    const ENTRY: &str = "Aboleth";
    const COPY_NAME: &str = "Aboleth";
    const SEEDED_ROW: &str = "1. Brannoch Vale";
    const LAST_PARTY_ROW: &str = "5. Sable Harrowgate";

    pub fn run(d: &mut Driver<'_>) {
        d.click(NEW_ENCOUNTER);
        d.type_text(DRAFT_NAME);
        d.expect(SEEDED_ROW);
        d.state("draft-open", Sizes::Default);
        d.begin_task();

        go_library(d);
        search_library(d, QUERY);
        d.click_containing(ENTRY);
        d.state("library-entry", Sizes::Default);
        copy_open_entry_to_campaign(d);
        d.state("copied", Sizes::Default);

        // One press, from the finished dialog, with nothing to reload or search for.
        d.click(&format!("Add to {DRAFT_NAME}"));
        go_encounters(d);
        d.expect(DRAFT_NAME);
        let name_kept = d
            .field_text(&format!("Encounter name: {DRAFT_NAME}"))
            .as_deref()
            == Some(DRAFT_NAME);
        d.state("copy-in-open-draft-results", Sizes::Default);
        d.observe(
            "T8.1",
            name_kept
                && d.has_label(SEEDED_ROW)
                && d.has_label(LAST_PARTY_ROW)
                && d.has_label(&format!("6. {COPY_NAME}")),
            "The party added before the copy is still in the draft, and the copy has joined it with no reload",
        );
        let cost = d.goal("copy an entry and use it in an encounter");
        d.observe(
            "T8.1",
            cost <= 9,
            format!("It took {cost}, with nothing to reload or search for again"),
        );
        d.replace_text_in(FIND_CREATURE, COPY_NAME);
        d.begin_task();

        d.click(SAVE_ENCOUNTER);
        let detail = d
            .place_of(&format!("6. {COPY_NAME}"))
            .map_or_else(String::new, |title| {
                d.painted_in(egui::Rect::from_min_max(
                    title.left_bottom(),
                    pos2(title.left() + 400.0, title.bottom() + 18.0),
                ))
                .replace('\u{a0}', " ")
            });
        d.observe(
            "C17",
            ["HP 150", "AC 17", "CR 10", "Campaign"]
                .iter()
                .all(|word| detail.contains(word)),
            format!("The copied Aboleth's roster detail identifies its campaign source: {detail}"),
        );
        d.state("copy-in-encounter", Sizes::All);
        d.goal("save the encounter with the copy");
    }
}

/// T5.5 from `docs/ux-intent.md`: edit the run folder with no draft, then use every
/// enabled button that was there. Save encounter now exists only with a draft.
pub mod run_folder_defect {
    use super::*;
    const RUN_FOLDER: &str = "Run folder";
    const DISCARD: &str = "Discard";
    const BACK_TO_LIST: &str = "Back to encounters";

    pub fn run(d: &mut Driver<'_>) {
        go_campaign(d);
        d.click(TAB_RUN_SETTINGS);
        d.replace_text_in(RUN_FOLDER, "Elsewhere");
        d.observe_both(
            "iter-18-A-lock",
            "Run settings state the campaign switch lock",
            |d| d.visible_text(crate::encounters::draft_switch_reason()),
        );
        d.observe_both(
            "iter-3-C-FT10",
            "The switch reason follows Discard on the save row",
            |d| {
                d.place_of("Discard")
                    .zip(d.place_of(crate::encounters::draft_switch_reason()))
                    .is_some_and(|(discard, reason)| {
                        reason.left() > discard.right()
                            && (reason.center().y - discard.center().y).abs() < crate::design::S
                    })
            },
        );
        d.observe_both(
            "iter-3-C-FT10-spacing",
            "Full save actions stay directly below their state and reason",
            |d| {
                d.place_of("Discard")
                    .zip(d.place_of("Move runs and save"))
                    .is_some_and(|(discard, save)| {
                        save.top() >= discard.bottom()
                            && save.top() - discard.bottom() <= crate::design::L
                    })
            },
        );
        // FT4: with no open run, the move is the one primary action on the whole screen.
        d.observe_both(
            "iter-16-B-FT4-primary",
            "Move runs and save is the only accent-filled control in view",
            |d| super::session_recording::accent_filled(d) == ["Move runs and save"],
        );
        // Must stay true: the other choice on the row stays quiet.
        d.observe_both(
            "iter-16-B-FT4-quiet",
            "Save without moving stays a quiet action",
            |d| {
                d.in_view("Save without moving")
                    && d.control_frame("Save without moving")
                        .is_some_and(|frame| frame.stroke.width == 0.0)
            },
        );
        d.state("run-folder-edited", Sizes::All);
        d.goal("edit the run folder with no draft");
        // A panic ends this app, so the video stops first and nothing follows.
        d.stop_filming();
        let click = std::panic::AssertUnwindSafe(|| {
            // Every enabled button on the page with no draft, each from a clean start.
            go_encounters(d);
            let names = d.enabled_page_buttons();
            for name in &names {
                if (name == NEW_ENCOUNTER || name == SAVE_ENCOUNTER || d.has(name))
                    && !d.is_disabled(name)
                {
                    d.click_first(name);
                }
                if d.has(CLOSE_RUN) && !d.is_disabled(CLOSE_RUN) {
                    close_run(d);
                }
                go_encounters(d);
                let discard = if d.has("Discard") { "Discard" } else { DISCARD };
                if d.has(discard) && !d.is_disabled(discard) {
                    d.click(discard);
                }
                if d.has(BACK_TO_LIST) && !d.is_disabled(BACK_TO_LIST) {
                    d.click(BACK_TO_LIST);
                }
            }
            go_campaign(d);
            d.click("Move runs and save");
            go_encounters(d);
            d.observe(
                "Run folder recovery",
                d.has(RESUME),
                "Moving the run folder keeps the paused run available",
            );
            assert!(!names.is_empty(), "the page has buttons to press");
        });
        let panicked = std::panic::catch_unwind(click).is_err();
        d.observe(
            "T5.5",
            !panicked,
            "Pressing every enabled button with no draft open does not panic. See the defect in docs/ux-intent.md W5",
        );
    }
}

/// Every token and component of the design system, in each state.
pub mod gallery {
    use super::*;
    const COMPONENTS: &str = "Components";
    const MESSAGES: &str = "Messages";
    const PRIMARY: &str = "Primary";
    const SECONDARY: &str = "Secondary";

    pub fn run(d: &mut Driver<'_>) {
        d.app_mut().gallery = true;
        d.settle();
        d.state("tokens", Sizes::StandardAndHiDpi);
        d.click(COMPONENTS);
        d.state("components", Sizes::AllAndHiDpi);
        d.check_tab_order("gallery, components");
        d.state_hovering("hover-primary", PRIMARY);
        d.state_hovering("hover-secondary", SECONDARY);
        d.press(Key::Tab);
        d.state("focus-ring", Sizes::Default);
        d.click(MESSAGES);
        d.expect("Spell · Level 3 · D&D 2024");
        d.state("messages", Sizes::AllAndHiDpi);
        let edges = (
            d.place_of("Use the saved HP").map(|rect| rect.left()),
            d.painted_left("Party HP is full after the long rest."),
        );
        d.observe(
            "U6",
            edges
                .0
                .zip(edges.1)
                .is_some_and(|(action, message)| (action - message).abs() < 1.0),
            format!("The bordered action starts at its message text: {edges:?}"),
        );
        d.observe(
            "U6",
            !d.visible_passage("Second use of widget ID"),
            "The message action registers its click target once",
        );
        d.goal("see every part of the design system");
        d.click("Markdown");
        d.state("markdown", Sizes::AllAndHiDpi);
        d.click(COMPONENTS);
        d.scroll_until(pos2(600.0, 300.0), "Group actions", 30);
        d.state_prepared("submenu-spacing", Sizes::All, |d| {
            d.press(Key::Escape);
            d.click("Group actions");
            d.expect("Move to…");
            let edges = (
                d.painted_left("Move to…"),
                d.painted_left("Make available to any encounter"),
            );
            d.observe(
                "iter-4-A-FD-13",
                edges
                    .0
                    .zip(edges.1)
                    .is_some_and(|(a, b)| (a - b).abs() < 1.0),
                format!("The submenu and quiet menu action share a text edge: {edges:?}"),
            );
        });
        // SD2/AD1, the other caller: with room on the right, the gallery's submenu still opens
        // there, clear of any splitter.
        d.press(Key::Escape);
        d.click("Group actions");
        d.click("Move to…");
        d.expect("Bridge Ambush");
        for size in [SMALL, DEFAULT, (1920.0, 1080.0)] {
            let placed = d.at_size(size, |d| {
                let parent = d.place_of("Make available to any encounter");
                super::submenu_clear_of_chrome(d, &["Bridge Ambush"], false).and_then(|frame| {
                    // Right beside its menu, where egui puts it: only the menus' frames
                    // and egui's small gap lie between them.
                    parent
                        .is_some_and(|parent| {
                            (0.0..=crate::design::XL).contains(&(frame.left() - parent.right()))
                        })
                        .then_some(frame)
                        .ok_or(format!("submenu {frame:?} is not beside {parent:?}"))
                })
            });
            d.observe(
                "iter-22-B-SD2-gallery",
                placed.is_ok(),
                format!("At {size:?}, the gallery's Move to… submenu: {placed:?}"),
            );
        }
        d.press(Key::Escape);
        d.scroll_until(pos2(600.0, 300.0), "More actions for Bridge Ambush", 30);
        d.click("More actions for Bridge Ambush");
        d.state("row-menu", Sizes::Default);
        d.press(Key::Escape);
        d.scroll_until(pos2(600.0, 300.0), "Major event", 30);
        d.state_prepared("save-rows-and-add-strip", Sizes::All, |d| {
            for _ in 0..3 {
                if d.nodes()
                    .iter()
                    .any(|node| node.name == "Major event" && node.fully_in_view())
                {
                    break;
                }
                d.scroll_at(pos2(600.0, 300.0), 1);
            }
            let nodes = d.nodes();
            let controls: Vec<_> = [
                "Fight",
                "Group",
                "Day",
                "Long rest",
                "Short rest",
                "Travel",
                "Major event",
            ]
            .iter()
            .map(|name| {
                nodes
                    .iter()
                    .find(|node| node.name == *name && node.fully_in_view())
                    .unwrap_or_else(|| panic!("Visible full-width Add action: {name}"))
            })
            .collect();
            d.observe(
                "iter-5-A-seven-actions",
                !d.has("More Add actions")
                    && controls
                        .iter()
                        .all(|node| (node.rect.top() - controls[0].rect.top()).abs() < 1.0),
                "All seven Add actions keep one line when their column holds them",
            );
        });
        let temp = crate::design::temp_hp_text(5);
        for _ in 0..30 {
            if d.visible_text(&temp) {
                break;
            }
            d.scroll_at(pos2(600.0, 300.0), 1);
        }
        d.observe(
            "Temporary HP",
            d.visible_text(&temp),
            "Temporary HP shows with its mark, as it does in the run and on Party",
        );
        d.scroll_until(pos2(600.0, 300.0), "Nothing here yet", 30);
        d.state("empty-state", Sizes::Standard);
    }
}

/// Drag the edges of the rail, a page's lists and a panel, then work at those widths.
/// The widths come back after a restart.
pub mod column_resize {
    use super::*;
    use crate::design::{PANEL_MIN_WIDTH, RAIL_DRAG_MAX_WIDTH, RAIL_WIDTH, S};

    const CHECK: &str = "Column resize";
    const MEMBER: &str = "Brannoch Vale";
    const ENCOUNTER: &str = "Shrine of Cinders";
    /// Far past either end of what any column can reach.
    const FAR_LEFT: f32 = 1.0;
    const FAR_RIGHT: f32 = 1190.0;

    fn rail_edge(d: &Driver<'_>) -> f32 {
        d.place_of(NAV_LIBRARY).map_or(0.0, |rect| rect.right() + S)
    }

    fn search(d: &Driver<'_>) -> egui::Rect {
        d.place_of(SEARCH_LIBRARY).unwrap_or(egui::Rect::ZERO)
    }

    fn sections_edge(d: &Driver<'_>) -> f32 {
        search(d).left() - S
    }

    fn entries_edge(d: &Driver<'_>) -> f32 {
        search(d).right() + S
    }

    fn near(a: f32, b: f32) -> bool {
        (a - b).abs() <= 1.0
    }

    fn right_of(d: &Driver<'_>, name: &str) -> f32 {
        d.place_of(name).map_or(0.0, |rect| rect.right())
    }

    /// What is beside a column reaches it, in a window with more room than it needs.
    fn content_follows_the_panel(d: &mut Driver<'_>) {
        let roster = right_of(d, NEXT_TURN);
        let edge = roster + crate::design::M;
        d.drag_column_edge(edge, edge + 300.0);
        d.observe(
            CHECK,
            near(right_of(d, NEXT_TURN), roster + 300.0),
            format!(
                "The roster follows the run's panel from {roster} to {}",
                right_of(d, NEXT_TURN)
            ),
        );
    }

    fn the_roster_follows_the_add_panel(d: &mut Driver<'_>) {
        d.set_base(LARGE);
        // Close the run before checking the editor at the full page height.
        close_run(d);
        go_encounters(d);
        open_saved_encounter(d, ENCOUNTER);
        let edge = |d: &Driver<'_>| d.place_of(FIND_CREATURE).map_or(0.0, |r| r.left()) - S;
        let start = edge(d);
        d.drag_column_edge(start, start + 100.0);
        let roster = d
            .nodes()
            .into_iter()
            .filter(|node| node.name.starts_with("More actions for") && node.in_view())
            .map(|node| node.rect.right())
            .fold(0.0, f32::max);
        d.observe(
            CHECK,
            edge(d) - roster <= crate::design::XL,
            format!("The roster reaches {roster}, beside a panel at {}", edge(d)),
        );
        d.state("roster-follows-add-panel", Sizes::All);
    }

    fn the_session_fills_its_column(d: &mut Driver<'_>) {
        d.click(NAV_SESSIONS);
        new_session(d, "Room to spare");
        let page = right_of(d, "New session");
        d.observe(
            CHECK,
            (right_of(d, "Record this session") - page).abs() <= S,
            format!(
                "A session's actions reach the edge of its column: {} of {page}",
                right_of(d, "Record this session")
            ),
        );
        d.state("session-fills-its-column", Sizes::All);
    }

    /// Write and Read share a prose measure; spare width stays outside the Write frame.
    fn the_session_tabs_fill_their_column(d: &mut Driver<'_>) {
        d.set_base(LARGE);
        let page = right_of(d, "New session");
        d.type_into_editor(&"The party crossed the old bridge at dusk. ".repeat(30));
        let write = d.longest_visible_line();
        d.state_prepared("session-write-keeps-prose-measure", Sizes::All, |d| {
            let longest = d.longest_visible_line();
            let note = d
                .place_of("Note text")
                .expect("The Write editor is visible");
            let frame = d
                .input_frame("Note text")
                .expect("The note frame is visible")
                .0;
            d.observe(
                "iter-8-C-FT4",
                frame.width()
                    <= crate::design::READER_MEASURE
                        + 2.0 * crate::design::S
                        + 2.0 * crate::design::HAIRLINE_WIDTH
                        + 1.0
                    && (frame.left() + crate::design::S - note.left()).abs() <= 1.0,
                format!(
                    "The Write frame stays at reader measure plus padding, at the left: {frame:?}"
                ),
            );
            assert!(d.in_view("Read"));
            let toggle = d.place_of("Read").unwrap();
            d.observe(
                "iter-9-C-FT5",
                (toggle.right() - frame.right()).abs() <= 1.0 && toggle.bottom() <= frame.top(),
                format!("Read sits at the frame's top right edge: {toggle:?}, {frame:?}"),
            );
            d.observe(
                "iter-6-C-FT5",
                longest > 0
                    && longest <= 75
                    && note.width() <= crate::design::READER_MEASURE + 1.0
                    && (note.left() - frame.left() - crate::design::S).abs() <= 1.0,
                format!(
                    "Write is anchored left and keeps its measure at the captured size: {longest}"
                ),
            );
        });
        d.click("Read");
        let read = d.longest_visible_line();
        d.observe(
            CHECK,
            read > 0 && read <= 75 && write > 0 && write <= 75,
            format!("Longest line in a wide window: {write} in Write, {read} in Read"),
        );
        d.state_prepared("session-read-keeps-prose-measure", Sizes::All, |d| {
            let read = d.longest_visible_line();
            assert!(
                read > 0 && read <= 75,
                "Read keeps its measure at the captured size: {read}"
            );
        });
        d.click_add_event("Add Major event");
        d.wait_for_text("Major event added to Session");
        d.wait_for_text("Add to notes");
        let action = d
            .nodes()
            .into_iter()
            .find(|node| node.name.starts_with("Add to notes"))
            .map_or(0.0, |node| node.rect.right());
        d.observe(
            CHECK,
            action > 0.0 && action < page,
            format!("An event in the plan has its action, at {action} of {page}"),
        );
        d.state_prepared("session-events-fill-their-column", Sizes::All, |d| {
            show_plan(d);
            d.wait_for_text("Add to notes");
            // iter-25-S FB7: in the tab layout, the plan rows end at the tab row's right edge.
            let tabs_end = d.place_of("Edit session details").map(|rect| rect.right());
            let rows_end: Vec<f32> = d
                .nodes()
                .iter()
                .filter(|node| {
                    node.in_view()
                        && (node.name.starts_with("Add to notes:")
                            || node.name.starts_with("More actions for Major event"))
                })
                .map(|node| node.rect.right())
                .collect();
            let tabbed = d.has("Note");
            d.observe(
                "iter-25-S-FB7",
                !tabbed
                    || (!rows_end.is_empty()
                        && tabs_end.is_some_and(|end| {
                            rows_end.iter().any(|right| (right - end).abs() < 1.0)
                                && rows_end.iter().all(|right| *right <= end + 1.0)
                        })),
                format!("Plan row actions {rows_end:?} end at the tab row's edge {tabs_end:?}"),
            );
            d.observe(
                "H1 one plan Undo",
                d.nodes()
                    .iter()
                    .filter(|node| node.in_view() && node.name.starts_with("Undo"))
                    .count()
                    == 1
                    && d.has("Undo add"),
                "The plan add has one named Undo at each window size",
            );
        });
    }

    fn draft_crosses_note_breakpoint(d: &mut Driver<'_>) {
        d.set_base(SMALL);
        write_note(d);
        assert!(d.has("Note"), "The note starts in the narrow tab layout");
        d.replace_editor_text(&"Unsent account of the evening.\n".repeat(60));
        d.press_ctrl(Key::End);
        d.type_text("Draft stays across the breakpoint.");
        let editor = d
            .nodes()
            .into_iter()
            .find(|node| node.name == "Note text" && node.in_view())
            .expect("The note is visible");
        d.scroll_up_at(editor.shown.center(), 8);
        let before = d.editor_text().expect("The draft is open");
        let (focused, scroll) = d.note_editor_view();
        assert!(
            focused && scroll > 0.0,
            "The typed draft has focus and is scrolled: {scroll}"
        );
        for size in [LARGE, SMALL] {
            d.set_base(size);
            assert_eq!(d.has("Plan and notes"), size == LARGE);
            let (focused, after) = d.note_editor_view();
            assert_eq!(d.editor_text().as_deref(), Some(before.as_str()));
            assert!(focused, "The note keeps focus at {size:?}");
            assert!(
                (after - scroll).abs() < 1.0,
                "The note keeps scroll at {size:?}: {scroll} -> {after}"
            );
        }
        d.type_text(" Still typing.");
        assert!(
            d.editor_text()
                .unwrap()
                .contains("Draft stays across the breakpoint. Still typing.")
        );
        d.state("draft-survives-note-breakpoint", Sizes::Default);
        // Once focus moves from the note to a plan control, narrowing shows the plan.
        d.set_base(LARGE);
        assert!(
            d.tab_to("Add a fight"),
            "A plan control takes focus beside the note"
        );
        d.set_base(SMALL);
        let shown = d.in_view("Add a fight");
        let focused = d.focused_name();
        d.observe(
            "iter-5-C-focus-follows-plan",
            shown,
            format!(
                "After focus moves to the plan, the narrow page shows the plan (focus: {focused:?})"
            ),
        );
    }

    pub fn run(d: &mut Driver<'_>) {
        d.set_base(LARGE);
        resume_run_from_cold_start(d);
        content_follows_the_panel(d);
        d.set_base(DEFAULT);
        let run_edge = right_of(d, NEXT_TURN) + crate::design::M;
        d.drag_column_edge(run_edge, FAR_RIGHT);
        d.state("run-panel-narrow", Sizes::All);
        // AD1: at 1920 the dock's controls are one block after the selected name. iter-26-R FK1:
        // they follow the name's slot (the widest roster name, at most `12 × L`) after `L`, so
        // they start right of the name and no further than that slot and its gap; FK1's own
        // check (run-long-names) holds their x for every selection.
        let (name, conditions) = d.at_size(LARGE, |d| {
            (
                d.nodes()
                    .into_iter()
                    .find(|node| node.name.starts_with("Selected: "))
                    .map(|node| node.rect),
                d.place_of("Conditions for this participant"),
            )
        });
        d.observe(
            "iter-24-R-AD1",
            name.zip(conditions).is_some_and(|(name, conditions)| {
                conditions.left() >= name.right()
                    && conditions.left() - name.left() <= 13.0 * crate::design::L + 1.0
            }),
            format!("At 1920 Conditions follows the selected name: {name:?} then {conditions:?}"),
        );
        d.drag_column_edge(DEFAULT.0 - crate::design::M - PANEL_MIN_WIDTH, FAR_LEFT);
        d.state("run-panel-wide", Sizes::All);

        go_library(d);
        d.drag_column_edge(rail_edge(d), RAIL_DRAG_MAX_WIDTH);
        d.observe(
            CHECK,
            near(rail_edge(d), RAIL_DRAG_MAX_WIDTH),
            format!("The rail follows its edge to {}", rail_edge(d)),
        );
        d.drag_column_edge(sections_edge(d), FAR_RIGHT);
        let start = entries_edge(d) - sections_edge(d);
        d.drag_column_edge(entries_edge(d), FAR_RIGHT);
        let wide = (rail_edge(d), sections_edge(d), entries_edge(d));
        d.observe(
            CHECK,
            wide.2 - wide.1 > start,
            "The entry list follows its edge",
        );
        d.observe(
            CHECK,
            wide.2 <= DEFAULT.0 - 320.0,
            "The reader keeps its room, however far the list is dragged",
        );
        d.state("library-wide-columns", Sizes::All);

        d.click(BACK_TO_RUN);
        d.restart();
        go_library(d);
        d.observe(
            CHECK,
            (rail_edge(d), sections_edge(d), entries_edge(d)) == wide,
            "Dragged widths return after a restart",
        );

        d.drag_column_edge(entries_edge(d), FAR_LEFT);
        d.drag_column_edge(sections_edge(d), FAR_LEFT);
        d.drag_column_edge(rail_edge(d), FAR_LEFT);
        d.observe(
            CHECK,
            near(rail_edge(d), RAIL_WIDTH),
            "The rail stops at its narrowest",
        );
        d.state("library-narrow-columns", Sizes::All);

        // Beside the widest rail, the pages and panels must still work.
        d.drag_column_edge(rail_edge(d), RAIL_DRAG_MAX_WIDTH);
        go_party(d);
        d.expect(MEMBER);
        d.state("party-beside-wide-rail", Sizes::All);
        d.click(BACK_TO_RUN);
        d.observe(
            CHECK,
            near(rail_edge(d), RAIL_WIDTH),
            "The run limits the rail to keep room for its roster",
        );
        d.click(NEXT_TURN);
        d.state("run-rail-keeps-roster-room", Sizes::All);
        // FB7: in the sheet panel, the gap before the first `·` (after the status word) is the
        // gap before the second.
        let mut line = None;
        d.each_painted_text(|text, _| {
            if text
                .galley
                .job
                .text
                .contains("Not connected · Fetched 3 h ago · HP from this run")
            {
                line = Some(text_glyphs(text));
            }
        });
        let gaps = line.and_then(|line| {
            let first = separator_gaps(&line, 0)?;
            let at = line.iter().position(|g| g.0 == '·')? + 1;
            Some((first, separator_gaps(&line, at)?))
        });
        d.observe(
            "iter-23-R-FB7",
            gaps.is_some_and(|(first, second)| {
                (first.0 - second.0).abs() <= 1.0 && (first.1 - second.1).abs() <= 1.0
            }),
            format!(
                "The gaps around the sheet's first `·` equal those around its second: {gaps:?}"
            ),
        );
        d.click(NAV_LOOKUP);
        d.settle();
        d.expect("Close Lookup");
        d.state("lookup-beside-run-rail", Sizes::All);
        // Lookup gives the run room beside its natural-width rail.
        let lookup_edge = d.column_edge("lookup");
        let before = d.place_of(crate::lookup::SEARCH_NAME).unwrap().width();
        d.drag_column_edge(lookup_edge, FAR_RIGHT);
        d.settle();
        d.expect("Close Lookup");
        d.observe(
            CHECK,
            d.place_of(crate::lookup::SEARCH_NAME).unwrap().width() < before,
            "Lookup is narrower before its capture",
        );
        d.state("lookup-narrow-beside-run-rail", Sizes::All);
        d.observe_both(
            "iter-4-F1",
            "The dock label names exactly one participant with Lookup open",
            |d| d.dock_names_one_participant(),
        );
        d.click(NAV_LOOKUP);
        d.expect(NEXT_TURN);
        d.goal("resize the columns and go on working");
        the_roster_follows_the_add_panel(d);
        d.set_base(DEFAULT);
        the_session_fills_its_column(d);
        the_session_tabs_fill_their_column(d);
        draft_crosses_note_breakpoint(d);

        // Keyboard proof follows the existing task-cost checks.
        d.click(NAV_LIBRARY);
        d.harness_focus_nothing();
        assert!(d.tab_to("Resize Library entries column"));
        d.press(Key::Home);
        d.state_prepared("keyboard-column-handle", Sizes::All, |d| {
            assert!(d.tab_to("Resize Library entries column"));
        });
        let edge = entries_edge(d);
        d.press(Key::ArrowRight);
        assert!(near(entries_edge(d), edge + crate::design::L));
        // A second press without tabbing back: the handle kept focus and keeps resizing.
        d.press(Key::ArrowRight);
        assert!(
            near(entries_edge(d), edge + 2.0 * crate::design::L),
            "the column handle lost focus after the first arrow key"
        );
        d.state_prepared("keyboard-column-width", Sizes::All, |d| {
            assert!(d.tab_to("Resize Library entries column"));
        });
        d.press(Key::Home);
        assert!(near(entries_edge(d), edge));
    }
}

/// Section 3's table: an open run survives a restart, on the current turn, with the
/// last page and party sync kept. The scenario really starts the app a second time.
pub mod run_restart {
    use super::*;
    const OTHER: &str = "Ember Cultist";
    const PREVIOUS_CAMPAIGN: &str = "previous campaign";
    const RUN_FOLDER: &str = "Run folder";

    pub fn run(d: &mut Driver<'_>) {
        resume_run_from_cold_start(d);
        d.click_row(OTHER);
        d.click(NEXT_TURN);
        let current = current_participant(d).unwrap_or_default();
        let undo = d.name_starting("Undo");
        d.state("before-restart", Sizes::All);
        d.goal("be in a run");
        d.restart();
        d.observe(
            "Restart",
            d.has_containing(RUN_HEADING)
                && current_participant(d).as_deref() == Some(current.as_str())
                && participant_is_selected(d, &current),
            format!("The run reopens by itself on the same turn, {current} selected"),
        );
        d.observe(
            "Restart",
            undo == "Undo: next turn" && d.name_starting("Undo") == undo,
            format!(
                "Undo names the same step after the restart: {undo}, then {}",
                d.name_starting("Undo")
            ),
        );
        // The run file keeps the Undo steps, so the run looks as it did before the restart.
        d.assert_state_matches("after-restart", "before-restart");
        d.goal("restart");
        d.begin_task();
        // The other pages load too, with the run still open behind them.
        go_encounters(d);
        d.observe(
            "Restart",
            d.has_containing(READY_TO_RUN) && !d.has_containing(PREVIOUS_CAMPAIGN),
            "After a restart into a run, Encounters lists the campaign's encounters",
        );
        d.state("encounters-after-restart", Sizes::Default);
        go_campaign(d);
        d.click(TAB_RUN_SETTINGS);
        d.observe(
            "Restart",
            d.has_containing(RUN_FOLDER),
            "After a restart into a run, Rules and runs loads",
        );
        d.click(BACK_TO_RUN);
        d.begin_task();
        d.click("Close run");
        let closed = !d.app_mut().encounters.has_run();
        d.observe(
            "T12.1",
            closed,
            "The saved run closes with one click in its footer",
        );
        d.state("run-closed-in-one-step", Sizes::All);
        go_library(d);
        d.restart();
        let no_open_run = !d.app_mut().encounters.has_run();
        d.observe(
            "Restart",
            d.has_containing(SEARCH_LIBRARY) && no_open_run,
            "With no open run, the app reopens on the last page",
        );
        d.state("restart-on-last-page", Sizes::Default);
        d.goal("restart on a page");
        d.begin_task();
        d.observe(
            "P3",
            d.has_containing("Paused"),
            "The rail keeps the paused run in view",
        );
        let before_resume = d.interaction_count();
        d.click_starting("Paused run ·");
        d.expect(NEXT_TURN);
        d.expect_same("Resuming the paused run restores the saved turn and participant state.");
        d.state("paused-run-resumed-from-rail", Sizes::All);
        d.observe(
            "P3",
            d.interaction_count() - before_resume == 1,
            "The paused run resumes with one click",
        );
    }
}

/// States a run can reach that a good session does not show.
pub mod run_edge_states {
    use super::*;
    const LONG_EFFECT: &str =
        "Wreathed in the cinders of the old shrine, so every light within thirty feet burns low";
    const TARGET: &str = "Kestrel Dunmoor";
    const OTHER_EFFECT: &str = "Other effect";
    const ADD_EFFECT: &str = "Add";
    const ABSENT: &str = "Absent";
    const CLOSE_ERROR: &str = "Dismiss this message";
    const MARK_PRESENT: &str = "Absent";
    const KEEP_RUN: &str = "Replace the file with this run";
    const SAVE_AS_NEW: &str = "Save as a new run";
    const ALL: [&str; 8] = [
        "Brannoch Vale",
        "Cinder Wolf 1",
        "Cinder Wolf 2",
        "Iselle Marrowind",
        "Kestrel Dunmoor",
        "Oren Thistlewick",
        "Sable Harrowgate",
        "Ember Cultist",
    ];

    pub fn run(d: &mut Driver<'_>) {
        resume_run_from_cold_start(d);
        d.click_row(TARGET);
        d.click(CONDITIONS);
        d.type_into(OTHER_EFFECT, LONG_EFFECT);
        d.click(ADD_EFFECT);
        d.expect(&format!("Undo: {LONG_EFFECT} on {TARGET}"));
        d.state_prepared("long-effect-text", Sizes::All, |d| {
            undo_keeps_its_word(d, &format!("Undo: {LONG_EFFECT} on {TARGET}"));
        });
        d.observe(
            "Long effects",
            d.visible_text("Poisoned") && d.has("Read all conditions"),
            "The crowded row keeps one condition and a count for the long effect",
        );
        d.goal("record a long effect");
        d.begin_task();
        // iter-25-R FH1: Conc keeps its frame's room when off, so turning it on moves neither its
        // own words nor Absent beside it.
        let dock = |d: &Driver<'_>| {
            (
                d.place_of("Concentrating")
                    .map(|rect| (rect.left(), rect.width())),
                d.place_of("Absent").map(|rect| rect.left()),
            )
        };
        let conc_off = dock(d);
        d.click("Concentrating");
        d.expect(&format!("Undo: start concentration for {TARGET}"));
        let conc_on = dock(d);
        d.observe(
            "iter-25-R-FH1",
            conc_off.0.is_some() && conc_off.1.is_some() && conc_off == conc_on,
            format!("Conc and Absent keep their places: off {conc_off:?}, on {conc_on:?}"),
        );
        d.state_prepared("start-concentration-undo", Sizes::All, |d| {
            undo_keeps_its_word(d, &format!("Undo: start concentration for {TARGET}"));
        });
        d.click("Concentrating");
        d.expect(&format!("Undo: stop concentration for {TARGET}"));
        // The row returns to the long-effect frame; Undo now names the other step.
        d.state_prepared("stop-concentration-undo", Sizes::All, |d| {
            undo_keeps_its_word(d, &format!("Undo: stop concentration for {TARGET}"));
            undo_keeps_a_target(d, "Sync off for the party", None);
        });
        d.click("Sheet");
        d.click("Read all conditions");
        d.observe(
            "Long effects",
            d.visible_text(LONG_EFFECT),
            "More opens Conditions with the full effect",
        );
        d.expect_same(
            "More returns to Conditions with the full effect already shown after adding it.",
        );
        d.state("full-effect-in-conditions", Sizes::Default);
        d.begin_task();

        for name in ALL {
            d.click_row(name);
            d.click(ABSENT);
        }
        d.expect("No participants are present");
        d.observe(
            "Nobody present",
            d.is_disabled(NEXT_TURN),
            "With nobody present, Next turn is off, and a steady line says why",
        );
        d.state("all-absent", Sizes::Default);
        d.goal("see everyone absent");
        for name in ALL {
            d.click_row(name);
            d.click(MARK_PRESENT);
        }
        // iter-26-R FI2: each row's top before the file changes, at every captured size.
        let tops = |d: &mut Driver<'_>| {
            [SMALL, DEFAULT, LARGE].map(|size| {
                d.at_size(size, |d| {
                    roster_places(d)
                        .into_iter()
                        .map(|(name, rect)| (name, rect.top().round()))
                        .collect::<Vec<_>>()
                })
            })
        };
        let quiet = tops(d);
        // A second instance writes the file. Saving must stop and say so.
        let path = d
            .app_mut()
            .encounters
            .run()
            .and_then(|run| run.path().map(std::path::Path::to_path_buf))
            .expect("a saved run");
        let text = std::fs::read_to_string(&path).expect("read the run") + "\n";
        std::fs::write(&path, text).expect("change the run on disk");
        d.click_row(TARGET);
        d.type_text("3");
        d.press(Key::Enter);
        d.expect("changed on disk");
        d.expect("Not saved");
        // iter-26-R FI2: the notice sits in the message footer below the roster: no row moves.
        let noticed = tops(d);
        d.observe(
            "iter-26-R-FI2",
            quiet.iter().all(|rows| rows.len() >= 8) && quiet == noticed,
            format!("Row tops without the notice {quiet:?}, with it {noticed:?}"),
        );
        // The red choice keeps an `L` gap from the safe one, which stays first on the same line.
        d.observe_both(
            "iter-12-FT2",
            "Replace the file with this run keeps an L gap after Save as a new run",
            |d| {
                d.place_of(SAVE_AS_NEW)
                    .zip(d.place_of(KEEP_RUN))
                    .is_some_and(|(safe, red)| {
                        (safe.center().y - red.center().y).abs() < 1.0
                            && red.left() - safe.right() >= crate::design::L
                    })
            },
        );
        // Both choices start at the message text, not at the warning bar.
        d.observe_both(
            "iter-14-B-AT2",
            "Save as a new run starts at the message text edge, and the red choice follows it",
            |d| {
                let text = d.painted_rect_in("This run changed on disk", egui::Rect::EVERYTHING);
                match (text, d.place_of(SAVE_AS_NEW), d.place_of(KEEP_RUN)) {
                    (Some(text), Some(safe), Some(red)) => {
                        (safe.left() - text.left()).abs() < 1.0 && red.left() > safe.right()
                    }
                    _ => false,
                }
            },
        );
        // Exactly `L`: egui's own item spacing once made this gap 24 px.
        d.observe_both(
            "iter-15-B-ST4",
            "Replace the file with this run starts exactly L after Save as a new run",
            |d| {
                d.place_of(SAVE_AS_NEW)
                    .zip(d.place_of(KEEP_RUN))
                    .is_some_and(|(safe, red)| {
                        (safe.center().y - red.center().y).abs() < 1.0
                            && (red.left() - safe.right() - crate::design::L).abs() < 0.5
                    })
            },
        );
        d.state_prepared("changed-on-disk", Sizes::Default, |d| {
            conflict_choices_read_height(d, "iter-15-B2-density-run", SAVE_AS_NEW, KEEP_RUN);
        });
        d.goal("see the run changed on disk");
        // The state cannot be dismissed, and closing tells the truth about it.
        d.observe(
            "Changed on disk",
            !d.has(CLOSE_ERROR) && d.has(KEEP_RUN) && d.has(SAVE_AS_NEW),
            "The message stays, with both ways out",
        );
        if !d.has(CLOSE_RUN) && d.has("Close menu") {
            d.click("Close menu");
        }
        d.click(CLOSE_RUN);
        d.observe(
            "Changed on disk",
            !d.has_containing("stays saved")
                && d.has("Close and lose changes")
                && d.visible_text("Closing loses this change:"),
            "Close says what it will lose, in place of \"The run stays saved\"",
        );
        d.state("close-while-not-saved", Sizes::Default);
        d.press(Key::Escape);
        d.observe(
            "Changed on disk",
            !d.has("Keep running") && !d.has("Close and lose changes") && d.has(KEEP_RUN),
            "Escape backs out of the close prompt and keeps the run",
        );
        // Keep this run: the file becomes this run, and later turns save by themselves.
        d.click(KEEP_RUN);
        let saved = std::fs::read_to_string(&path).expect("read the run");
        d.observe(
            "Changed on disk",
            d.visible_text("Saved")
                && !d.has_containing("changed on disk")
                && saved.contains(LONG_EFFECT),
            "Keep this run saves it over the changed file",
        );
        // It again, and this time save beside it.
        std::fs::write(&path, saved.clone() + "\n").expect("change the run on disk");
        d.click_row(TARGET);
        d.type_text("1");
        d.press(Key::Enter);
        d.expect("changed on disk");
        d.click(SAVE_AS_NEW);
        let new_path = d
            .app_mut()
            .encounters
            .run()
            .and_then(|run| run.path().map(std::path::Path::to_path_buf))
            .expect("a saved run");
        d.observe(
            "Changed on disk",
            new_path != path
                && new_path.exists()
                && std::fs::read_to_string(&path).is_ok_and(|text| text == saved.clone() + "\n")
                && d.visible_text("Saved"),
            "Save as a new run keeps the changed file as it was, and saves this one beside it",
        );
        // The close dialog can keep both files and finish closing in one step.
        let before = std::fs::read_to_string(&new_path).expect("read the new run");
        std::fs::write(&new_path, before.clone() + "\n").expect("change the new run on disk");
        d.click_row(TARGET);
        d.type_text("1");
        d.press(Key::Enter);
        if !d.has(CLOSE_RUN) && d.has("Close menu") {
            d.click("Close menu");
        }
        d.click(CLOSE_RUN);
        d.expect_same(
            "A second disk conflict opens the same close recovery choices for the new file.",
        );
        d.state("save-new-run-before-close", Sizes::Standard);
        d.click("Save as a new run and close");
        let closed = !d.app_mut().encounters.has_run();
        d.observe(
            "Changed on disk",
            closed && std::fs::read_to_string(&new_path).is_ok_and(|text| text == before + "\n"),
            "Save as a new run and close keeps the other file and closes the run",
        );
        d.state("saved-new-run-and-closed", Sizes::All);
        d.observe(
            "Recovery selection",
            d.has_containing("Changed on disk") && d.has_containing("Copy 2"),
            "Recovery copies show their number, and the preserved file is marked Changed on disk",
        );
        // FJ6: every paused run's line two starts with the run icon, also the one whose first
        // part is the disk warning.
        let mut details = Vec::new();
        d.each_painted_text(|text, _| {
            let words = &text.galley.job.text;
            if words.contains("Paused") && !words.contains('\n') && words.contains(" · ") {
                details.push(words.clone());
            }
        });
        let run = crate::design::icons::RUN;
        d.observe(
            "iter-25-B-FJ6",
            details.len() >= 3
                && details.iter().any(|words| {
                    words.starts_with(&format!(
                        "{run} {} Changed on disk",
                        crate::design::icons::WARN
                    ))
                })
                && details.iter().all(|words| words.starts_with(run)),
            format!("Each paused run's line two starts with the run icon: {details:?}"),
        );
        d.click(RESUME);
        let resumed_copy = d
            .app_mut()
            .encounters
            .run()
            .is_some_and(|run| run.path().is_some_and(|path| path != new_path));
        d.observe(
            "Recovery selection",
            resumed_copy,
            "Resume opens the recovery copy closed last",
        );
        d.observe_both(
            "iter-5-B-FT1",
            "A recovery copy keeps End visible beside Close run",
            |d| {
                d.in_view("End encounter…")
                    && (d.visible_text("End…") || d.visible_text("End encounter…"))
            },
        );
        d.state("resumed-last-closed-copy", Sizes::All);
        for size in [SMALL, DEFAULT, (1920.0, 1080.0)] {
            d.set_base(size);
            assert!(d.in_view("End encounter…"));
            assert!(d.visible_text("End…") || d.visible_text("End encounter…"));
            d.click("End encounter…");
            d.expect("Outcome");
            d.click("Keep playing");
        }
        d.set_base(SMALL);
        d.unreached(
            "HP unknown on a live row",
            "the setup view, because a live run requires known HP (Run::open and Begin refuse otherwise)",
        );
    }
}

/// T3.1: each synced row says its own state, with nothing expanded. The states are
/// simulated, because test builds cannot read a live sheet.
pub mod run_sync_states {
    use super::*;
    const KEEP_LOCAL: &str = "Keep Splat values and retry";
    use crate::combat_sync::SyncState;
    const STATES: [(&str, SyncState, &str); 5] = [
        ("Brannoch Vale", SyncState::Conflict, "Conflict"),
        ("Iselle Marrowind", SyncState::Matching, "Synced"),
        ("Kestrel Dunmoor", SyncState::Pending, "Sync pending"),
        ("Oren Thistlewick", SyncState::ReadOnly, "Sync read-only"),
        ("Sable Harrowgate", SyncState::Error, "Sync error"),
    ];
    /// Linked rows keep their word during the first read and while sync is off.
    const FIRST_READ: (&str, SyncState, &str) =
        ("Sable Harrowgate", SyncState::FirstRead, "Reading sheet…");
    const UNSYNCED_ROW: &str = "Cinder Wolf 1";
    const DISMISS: &str = "Dismiss this message";
    const TWO_LINES: &str =
        "Brannoch Vale, Iselle Marrowind, Kestrel Dunmoor, Oren Thistlewick, Sable Harrowgate";
    const USE_REMOTE: &str = "Use D&D Beyond values";

    pub fn run(d: &mut Driver<'_>) {
        resume_run_from_cold_start(d);
        d.app_mut()
            .settings
            .simulate(crate::settings::Account::Connected);
        for (name, state, _) in STATES {
            d.app_mut().encounters.simulate_sync_state(name, state);
        }
        d.settle();
        for (name, _, word) in STATES {
            let row = d.row_node(name);
            let in_view = row.as_ref().is_some_and(|row| row.fully_in_view());
            let text = row.map_or_else(String::new, |row| row.name);
            d.observe(
                "T3.1",
                in_view && text.contains(word),
                format!("{name}'s row, in view, says {word}: {text}"),
            );
            let painted = d.row_words(name).replace('\u{a0}', " ");
            d.observe(
                "T3.1",
                painted.contains(word),
                format!("At 960x640, simulated state: {name}'s row paints {word}: {painted}"),
            );
        }
        let plain = d
            .row_node(UNSYNCED_ROW)
            .map_or_else(String::new, |row| row.name);
        d.observe(
            "T3.1",
            !plain.contains("Sync") && !plain.contains("Conflict"),
            format!("A row with no link says nothing about sync: {plain}"),
        );
        let (name, state, word) = FIRST_READ;
        d.app_mut().encounters.simulate_sync_state(name, state);
        d.settle();
        d.observe(
            "T3.1",
            d.row_words(name).contains(word),
            format!("Simulated: {name}'s row paints {word}"),
        );
        // Back to the state this row had, so that the capture shows the other five.
        let (name, state, _) = STATES[4];
        d.app_mut().encounters.simulate_sync_state(name, state);
        d.settle();
        d.state_prepared("every-sync-state", Sizes::All, |d| {
            undo_keeps_a_target(d, "Sync conflict: Brannoch Vale", Some("HP 58 → 54"));
        });
        d.goal("show every sync state");
        d.begin_task();
        d.app_mut()
            .encounters
            .simulate_sync_state("Sable Harrowgate", SyncState::Off);
        d.settle();
        d.observe_both(
            "AT1",
            "A linked row says Sync off; unlinked creatures keep no sync word",
            |d| {
                d.row_words("Sable Harrowgate").contains("Sync off")
                    && !d.row_words(UNSYNCED_ROW).contains("Sync")
            },
        );
        d.click_row("Sable Harrowgate");
        d.state_prepared("linked-row-sync-off", Sizes::All, |d| {
            undo_keeps_a_target(d, "Sync conflict: Brannoch Vale", Some("HP 58 → 54"));
        });
        d.click_row("Cinder Wolf 2");
        d.app_mut()
            .encounters
            .simulate_sync_state("Sable Harrowgate", SyncState::Error);
        d.settle();

        // At the default size, 8 rows have the room, and a two-line message takes none
        // of it from them. Each row's height, then the same places with the message up.
        d.click(DISMISS);
        let (quiet, quiet_dock) = d.at_size(DEFAULT, |d| (roster_places(d), d.place_of("Amount")));
        d.app_mut().encounters.simulate_sheet_update(TWO_LINES);
        d.settle();
        let (loud, loud_dock) = d.at_size(DEFAULT, |d| (roster_places(d), d.place_of("Amount")));
        let shortest = quiet
            .iter()
            .chain(&loud)
            .fold(f32::MAX, |short, row| short.min(row.1.height()));
        d.observe(
            "Rows have room",
            quiet.len() == 8 && shortest >= 52.0,
            format!("At 1200x800, with and without a two-line message, 8 rows are not compressed: the shortest is {shortest} px"),
        );
        d.observe(
            "Stable rows",
            quiet.len() == 8 && quiet == loud && quiet_dock == loud_dock,
            format!("At 1200x800, a two-line message moves no row and not the dock: {quiet:?} then {loud:?}"),
        );
        d.begin_task();

        // Ending the encounter does not end the sync. Close waits, and the conflict can be
        // resolved in the ended view.
        d.app_mut()
            .encounters
            .simulate_sync_conflict("Iselle Marrowind");
        d.settle();
        d.click("End encounter…");
        d.click("End encounter");
        d.observe(
            "Ended run close",
            d.is_disabled(CLOSE_RUN)
                && d.row_words("Iselle Marrowind").contains("Conflict")
                && d.has(USE_REMOTE),
            "With a conflict, Close is blocked in the ended view, whose row and controls show it",
        );
        // iter-26-R FK3: every sync block heading has one form: the name in the text colour,
        // ` · `, then the status's icon and word in its own colour.
        let mut headings = Vec::new();
        d.each_painted_text(|text, _| {
            let job = &text.galley.job;
            if job.sections.len() == 3
                && STATES
                    .iter()
                    .any(|(name, _, _)| job.text.starts_with(&format!("{name} · ")))
            {
                headings.push((
                    job.text.clone(),
                    job.sections
                        .iter()
                        .map(|s| s.format.color)
                        .collect::<Vec<_>>(),
                ));
            }
        });
        let text = crate::design::colors().text;
        d.observe(
            "iter-26-R-FK3",
            headings
                .iter()
                .any(|(words, _)| words.ends_with("HP conflict"))
                && headings
                    .iter()
                    .any(|(words, _)| words.ends_with("Sync pending"))
                && headings
                    .iter()
                    .all(|(_, colors)| colors[0] == text && colors[2] != text),
            format!("Sync block headings, one form: {headings:?}"),
        );
        d.state("ended-with-conflict", Sizes::All);
        // FJ3: the closed Prep and Combat log sections start at one x.
        for size in [SMALL, DEFAULT, LARGE] {
            let lefts = d.at_size(size, |d| {
                (
                    d.place_of("Prep").map(|r| r.left()),
                    d.place_of("Combat log").map(|r| r.left()),
                )
            });
            d.observe(
                "iter-23-R-FJ3",
                matches!(lefts, (Some(prep), Some(log)) if (prep - log).abs() < 0.5),
                format!("At {size:?} Prep and Combat log start at one x: {lefts:?}"),
            );
        }
        // iter-17-B: each Stop sync for <name> starts its line at the sync block's text edge.
        let stops: Vec<String> = d
            .nodes()
            .into_iter()
            .filter(|node| node.name.starts_with("Stop sync for ") && node.in_view())
            .map(|node| node.name)
            .collect();
        d.observe(
            "iter-17-B-stop-sync",
            !stops.is_empty(),
            format!("The ended view offers Stop sync: {stops:?}"),
        );
        for stop in &stops {
            check_line_start(
                d,
                "iter-17-B-stop-sync",
                stop,
                "Splat and D&D Beyond differ.",
                &[SMALL, DEFAULT, LARGE],
                None,
            );
        }
        for size in [SMALL, LARGE] {
            let (row_edge, undo_edge) = d.at_size(size, |d| {
                (
                    roster_places(d).first().map(|(_, rect)| rect.right()),
                    d.place_of("Undo: end encounter").map(|rect| rect.right()),
                )
            });
            d.observe(
                "Ended header alignment",
                row_edge
                    .zip(undo_edge)
                    .is_some_and(|(row, undo)| (row - undo).abs() <= 1.0),
                format!("At {size:?}, Undo ends at {undo_edge:?} and the roster at {row_edge:?}"),
            );
        }
        // A pair of choices is never cut: both are in view, at the minimum window too.
        let both = |d: &Driver<'_>| {
            let nodes = d.nodes();
            [USE_REMOTE, KEEP_LOCAL]
                .iter()
                .all(|name| nodes.iter().any(|n| n.name == *name && n.fully_in_view()))
        };
        d.observe_both(
            "Ended run close",
            "Both choices of a conflict are in view",
            both,
        );
        // Keeping the DM's value changes nothing in the run, so the steps before it stay.
        d.click(KEEP_LOCAL);
        for (name, ..) in STATES {
            let stop = format!("Stop sync for {name}");
            if d.has(&stop) {
                d.click(&stop);
            }
        }
        d.observe(
            "Ended run close",
            !d.is_disabled(CLOSE_RUN),
            "With the sync resolved or stopped, the ended run can close",
        );
        d.click_containing("Undo: end encounter");
        for (name, state, _) in STATES {
            d.app_mut().encounters.simulate_sync_state(name, state);
        }
        d.settle();
        d.app_mut()
            .encounters
            .simulate_sync_conflict("Brannoch Vale");
        d.settle();
        go_library(d);
        d.state("conflict-on-strip", Sizes::All);
    }
}

/// T1.3 under pressure: a sync conflict must not push any of 12 rows out of view. Then
/// the Tab order on a scrolling roster of 20, where scroll handles appear.
pub mod run_roster_conflict {
    use super::*;
    const CLOSE_RUN: &str = "Close run";
    use crate::combat_sync::SyncState;
    const SYNCED: &str = "Brannoch Vale";
    const USE_REMOTE: &str = "Use D&D Beyond values";
    const SYNC_PARTY: &str = "Sync party";
    const STOP_SYNC: &str = "Stop party sync";
    const AMOUNT: &str = "Amount";
    const LOADED_ROW: &str = "Kestrel Dunmoor";
    const ONE_MORE_LINE: &str = "Cinder Wolf 3";
    const ONE_MORE_CONDITION: &str = "Prone";
    const LONG_MESSAGE: &str =
        "Brannoch Vale HP 58 → 54, Iselle Marrowind HP 27 → 18, Kestrel Dunmoor HP 11 → 8";
    const LOADED_CONDITIONS: [&str; 5] =
        ["Blinded", "Charmed", "Grappled", "Paralyzed", "Restrained"];

    /// The rows of the 12 that are not in full view.
    fn rows_cut(d: &Driver<'_>) -> Vec<&'static str> {
        let nodes = d.nodes();
        run_large_roster::ROSTER_12
            .into_iter()
            .filter(|name| {
                !nodes
                    .iter()
                    .any(|n| n.name.starts_with(&format!("{name},")) && n.fully_in_view())
            })
            .collect()
    }

    pub fn run(d: &mut Driver<'_>) {
        resume_run(d, RUN_12);
        d.app_mut()
            .settings
            .simulate(crate::settings::Account::Connected);
        d.app_mut()
            .encounters
            .simulate_sync_state(SYNCED, SyncState::Conflict);
        d.settle();
        d.state("roster-of-12-with-conflict", Sizes::All);
        // iter-25-R FJ4: the dock's place with no notice below the roster.
        let amount_at = |d: &mut Driver<'_>| {
            [SMALL, DEFAULT, LARGE]
                .map(|size| d.at_size(size, |d| d.place_of("Amount").map(|rect| rect.top())))
        };
        let with_none = amount_at(d);
        // On the compact row, the conflict's icon starts where Bloodied's icon starts on the
        // rows below: the second lines share one left edge.
        let (aligned, detail) = d.at_size(SMALL, |d| {
            let conflict = painted_starts(d, "Conflict", 2);
            let bloodied = painted_starts(d, "Bloodied", 2);
            let aligned = matches!(conflict.as_slice(), [conflict] if !bloodied.is_empty()
                && bloodied.iter().all(|b| (b.x - conflict.x).abs() < 0.5));
            (
                aligned,
                format!("conflict {conflict:?}, bloodied {bloodied:?}"),
            )
        });
        d.observe(
            "iter-15-B-FT3",
            aligned,
            format!("The compact row's Conflict starts at the second line's edge: {detail}"),
        );
        let nodes = d.nodes();
        let rows = run_large_roster::ROSTER_12
            .iter()
            .filter(|name| {
                nodes
                    .iter()
                    .any(|n| n.name.starts_with(&format!("{name},")) && n.fully_in_view())
            })
            .count();
        d.observe(
            "T1.3",
            rows == run_large_roster::ROSTER_12.len() && d.in_view(APPLY_DAMAGE),
            format!("During a conflict at 960x640: {rows} of 12 rows in full view, with the damage entry"),
        );
        d.goal("show a conflict with 12 rows");

        // Section 6's hard case: 5 conditions on a row, the party's sync words, and an
        // error line, with all 12 rows and the amount field still in view.
        d.app_mut().encounters.simulate_connection();
        d.settle();
        d.click(SYNC_PARTY);
        d.click_row(LOADED_ROW);
        d.click(CONDITIONS);
        for condition in LOADED_CONDITIONS {
            d.click(condition);
        }
        // A message the DM has not dismissed takes one line here, so no row has to scroll.
        d.app_mut().encounters.simulate_sheet_update(LONG_MESSAGE);
        d.settle();
        d.click_row(ONE_MORE_LINE);
        d.click(CONDITIONS);
        d.click(ONE_MORE_CONDITION);
        d.observe(
            "Run notices",
            !d.visible_text("Undo stopped. See Log"),
            "A new action clears the old sync notice",
        );
        d.app_mut().encounters.simulate_sheet_update(LONG_MESSAGE);
        d.settle();
        for size in [SMALL, DEFAULT] {
            let fits = d.at_size(size, |d| {
                let right = d.place_of(NEXT_TURN).unwrap().right();
                d.place_of("Absent")
                    .is_some_and(|rect| rect.right() <= right + 1.0)
                    && d.visible_text("Saved")
            });
            d.observe(
                "R1",
                fits,
                format!("At {size:?}, the dock and save status fit the roster column"),
            );
        }
        d.state("message-on-one-line", Sizes::Default);
        // iter-25-R FJ4: the dock's place with a warning notice, to compare with an error.
        let with_warning = amount_at(d);
        let cut = rows_cut(d);
        d.observe(
            "T1.3",
            cut.is_empty() && d.has("Log"),
            format!(
                "At 960x640 with a long message, it takes one line and every row stays: {cut:?}"
            ),
        );
        if d.has("Read all conditions") {
            d.click("Read all conditions");
        } else {
            d.click_row(LOADED_ROW);
            d.click(CONDITIONS);
        }
        d.observe(
            "Condition summary",
            d.has("Kestrel Dunmoor") && d.has("Restrained"),
            "The crowded row's count link opens every condition for that participant",
        );
        d.state("all-conditions-from-summary", Sizes::Default);
        d.click_row(ONE_MORE_LINE);
        d.click(CONDITIONS);
        d.click(ONE_MORE_CONDITION);
        d.click_row(LOADED_ROW);
        let errored = LOADED_CONDITIONS.len() + 1;
        // With no notice at all, too: the dock's place does not depend on any notice.
        let no_notice = (!d.has("Dismiss this message")).then(|| amount_at(d));
        d.app_mut().encounters.simulate_run_error(
            "Ember Cultist's stat block could not be read. Its row still works. Repair it in",
        );
        d.settle();
        let with_error = amount_at(d);
        d.observe(
            "iter-25-R-FJ4",
            d.has_containing("stat block could not be read")
                && with_warning.iter().all(Option::is_some)
                && with_error == with_warning
                && no_notice == Some(with_error)
                && with_none == with_error,
            format!("Amount keeps its y with an error notice {with_error:?} as with a warning {with_warning:?} and none {no_notice:?} {with_none:?}"),
        );
        d.state("loaded-roster-of-12", Sizes::All);
        let fault = "Ember Cultist's stat block could not be read.";
        // The words fit their line by whole words (`words_that_fit`), so a narrow roster can
        // cut them: find them by their start, and their full words in hover.
        let fault_words = |d: &Driver<'_>| {
            d.nodes()
                .into_iter()
                .find(|node| {
                    node.name.starts_with("Ember Cultist")
                        && node.name.contains("stat block")
                        && node.role == egui::accesskit::Role::Label
                        && node.in_view()
                })
                .map(|node| node.rect)
        };
        let words = fault_words(d);
        d.observe(
            "iter-23-R-FJ6",
            words.is_some_and(|words| d.has_tone_bar(words, crate::design::colors().result.error))
                && !d
                    .nodes()
                    .iter()
                    .any(|node| node.name == fault && node.is_link())
                // iter-26-R FI11, SI2: the repair is a bordered button, as in Arrivals.
                && d.nodes().iter().any(|node| {
                    node.name == "Repair in Campaign → Packages"
                        && node.role == egui::accesskit::Role::Button
                })
                && d.control_frame("Repair in Campaign → Packages")
                    .is_some_and(|frame| frame.stroke.width > 0.0)
                && d.visible_text("Campaign → Packages"),
            "A stat-block error is an error message, and its repair place is a bordered button",
        );
        // iter-26-R AI1, Astra 3: the error is in the run's message footer, its whole fault
        // wrapped, the bordered repair under its words with its frame at them, and the close
        // icon beside the fault; FI11: the button lies whole inside the roster column.
        for size in [SMALL, DEFAULT, LARGE] {
            let places = d.at_size(size, |d| {
                (
                    fault_words(d),
                    d.place_of("Repair in Campaign → Packages"),
                    d.place_of("Dismiss this error"),
                    d.place_of(NEXT_TURN),
                )
            });
            d.observe(
                "iter-26-R-AI1",
                matches!(places, (Some(words), Some(button), Some(close), Some(next))
                    if (button.left() - words.left()).abs() <= 1.0
                        && button.top() >= words.bottom() - 0.5
                        && close.top() < words.bottom()
                        && button.right() <= next.right() + 1.0),
                format!("At {size:?}, error words, repair, close and Next turn: {places:?}"),
            );
        }
        // Astra 3: at 960 the whole fault is read in view, reached by the keyboard alone.
        let full_fault = "Ember Cultist's stat block could not be read. Its row still works.";
        let reached = d.tab_to("Repair in Campaign → Packages");
        d.observe(
            "iter-26-R-AI1-whole",
            d.window_size() == SMALL && reached && d.visible_text(full_fault),
            format!("At 960, Tab reaches the repair and the whole fault shows: reached {reached}"),
        );
        // Astra 3 changes this check on purpose: the error's whole fault wraps in the run's message
        // footer (the lead's footer ruling: the list keeps its top and only its visible height
        // shrinks), so at 960 a row may sit behind it. Every such row is counted by the overflow
        // line, and the dock stays in view. Before, the fault was cut to one line to keep every
        // row in view.
        let cut = rows_cut(d);
        let counted = cut.is_empty() || d.visible_text(&format!("{} below", cut.len()));
        d.observe(
            "T1.3",
            counted && d.in_view(AMOUNT) && d.in_view(APPLY_DAMAGE),
            format!("At 960x640 with {errored} extra lines, a message not dismissed and the error in the footer, rows not in full view are counted: {cut:?}"),
        );
        d.app_mut().encounters.simulate_run_error("");
        d.settle();
        d.click(STOP_SYNC);
        d.click_row(SYNCED);
        d.click(USE_REMOTE);
        // Other rows still hold simulated changes, so closing says so first.
        d.click(CLOSE_RUN);
        d.click("Close now");
        d.begin_task();
        resume_run(d, RUN_20);
        d.check_tab_order("run, roster of 20");
        d.goal("tab through a roster of 20");
        for _ in 0..20 {
            d.click(NEXT_TURN);
        }
        d.state("after-full-round", Sizes::Standard);
    }
}

/// The catalogue opens on what this campaign can use, a switch shows everything, and
/// browsing never changes the active campaign.
pub mod library_scope {
    use super::*;
    const USES: &str = "Can use";
    const EVERYTHING: &str = "Everything";
    const ACTIVE: &str = "The Emberfall Marches";

    pub fn run(d: &mut Driver<'_>) {
        go_library(d);
        search_library(d, "fireball");
        d.click("Fireball · Spell · Level 3");
        d.begin_task();
        let before = d.app_mut().active_campaign_name();
        d.open_combo_at(LIBRARY_PACKAGE_DROPDOWN);
        d.expect(USES);
        // Round 23 AC1: the menu is checked against the splitter beside its column at 960 and
        // 1200, and every choice ends at the control's right edge, `S` before that splitter.
        d.state("scope-menu", Sizes::Standard);
        for size in [SMALL, DEFAULT] {
            let (control, choices) = d.at_size(size, |d| {
                (
                    d.place_of(&format!("What the Library shows: {USES}")),
                    [USES, EVERYTHING, MODULE, "Campaign entries"]
                        .map(|name| d.place_of(name).map(|rect| (name, rect))),
                )
            });
            d.observe(
                "iter-23-L-AC1",
                control.is_some_and(|control| {
                    choices.iter().all(|choice| {
                        choice.is_some_and(|(_, rect)| rect.right() <= control.right() + 0.5)
                    })
                }),
                format!(
                    "At {size:?} every scope choice {choices:?} ends inside the scope control {control:?}"
                ),
            );
        }
        // Round 24 FA5: the open menu is as wide as its control, so it covers the counts beside
        // the control. Each choice fills the menu's content width, so the choices' insets from
        // the control's two edges are equal, and each is the menu's frame (its margin and rule).
        let frame = f32::from(d.app_style().spacing.menu_margin.left)
            + d.app_style().visuals.window_stroke.width;
        for size in [SMALL, DEFAULT] {
            let (control, choices) = d.at_size(size, |d| {
                (
                    d.place_of(&format!("What the Library shows: {USES}")),
                    [USES, EVERYTHING, MODULE].map(|name| d.place_of(name)),
                )
            });
            d.observe(
                "iter-24-L-FA5",
                control.is_some_and(|control| {
                    choices.iter().all(|choice| {
                        choice.is_some_and(|choice| {
                            let left = choice.left() - control.left();
                            let right = control.right() - choice.right();
                            (left - right).abs() <= 1.0 && right <= frame + 1.0
                        })
                    })
                }),
                format!(
                    "At {size:?} the scope menu's choices {choices:?} sit the menu frame ({frame}) inside both edges of the control {control:?}"
                ),
            );
        }
        d.click(EVERYTHING);
        d.observe(
            "Library scope search",
            d.field_text("Search the Library").as_deref() == Some("fireball"),
            "A scope change keeps the query and selected Fireball",
        );
        d.state("scope-everything", Sizes::All);
        d.observe(
            "Reader selection",
            d.visible_text("Casting Time"),
            "The open spell stays in the reader when it belongs to the new scope",
        );
        d.goal_within("A9", "show everything", 2);
        let after = d.app_mut().active_campaign_name();
        d.observe(
            "A9",
            before == after && after.as_deref() == Some(ACTIVE),
            format!("The active campaign is {after:?} after browsing, as before: {before:?}"),
        );
        // The file line is one Read row for every owner, so the title does not move
        // between the built-in Fireball, the campaign Fireball and a package About.
        assert!(
            d.visible_text(BUILT_IN) && !d.has("Show file"),
            "fixture: the built-in Fireball is open"
        );
        let built_in = title_offsets(d, "Fireball");
        d.click_starting("Fireball · Spell · Level 3 · Campaign");
        d.wait_for_text(CAMPAIGN_OWNER);
        d.expect("Show file");
        let campaign = title_offsets(d, "Fireball");
        file_line_centred(d, &[CAMPAIGN_OWNER], &["Show file"], "campaign Fireball");
        d.state("campaign-fireball", Sizes::All);
        d.open_combo_at(LIBRARY_PACKAGE_DROPDOWN);
        d.click(MODULE);
        d.click("About Ashen Marches");
        d.wait_for_text(PACKAGE_OWNER);
        d.expect("Choose every entry");
        let about = title_offsets(d, MODULE);
        file_line_centred(
            d,
            &[PACKAGE_OWNER],
            // Round 25 FF3: `Choose every entry` is a toolbar action now.
            &["Show file"],
            "package About",
        );
        d.state("package-about-title", Sizes::All);
        d.observe(
            "iter-17-E-FD2",
            built_in.iter().all(Option::is_some)
                && built_in == campaign
                && built_in == about,
            format!(
                "The reader title is the same distance below Read for every owner (960, 1200, 1920): built-in {built_in:?}, campaign {campaign:?}, package About {about:?}"
            ),
        );
        // Round 23 FC5, FD9: the empty search names its scope, and quiet `Search Everything`
        // keeps the words and shows the matches in Everything.
        d.observe(
            "iter-23-L-FC5-words",
            d.field_text("Search the Library").as_deref() == Some("fireball")
                && d.has(&format!(
                    "No entries match in {MODULE}. Try fewer words, or search Everything."
                ))
                && d.in_view(crate::library::SEARCH_EVERYTHING),
            "The empty search names the package scope and offers Search Everything",
        );
        d.click(crate::library::SEARCH_EVERYTHING);
        d.expect("Fireball · Spell · Level 3");
        d.observe(
            "iter-23-L-FC5",
            d.field_text("Search the Library").as_deref() == Some("fireball")
                && d.has(&format!("What the Library shows: {EVERYTHING}"))
                && !d.has_containing("No entries match"),
            "Search Everything keeps the words, sets the scope to Everything and lists the matches",
        );
    }

    const BUILT_IN: &str = "Built-in · Read only";
    const CAMPAIGN_OWNER: &str = "Campaign · Editable";
    const PACKAGE_OWNER: &str = "4 of 6 entries chosen · 1 still needed";

    /// The reader title's distance below the Read tab, at each captured size.
    fn title_offsets(d: &mut Driver<'_>, title: &str) -> Vec<Option<f32>> {
        [SMALL, DEFAULT, LARGE]
            .into_iter()
            .map(|size| {
                d.at_size(size, |d| {
                    let read = d.place_of("Read")?;
                    super::world_build::reader_title(d, title)
                        .map(|rect| rect.top() - read.bottom())
                })
            })
            .collect()
    }

    /// Must stay true: the owner words and every control on the file line share one centre
    /// line, at each captured size.
    fn file_line_centred(d: &mut Driver<'_>, words: &[&str], controls: &[&str], what: &str) {
        for size in [SMALL, DEFAULT, LARGE] {
            let centres = d.at_size(size, |d| {
                let read = d.place_of("Read")?;
                let below = egui::Rect::from_min_max(
                    egui::pos2(read.left() - 1.0, read.bottom()),
                    egui::pos2(f32::INFINITY, f32::INFINITY),
                );
                words
                    .iter()
                    .map(|word| d.painted_rect_in(word, below).map(|rect| rect.center().y))
                    .chain(
                        controls
                            .iter()
                            .map(|name| d.place_of(name).map(|rect| rect.center().y)),
                    )
                    .collect::<Option<Vec<f32>>>()
            });
            let line = centres.as_ref().map(|centres| {
                centres.iter().copied().fold(f32::INFINITY, f32::min)
                    ..=centres.iter().copied().fold(f32::NEG_INFINITY, f32::max)
            });
            d.observe(
                "iter-17-E-FD2-centre",
                line.as_ref()
                    .is_some_and(|line| line.end() - line.start() <= 1.0),
                format!(
                    "The {what} file line keeps its words and controls on one centre line at {size:?}: {centres:?}"
                ),
            );
        }
    }
}

/// The signed-in path. A test session makes the rail, the page, Party and the import list
/// share one state. No request leaves the process: see `crate::hermetic`.
pub mod signed_in {
    use super::*;
    use crate::settings::Account;
    const RAIL_CONNECTED: &str = "Connected";
    const REMOTE: &str = "Add from D&D Beyond";
    const REMOTE_NAME: &str = "Salt Road";
    // Round 24 FK2: no `Cached`; the age says the sheet is the one Splat saved.
    const FRESHNESS: &str = "Connected · Fetched";
    const PAGES: [&str; 5] = ["Actions", "Spells", "Equipment", "Features", "Background"];
    const OVERVIEW: &str = "Overview";
    const MEMBER: &str = "Brannoch Vale";
    const SYNC_SECTION: &str = "Combat sync";
    const SYNC_ON: &str = "Combat sync · On";
    const SYNC_OFF: &str = "Combat sync · Off";
    const RAIL_STOPPED: &str = "Sync stopped";
    const SYNC_TOGGLE: &str = "Sync this sheet with D&D Beyond";
    const STOPPED: &str = "Combat sync stopped";
    const FIRST_READ: &str = "Reading sheet…";
    const SYNC_PARTY: &str = "Sync party";
    const STOP_SYNC: &str = "Stop party sync";
    const LINK_TAB: &str = "Link and folder";
    const BLOCKED_REASON: &str =
        "Close the run to change the folder, the session folder or the link.";
    const EDIT_DETAILS: &str = "Edit details";
    const CONFLICT: &str = "Conflict";
    const RAIL_FAILING: &str = "Sign-in failing";
    const RAIL_OFFLINE: &str = "Signed out";
    const STOPPED_ROW: &str = "Sync stopped";
    const NOT_SENT: &str = "Not sent";
    const NOTHING_TO_SEND: &str = "Iselle Marrowind";
    const CLOSE_RUN: &str = "Close run";
    const STRANDED: &str = "were not sent to D&D Beyond";
    /// The run's close dialog. Its page stays in the tree behind it.
    const CLOSE_DIALOG: [&str; 6] = [
        "Connect to sync",
        "Review conflict",
        "Close and lose changes",
        "Save as a new run and close",
        "Close now",
        "Keep running",
    ];

    pub fn run(d: &mut Driver<'_>) {
        d.app_mut().settings.simulate(Account::Connected);
        d.settle();
        go_settings(d);
        d.observe(
            "T10.3",
            d.visible_text(RAIL_CONNECTED) && d.rail_words().contains(RAIL_CONNECTED),
            "With a session, the page and the rail both say Connected",
        );
        // Settings / account-connected supplies the larger views. Keep this compact view.
        d.state_captured_at("settings-connected", Sizes::All, Sizes::Small);
        d.click(REMOTE);
        d.expect(REMOTE_NAME);
        d.state_prepared("remote-campaign-list", Sizes::Default, |d| {
            if !d.has("Add Salt Road from D&D Beyond") {
                d.click(REMOTE);
            }
            d.row_of(REMOTE_NAME);
            d.wait_for_text(REMOTE_NAME);
        });
        d.goal("see the remote campaign list");
        d.observe(
            "Added campaigns",
            d.has_containing("Added")
                && !d.has("Add The Emberfall Marches from D&D Beyond")
                && !d.has("Add Quiet Harbor from D&D Beyond")
                && d.has("Add Salt Road from D&D Beyond"),
            "An added remote campaign has a status; an available campaign has an Add action",
        );
        d.click("Add Salt Road from D&D Beyond");
        // Round 23 AM2/FM7: quiet Cancel starts the form's last row at the labels' text edge.
        let (cancel, labels) = d.at_size(DEFAULT, |d| {
            (
                d.place_of("Cancel")
                    .and_then(|cancel| words_left_in(d, cancel)),
                [d.painted_left("Campaign name"), d.painted_left("Folder")],
            )
        });
        d.observe(
            "iter-23-C-AM2",
            cancel.is_some_and(|cancel| {
                labels
                    .iter()
                    .all(|label| label.is_some_and(|label| (cancel - label).abs() <= 1.0))
            }),
            format!("Cancel’s words start at {cancel:?}; the field labels at {labels:?}"),
        );
        d.state("import-remote-campaign", Sizes::Default);
        d.observe(
            "T10.3",
            d.has_containing(REMOTE_NAME),
            "Choosing a remote campaign opens its import, with the name filled in",
        );
        d.click("Cancel");
        d.begin_task();

        go_party(d);
        d.click_containing(MEMBER);
        d.state("party-connected", Sizes::All);
        d.observe(
            "T7.2",
            d.visible_text(FRESHNESS),
            "Signed in: the sheet says it is connected, and how old it is",
        );
        d.observe(
            "iter-24-C-FK2",
            d.visible_text(FRESHNESS) && !d.visible_text("Cached"),
            "The Party header says Connected and the sheet's age, without Cached",
        );
        for page in PAGES {
            if !d.in_view(page) {
                assert!(d.in_view("More sheet pages"));
                d.click("More sheet pages");
            }
            assert!(d.in_view(page));
            d.click(page);
            d.observe(
                "T7.2",
                d.visible_text(FRESHNESS),
                format!("Signed in: freshness on {page}"),
            );
        }
        d.click(OVERVIEW);
        d.begin_task();

        // T7.3: sync is on from the time Splat connects, and leaving the page does not stop it.
        // FM3: before the first read, the closed heading says sync waits for it, not `On`.
        let on = d.has_containing("Combat sync · Waiting for the first read")
            && !d.has_containing(SYNC_ON);
        d.click_containing(SYNC_SECTION);
        d.state("party-sync-on", Sizes::All);
        // FM2: the heading and its body are one block below the sheet tabs.
        for size in [SMALL, DEFAULT, LARGE] {
            let places = d.at_size(size, |d| {
                let heading = d
                    .nodes()
                    .into_iter()
                    .find(|node| node.name.contains("Combat sync ·"))
                    .map(|node| node.rect);
                (d.place_of(OVERVIEW), heading, d.place_of(SYNC_TOGGLE))
            });
            d.observe(
                "iter-23-R-FM2",
                matches!(places, (Some(tab), Some(heading), Some(body))
                    if tab.bottom() <= heading.top() && heading.bottom() <= body.top()),
                format!("At {size:?} the tabs come before the Combat sync heading, and its body follows it: {places:?}"),
            );
        }
        // Round 24 review: the stepper wells are measured once per change of the values. The
        // count of value reads lives in `measure_wells`, the count of frames in `wells`.
        let wells = crate::test_support::CachedWork::start("Sync wells", d, |d| {
            let (reached, work) = d.app_mut().party.sync_well_counts();
            crate::test_support::Counts { reached, work }
        });
        // After a read, every value Splat can send has − and + beside it.
        let sheet = serde_json::json!({
            "id": 9100001, "canEdit": true, "removedHitPoints": 4, "temporaryHitPoints": 5,
            "inspiration": true, "bonusHitPoints": 0,
            "deathSaves": {"successCount": 0, "failCount": 0}, "conditions": [],
            "currencies": {"cp": 12, "sp": 30, "ep": 0, "gp": 145, "pp": 2},
            "actions": {"class": [{"id": 1, "entityTypeId": 2, "name": "Second Wind", "limitedUse": {"numberUsed": 1}}]},
            "inventory": [
                {"id": 3, "quantity": 18, "definition": {"name": "Crossbow Bolts"}},
                {"id": 4, "quantity": 1, "limitedUse": {"maxUses": 3, "numberUsed": 1}, "definition": {"name": "Wand of Web"}}
            ],
            "creatures": [{"id": 5, "name": "Grit", "removedHitPoints": 2, "temporaryHitPoints": 0, "definition": {"name": "Mastiff", "averageHitPoints": 5}}],
            "classes": [{"id": 6, "level": 6, "hitDiceUsed": 2, "definition": {"name": "Fighter", "hitDice": 10}}]
        });
        d.app_mut().party.simulate_sheet_read(62, &sheet);
        d.settle();
        d.observe(
            "T7.3",
            d.has("Increase Current HP")
                && d.has("Increase Temporary HP")
                && d.has_containing("Values with − and + are editable"),
            "After a read, editable values have steppers and the panel says what they do",
        );
        d.click_containing("Short rest");
        d.state("party-short-rest", Sizes::Default);
        d.click("Cancel");
        for section in ["Currency", "Inventory", "Companions and summons"] {
            d.click_containing(section);
        }
        // Round 24 FK4: in each block every − shares an x and every + shares an x, whatever the
        // label or the number (Gold 145 beside the one-digit coins).
        let blocks: [&[&str]; 3] = [
            &["Platinum", "Gold", "Electrum", "Silver", "Copper"],
            &[
                "Crossbow Bolts quantity",
                "Wand of Web quantity",
                "Wand of Web charges spent",
            ],
            &["Grit damage taken", "Grit temporary HP"],
        ];
        d.observe_both(
            "iter-24-C-FK4",
            "Each sync block keeps its − and + in one column",
            |d| {
                blocks.iter().all(|names| {
                    ["Decrease", "Increase"].iter().all(|side| {
                        let xs: Vec<_> = names
                            .iter()
                            .map(|name| d.place_of(&format!("{side} {name}")).map(|r| r.left()))
                            .collect();
                        xs.iter().all(Option::is_some)
                            && xs
                                .iter()
                                .flatten()
                                .all(|x| (x - xs[0].unwrap()).abs() < 0.5)
                    })
                })
            },
        );
        // Round 25 review: the sync panel's help keeps the reader measure at every size. A label
        // out of view is not painted, so each is scrolled into view first.
        let base = d.window_size();
        for size in [SMALL, DEFAULT, LARGE] {
            d.set_base(size);
            let sheet_at = d
                .nodes()
                .into_iter()
                .find(|node| node.name.contains("Combat sync"))
                .map(|node| egui::pos2(node.rect.center().x, size.1 * 0.6))
                .unwrap();
            let widths = [
                "are editable. Splat sends a change",
                "Concentration, custom effects",
            ]
            .map(|words| {
                d.scroll_up_at(sheet_at, 30);
                d.scroll_until(sheet_at, words, 60);
                let mut width = None;
                d.each_painted_text(|text, _| {
                    if text.galley.job.text.contains(words) {
                        width = Some(text.galley.size().x);
                    }
                });
                width
            });
            d.observe(
                "iter-25-C-sync-help",
                widths.iter().all(|width| {
                    width.is_some_and(|width| width <= crate::design::READER_MEASURE + 0.5)
                }),
                format!("At {size:?}, the sync help widths are {widths:?}"),
            );
        }
        d.set_base(base);
        let sheet_at = d
            .nodes()
            .into_iter()
            .find(|node| node.name.contains("Combat sync"))
            .map(|node| node.rect.center())
            .unwrap();
        d.scroll_up_at(sheet_at, 30);
        d.state("party-sync-editable", Sizes::All);
        let shown = d.has("Increase Gold") && d.has("Increase Grit temporary HP");
        let result = wells.warm(d, shown, 1);
        d.observe(
            "iter-24-C-FK4-measured",
            result.is_ok(),
            format!("{result:?}"),
        );
        // Unchanged frames and a resize read no values, with the blocks open and with them
        // closed.
        for open in [true, false] {
            if !open {
                for section in ["Currency", "Inventory", "Companions and summons"] {
                    d.click_containing(section);
                }
            }
            let steady = wells.steady(d, |d| {
                d.settle();
                let small = d.at_size(SMALL, |d| d.has("Increase Gold") == open);
                d.settle();
                small && d.has("Increase Gold") == open && d.has_containing(SYNC_SECTION)
            });
            d.observe(
                if open {
                    "iter-24-C-FK4-steady-open"
                } else {
                    "iter-24-C-FK4-steady-closed"
                },
                steady.is_ok(),
                format!("{steady:?}"),
            );
        }
        for section in ["Currency", "Inventory", "Companions and summons"] {
            d.click_containing(section);
        }
        // A new read that widens Gold measures again: every coin's + moves to the new edge.
        let plus = |d: &Driver<'_>, coin: &str| {
            d.place_of(&format!("Increase {coin}"))
                .map(|rect| rect.left())
        };
        let before = plus(d, "Gold");
        let mut wider = sheet.clone();
        wider["currencies"]["gp"] = 14_500.into();
        let refreshed = wells.refresh(d, 1, |d| {
            d.app_mut().party.simulate_sheet_read(62, &wider);
            d.settle();
            let after = plus(d, "Gold");
            before
                .zip(after)
                .is_some_and(|(before, after)| after > before)
                && ["Platinum", "Electrum", "Silver", "Copper"]
                    .iter()
                    .all(|coin| plus(d, coin) == after)
        });
        d.observe(
            "iter-24-C-FK4-refresh",
            refreshed.is_ok(),
            format!("{refreshed:?}"),
        );
        d.app_mut().party.simulate_sheet_read(62, &sheet);
        d.settle();
        go_library(d);
        let rail = !d.visible_text(RAIL_STOPPED);
        go_party(d);
        d.observe(
            "T7.3",
            on && rail && d.has_containing(SYNC_ON) && !d.has_containing(STOPPED),
            format!(
                "Connected, sync is on with no click: {on}. Leaving Party does not stop it, and the rail stays quiet: {rail}"
            ),
        );
        // The DM can turn it off here, and it stays off.
        d.click(SYNC_TOGGLE);
        go_library(d);
        go_party(d);
        d.observe(
            "T7.3",
            d.has_containing(SYNC_OFF) && !d.has_containing(STOPPED),
            "Turned off by the DM, sync stays off and nothing calls that a fault",
        );
        // The panel stays open when sync turns off, so the box is still there.
        d.observe(
            "T7.3",
            d.has(SYNC_TOGGLE),
            "Turning sync off keeps its panel open",
        );
        d.click(SYNC_TOGGLE);
        d.begin_task();

        // A short network drop keeps the session, and says so.
        d.app_mut().settings.simulate(Account::RefreshFailing);
        d.settle();
        go_settings(d);
        let rail = d.rail_words();
        d.observe(
            "T10.3",
            d.visible_text(RAIL_FAILING)
                && rail.contains(RAIL_FAILING)
                && !rail.contains(RAIL_CONNECTED),
            format!("A failed refresh has its own words, on the page and in the rail: {rail}"),
        );
        d.state("refresh-failing", Sizes::Default);
        d.app_mut().settings.simulate(Account::Connected);
        d.settle();
        d.begin_task();

        // At the table: every party row states its sync, and Stop is offered.
        go_encounters(d);
        resume_run(d, RUN);
        d.click(SYNC_PARTY);
        d.state("run-connected", Sizes::All);
        d.observe(
            "T3.3",
            d.row_words(MEMBER).contains(FIRST_READ) && d.has(STOP_SYNC),
            "Signed in: a real click on Sync party puts its first read on the row",
        );

        // The session ends at the table, with a change on its way. The DM must know, and
        // the change must still count until they decide.
        d.begin_task();
        d.app_mut()
            .encounters
            .simulate_sync_state(MEMBER, crate::combat_sync::SyncState::Pending);
        d.settle();
        d.app_mut().encounters.simulate_session_end();
        d.app_mut().settings.simulate_session_expiry();
        d.settle();
        let row = d.row_words(MEMBER);
        let rail = d.rail_words();
        d.observe(
            "T10.3",
            row.contains(NOT_SENT)
                && !d.row_words(NOTHING_TO_SEND).contains(STOPPED_ROW)
                && d.visible_text("Signed out")
                && rail.contains(RAIL_OFFLINE)
                && rail.contains(RAIL_STOPPED),
            format!(
                "A session that ends during a run: the row says {row}, the header says so, and the rail says {rail}"
            ),
        );
        d.state("session-ended-during-run", Sizes::All);
        d.click("Connect to sync");
        d.observe(
            "Sign in recovery",
            d.focused_name().as_deref() == Some("Paste session cookie"),
            "Sign in again opens Settings with the cookie field focused",
        );
        d.state("sign-in-from-run", Sizes::Default);
        d.observe_both(
            "iter-12-R4",
            "Sign-in recovery shows the cookie field without unexplained locked actions",
            |d| {
                d.in_view("Paste session cookie")
                    && (!d.in_view("Edit details")
                        || d.in_view("Close the run to change campaigns."))
            },
        );
        d.scroll_up_at(pos2(800.0, 250.0), 8);
        d.expect("Close the run to change campaigns.");
        assert!(d.in_view("Close the run to change campaigns."));
        d.expect_same("At 1200x800 the campaign lock reason already fits. Scrolling changes only the 960x640 view.");
        d.state_changed_at(
            "campaigns-locked-during-run",
            "sign-in-from-run",
            Sizes::Small,
        );
        d.click(BACK_TO_RUN);
        d.click(CLOSE_RUN);
        d.observe(
            "T10.3",
            d.visible_text(&format!("Changes for {MEMBER} {STRANDED}"))
                && !d.visible_text("Nothing is lost")
                && !d.visible_text("ends Undo"),
            "Closing after that names the changes that were never sent, and promises only what it keeps. A close keeps Undo, so it never says Undo ends",
        );
        d.state("close-with-unsent-changes", Sizes::Default);
        // iter-17-B: Connect to sync starts its line at the dialog's text edge. iter-26-R FK2:
        // it is a bordered button, its frame at that edge.
        check_frame_start(
            d,
            "iter-17-B-connect",
            "Connect to sync",
            STRANDED,
            &[SMALL, DEFAULT, LARGE],
            Some(&CLOSE_DIALOG),
        );
        d.press(Key::Escape);
        // iter-25-S AL1/FL1/SL1 caller: the app's close dialog names the unsent changes and never
        // says that closing ends Undo (a saved run keeps its Undo history).
        d.request_close();
        d.observe(
            "iter-25-S-close-keeps-undo",
            d.has_containing("D&D Beyond has not confirmed changes to")
                && !d.has_containing("Closing ends Undo"),
            "The app's close dialog names the unsent changes and no Undo-ends sentence",
        );
        d.press(Key::Escape);
        // Damage recorded after the session ended. Reconnecting must not take it away.
        d.click_row(MEMBER);
        d.type_text("3");
        d.press(Key::Enter);
        let hp_in_run = d.row_hp(MEMBER);
        d.app_mut().settings.simulate(Account::Connected);
        d.settle();
        let row = d.row_words(MEMBER);
        d.observe(
            "T3.3",
            row.contains(FIRST_READ)
                && !d.visible_text(STOPPED_ROW)
                && !d.rail_words().contains(RAIL_STOPPED),
            format!("After a reconnect, the stopped rows start again with no click: {row}"),
        );
        // The sheet still holds what it held before the outage.
        d.app_mut().encounters.simulate_read(MEMBER, 0);
        d.settle();
        let row = d.row_words(MEMBER);
        d.observe(
            "T3.3",
            row.contains(CONFLICT) && d.row_hp(MEMBER) == hp_in_run,
            format!(
                "Simulated read after a reconnect: the HP recorded during the outage stands ({hp_in_run:?}), and the row asks: {row}"
            ),
        );
        d.state_prepared("reconnect-conflict", Sizes::Default, |d| {
            undo_keeps_a_target(d, &format!("Sync conflict: {MEMBER}"), None);
        });
        // iter11-B2: only a step the sheet made gives up `D&D Beyond`. A DM's own effect with
        // those words keeps them, or Undo shows its word alone.
        const DM_EFFECT: &str = "D&D Beyond boon";
        let dm_undo = format!("Undo: {DM_EFFECT} on {MEMBER}");
        d.click(CONDITIONS);
        d.type_into("Other effect", DM_EFFECT);
        d.click("Add");
        d.expect(&dm_undo);
        let painted = d.at_size(SMALL, |d| {
            d.place_of(&dm_undo)
                .map_or_else(String::new, |rect| d.painted_in(rect))
        });
        d.observe(
            "iter11-B2-source",
            painted.contains("Undo")
                && !painted.contains("Undo: boon")
                && (painted.contains(DM_EFFECT) || !painted.contains("Undo:")),
            format!(
                "At 960x640, beside the conflict, the DM's effect keeps its words: {painted:?}"
            ),
        );
        d.click(&dm_undo);
        d.expect(&format!("Undo: 3 damage to {MEMBER}"));
        d.click("Sheet");
        d.begin_task();

        // The link cannot change under a run.
        d.click(STOP_SYNC);
        go_campaign(d);
        d.click(LINK_TAB);
        d.observe(
            "Locked link",
            d.visible_text(BLOCKED_REASON)
                && d.is_disabled(EDIT_DETAILS)
                && d.is_disabled("Folder"),
            "With a run open, the link tab states why it is locked, and its controls are",
        );
        d.state("link-tab-during-run", Sizes::Default);
        d.begin_task();

        // iter-17-B: the close dialog with a conflict. Signed out, Review conflict follows
        // Connect to sync on one line, and neither press area overlaps the other. A conflict
        // that the reconnect brings while the dialog is open leaves Review conflict alone,
        // first on its line.
        const REVIEW: &str = "Review conflict";
        const OTHER: &str = "Sable Harrowgate";
        const CHOOSE: &str = "Choose which HP to keep";
        d.click(BACK_TO_RUN);
        d.app_mut()
            .encounters
            .simulate_sync_state(MEMBER, crate::combat_sync::SyncState::Pending);
        d.settle();
        d.app_mut().encounters.simulate_session_end();
        d.app_mut().settings.simulate_session_expiry();
        d.settle();
        d.click(CLOSE_RUN);
        d.app_mut().encounters.simulate_sync_conflict(OTHER);
        d.settle();
        for size in [SMALL, DEFAULT, LARGE] {
            let pair = d.at_size(size, |d| {
                let connect = line_and_action_below(d, CHOOSE, "Connect to sync");
                let review = line_and_action_below(d, CHOOSE, REVIEW);
                let clear = review.as_ref().map(|(_, review)| {
                    press_area_clear(d, review.shown, REVIEW, Some(&CLOSE_DIALOG))
                });
                (
                    connect
                        .map(|(line, node)| (line.left(), node.shown, words_left_in(d, node.rect))),
                    review.map(|(_, node)| node.shown),
                    clear,
                )
            });
            // iter-26-R FK2: Connect to sync is bordered, so its frame (not its words) starts
            // at the dialog text, and its words sit inside the frame.
            d.observe(
                "iter-17-B-review-beside",
                matches!(
                    pair,
                    (Some((edge, connect, Some(words))), Some(review), Some(Ok(_)))
                        if (connect.center().y - review.center().y).abs() < 1.0
                            && review.left() >= connect.right()
                            && (connect.left() - edge).abs() <= 1.0
                            && words > connect.left() + 1.0
                ),
                format!("At {size:?}, Connect to sync starts the line and Review conflict follows it, apart: {pair:?}"),
            );
        }
        // iter-23-S AM1: the close is blocked, so the words do not offer it, and the dialog
        // shows why in view, not only on hover.
        let blocked_words = |d: &Driver<'_>| {
            d.is_disabled("Close now")
                && d.visible_text("Choose which HP to keep before you close.")
                // iter-25-R FL4: the cause is the ended view's one exact sentence, not the
                // general three-cause line.
                && d.visible_text("Settle 1 conflict first.")
                && !d.visible_text(crate::encounter_run::CLOSE_BLOCKED)
                && !d.visible_text("or close and choose next time")
        };
        d.observe(
            "iter-23-S-AM1-signed-out",
            blocked_words(d),
            "Signed out, a blocked Close now is not offered in the words, and its reason shows",
        );
        d.state("close-with-conflict-signed-out", Sizes::Default);
        d.app_mut().settings.simulate(Account::Connected);
        d.settle();
        d.observe(
            "iter-17-B-review-alone",
            line_and_action_below(d, CHOOSE, REVIEW).is_some()
                && line_and_action_below(d, CHOOSE, "Connect to sync").is_none(),
            "Signed in again with the dialog open, Review conflict is the only action on its line",
        );
        check_frame_start(
            d,
            "iter-17-B-review",
            REVIEW,
            CHOOSE,
            &[SMALL, DEFAULT, LARGE],
            Some(&CLOSE_DIALOG),
        );
        d.observe(
            "iter-23-S-AM1-signed-in",
            blocked_words(d),
            "Signed in, a blocked Close now is not offered in the words, and its reason shows",
        );
        d.state("close-with-conflict-signed-in", Sizes::Default);
        d.press(Key::Escape);
    }
}

/// G2: section 3's table, one row at a time. Each row sets state, leaves, and comes back.
pub mod nav_table {
    use super::*;
    const EVERYTHING: &str = "Everything";
    const SECTION: &str = "Creatures";
    const ENTRY_ROW: &str = "Aboleth · ";
    const ENTRY: &str = "Aboleth";
    const LOOKUP_QUERY: &str = "fireball";
    const LOOKUP_ENTRY: &str = "Fireball";
    const SELECTED: &str = "Cinder Wolf 7";
    const UNDO: &str = "Undo: next turn";
    const MEMBER: &str = "Iselle Marrowind";
    const SHEET_PAGE: &str = "Spells";
    const FILTER: &str = "Filter this page";
    const FILTER_TEXT: &str = "fire";
    const NOTE_ROW: &str = "Ash Exposure · ";
    const DRAFT_WORDS: &str = "Kept across pages. ";
    const DISCARD: &str = "Discard";
    const DRAFT_NAME: &str = "Deep Ford";
    const FIRST_SLOT: &str = "1. Brannoch Vale";
    const LINKED_QUERY: &str = "bard subclass";
    const LINKED_ROW: &str = "Level 3: Bard Subclass";
    const LINK: &str = "College of Lore";
    const LINKED_TITLE: &str = "Bard Subclass";
    const SYNC_PARTY: &str = "Sync party";
    const STOP_SYNC: &str = "Stop party sync";
    const FIRST_READ: &str = "Reading sheet…";
    const ACTIVE: &str = "Active campaign: The Emberfall Marches";
    const BACK_TO_RUN: &str = "Back to run";
    const ENCOUNTER: &str = "Shrine of Cinders";

    pub fn run(d: &mut Driver<'_>) {
        // T4.4 with more than one unfinished run: each row names its encounter.
        let nodes = d.nodes();
        let named = [RUN, RUN_12, RUN_20].iter().all(|run| {
            let name = nodes.iter().position(|node| node.name == *run);
            let row = nodes
                .iter()
                .position(|node| node.name.starts_with(&format!("{run} · ")));
            matches!((name, row), (Some(name), Some(row)) if name < row)
        });
        d.observe(
            "T4.4",
            named,
            "Three unfinished runs, each with its encounter's name on its row",
        );

        // Open run first, because its strip moves every other page down.
        resume_run(d, RUN_20);

        // Library: scope, section, entry, scroll.
        go_library(d);
        d.open_combo_at(LIBRARY_PACKAGE_DROPDOWN);
        d.click(EVERYTHING);
        d.click("Section");
        d.click(&format!("Library section: {SECTION}"));
        d.expect(ENTRY_ROW);
        d.scroll_at(pos2(720.0, 400.0), 2);
        let entry_scroll = d.scroll_position_of_reader(ENTRY);
        d.begin_task();

        // Open run: selection, amount, scroll, undo, and sync.
        d.click(BACK_TO_RUN);
        d.app_mut().encounters.simulate_connection();
        d.settle();
        d.click(SYNC_PARTY);
        for _ in 0..8 {
            d.click(NEXT_TURN);
        }
        d.click_row(SELECTED);
        d.type_text("7");
        d.begin_task();

        // Party: member, page, filter.
        go_party(d);
        d.click_containing(MEMBER);
        d.click(SHEET_PAGE);
        d.type_into(FILTER, FILTER_TEXT);
        d.begin_task();

        // Lookup: last query and entry, opened over the run.
        d.click(BACK_TO_RUN);
        look_up(d, LOOKUP_QUERY);
        d.expect(LOOKUP_ENTRY);
        // Measured with Lookup open, because it changes the roster's width.
        let run_y = d.row_node(SELECTED).map(|row| row.rect.min.y);

        // Away, then back to each.
        go_settings(d);
        d.click(BACK_TO_RUN);
        let amount = d.field_text("Amount");
        let selected = participant_is_selected(d, SELECTED);
        let row_y = d.row_node(SELECTED).map(|row| row.rect.min.y);
        let undo = d.has_containing(UNDO);
        d.observe(
            "G2",
            amount.as_deref() == Some("7") && selected && row_y == run_y && undo,
            format!("Run: amount {amount:?}, selected {selected}, row {row_y:?} was {run_y:?}, undo {undo}"),
        );
        d.observe(
            "G2",
            d.row_node("Brannoch Vale")
                .is_some_and(|row| row.name.contains(FIRST_READ)),
            "Run: sync stays on across pages",
        );
        d.observe(
            "G2",
            d.field_text(SEARCH_LOOKUP).as_deref() == Some(LOOKUP_QUERY)
                && d.has_label(LOOKUP_ENTRY),
            "Lookup: the query and the entry are as they were",
        );
        d.press(Key::Escape);

        go_library(d);
        let scope = d.visible_text(EVERYTHING);
        let entry = d.has_label(ENTRY);
        let scroll = d.scroll_position_of_reader(ENTRY);
        d.observe(
            "G2",
            scope && entry && scroll == entry_scroll && entry_scroll.is_some_and(|y| y < 300.0),
            format!(
                "Library: scope {scope}, entry {entry}, scroll {scroll:?} was {entry_scroll:?}"
            ),
        );
        go_party(d);
        d.observe(
            "G2",
            d.has_label(MEMBER) && d.field_text(FILTER).as_deref() == Some(FILTER_TEXT),
            "Party: member, page and filter are as they were",
        );
        d.observe(
            "G2",
            d.has(ACTIVE),
            "Active campaign: the selector still names it",
        );

        // Reading history.
        go_library(d);
        search_library(d, LINKED_QUERY);
        d.click_containing(LINKED_ROW);
        d.click(LINK);
        go_settings(d);
        go_library(d);
        d.press_alt(Key::ArrowLeft);
        d.observe(
            "G2",
            d.in_view(LINKED_TITLE),
            "Reading history: Alt+Left still returns to the entry before the link",
        );
        d.press_side_button(egui::PointerButton::Extra2, LINKED_TITLE);
        let forward = !d.in_view(LINKED_TITLE) && d.in_view(LINK);
        d.press_side_button(egui::PointerButton::Extra1, LINK);
        d.observe(
            "G2",
            forward && d.in_view(LINKED_TITLE),
            "Reading history: the mouse's Forward and Back buttons step through it",
        );
        d.click(CLEAR_SEARCH);
        d.begin_task();

        // Close the run, to reach the two drafts.
        d.click(BACK_TO_RUN);
        d.click(STOP_SYNC);
        close_run(d);

        // Library draft.
        go_library(d);
        open_local_package(d, MODULE);
        d.click_starting(NOTE_ROW);
        d.click(EDIT_MARKDOWN);
        d.type_into_editor(DRAFT_WORDS);
        go_settings(d);
        go_library(d);
        d.observe(
            "G2",
            d.editor_text()
                .is_some_and(|text| text.contains(DRAFT_WORDS)),
            "Library draft: its words are as they were",
        );
        d.click(DISCARD);

        // Encounter draft.
        go_encounters(d);
        d.click(NEW_ENCOUNTER);
        d.type_text(DRAFT_NAME);
        go_settings(d);
        go_encounters(d);
        d.observe(
            "G2",
            d.field_text(&format!("Encounter name: {DRAFT_NAME}"))
                .as_deref()
                == Some(DRAFT_NAME)
                && d.has_label(FIRST_SLOT),
            "Encounter draft: its name and rows are as they were",
        );
        d.click("Discard");
        d.goal("walk section 3's table");

        // G3: every control in the rail goes somewhere of its own.
        go_library(d);
        let mut titles = Vec::new();
        for name in d.rail_page_buttons() {
            d.click(&name);
            let title = d.page_title();
            if name.starts_with("Run · ") {
                close_run(d);
            }
            if name.starts_with("Paused runs · ") {
                d.observe(
                    "G3",
                    title.as_deref() == Some("Encounters"),
                    "Several paused runs open the Encounters list",
                );
            } else if name == crate::no_run_name() {
                d.observe(
                    "G3",
                    title.as_deref() == Some("Encounters"),
                    "The empty Run slot opens the Encounters list",
                );
            } else {
                titles.push((name, title));
            }
        }
        let mut distinct: Vec<&Option<String>> = titles.iter().map(|(_, title)| title).collect();
        distinct.sort();
        distinct.dedup();
        d.observe(
            "G3",
            titles.len() >= 5
                && distinct.len() == titles.len()
                && titles.iter().all(|(_, t)| t.is_some()),
            format!("Each control in the rail opens a page with its own title: {titles:?}"),
        );

        // G1: a disabled control says why, on the pages that have the most of them.
        go_encounters(d);
        d.click(NEW_ENCOUNTER);
        let mut silent = d.disabled_without_reason();
        d.click("Discard");
        go_settings(d);
        silent.extend(d.disabled_without_reason());
        go_campaign(d);
        silent.extend(d.disabled_without_reason());
        go_party(d);
        silent.extend(d.disabled_without_reason());
        go_library(d);
        search_library(d, &NOTE_ROW.replace(" · ", "").to_lowercase());
        d.click_containing(NOTE_ROW);
        silent.extend(d.disabled_without_reason());
        // The run and everything it locks: the footer, the Encounters list, the selector.
        go_encounters(d);
        resume_run(d, RUN);
        silent.extend(d.disabled_without_reason());
        go_encounters(d);
        silent.extend(d.disabled_without_reason());
        go_settings(d);
        silent.extend(d.disabled_without_reason());
        // The run's tabs, then a fresh run's setup and a finished run's end.
        d.click(BACK_TO_RUN);
        for tab in ["Conditions", "Correct", "Log", "Prep"] {
            if ["Log", "Prep"].contains(&tab) {
                fight_tab(d, tab);
            } else {
                d.click(tab);
            }
            silent.extend(d.disabled_without_reason());
        }
        d.click("End encounter…");
        d.click("End encounter");
        silent.extend(d.disabled_without_reason());
        d.click("Close run");
        run_saved_encounter(d, ENCOUNTER);
        silent.extend(d.disabled_without_reason());
        close_run(d);
        // A Library draft, and a dialog above it.
        go_library(d);
        d.click_containing(NOTE_ROW);
        d.click("Edit Markdown");
        d.type_into_editor(DRAFT_WORDS);
        silent.extend(d.disabled_without_reason());
        d.click(DISCARD);
        let behind: Vec<String> = d.nodes().into_iter().map(|node| node.name).collect();
        d.click("Copy to…");
        silent.extend(d.disabled_without_reason_except(&behind));
        d.press(Key::Escape);
        d.observe(
            "G1",
            silent.is_empty(),
            format!(
                "Disabled controls with no reason, in words, on hover, across 15 views: {silent:?}"
            ),
        );
    }
}

/// World notes, roll tables, and an NPC made at the table.
pub mod world {
    use super::*;
    use egui::accesskit::Role;

    const NPC: &str = "Ashori Deepcut";

    pub fn run(d: &mut Driver<'_>) {
        d.click(NAV_WORLD);
        d.expect("Search the world");
        d.state("world-list", Sizes::All);
        d.check_tab_order("world-list");

        d.click("New ▾");
        // Round 23 FG4: `…` marks each choice that asks for a name first. Generate NPC makes its
        // card at once, so it has no mark.
        d.observe(
            "iter-23-D-FG4",
            ["Note…", "NPC…", "Location…", "Generate NPC"]
                .iter()
                .all(|name| d.in_view(name))
                && !d.has("NPC")
                && !d.has("Location")
                && !d.has("Generate NPC…"),
            "The New menu marks Note…, NPC… and Location…, and not Generate NPC",
        );
        d.state("world-new-menu", Sizes::Default);
        d.click("NPC…");
        d.expect("Type a name first.");
        d.type_into("Name", "Watchkeeper");
        d.wait_for_text("Will create NPCs/Watchkeeper.md");
        d.state("world-new-npc-form", Sizes::All);
        d.click("Create NPC");
        d.expect("Watchkeeper");
        d.observe(
            "World creation",
            !d.has("Create NPC") && d.has_containing("Watchkeeper"),
            "Creating a World note closes the form and opens the note",
        );
        for size in [SMALL, DEFAULT, LARGE] {
            let clear = d.at_size(size, |d| {
                d.expect("NPC · Campaign");
                d.expect("Status");
                d.visible_text("alive") && !d.visible_text("NPC · alive · Campaign")
            });
            d.observe(
                "iter-6-D-FD8",
                clear,
                format!("The full NPC reader shows status once at {size:?}"),
            );
        }
        d.state("world-new-note-created", Sizes::All);

        d.click("Setting · ");
        d.observe(
            "T14.1",
            d.visible_text("A cold war around an inner sea."),
            "A plain note reads in Splat, with no change to the file",
        );
        d.click_containing("The home country");
        d.observe(
            "T14.2",
            d.visible_text("No ruler, only a law code."),
            "An Obsidian link in a note opens the note it names",
        );

        d.click_containing("Gate Mechanics");
        d.click("Roll 1d20");
        d.observe(
            "T14.3",
            d.visible_text("Rolled ") || d.has_containing("Rolled "),
            "A marked table rolls, and its row shows",
        );
        d.observe(
            "World table header",
            d.visible_text("d20"),
            "The roll table names its first column",
        );
        d.state("world-table-rolled", Sizes::All);

        d.click(NAV_ENCOUNTERS);
        resume_run_from_cold_start(d);
        d.begin_task();
        look_up(d, "reliability");
        d.goal_within("T14.4", "find a rule by its heading, beside a run", 2);
        d.observe(
            "T14.4",
            d.visible_text("A field gate fails on a low roll."),
            "A heading's result opens its note at that heading",
        );
        // Round 24 FD4: a plain World note has `Open in World` too, and it opens the note there.
        d.observe(
            "iter-24-D-FD4",
            d.has("Open in World") && !d.has("Open in Library"),
            "Lookup offers Open in World for the plain note Gate Mechanics",
        );
        d.state("world-heading-beside-run", Sizes::All);
        d.click("Open in World");
        d.settle();
        d.observe(
            "iter-24-D-FD4-opens",
            d.current_page() == "World" && d.visible_text("A field gate fails on a low roll."),
            "Open in World shows Gate Mechanics on the World page",
        );
        d.click("Back to run");
        look_up(d, "reliability");
        d.expect("Clear");

        d.click("Clear");
        d.click("Generate NPC");
        d.expect("Save NPC");
        d.observe(
            "T14.7",
            d.has_containing("Look: ")
                && d.has_containing("Manner: ")
                && d.has_containing("Want: "),
            "The generator gives a name and three attributes, and writes nothing yet",
        );
        d.observe(
            "iter-14-D-FD5",
            d.visible_text(crate::lookup::BUILT_IN_NAMES)
                && d.has(crate::lookup::USE_OWN_NAMES)
                && !d.visible_text("splat:rolls"),
            "Lookup beside a run says Built-in names. with a link, not the table marker",
        );
        d.state("world-generated-npc", Sizes::All);
        d.click("Reroll");
        d.click("Save NPC");
        d.settle();
        d.observe(
            "T14.7",
            d.has("Add to fight") && !d.has("Save NPC"),
            "A saved NPC opens as a note, ready to join the fight",
        );
        d.state("world-generated-npc-saved", Sizes::Default);

        look_up_again(d, NPC);
        d.click(&format!("New NPC “{NPC}”"));
        d.settle();
        d.expect("Add to fight");
        d.expect("NPC · alive · Campaign");
        d.state("world-new-npc", Sizes::Default);
        d.click("Add to fight");
        d.type_into("Stat block", "Guard");
        d.click("Guard");
        d.type_into("Initiative", "11");
        d.expect("Side");
        d.wait_for_text("Enemies");
        for size in [SMALL, DEFAULT, LARGE] {
            let (words, side, width, change) = d.at_size(size, |d| {
                d.expect("Side");
                d.expect("Initiative");
                d.expect("Change");
                d.expect("Add to fight");
                let label = d
                    .nodes()
                    .into_iter()
                    .find(|n| n.name == "Side" && n.role == Role::Label);
                (
                    d.visible_text(&format!("Add {NPC} to the fight")) && !d.has("Add to run"),
                    label.is_some_and(|label| {
                        d.painted_size(label.rect, "Side")
                            == Some(crate::design::Type::Small.size())
                    }),
                    d.nodes()
                        .into_iter()
                        .find(|n| n.name == "Initiative" && n.role == Role::TextInput)
                        .is_some_and(|field| field.rect.width() <= crate::design::L * 5.0 + 1.0),
                    d.nodes()
                        .into_iter()
                        .any(|n| n.name == "Change" && n.role == Role::Button)
                        && d.control_frame("Change")
                            .is_some_and(|frame| frame.stroke.width == 0.0),
                )
            });
            for (id, passed) in [
                ("FT2", words),
                ("FD5-label", side),
                ("FD5-width", width),
                ("FD5-change", change),
            ] {
                d.observe(
                    &format!("iter-6-D-{id}"),
                    passed,
                    format!("NPC Add form: {id} at {size:?}"),
                );
            }
        }
        d.state("npc-add-to-run-form", Sizes::All);
        d.expect("Add to fight");
        let add_buttons = |d: &Driver<'_>| {
            d.nodes()
                .into_iter()
                .filter(|n| n.name == "Add to fight" && n.role == Role::Button)
                .count()
        };
        let before_join = add_buttons(d);
        d.click("Add to fight");
        d.settle();
        d.observe(
            "T14.5",
            d.has_containing(NPC) && d.has(CLOSE_RUN),
            "A new NPC joins the fight, and the run stays on screen",
        );
        d.expect("Stat block: not set → Guard.");
        d.expect("Undo stat block change");
        let (success, detail) =
            world_build::finished_change_tone(d, "Stat block: not set → Guard.");
        d.observe("iter-20-D-FD5-stat-block", success, detail);
        d.state("world-npc-in-run", Sizes::All);
        d.observe(
            "NPC arrival",
            // The NPC's own Add to fight goes; any other (the run header's) stays.
            d.has_containing("In the run · initiative 11")
                && before_join >= 1
                && add_buttons(d) == before_join - 1,
            "The NPC's panel links to their row after they join",
        );
        d.click("In the run · initiative 11");
        d.observe(
            "NPC arrival",
            participant_is_selected(d, NPC),
            "The status selects the NPC's row",
        );
        d.state("npc-selected-from-status", Sizes::Default);
    }
}

/// W11: notes, templates and a timeline that stands apart from them.
pub mod session_notes {
    use super::*;

    const TITLE: &str = "Ash on the Road";
    const LONG_TITLE: &str =
        "The Long Road from Emberfall Through the Ash Marshes to the Drowned Mill";
    const UNDO_NEXT_TURN: &str = "Undo: next turn";
    const ADD_DAY: &str = "Add Day";
    const ADD_LONG_REST: &str = "Add Long rest";
    const ADD_SHORT_REST: &str = "Add Short rest";
    const ADD_TRAVEL: &str = "Add Travel";
    const ADD_EVENT: &str = "Add Major event";

    pub fn run(d: &mut Driver<'_>) {
        d.click(NAV_SESSIONS);
        d.expect("No session notes yet");
        // Round 24 FF7: the empty state says what a session gives, not how it is stored.
        d.expect("Start a session from a template. It holds tonight’s plan and your notes.");
        d.state("sessions-empty", Sizes::Default);
        d.check_tab_order("sessions-empty");

        d.begin_task();
        d.click("New session");
        d.state("session-form", Sizes::Default);
        d.type_into("Session title", TITLE);
        d.click("Create session");
        d.expect(&format!("Session 1: {TITLE}"));
        d.goal_within("T11.1", "start a titled session from a template", 4);
        d.state("session-write", Sizes::AllAndHiDpi);
        let filled = d
            .editor_text()
            .is_some_and(|text| text.contains("Recap of last time") && text.contains(TITLE));
        d.observe(
            "T11.1",
            filled,
            "The new note holds the template, filled in",
        );

        d.state_prepared("empty-note-read", Sizes::All, |d| {
            read_note(d);
            let writes = d.nodes().iter().filter(|node| node.name == "Write").count();
            d.observe(
                "iter-3-C-ST3-empty",
                writes == 1 && d.visible_text("Nothing written yet."),
                "An empty Read view has one inline Write action",
            );
        });
        write_note(d);
        let note_top = |d: &Driver<'_>| {
            d.nodes()
                .into_iter()
                .find(|node| node.name == "Note text" && node.in_view())
                .map(|node| node.shown.top())
        };
        let note_before = [SMALL, DEFAULT, LARGE].map(|size| d.at_size(size, note_top));
        d.type_into_editor("\nThe road was covered in ash, and the wolves followed us.\n");
        let draft = d.editor_text().unwrap();
        let long_draft = (0..60)
            .map(|at| format!("Unsent line {at}.\n"))
            .collect::<String>();
        d.type_into_editor(&long_draft);
        let unsent = d.editor_text().unwrap();
        let focus = d.focused_name();
        assert!(focus.is_some());
        assert!(d.visible_passage("Unsent line 59."));
        assert!(!d.visible_passage("Unsent line 0."));
        // iter-25-S lead ruling: a notice that arrives on its own (here, processing finished)
        // sits in the bottom banner, so the note the DM types in keeps its text, its focus and
        // its place at every size.
        assert!(!d.has_containing("is processed."));
        d.app_mut().sessions.simulate_job_finished();
        d.settle();
        let processed = format!("“Session 1: {TITLE}” is processed.");
        assert!(d.has_containing(&processed));
        let kept_parts = (
            d.note_editor_view().0,
            d.editor_text().as_deref() == Some(unsent.as_str()),
            d.visible_passage("Unsent line 59."),
        );
        let kept_typing = kept_parts.0 && kept_parts.1 && kept_parts.2;
        let note_after = [SMALL, DEFAULT, LARGE].map(|size| {
            d.at_size(size, |d| {
                let banner = d
                    .nodes()
                    .into_iter()
                    .find(|node| node.name.starts_with(&processed) && node.in_view())
                    .map(|node| node.rect);
                let note = d
                    .nodes()
                    .into_iter()
                    .find(|node| node.name == "Note text" && node.in_view())
                    .map(|node| node.shown);
                (
                    note.map(|note| note.top()),
                    banner
                        .zip(note)
                        .is_some_and(|(banner, note)| banner.top() >= note.bottom()),
                )
            })
        });
        let kept = kept_typing
            && note_before
                .iter()
                .zip(&note_after)
                .all(|(before, (after, below))| {
                    *below && before.zip(*after).is_some_and(|(a, b)| (a - b).abs() < 1.0)
                });
        d.observe(
            "iter-25-S-notice-banner",
            kept,
            format!(
                "Focus, text and last line in view {kept_parts:?}; the note's y {note_before:?} then {note_after:?} (banner below it)"
            ),
        );
        d.click("Dismiss");
        assert!(!d.has_containing("is processed."));
        let (wide_text, wide_state) = d.at_size(LARGE, |d| {
            eprintln!(
                "draft breakpoint large: words {}, focus {:?} -> {:?}, end {}, start {}",
                d.editor_text().as_ref() == Some(&unsent),
                focus,
                d.focused_name(),
                d.visible_passage("Unsent line 59."),
                d.visible_passage("Unsent line 0.")
            );
            (
                d.editor_text().as_ref() == Some(&unsent),
                d.focused_name() == focus
                    && d.visible_passage("Unsent line 59.")
                    && !d.visible_passage("Unsent line 0."),
            )
        });
        eprintln!(
            "draft breakpoint small: words {}, focus {:?} -> {:?}, end {}, start {}",
            d.editor_text().as_ref() == Some(&unsent),
            focus,
            d.focused_name(),
            d.visible_passage("Unsent line 59."),
            d.visible_passage("Unsent line 0.")
        );
        d.observe("iter-5-A-draft-breakpoint", wide_text && d.editor_text().as_ref() == Some(&unsent),
            "The unsent note keeps its words through SMALL to LARGE to SMALL without reopening Write");
        if !wide_state
            || d.focused_name() != focus
            || !d.visible_passage("Unsent line 59.")
            || d.visible_passage("Unsent line 0.")
        {
            d.outcome.look_review.push("Known Sessions fault, assigned to package C by the lead: crossing the note/plan breakpoint keeps the draft but loses focus and the wide view's scroll. Package A does not change the editor parent or IDs; see focused-2.log.".into());
        }
        d.replace_editor_text(&draft);
        d.press_ctrl(Key::S);
        d.expect("Notes saved");
        // Round 24 FL5: a note just written says `Notes saved` in the same muted colour as a
        // note opened unchanged, here and in the Notes panel beside a run.
        let saved = format!("{} Notes saved", crate::design::icons::OK);
        d.observe(
            "iter-24-S-FL5",
            d.painted_color(&saved) == Some(crate::design::colors().document.clean),
            format!(
                "A written note's `Notes saved` is muted: {:?}",
                d.painted_color(&saved)
            ),
        );
        d.state("session-written", Sizes::Default);

        read_note(d);
        d.state("session-read", Sizes::All);

        // A rest, on the session's own page, with no step away from it.
        assert!(d.in_view("Write"), "The narrow view starts on the note");
        show_plan(d);
        d.wait_for_text("Nothing planned yet");
        d.expect("Use Add to plan a fight, a group, or an event.");
        // Round 22 FT3: an empty-state title has no full stop, as `No session notes yet` has none.
        d.observe(
            "iter-22-C-FT3",
            d.in_view("Nothing planned yet") && !d.visible_text("Nothing planned yet."),
            "The empty plan's title has no full stop",
        );
        assert!(d.has("Add a group"));
        assert!(
            !d.in_view("Write"),
            "Plan replaces the note in the narrow view"
        );
        d.state_changed_at_prepared("empty-plan-add-at-top", "session-read", Sizes::Small, |d| {
            let names = [
                "Add a fight",
                "Add a group",
                ADD_LONG_REST,
                "More Add actions",
            ];
            let nodes = d.nodes();
            let controls: Vec<_> = names
                .iter()
                .map(|name| {
                    nodes
                        .iter()
                        .find(|node| node.name == *name && node.fully_in_view())
                        .unwrap_or_else(|| panic!("Visible plan action: {name}"))
                })
                .collect();
            d.observe(
                "iter-5-A-FT5",
                controls
                    .iter()
                    .all(|node| (node.rect.top() - controls[0].rect.top()).abs() < 1.0)
                    && d.visible_text(&format!(
                        "{}{}",
                        crate::design::icons::EXPAND,
                        crate::design::icons::DOWN
                    )),
                "Fight, Group, Long rest and More keep one line in the plan at every capture size",
            );
        });
        d.state_prepared("session-plan-fight-picker", Sizes::Default, |d| {
            if !d.has("Plan Bridge Ambush") {
                d.click("Add a fight");
            }
            d.wait_for_text("Bridge Ambush");
            // FT3: with temporary HP and a Bloodied player, AC stays at 1200 and names stay at
            // every size. Portraits give way before AC.
            let (items, painted) = session_recording::party_bar_items(d);
            let (width, _) = d.window_size();
            session_recording::check_party_names(
                d,
                &format!("iter-12-C-FT3-names-{width}"),
                &items,
                &painted,
            );
            let has = |words: &str, label: &str| words.split_whitespace().any(|word| word == label);
            let hurt = items
                .iter()
                .filter(|item| item.name.contains("Temp HP"))
                .count();
            let bloodied = items
                .iter()
                .filter(|item| item.name.contains("Bloodied"))
                .count();
            if width >= crate::visual_tests::driver::DEFAULT.0 {
                d.observe(
                    "iter-12-C-FT3",
                    items.len() >= 4
                        && hurt >= 2
                        && bloodied >= 1
                        && painted.iter().all(|(_, words)| has(words, "AC")),
                    format!(
                        "At {width}, with {hurt} temporary HP and {bloodied} Bloodied, every item \
                         shows AC: {painted:?}"
                    ),
                );
            }
        });
        d.press(Key::Escape);
        d.state_prepared("plan-more-add-actions", Sizes::All, |d| {
            d.press(Key::Escape);
            // Long rest is a visible button, one click away, before More opens.
            d.expect(ADD_LONG_REST);
            assert!(
                d.nodes()
                    .iter()
                    .any(|node| node.name == "More Add actions" && node.fully_in_view())
            );
            d.click("More Add actions");
            d.expect(ADD_SHORT_REST);
            d.expect(ADD_LONG_REST);
        });
        d.press(Key::Escape);
        d.begin_task();
        assert!(
            d.nodes()
                .iter()
                .any(|node| node.name == ADD_LONG_REST && node.fully_in_view())
        );
        d.click(ADD_LONG_REST);
        d.wait_for_text("Long rest added to Session 1. Everyone starts the next fight at full.");
        d.goal_within("T11.2", "put a long rest on the timeline", 2);
        d.expect("Long rest added to Session 1. Everyone starts the next fight at full.");
        d.state("session-event-added", Sizes::Default);
        d.click_add_event(ADD_SHORT_REST);
        d.observe(
            "T11.2",
            d.has_containing("Short rest added to Session 1."),
            "An event added here says so, in words",
        );
        d.click("Undo add");
        d.observe(
            "T11.2",
            !d.has("Short rest") && d.has_containing("Removed"),
            "A mis-press has one step of undo, and that has one too",
        );
        d.click("Undo remove");
        d.expect("Short rest");
        d.click("Short rest");
        d.click("More actions for Short rest");
        d.click("Remove this event");
        d.click(NAV_SESSIONS);
        d.click_add_event(ADD_TRAVEL);
        d.observe(
            "iter-2-S2",
            !d.has("Title"),
            "A new plan event stays closed",
        );
        d.click("Travel");
        d.observe(
            "iter-2-S2",
            d.field_text("Title").as_deref() == Some("Travel")
                && !d.has("How long")
                && !d.has("Where"),
            "An opened plan event has its kind as Title and keeps duration and place on Timeline",
        );
        // F9 (section 2.5, row 12): a campaign with no marked table has no checks to offer, so
        // the Travel row's menu is as before.
        d.click("More actions for Travel");
        d.observe(
            "T11.10",
            d.has("Remove this event") && !d.has("Roll for encounters"),
            "With no marked table, a Travel row's menu has no Roll for encounters",
        );
        d.press(Key::Escape);
        d.observe(
            "S13",
            d.has("Travel"),
            "A new event starts with its kind as its title",
        );
        d.state("plan-event-title", Sizes::Default);
        d.click_add_event(ADD_EVENT);
        d.state("session-events", Sizes::Default);
        d.click("Add to notes: Long rest");
        d.expect("In notes");
        d.observe(
            "T11.2",
            !d.has("Undo"),
            "Undo goes at the next thing the DM does, so it cannot undo the wrong event",
        );
        d.state_prepared("session-event-in-notes", Sizes::Default, |d| {
            write_note(d);
            d.expect("Note text");
        });
        write_note(d);
        let linked = d.editor_text().is_some_and(|text| {
            let what = text.find("## What happened");
            let next = text.find("## People and places");
            let link = text.find("[[Timeline#^");
            matches!((what, link, next), (Some(a), Some(b), Some(c)) if a < b && b < c)
        });
        d.observe(
            "T11.4",
            linked,
            "An event's link goes under What happened in the note",
        );
        d.press_ctrl(Key::S);
        read_note(d);
        d.observe(
            "S2",
            !d.has("ON THE TIMELINE") && !d.has("LINKED IN THIS NOTE"),
            "The note has inline links, with no repeated lists",
        );
        d.state_prepared("session-read-linked", Sizes::Default, |d| {
            read_note(d);
            d.wait_for_text("Write");
            d.wait_for_text("What happened");
        });
        let reads_as_words = !d.visible_text("[[Timeline");
        d.observe(
            "T11.4",
            reads_as_words,
            "The Reader shows the link as its words",
        );

        // The timeline is a place of its own, with every kind of event.
        d.click(NAV_TIMELINE);
        d.expect("Long rest");
        d.expect("Everyone at full");
        d.expect("Adds to");
        assert!(!d.has("Adds to:"));
        d.state_prepared("timeline-session-events", Sizes::All, |d| {
            let names = [
                "Add Fight",
                ADD_LONG_REST,
                ADD_SHORT_REST,
                ADD_TRAVEL,
                ADD_EVENT,
                ADD_DAY,
            ];
            let nodes = d.nodes();
            let controls: Vec<_> = names
                .iter()
                .filter_map(|name| {
                    nodes
                        .iter()
                        .find(|node| node.name == *name && node.fully_in_view())
                })
                .collect();
            assert!(!controls.is_empty());
            // Fight and Long rest are always one click away; More shows whenever the rest fold.
            let required = ["Add Fight", ADD_LONG_REST];
            let required_shown = required.iter().all(|name| {
                nodes
                    .iter()
                    .any(|node| node.name == *name && node.fully_in_view())
            });
            let more_ok = controls.len() == names.len()
                || nodes
                    .iter()
                    .any(|node| node.name == "More Add actions" && node.fully_in_view());
            d.observe(
                "iter-5-A-Timeline-strip",
                required_shown
                    && more_ok
                    && controls
                        .iter()
                        .all(|node| (node.rect.top() - controls[0].rect.top()).abs() < 1.0),
                "The Timeline's visible Add actions keep one line",
            );
            if !d.has("More Add actions") {
                assert_eq!(
                    controls.len(),
                    names.len(),
                    "A wide Timeline exposes every action without More"
                );
            }
            // iter-23-S FK7: with no day yet, the header names no clock.
            d.observe(
                "iter-23-S-FK7",
                d.visible_text("events") && !d.visible_text("Now:"),
                "A timeline with no day yet has no Now in its header",
            );
        });
        d.check_tab_order("timeline");
        d.click_add_event(ADD_DAY);
        d.type_text("Harvest Eve");
        // A day opens what follows it, so move it to the top.
        d.scroll_until(pos2(850.0, 550.0), "Place this event after", 12);
        d.click("Place this event after");
        d.click("Start of the timeline");
        d.state("timeline-day-added", Sizes::All);
        d.click_containing("Long rest · Session 1");
        d.expect("Link text");
        d.replace_text_in("Title", "Rest at the mill");
        d.type_into("Where", "Old mill");
        d.press_ctrl(Key::S);
        d.expect("Copy link");
        d.click("Copy link");
        d.expect("Copied");
        d.expect("Mentioned in");
        d.state_prepared("timeline-event", Sizes::AllAndHiDpi, |d| {
            // iter-23-S FK5: the list row draws the rest ruling as the plan row and the detail
            // do: its icon and word in the ruling's colour.
            let mut rulings = Vec::new();
            let mut in_row = false;
            d.each_painted_text(|text, _| {
                let job = &text.galley.job;
                if job.text.contains("Everyone at full") {
                    // The list row's second line puts its other facts before the ruling.
                    in_row |= job.text.contains(" · ");
                    rulings.extend(job.sections.iter().filter_map(|section| {
                        let words = &job.text[section.byte_range.clone()];
                        words
                            .contains("Everyone at full")
                            .then_some(section.format.color)
                    }));
                }
            });
            d.observe(
                "iter-23-S-FK5",
                in_row
                    && !rulings.is_empty()
                    && rulings
                        .iter()
                        .all(|color| *color == crate::design::colors().document.saved),
                format!("The list row's ruling uses the ruling's colour: {rulings:?}"),
            );
            let label_visible = d.visible_text("TITLE") || d.visible_text("Title");
            d.observe(
                "iter-2-C-FT12",
                label_visible,
                "The Title label stays visible with its field after a rename",
            );
        });
        d.observe(
            "T11.4",
            d.has_containing(&format!("Session 1: {TITLE}")),
            "An event lists the session notes that link to it",
        );
        // Renaming keeps notes unchanged until the DM accepts the update.
        // iter-25-S FL2: the Library's offer, from the same function.
        const LINK_OFFER: &str = "Change links from “Long rest” to “Rest at the mill”.";
        d.expect(LINK_OFFER);
        d.expect("Update 1 link");
        d.observe(
            "iter-25-S-FL2",
            !d.has_containing("Notes still say") && !d.has("Update 1 link in notes"),
            "Timeline offers the link update in the Library's words",
        );
        // iter-25-S FL3: the offer sits in the list column, so the form keeps its place.
        let kind_top = |d: &Driver<'_>| d.place_of("Kind of event").map(|rect| rect.top());
        let detail_left = |d: &Driver<'_>| d.place_of("Kind of event").map(|rect| rect.left());
        let with_offer = [SMALL, DEFAULT, LARGE].map(|size| {
            d.at_size(size, |d| {
                let offer = d.place_of(LINK_OFFER);
                (
                    kind_top(d),
                    offer.zip(detail_left(d)).map(|(o, l)| o.right() < l),
                )
            })
        });
        d.observe(
            "iter-25-S-FL3-list-column",
            with_offer
                .iter()
                .all(|(kind, beside)| kind.is_some() && *beside == Some(true)),
            format!("The offer is in the list column, left of the form: {with_offer:?}"),
        );
        d.expect_same("The rename offer is already in the list column. Checking it leaves the view unchanged.");
        d.state("event-link-update-offered", Sizes::All);
        d.click("Update 1 link");
        d.expect("Links now say “Rest at the mill”.");
        d.expect("Undo link update");
        assert!(!d.has(LINK_OFFER));
        let without_offer = [SMALL, DEFAULT, LARGE].map(|size| d.at_size(size, kind_top));
        d.observe(
            "iter-25-S-FL3",
            with_offer
                .iter()
                .zip(without_offer)
                .all(|((with, _), without)| {
                    with.zip(without).is_some_and(|(a, b)| (a - b).abs() < 1.0)
                }),
            format!(
                "Kind keeps its y with and without the offer: {with_offer:?} then {without_offer:?}"
            ),
        );
        d.state("event-links-updated", Sizes::Default);
        d.click("Undo link update");
        d.expect("Update 1 link");
        d.click("Update 1 link");
        d.click_starting("Rest at the mill ·");
        d.click("Open this session");
        d.settle();
        d.observe(
            "iter-4-S2",
            !d.has("Undo link update"),
            "Leaving Timeline clears the link-update Undo",
        );
        read_note(d);
        d.observe(
            "T11.4",
            d.visible_text("Rest at the mill"),
            "A renamed event reads under its new name in the note",
        );
        write_note(d);
        d.click(NAV_TIMELINE);
        d.click("Rests");
        d.observe(
            "iter-5-S1",
            d.in_view("Title"),
            "Changing a Timeline filter resets the selected detail to the top",
        );
        d.expect("of 4 events");
        d.state("timeline-filtered", Sizes::Default);
        d.observe(
            "S16",
            !d.has("Move earlier") && !d.has("Move later") && d.has("Place this event after"),
            "Place after is the one way to change an event's position",
        );
        d.click("All");
        d.click("More actions for Rest at the mill");
        d.click("Remove this event");
        d.expect("Undo remove");
        d.expect("1 note still links to it.");
        d.state("timeline-removed", Sizes::Default);
        d.click_containing("Travel ·");
        d.observe(
            "T11.4",
            d.has("Undo remove"),
            "Selecting another event keeps the undo for a removal",
        );
        d.click("Undo remove");
        d.expect("Rest at the mill");

        // Back to the note from the event, and the file is readable outside Splat.
        d.click_starting("Rest at the mill ·");
        d.click("Open this session");
        d.expect(&format!("Session 1: {TITLE}"));

        // Words that could not be saved survive anything that would open the note again.
        write_note(d);
        d.type_into_editor("\nMy words during the conflict.\n");
        d.app_mut().sessions.simulate_disk_change();
        d.press_ctrl(Key::S);
        d.expect("Changed on disk");
        d.expect("Overwrite with my draft");
        d.expect("line differs");
        d.observe(
            "iter-2-S6",
            !d.has("Show disk version") && d.in_view_containing("Written in Obsidian."),
            "The comparison opens at its first difference without another action",
        );
        d.observe(
            "S4",
            !d.has("Next difference"),
            "One changed line needs no Next difference action",
        );
        d.observe(
            "T11.4",
            d.has_containing("Written in Obsidian."),
            "A conflict shows the file's words, so the DM chooses knowing both",
        );
        check_choices_at_text(
            d,
            "iter-15-C-FT1-conflict",
            "This note changed on disk.",
            &["Overwrite with my draft", "Reload and lose my draft"],
            Some("Reload and lose my draft"),
            &[SMALL, DEFAULT, LARGE],
        );
        d.state("note-conflict", Sizes::All);
        d.click("Record this session");
        d.expect("Start recording");
        // Two stacked messages share one left edge for their choices: each message's text.
        for (words, choices, gap) in [
            (
                "This note changed on disk.",
                &["Overwrite with my draft", "Reload and lose my draft"][..],
                Some("Reload and lose my draft"),
            ),
            (
                "Tell everyone at the table first.",
                &["Start recording", "Not now"][..],
                None,
            ),
        ] {
            check_choices_at_text(d, "iter-15-C-FT1-stacked", words, choices, gap, &[DEFAULT]);
        }
        d.observe(
            "iter-5-S2",
            d.has("Overwrite with my draft"),
            "Recording consent opens while the note conflict stays unresolved",
        );
        d.state("note-conflict-record", Sizes::Default);
        d.click("Start recording");
        d.expect("Stop recording");
        let recording_in_conflict = d.app_mut().sessions.recording();
        d.observe(
            "iter-5-S2",
            recording_in_conflict,
            "Audio recording starts while the note conflict stays unresolved",
        );
        check_choices_at_text(
            d,
            "iter-15-C-ST1",
            "This note changed on disk.",
            &["Overwrite with my draft", "Reload and lose my draft"],
            Some("Reload and lose my draft"),
            &[DEFAULT],
        );
        d.state("note-conflict-recording", Sizes::Default);
        d.click("Stop recording");
        d.click(NAV_TIMELINE);
        d.observe(
            "T11.4",
            d.has_containing("Notes changed on disk"),
            "A note that could not be saved stays in the rail on other pages",
        );
        d.observe_both("iter-1-C-FT3", "The rail names the changed notes", |d| {
            d.rail_words().contains("Notes changed")
        });
        d.state("note-conflict-elsewhere", Sizes::All);
        d.click("Notes changed on disk");
        d.expect("Overwrite with my draft");
        d.click(NAV_TIMELINE);
        d.click_starting("Rest at the mill ·");
        d.click("Open this session");
        d.observe(
            "T11.4",
            d.has_containing("My words during the conflict."),
            "Opening the session again keeps the words that could not be saved",
        );
        // Reload names itself as World does, and Undo reload brings the draft back. A new outside
        // change then brings the same choices back, so the step below starts where it did.
        d.aside(|d| {
            d.click("Reload and lose my draft");
            d.expect("Undo reload");
            d.observe(
                "iter-15-C-FT4-reload",
                d.visible_text("Reloaded from disk.")
                    && !d.visible_text("Draft dropped.")
                    && d.editor_text()
                        .is_some_and(|text| !text.contains("My words during the conflict.")),
                "Reload says Reloaded from disk. with Undo reload, and the editor shows the file",
            );
            d.click("Undo reload");
            d.observe(
                "iter-15-C-FT4-undo-reload",
                !d.has("Undo reload")
                    && d.editor_text()
                        .is_some_and(|text| text.contains("My words during the conflict.")),
                "Undo reload puts the draft's words back in the editor",
            );
            d.app_mut().sessions.simulate_disk_change();
            d.press_ctrl(Key::S);
            d.expect("Overwrite with my draft");
        });
        d.click("Overwrite with my draft");
        d.expect("Notes saved");

        // A long title wraps, and the controls stay where they are.
        d.click("Edit session details");
        d.replace_text_in("Date", "Nov 2");
        d.click("Save details");
        d.expect("Enter the date as YYYY-MM-DD.");
        d.replace_text_in("Date", crate::hermetic::today().as_str());
        d.replace_text_in("Session title", LONG_TITLE);
        d.click("Save details");
        d.observe(
            "T11.1",
            d.editor_text()
                .is_some_and(|text| text.contains(&format!("# Session 1: {LONG_TITLE}"))),
            "Renaming a session moves the heading the template made",
        );
        d.observe_both(
            "i19-S4",
            "The renamed heading stays visible at the top of the note",
            |d| d.first_source_line_visible("# Session 1: The Long Road"),
        );
        d.state("session-long-title", Sizes::All);
        d.click("Edit session details");
        d.replace_text_in("Session title", TITLE);
        d.click("Save details");

        d.click("Templates");
        d.expect("Standard session");
        d.state("templates-built-in", Sizes::All);
        d.click("Customize this template");
        d.expect("Campaign");
        d.state("templates-own", Sizes::Default);
        d.type_into_editor("Edited in the template.\n");
        d.click("New template");
        d.type_into("Template name", "Downtime");
        d.observe(
            "iter-13-S1",
            !d.has("New template") && d.has("Cancel"),
            "An open new-template form cannot be reset from the header",
        );
        d.state("templates-new", Sizes::Default);
        d.click("Create template");
        d.expect("Downtime");
        d.click("More actions for Downtime");
        d.click("Delete this template");
        d.expect("Undo delete");
        d.state("templates-deleted", Sizes::Default);
        d.click("Undo delete");
        d.expect("Downtime");

        d.click("Sessions");
        d.click("Campaign");
        d.click("Link and folder");
        d.expect("SESSION FOLDER");
        d.observe_both(
            "iter-2-C-FT11",
            "The clean session folder has a No changes status",
            |d| d.visible_text("No changes") && !d.visible_text("No changes to save."),
        );
        d.expect("Not connected · showing the name saved on this computer");
        d.state("session-folder-in-campaign", Sizes::All);
        d.app_mut().timeline.save();
        d.replace_text_in("Folder", "30 Sessions");
        d.expect("Move files and save");
        d.expect("Unsaved");
        d.click("Discard");
        d.observe(
            "iter-7-S3",
            d.field_text("Folder").as_deref() == Some("Sessions"),
            "Discard restores the saved session folder",
        );
        d.replace_text_in("Folder", "30 Sessions");
        d.expect("Save without moving");
        // FT4 (round 16): with no run and no other form, moving the files is the one primary,
        // as Move runs and save is on Rules and runs.
        d.state_prepared("session-folder-move-offered", Sizes::All, |d| {
            super::session_recording::check_one_primary(d, "Move files and save", "iter-16-C-FT4");
            // iter-23-S FK8: the warning names which choice hides the files.
            d.observe(
                "iter-23-S-FK8",
                d.visible_text("If you save without moving, Splat no longer shows these files.")
                    && !d.visible_text("will not show these files from the new folder"),
                "The folder warning says Save without moving hides the files",
            );
        });
        // Must stay true: the open details form owns its own save, so the move is not primary.
        d.click("Edit details");
        d.expect("Save details");
        // The details form's own save is the one primary once its details change.
        let name = d
            .field_text("Campaign name")
            .expect("the campaign name field");
        d.type_into("Campaign name", " (edited)");
        super::session_recording::check_one_primary(
            d,
            "Save details",
            "iter-16-C-FT4-details-primary",
        );
        d.replace_text_in("Campaign name", &name);
        d.expect("Cancel");
        d.scroll_until(pos2(600.0, 400.0), "Move files and save", 20);
        let frame = d.control_frame("Move files and save");
        assert!(
            frame.is_some(),
            "Move files and save is painted below the details form"
        );
        d.observe(
            "iter-16-C-FT4-details",
            frame.as_ref().is_some_and(|frame| {
                let colors = crate::design::colors();
                frame.fill != colors.accent_fill && frame.fill != colors.accent_hover
            }),
            format!(
                "With the details form open, Move files and save has no accent fill: {frame:?}"
            ),
        );
        d.click("Cancel");
        d.scroll_up_at(pos2(600.0, 400.0), 20);
        d.expect("Move files and save");
        d.click("Move files and save");
        d.expect("Moved 1 note, 4 events, 2 templates and 1 recording to 30 Sessions.");
        d.observe_both(
            "iter-1-C-FT15-18",
            "The session folder uses one label and one right edge",
            |d| {
                let nodes = d.nodes();
                nodes
                    .iter()
                    .find(|node| {
                        node.name == "Folder" && node.role == egui::accesskit::Role::TextInput
                    })
                    .zip(nodes.iter().find(|node| node.name == "Save folder"))
                    .is_some_and(|(field, save)| {
                        (field.rect.right() - save.rect.right()).abs() < 1.0
                            && field.rect.width() <= crate::design::FORM_WIDTH + 1.0
                            && d.place_of("Edit details")
                                .is_some_and(|edit| (field.rect.right() - edit.right()).abs() < 1.0)
                    })
            },
        );
        d.state("session-folder-moved", Sizes::All);
        d.click(NAV_SESSIONS);
        d.expect(&format!("Session 1: {TITLE}"));

        // Reading last week's note does not send tonight's run there.
        d.click(NAV_TIMELINE);
        d.click("Session for new events");
        d.click("No session");
        d.click(NAV_SESSIONS);
        d.click("New session");
        d.type_into("Session title", "Tonight");
        d.click("Create session");
        d.expect("Session 2: Tonight");
        d.observe(
            "T11.1",
            d.editor_text()
                .is_some_and(|text| text.contains("Edited in the template.")),
            "A template edited a moment ago is what a new session starts from",
        );
        d.click_starting(&format!("Session 1: {TITLE} ·"));
        show_plan(d);
        d.observe(
            "T11.3",
            d.has_containing(
                "Session 1 is not the session being played. What you write and add on this page goes into Session 1 - Ash on the Road.md.",
            ),
            "Adding an event to an older session says so before it happens",
        );
        d.state("session-events-older", Sizes::Default);
        d.observe_both(
            "iter-18-B-rest-row",
            "The rest summary and ruling share one line",
            |d| {
                let title = d
                    .nodes()
                    .into_iter()
                    .find(|node| node.name == "Rest at the mill");
                title.is_some_and(|title| {
                    let area = egui::Rect::from_min_max(
                        title.rect.left_top(),
                        egui::pos2(title.rect.left() + 400.0, title.rect.bottom() + 36.0),
                    );
                    d.painted_rect_in("Everyone", area)
                        .is_some_and(|line| line.height() <= crate::design::Type::Small.line())
                })
            },
        );
        write_note(d);
        d.observe(
            "T11.8",
            d.has_containing(
                "Session 1 is not the session being played. What you write and add on this page goes into Session 1 - Ash on the Road.md.",
            ),
            "Writing in an older note says so",
        );
        d.observe(
            "iter-8-S3",
            d.has("Recordings, 1 recording"),
            "The Recordings tab includes its count in its accessible name",
        );
        d.state("session-write-older", Sizes::Default);
        go_encounters(d);
        resume_run(d, RUN);

        // T11.8: a note during a run, with the roster still in view.
        d.begin_task();
        d.press_ctrl(Key::J);
        d.type_text("The ogre fled over the bridge.");
        d.goal_within("T11.8", "take a note during a run", 2);
        d.expect("Close notes");
        d.observe(
            "T11.8",
            d.in_view(NEXT_TURN)
                && d.editor_text()
                    .is_some_and(|text| text.contains("The ogre fled")),
            "The note goes in the session being played, and the run stays in view",
        );
        d.observe(
            "iter-2-S10",
            !d.visible_passage("Recap of last time") && d.visible_passage("What happened"),
            "The Notes panel opens at What happened with repeated note headings scrolled out",
        );
        d.observe(
            "Keys",
            d.visible_text("Keys"),
            "Keys keeps its word beside Notes at 960x640",
        );
        d.state("run-with-notes", Sizes::All);
        let wide_screen = d.at_size((1536.0, 864.0), |d| d.in_view(NEXT_TURN) && !d.has("Prep"));
        d.observe(
            "T11.8",
            wide_screen,
            "At 1536 px, a panel beside the run leaves the roster its room",
        );
        // A click on Next turn moves the caret, so what is typed next must not change the run.
        let turn = current_participant(d);
        d.click(NEXT_TURN);
        let moved = current_participant(d);
        d.observe(
            "T11.8",
            moved != turn,
            "Next turn still works with Notes open",
        );
        // Words typed next go to the note. A digit and Enter must not become damage.
        let hp = |d: &Driver<'_>| {
            d.nodes()
                .iter()
                .map(|node| node.name.clone())
                .filter(|name| name.contains("HP"))
                .collect::<Vec<_>>()
        };
        let before = hp(d);
        d.type_text("Ogre took 12 from Rohan");
        d.press(Key::Enter);
        d.observe(
            "T11.8",
            d.editor_text()
                .is_some_and(|text| text.contains("Ogre took 12 from Rohan")),
            "After Next turn, what the DM types still goes to the note",
        );
        d.observe(
            "T11.8",
            before == hp(d),
            "A number typed for the note never becomes damage",
        );
        d.press(Key::N);
        d.press(Key::U);
        d.observe(
            "T11.8",
            current_participant(d) == moved,
            "With Notes open, stray typing never changes the run",
        );
        d.observe(
            "T11.8",
            !d.has("Next turn N"),
            "With Notes open, the run shows no key hint that would not work",
        );
        d.click(UNDO_NEXT_TURN);
        d.observe(
            "T11.8",
            d.editor_text().is_some_and(|text| {
                let what = text.find("## What happened");
                let note = text.find("The ogre fled");
                let next = text.find("## People and places");
                matches!((what, note, next), (Some(a), Some(b), Some(c)) if a < b && b < c)
            }),
            "A note typed at the table goes under What happened",
        );
        // A click on a row takes the keyboard to Amount for one number, and then it goes back.
        d.click_row("Ember Cultist");
        let full = hp(d);
        d.type_text("7");
        d.press(Key::Enter);
        let hurt = hp(d);
        d.observe(
            "T11.8",
            full != hurt,
            "With Notes open, a click on a row still lets a number apply",
        );
        d.type_text("The cultist took 12 more");
        d.press(Key::Enter);
        d.observe(
            "T11.8",
            d.editor_text()
                .is_some_and(|text| text.contains("The cultist took 12 more")),
            "After a number applies, what the DM types goes to the note",
        );
        d.observe(
            "T11.8",
            hurt == hp(d),
            "The words after a number never become more damage",
        );
        d.click_containing("Undo: 7 damage");
        d.type_text("Undone.");
        d.observe(
            "T11.8",
            full == hp(d) && d.editor_text().is_some_and(|text| text.contains("Undone.")),
            "After Undo by its button, what the DM types goes to the note",
        );
        // After a click on a row, the DM can go on writing. A digit in those words is not damage.
        d.click_row("Ember Cultist");
        let whole = hp(d);
        // One key at a time, as a person types, because the keyboard moves on the way.
        for key in "2 wolves flee".chars() {
            d.type_text(&key.to_string());
        }
        d.press(Key::Enter);
        d.observe(
            "T11.8",
            d.editor_text()
                .is_some_and(|text| text.contains("2 wolves flee")),
            "After a click on a row, words typed go to the note",
        );
        d.observe(
            "T11.8",
            whole == hp(d),
            "A number in those words never becomes damage",
        );
        // Lookup takes the place of the notes, and they come back with it closed.
        let turn = current_participant(d);
        d.type_text("Iselle casts ");
        d.press_ctrl(Key::K);
        d.observe(
            "T11.8",
            !d.has("Close notes"),
            "One panel beside the page at a time",
        );
        d.press(Key::Escape);
        d.type_text("Bless");
        d.press(Key::N);
        d.observe(
            "T11.8",
            d.has("Close notes")
                && d.editor_text()
                    .is_some_and(|text| text.contains("Iselle casts Bless"))
                && current_participant(d) == turn,
            "After Lookup, the sentence goes on where it was, and typing never changes the run",
        );
        // A stray click takes the keyboard from the note, and the words still go there.
        d.harness_focus_nothing();
        d.type_text(" ");
        d.type_text("too");
        d.observe(
            "T11.8",
            d.editor_text()
                .is_some_and(|text| text.contains("Iselle casts Bless too")),
            "After a stray click, what the DM types still goes on in the note",
        );
        // Tab indents. It must not walk to Next turn, where the next space would press it.
        d.press(Key::Tab);
        d.type_text(" ");
        d.observe(
            "T11.8",
            d.editor_text()
                .is_some_and(|text| text.contains("Bless too\t "))
                && current_participant(d) == turn,
            "Tab and space in the note stay in the note, and the turn does not move",
        );
        // A step changes nothing in the note until the DM types.
        let unchanged = d.editor_text();
        d.click(NEXT_TURN);
        d.observe(
            "T11.8",
            unchanged == d.editor_text(),
            "A click on Next turn alone leaves the note as it was",
        );
        d.type_text("Next thought.");
        d.observe(
            "T11.8",
            d.editor_text()
                .is_some_and(|text| text.contains("\n\nNext thought.")),
            "What is typed after a step starts a line of its own",
        );
        d.click(NEXT_TURN);
        d.press(Key::Backspace);
        d.type_text("!");
        d.observe(
            "T11.8",
            d.editor_text()
                .is_some_and(|text| text.contains("Next thought!")),
            "A correction after a step goes on with the same line",
        );
        d.click_containing("Undo");
        d.click_containing("Undo");
        // What the DM asks for in the side panel must not stay hidden behind the notes.
        d.click_row("Ember Cultist");
        d.click("Conditions for this participant");
        d.observe(
            "T11.8",
            !d.has("Close notes") && d.has_containing("Poisoned"),
            "Conditions show, in place of the notes, when both have no room",
        );
        d.press_ctrl(Key::J);
        d.observe(
            "T11.8",
            d.editor_text()
                .is_some_and(|text| text.contains("The cultist took 12 more")),
            "Notes closed that way keep their words",
        );
        d.click_row("Iselle Marrowind");
        d.type_text("99");
        d.press(Key::Enter);
        d.click("Death saves");
        d.observe(
            "T11.8",
            !d.has("Close notes") && d.has("DEATH SAVES"),
            "Death saves show, in place of the notes, when both have no room",
        );
        d.click_containing("Undo: 99 damage");
        d.press_ctrl(Key::J);
        // The panel must not follow the page to an older note.
        d.click(NAV_SESSIONS);
        d.click_starting(&format!("Session 1: {TITLE} ·"));
        d.observe(
            "T11.8",
            !d.has("Close notes"),
            "Opening an older note closes the panel, so that no word goes there",
        );
        d.press_ctrl(Key::J);
        d.observe(
            "T11.8",
            d.has_containing("Session 2: Tonight") && !d.has("Close notes"),
            "Notes open on the session being played. On this page the page itself holds them",
        );
        d.state("sessions-with-notes", Sizes::All);
        d.click(BACK_TO_RUN);
        d.press_ctrl(Key::J);
        // Coming back from the page, where the note can change, the caret starts fresh.
        d.click(NAV_SESSIONS);
        d.click(BACK_TO_RUN);
        d.type_text("The wolves returned.");
        d.observe(
            "T11.8",
            d.editor_text()
                .is_some_and(|text| text.contains("The wolves returned.\n\n## People and places")),
            "A second note keeps the next heading on its own line",
        );
        d.press(Key::Escape);
        d.observe(
            "T11.8",
            !d.has("Close notes"),
            "Escape closes the notes beside the run",
        );
        // A note that looks like last week's says so where the words go.
        d.click(NAV_SESSIONS);
        d.click("Edit session details");
        d.replace_text_in("Date", "2025-12-25");
        d.click("Save details");
        d.click(BACK_TO_RUN);
        d.press_ctrl(Key::J);
        d.observe(
            "T11.8",
            d.has_containing("This session is dated 2025-12-25."),
            "The panel warns when the session being played looks like an old one",
        );
        check_choices_at_text(
            d,
            "iter-15-C-ST3",
            "This session is dated 2025-12-25.",
            &["Play it tonight", "New session"],
            None,
            &[DEFAULT],
        );
        // Round 24 FL4: both choices have no icon, so both have the secondary border and fill.
        let frames = ["Play it tonight", "New session"].map(|name| {
            d.control_frame(name)
                .map(|frame| (frame.stroke, frame.fill))
        });
        d.observe(
            "iter-24-S-FL4",
            frames[0].is_some_and(|(stroke, _)| stroke.width > 0.0) && frames[0] == frames[1],
            format!("`New session` has the border of `Play it tonight`: {frames:?}"),
        );
        d.state("run-with-old-note", Sizes::Default);
        // The action bar above the dock sits an `XS` gap apart from it, and its first button
        // starts at the dock's content edge, where Amount starts (iter-19 FT6).
        d.observe_both(
            "iter-19-R-FT6",
            "The action bar keeps a gap above the dock and starts at its content edge",
            |d| {
                d.place_of("Dagger +6")
                    .zip(d.place_of("Amount"))
                    .is_some_and(|(bar, amount)| {
                        (bar.left() - amount.left()).abs() < 1.0
                            && amount.top() - bar.bottom() >= 2.0 * crate::design::XS - 0.5
                    })
            },
        );
        d.observe(
            "T11.8",
            d.has_containing("Old session"),
            "During a run, the rail says that events are going to an old session",
        );
        d.click("Play it tonight");
        d.observe(
            "T11.8",
            !d.has_containing("This session is dated"),
            "A session prepared earlier can be played with one press",
        );
        d.press(Key::Escape);

        d.click(NAV_TIMELINE);
        d.expect(RUN);
        d.observe(
            "iter-5-S1",
            d.in_view("Title"),
            "Opening Timeline resets its selected detail to the top",
        );
        d.state("timeline-with-run", Sizes::All);
        d.observe_both(
            "iter-18-B-S2-timeline",
            "The open fight uses the same second-line offset as other Timeline rows",
            |d| {
                let nodes = d.nodes();
                let offsets: Vec<_> = nodes
                    .iter()
                    .filter(|node| node.name.contains(" · "))
                    .filter_map(|node| {
                        let (title, detail) = node.name.split_once(" · ")?;
                        let first = d.painted_rect_in(title, node.rect)?;
                        let second = d.painted_rect_in(
                            if title == RUN {
                                "Round 2"
                            } else {
                                detail.split(" · ").next()?
                            },
                            node.rect,
                        )?;
                        (second.top() > first.top()).then_some((title, second.top() - first.top()))
                    })
                    .collect();
                offsets
                    .iter()
                    .find(|(title, _)| *title == RUN)
                    .is_some_and(|(_, offset)| {
                        offsets
                            .iter()
                            .any(|(title, other)| *title != RUN && (offset - other).abs() <= 1.0)
                    })
            },
        );
        d.observe_both(
            "iter-14-S1",
            "The open fight's Timeline row says Round 2 without Paused",
            |d| {
                d.nodes().iter().any(|node| {
                    node.name.starts_with(&format!("{RUN} ·"))
                        && node.name.contains("Round 2")
                        && !node.name.contains("Paused")
                })
            },
        );
        // Beside a panel at the minimum window, the list keeps its room.
        d.press_ctrl(Key::K);
        d.state("timeline-beside-lookup", Sizes::All);
        d.observe_both(
            "iter-10-S3",
            "Timeline header actions end at the form edge",
            |d| {
                let field = d
                    .nodes()
                    .into_iter()
                    .find(|node| {
                        node.name == "Title" && node.role == egui::accesskit::Role::TextInput
                    })
                    .map(|node| node.rect);
                let menu = d
                    .nodes()
                    .into_iter()
                    .find(|node| node.name.starts_with("More actions for "));
                field
                    .zip(menu)
                    .is_some_and(|(field, menu)| (field.right() - menu.rect.right()).abs() < 1.0)
            },
        );
        d.click(NAV_SESSIONS);
        d.expect("All sessions");
        // FT2: Generate NPC starts a line at the search field's edge.
        lookup_action_at_text_edge(
            d,
            "iter-17-D-FT2-generate",
            "Generate NPC",
            &[SMALL, DEFAULT, LARGE],
            |d| d.place_of(SEARCH_LOOKUP).map(|field| field.left()),
        );
        d.state("session-beside-lookup", Sizes::Default);
        d.click("All sessions");
        d.expect("Session 1: ");
        d.state("session-list-beside-lookup", Sizes::Default);
        d.click_containing("Session 2: Tonight");
        d.expect("All sessions");
        d.press_ctrl(Key::K);
        d.click(NAV_TIMELINE);
        d.observe(
            "T11.3",
            d.has_containing("Fight · Round 2 · Session 2"),
            "A run went to the newest session, with no step, though an older note was read",
        );
        d.goal("use the timeline during a run");
        d.click("Round 2");
        let opened_run = d.app_mut().page == crate::Page::Run;
        d.observe(
            "iter-14-S1",
            opened_run,
            "The Timeline round link opens the run",
        );
        d.audit_goal_within("iter-14-S1", "open the run from its Timeline round", 1);
        d.click(NAV_SESSIONS);
        show_plan(d);
        d.observe_both(
            "iter-18-B-S2-plan",
            "The open fight's plan summary stays on the standard second line",
            |d| {
                let title = d
                    .nodes()
                    .into_iter()
                    .find(|node| node.name == RUN && node.is_link());
                title.is_some_and(|title| {
                    let area = egui::Rect::from_min_max(
                        title.rect.left_top(),
                        egui::pos2(title.rect.left() + 400.0, title.rect.bottom() + 36.0),
                    );
                    d.painted_rect_in("Round 2", area)
                        .zip(d.painted_rect_in(RUN, title.rect))
                        .is_some_and(|(line, first)| {
                            (line.top() - first.top() - crate::design::Density::Read.control())
                                .abs()
                                <= 2.0
                        })
                })
            },
        );
        // The open fight's state is a link whose run icon is its own glyph: only `Round 2` is
        // underlined, and the link's name is the words.
        d.expect("Round 2");
        let parts = icon_link_parts(d, crate::design::icons::RUN, "Round 2");
        d.observe(
            "iter-17-D-plan-run-link",
            parts.is_some_and(|(icon, words, _)| !icon && words),
            format!("(icon underlined, words underlined, icon colour) is {parts:?}"),
        );
        d.state("plan-open-fight-aligned", Sizes::Default);
        write_note(d);
        d.click(NAV_TIMELINE);
        d.begin_task();

        // A new session changes where the next events go, and the open run stays.
        d.click(NAV_SESSIONS);
        d.click("New session");
        d.type_into("Session title", "Later that night");
        d.press(Key::Enter);
        d.observe(
            "T11.3",
            d.has_containing("stays in Session 2"),
            "A new session during a run says that the run's event stays where it was",
        );
        d.state("new-session-during-run", Sizes::Default);
        d.click("Move it here");
        let current = d.app_mut().sessions.current();
        let moved = d
            .app_mut()
            .timeline
            .events
            .iter()
            .any(|event| event.run.is_some() && event.session == current);
        d.observe(
            "iter-2-S17",
            moved,
            "The running fight moves to this session without leaving the page",
        );
        d.state("run-moved-to-session", Sizes::Default);
        d.expect("Bridge Ambush moved to Session 3.");
        d.expect("Undo move");
        d.click("Undo move");
        let restored = d
            .app_mut()
            .timeline
            .events
            .iter()
            .any(|event| event.run.is_some() && event.session != current);
        d.observe(
            "iter-6-S2",
            restored,
            "Undo move restores the run's previous session",
        );

        // Setup beside the notes. Each digit must stay in its field.
        d.click(BACK_TO_RUN);
        d.press_ctrl(Key::J);
        d.expect("Close notes");
        d.app_mut()
            .encounters
            .simulate_sync_conflict("Iselle Marrowind");
        d.settle();
        d.click("Review");
        d.observe(
            "T11.8",
            !d.has("Close notes"),
            "A conflict's controls show, in place of the notes, when both have no room",
        );
        d.click("Use D&D Beyond values");
        d.app_mut().encounters.simulate_session_end();
        d.settle();
        d.click("Close run");
        if d.has("Close and lose changes") {
            d.click("Close and lose changes");
        } else if d.has("Close now") {
            d.click("Close now");
        }
        run_saved_encounter(d, "Shrine of Cinders");
        d.press_ctrl(Key::J);
        let note = d.editor_text();
        d.click("Initiative for Iselle Marrowind");
        d.type_text("1");
        d.type_text("7");
        d.observe(
            "T11.8",
            note.is_some() && note == d.editor_text(),
            "Initiative typed beside the notes stays in its field",
        );
        d.state("run-setup-with-notes", Sizes::Default);
        d.click("Initiative for Oren Thistlewick");
        d.type_text("2");
        d.type_text(" ");
        d.type_text("wolves");
        d.observe(
            "T11.8",
            d.editor_text().is_some_and(|text| text.contains(" wolves")),
            "A letter typed in an initiative field goes to the note",
        );
        for (name, total) in [
            ("Brannoch Vale", "17"),
            ("Kestrel Dunmoor", "15"),
            ("Oren Thistlewick", "14"),
            ("Sable Harrowgate", "13"),
            ("Ember Cultist 1", "12"),
            ("Ember Cultist 2", "11"),
        ] {
            d.click(&format!("Initiative for {name}"));
            d.type_text(total);
        }
        d.observe(
            "T11.8",
            d.visible_text("Everyone is ready") && d.has_containing("2 tied"),
            "Notes hide the order of ties, so setup says that there are ties",
        );
        d.click("Close notes");
        d.observe(
            "T11.8",
            !d.has("Close notes") && d.has_containing(" tied"),
            "What Notes hide in setup is one click away",
        );
        d.click(NAV_TIMELINE);
        let campaign = d.app_mut().campaigns.active_campaign().unwrap();
        let path = crate::sessions::folder(&campaign)
            .unwrap()
            .join("Timeline.md");
        let disk = std::fs::read_to_string(&path).unwrap();
        d.app_mut().timeline.touch();
        std::fs::write(&path, format!("{disk}\n<!-- Changed outside Splat. -->\n")).unwrap();
        d.app_mut().timeline.save();
        d.settle();
        d.expect("Timeline.md also changed outside Splat. Both changes are kept.");
        // iter-23-S FK6: the outside change is a result like the others in the list column, with
        // the info icon on its line and words in the text colour, not muted help text.
        let mut words = None;
        let mut icons = Vec::new();
        let icon = crate::design::icons::INFO;
        d.each_painted_text(|text, _| {
            let rect = egui::Rect::from_min_size(text.pos, text.galley.size());
            if text
                .galley
                .job
                .text
                .starts_with("Timeline.md also changed outside Splat.")
            {
                words = Some((rect, text.galley.job.sections[0].format.color));
            } else if text.galley.job.text == icon {
                icons.push(rect);
            }
        });
        d.observe(
            "iter-23-S-FK6",
            words.is_some_and(|(words, color)| {
                color == crate::design::colors().text
                    && icons.iter().any(|icon| {
                        icon.right() <= words.left() && (icon.top() - words.top()).abs() <= 1.0
                    })
            }),
            format!("The outside change reads as an info message: {words:?}"),
        );
        d.observe_both(
            "i20-S1",
            "Separate Timeline edits save without a conflict or a rail error",
            |d| {
                !d.has("Overwrite with my timeline")
                    && !d.has("Timeline not saved")
                    && !d.has("Timeline changed on disk")
                    && d.visible_text("Both changes are kept.")
            },
        );
        d.state("timeline-disk-conflict", Sizes::All);
        links_beside_lookup(d);
    }

    /// A Sessions note's link keeps focus when Lookup opens a World note beside it, and
    /// Lookup's written-note card gives the same link an XL target on its own line.
    fn links_beside_lookup(d: &mut Driver<'_>) {
        const LINK: &str = "Wren (opens Wren)";
        const NOTE_TEXT: &str = "Wren runs the ferry at dusk.";
        let id = splat::character::stable_id("Wren.md");
        let text = splat::content::formats::fixture(
            "World.md",
            &format!(
                "splat_id: {id}\nsplat_kind: world\nsplat_name: Wren\nsplat_world:\n  type: npc\n"
            ),
            &format!("\n# Wren\n\n{NOTE_TEXT}\n"),
        )
        .unwrap();
        std::fs::write(d.root.join("campaigns/emberfall/Wren.md"), text).unwrap();
        d.app_mut().lookup.refresh();
        d.settle();
        d.click(NAV_SESSIONS);
        d.click("New session");
        d.type_into("Session title", "Ferry Watch");
        d.click("Create session");
        d.expect("Ferry Watch");
        write_note(d);
        d.replace_editor_text(
            "The ferry crosses the inner sea twice each day. On the way back, ask [[Wren]] \
             about the rope, the toll at the old mill, and the bells that ring at dusk.\n",
        );
        d.press_ctrl(Key::S);
        d.expect("Notes saved");
        read_note(d);
        let reached = d.tab_to(LINK);
        let before = !d.visible_text(NOTE_TEXT);
        // Lookup opens a document reader beside the page, without taking focus.
        d.app_mut().lookup.open_name("Wren");
        d.settle();
        d.observe(
            "iter-17-A-L1-sessions-focus",
            reached
                && before
                && d.visible_text(NOTE_TEXT)
                && d.focused_name().as_deref() == Some(LINK),
            format!(
                "Focus stays on the Sessions link when Lookup opens Wren beside it: {:?}",
                d.focused_name()
            ),
        );
        // Lookup's written-note card shows the session note in compact rows.
        d.app_mut().lookup.search_for("toll at the old mill");
        d.settle();
        d.click_containing("Ferry Watch · Session note");
        d.expect("Open this session");
        // The card's link, right of the Sessions page, and the card's next line of words.
        let card_link = |d: &Driver<'_>| {
            let nodes = d.nodes();
            let page_right = nodes
                .iter()
                .find(|node| node.name == "New session")
                .map(|node| node.rect.right())?;
            let link = nodes
                .iter()
                .filter(|node| {
                    node.name == LINK && node.role == egui::accesskit::Role::Link && node.in_view()
                })
                .find(|node| node.rect.left() > page_right)?
                .rect;
            let runs: Vec<egui::Rect> = nodes
                .iter()
                .filter(|node| {
                    node.role == egui::accesskit::Role::TextRun && node.rect.left() > page_right
                })
                .map(|node| node.rect)
                .collect();
            // The card's lines just above and just below the link's line.
            let above = runs
                .iter()
                .filter(|run| run.bottom() <= link.center().y && run.bottom() > link.top() - 30.0)
                .max_by(|a, b| a.bottom().total_cmp(&b.bottom()))?;
            let below = runs
                .iter()
                .filter(|run| run.top() >= link.center().y && run.top() < link.bottom() + 30.0)
                .min_by(|a, b| a.top().total_cmp(&b.top()))?;
            Some((link, *above, *below))
        };
        d.observe_both(
            "iter-17-A-AD1-written-card",
            "In Lookup's written-note card, Wren's target is XL square on its own line, clear of \
             the lines above and below it",
            |d| {
                card_link(d).is_some_and(|(link, above, below)| {
                    link.width() >= crate::design::XL
                        && link.height() >= crate::design::XL
                        && link.top() >= above.bottom() - 0.01
                        && link.bottom() <= below.top() + 0.01
                })
            },
        );
        // FT1: the paragraph that holds Wren has one line pitch, the link pitch, in the Session
        // reader and in Lookup's compact card, and Wren's underline is under its words.
        const PARAGRAPH: &str = "The ferry crosses the inner sea twice each day.";
        let pitches = |d: &Driver<'_>| {
            paragraph_row_tops(d, PARAGRAPH)
                .into_iter()
                .map(|tops| {
                    tops.windows(2)
                        .map(|pair| pair[1] - pair[0])
                        .collect::<Vec<f32>>()
                })
                .collect::<Vec<_>>()
        };
        for size in [SMALL, DEFAULT] {
            let found = d.at_size(size, pitches);
            d.observe(
                "iter-18-A-FT1-pitch",
                found.len() == 2
                    && found.iter().all(|pitch| {
                        pitch.len() >= 2
                            && pitch
                                .iter()
                                .all(|step| (step - crate::design::XL).abs() < 0.5)
                    }),
                format!(
                    "At {size:?}, the Session reader and Lookup's card each keep one {} px pitch \
                     in the paragraph that holds Wren: {found:?}",
                    crate::design::XL
                ),
            );
            // FT5: the words keep that pitch too, from the first line. Each line's baseline is
            // the same distance below its row top, so the baselines are `XL` apart.
            let found = d.at_size(size, |d| paragraph_baselines(d, PARAGRAPH));
            d.observe(
                "iter-19-A-FT5-baseline-pitch",
                found.len() == 2
                    && found.iter().all(|rows| {
                        rows.len() >= 2
                            && rows.iter().all(|(top, baseline)| {
                                ((baseline - top) - (rows[0].1 - rows[0].0)).abs() < 0.5
                            })
                            && rows
                                .windows(2)
                                .all(|pair| (pair[1].1 - pair[0].1 - crate::design::XL).abs() < 0.5)
                    }),
                format!(
                    "At {size:?}, every line of the paragraph that holds Wren, the first line too, \
                     has its baseline at one offset in its row and {} px below the last, in the \
                     Session reader and in Lookup's card (row top, baseline): {found:?}",
                    crate::design::XL
                ),
            );
            let under = d.at_size(size, |d| {
                let spans = glyph_spans(d, PARAGRAPH, "Wren");
                let lines: Vec<Vec<egui::Rect>> = spans
                    .iter()
                    .map(|(left, right, baseline)| {
                        let area = egui::Rect::from_min_max(
                            egui::pos2(*left + 1.0, *baseline),
                            egui::pos2(*right - 1.0, *baseline + crate::design::S),
                        );
                        d.line_segments_in(area)
                    })
                    .collect();
                (spans, lines)
            });
            d.observe(
                "iter-18-A-FT1-underline",
                under.0.len() == 2
                    && under
                        .0
                        .iter()
                        .zip(&under.1)
                        .all(|((left, right, baseline), lines)| {
                            lines.len() == 1
                                && (lines[0].center().y - (baseline + 2.0)).abs() <= 1.0
                                && (lines[0].left() - left).abs() < 1.0
                                && (lines[0].right() - right).abs() < 1.0
                        }),
                format!(
                    "At {size:?}, Wren's underline sits 2 px below its baseline, under its words, \
                     in both readers: {under:?}"
                ),
            );
        }
        // FT5 (round 20): the card has one `L` gap after its kind line, as a spell has in
        // Lookup. Bless below sets the gap.
        let card_gaps: Vec<_> = [SMALL, DEFAULT, LARGE]
            .into_iter()
            .map(|size| {
                d.at_size(size, |d| {
                    world_build::lookup_prose_gap(d, "Session 4: Ferry Watch", "Session note · ")
                })
            })
            .collect();
        // iter-21-T-FT1: the New actions keep the letters the DM typed; with no capital typed,
        // only the first letter becomes one.
        d.observe(
            "iter-21-T-FT1",
            d.has("New NPC “Toll at the old mill”")
                && d.has("New location “Toll at the old mill”")
                && !d.has_containing("“Toll At The Old Mill”"),
            "Lookup offers New NPC “Toll at the old mill” for the typed toll at the old mill",
        );
        d.state("lookup-written-card-link", Sizes::Default);
        look_up_again(d, "bless");
        d.wait_for_text("Spell · Level 1 · Campaign");
        for (size, card) in [SMALL, DEFAULT, LARGE].into_iter().zip(card_gaps) {
            let spell = d.at_size(size, |d| {
                world_build::lookup_prose_gap(d, "Bless", "Spell · Level 1 · Campaign")
            });
            d.observe(
                "iter-20-D-FT5",
                card.zip(spell).is_some_and(|(card, spell)| {
                    card >= crate::design::L && (card - spell).abs() <= 1.0
                }),
                format!(
                    "At {size:?}, Lookup's session-note card has {card:?} px from its kind line \
                     to its words, Bless {spell:?} px"
                ),
            );
        }
    }
}

/// W12: record a session, and process it later with the DM's chosen services.
pub mod session_recording {
    use super::*;
    use crate::{
        processing::{CANCELLED, Progress},
        recording::Health,
    };

    const TITLE: &str = "The Bridge";
    const RECORDING: &str = "19:00";
    const WARNING: &str = "The check returned nothing, so the clean transcript was not checked.";
    const DISCARD: &str = "Discard";

    /// FT2: one `L` gap between the recording's result links and its marked events heading.
    fn check_events_gap(d: &mut Driver<'_>, id: &str) {
        let heading = d.painted_rect_in("EVENTS MARKED IN THIS RECORDING", egui::Rect::EVERYTHING);
        let links = d.place_of("Raw transcript");
        let gap = heading
            .zip(links)
            .map(|(heading, links)| heading.top() - links.bottom());
        d.observe(
            id,
            // The heading's capitals sit 1 px inside their `L`-spaced line.
            gap.is_some_and(|gap| (gap - crate::design::L).abs() <= 1.0),
            format!("The marked events heading sits L below Raw transcript: gap {gap:?}"),
        );
    }

    fn check_processing_action(d: &mut Driver<'_>, prefix: &str, primary: bool) {
        let actions: Vec<_> = d
            .nodes()
            .into_iter()
            .filter(|node| {
                node.in_view()
                    && node.name.starts_with(prefix)
                    && node.role == egui::accesskit::Role::Button
            })
            .collect();
        assert!(!actions.is_empty(), "The processing action is visible");
        for (index, action) in actions.into_iter().enumerate() {
            let expected = primary && index == 0;
            let frame = d
                .control_frame(&action.name)
                .expect("The processing action has a frame");
            d.observe(
                "iter-8-C-FT6",
                (frame.fill == crate::design::colors().accent_fill) == expected,
                format!(
                    "{} has primary={expected}, fill {:?}",
                    action.name, frame.fill
                ),
            );
        }
    }

    /// Every accent-filled button in view, across the whole window.
    pub(super) fn accent_filled(d: &Driver<'_>) -> Vec<String> {
        let colors = crate::design::colors();
        d.nodes()
            .into_iter()
            .filter(|node| node.in_view() && node.role == egui::accesskit::Role::Button)
            .filter(|node| {
                d.control_frame_at(node.rect).is_some_and(|frame| {
                    frame.fill == colors.accent_fill || frame.fill == colors.accent_hover
                })
            })
            .map(|node| node.name)
            .collect()
    }

    pub(super) fn check_one_primary(d: &mut Driver<'_>, expected: &str, id: &str) {
        assert!(
            d.in_view(expected),
            "The expected primary action is visible"
        );
        let primary = accent_filled(d);
        d.observe(
            id,
            primary == [expected],
            format!("Exactly one accent-filled control is visible, {expected}: {primary:?}"),
        );
    }

    pub fn banner_priority(d: &mut Driver<'_>) {
        d.click(NAV_SESSIONS);
        new_session(d, "Failed processing");
        let first = d.app_mut().sessions.current().unwrap();
        d.app_mut().processing.simulate_keys();
        d.app_mut().processing.simulate_input("table.mic");
        d.settle();
        d.click("Record this session");
        d.click("Start recording");
        d.click("Stop recording");
        d.click_starting("Recordings");
        d.app_mut().sessions.simulate_job(Progress::Asking {
            pass: crate::processing::Pass::Review,
            seconds: 42,
        });
        d.app_mut().sessions.simulate_job_end(Err(
            "The transcript and summary service is busy. Wait, then select Continue processing."
                .to_owned(),
        ));
        d.settle();
        d.click("New session");
        d.type_into("Session title", "Unprocessed recording");
        d.click("Create session");
        d.expect("Session 2: Unprocessed recording");
        let second = d.app_mut().sessions.current().unwrap();
        d.click("Record this session");
        d.click("Start recording");
        d.click("Stop recording");
        d.click_starting("Recordings");
        assert!(!d.app_mut().encounters.has_run());
        d.state_prepared("failed-elsewhere-with-unprocessed-row", Sizes::All, |d| {
            d.expect("Session 2: Unprocessed recording");
            d.expect("Not processed");
            d.expect("Processing “Session 1: Failed processing” failed");
            assert!(
                d.nodes()
                    .iter()
                    .any(|node| node.in_view() && node.name.starts_with("Process:"))
            );
            check_one_primary(d, "Continue processing", "iter-8-C-banner-failed-primary");
            // iter-23-S FE1 lookalike: with the audio there, the advice stays and no reason shows.
            d.observe(
                "iter-23-S-FE1-audio-there",
                d.visible_text("Wait, then select Continue processing.")
                    && !d.visible_text("Audio removed. Restore the WAV file"),
                "A retry that can run keeps its advice",
            );
            let size = d.window_size();
            check_choices_at_text(
                d,
                "iter-15-C-FD5",
                "Processing “Session 1: Failed processing” failed",
                &["Continue processing", "Open the session"],
                None,
                &[size],
            );
            check_dismiss_at_message_end(
                d,
                "iter-25-S-FM4-dismiss",
                "Processing “Session 1: Failed processing” failed",
            );
            // iter-25-S FM4, AM1: the banner's other choice has the secondary border.
            let frame = d.control_frame("Open the session");
            d.observe(
                "iter-25-S-FM4",
                frame.as_ref().is_some_and(|frame| frame.stroke.width > 0.0),
                format!("Open the session is a bordered secondary choice: {frame:?}"),
            );
        });
        let campaign = d.app_mut().campaigns.active_campaign().unwrap();
        let folder = crate::sessions::folder(&campaign).unwrap();
        let original_recordings: Vec<_> = crate::sessions::list_recordings(&folder, second)
            .into_iter()
            .map(|recording| recording.path)
            .collect();
        // Two faults at once: A can retry processing, while B needs recording recovery.
        d.click("Record this session");
        d.click("Start recording");
        d.app_mut()
            .sessions
            .simulate_recorder_health(Health::Stopped("microphone stopped".to_owned()));
        d.settle();
        d.state_prepared("failed-processing-and-stopped-recording", Sizes::All, |d| {
            d.expect("Processing “Session 1: Failed processing” failed");
            d.expect("microphone stopped");
            assert!(!d.app_mut().encounters.has_run());
            assert!(d.in_view("Continue processing") && !d.is_disabled("Continue processing"));
            check_one_primary(d, "Record again", "iter-9-C2-ST1-two-alerts");
            check_processing_action(d, "Process:", false);
        });
        assert!(d.in_view("Record again"));
        d.click("Record again");
        d.expect("Stop recording");
        d.click("Stop recording");
        d.expect("Processing “Session 1: Failed processing” failed");
        // Restore B's original plain recording for the existing blocked-retry checks.
        for recording in crate::sessions::list_recordings(&folder, second) {
            if !original_recordings.contains(&recording.path) {
                crate::sessions::delete_recording(&recording.path).unwrap();
            }
        }
        d.app_mut().sessions.open(first);
        d.app_mut().sessions.open(second);
        d.settle();
        d.click_starting("Recordings");
        let campaign = d.app_mut().campaigns.active_campaign().unwrap();
        let folder = crate::sessions::folder(&campaign).unwrap();
        let failed = crate::sessions::list_recordings(&folder, first)
            .remove(0)
            .path;
        let audio = std::fs::read(&failed).unwrap();
        std::fs::remove_file(&failed).unwrap();
        d.settle();
        d.state_prepared("blocked-banner-retry", Sizes::All, |d| {
            d.observe(
                "iter-9-C-ST1-blocked-availability",
                d.is_disabled("Continue processing"),
                "A banner retry with removed audio is blocked",
            );
            // iter-23-S FE1: the reason stands beside the blocked action, and the words no
            // longer tell the DM to select it.
            const GONE: &str = "Audio removed. Restore the WAV file to process it.";
            // A wrapped label in a wrapped row starts its first line after the controls before
            // it; measure that line.
            let mut reason = None;
            d.each_painted_text(|text, _| {
                if text.galley.job.text == GONE
                    && let Some(row) = text.galley.rows.first()
                {
                    let left = row.glyphs.first().map_or(0.0, |glyph| glyph.pos.x);
                    let min = text.pos + row.pos.to_vec2() + egui::vec2(left, 0.0);
                    reason = Some(egui::Rect::from_min_size(min, row.size));
                }
            });
            let button = d.place_of("Continue processing");
            d.observe(
                "iter-23-S-FE1",
                !d.visible_text("Wait, then select Continue processing")
                    && d.visible_text("The transcript and summary service is busy.")
                    && reason.zip(button).is_some_and(|(reason, button)| {
                        reason.left() >= button.right() && reason.center().y > button.top()
                            && reason.center().y < button.bottom()
                    }),
                format!("The blocked reason follows Continue processing on its line: {reason:?}, {button:?}"),
            );
            let process = d
                .nodes()
                .into_iter()
                .find(|node| {
                    node.in_view()
                        && node.role == egui::accesskit::Role::Button
                        && node.name.starts_with("Process:")
                })
                .unwrap()
                .name;
            check_one_primary(d, &process, "iter-9-C-ST1-blocked-banner");
        });
        std::fs::write(&failed, audio).unwrap();
        d.settle();
        assert!(d.in_view("Continue processing"));
        d.click("Continue processing");
        d.expect("Processed");
        assert!(!d.visible_text("The transcript and summary service is busy"));
        d.app_mut().sessions.open(first);
        d.settle();
        d.click("Record this session");
        d.click("Start recording");
        d.app_mut()
            .sessions
            .simulate_recorder_health(Health::Stalled);
        d.app_mut().sessions.open(second);
        d.settle();
        d.state_prepared("stalled-elsewhere-with-unprocessed-row", Sizes::All, |d| {
            d.expect("Session 2: Unprocessed recording");
            d.expect("Not processed");
            d.expect("The recording of “Session 1: Failed processing” stopped saving audio.");
            // Round 24 FC7: on the line below the session title, only the icon and `Not saving
            // audio` are red; the other session's time and title are muted.
            let mut parts = Vec::new();
            d.each_painted_text(|text, _| {
                let job = &text.galley.job;
                if job.text.contains("Not saving audio")
                    && job.text.contains(" in Session 1: Failed processing")
                {
                    parts.extend(job.sections.iter().map(|section| {
                        (
                            job.text[section.byte_range.clone()].to_owned(),
                            section.format.color,
                        )
                    }));
                }
            });
            let c = crate::design::colors();
            d.observe(
                "iter-24-S-FC7",
                parts
                    .iter()
                    .any(|(words, _)| words.contains("in Session 1"))
                    && parts.iter().all(|(words, color)| {
                        *color
                            == if words.contains("in Session 1") {
                                c.muted
                            } else {
                                c.result.error
                            }
                    }),
                format!("Only the state is red on the other session's line: {parts:?}"),
            );
            assert!(
                d.nodes()
                    .iter()
                    .any(|node| node.in_view() && node.name.starts_with("Process:"))
            );
            check_one_primary(d, "Record again", "iter-8-C-banner-stalled-primary");
        });
        assert!(d.in_view("Record again"));
        d.click("Record again");
        d.expect("Stop recording");
        d.observe(
            "iter-8-C-banner-restart",
            !d.visible_text("stopped saving audio"),
            "Record again clears the stalled recorder's warning",
        );
        d.app_mut()
            .sessions
            .simulate_recorder_health(Health::Stopped("microphone stopped".to_owned()));
        d.settle();
        d.click_starting("Recordings");
        d.state_prepared("recording-retry-with-input", Sizes::All, |d| {
            check_one_primary(d, "Record again", "iter-9-C-ST1-retry");
            check_processing_action(d, "Process:", false);
            assert!(d.visible_text("microphone stopped"));
        });
        assert!(d.in_view("Record again"));
        d.click("Record again");
        d.expect("Stop recording");
        d.app_mut()
            .sessions
            .simulate_recorder_health(Health::Stopped("no such device".to_owned()));
        d.settle();
        assert!(d.in_view("Choose an input"));
        d.click("Choose an input");
        d.expect("Recording input");

        // Dismiss alone: Session 1's recording stopped, another recording runs (so no Record
        // again), and Session 1's own Recordings tab is open behind its details form (so no Open
        // the session). Dismiss is then the first choice, at the message text.
        // A stopped recorder ends its recording. Record again starts Session 1 anew and clears
        // its old alert; the next stop has an error with no plain words, so no Choose an input.
        go_library(d);
        d.click("Record again");
        d.app_mut()
            .sessions
            .simulate_recorder_health(Health::Stopped("stream error 42".to_owned()));
        d.settle();
        d.app_mut().sessions.open(second);
        d.click(NAV_SESSIONS);
        d.click("Record this session");
        d.click("Start recording");
        d.expect("Stop recording");
        d.app_mut().sessions.open(first);
        d.settle();
        d.click_starting("Recordings");
        d.click("Edit session details");
        d.expect("Dismiss this error");
        assert!(!d.has("Open the session") && !d.has("Record again"));
        let size = d.window_size();
        let _ = size;
        check_dismiss_at_message_end(d, "iter-15-C-FD5-dismiss-alone", "stream error 42");
    }

    fn check_marker_columns(d: &mut Driver<'_>) {
        let add_name = "Add to timeline: Wolves at the bridge (0)";
        assert!(d.in_view(add_name) && d.in_view("On the timeline"));
        let add = d.place_of(add_name).expect("Add is visible");
        let added = d
            .place_of("On the timeline")
            .expect("On the timeline is visible");
        let title = d
            .place_of("Wolves at the bridge")
            .expect("The first marker is visible");
        let rest = d
            .place_of("Rest at the mill")
            .expect("The second marker is visible");
        let nodes = d.nodes();
        assert_eq!(
            nodes
                .iter()
                .filter(|node| node.in_view() && node.name.starts_with("Add to timeline:"))
                .count(),
            1
        );
        assert_eq!(
            nodes
                .iter()
                .filter(|node| node.in_view() && node.name == "On the timeline")
                .count(),
            1
        );
        let right = nodes
            .iter()
            .filter(|node| {
                node.in_view()
                    && (node.name == "Wolves at the bridge"
                        || node.name == "Rest at the mill"
                        || node.name == "00:00:00"
                        || node.name == "00:10:00"
                        || node.name.starts_with("Fight · Emberfall bridge")
                        || node.name.starts_with("Long rest · 8 hours"))
            })
            .map(|node| node.rect.right())
            .fold(title.right().max(rest.right()), f32::max);
        let gap = add.left() - right;
        d.observe(
            "iter-8-C-AT1",
            (add.left() - added.left()).abs() <= 1.0
                && (crate::design::L - 1.0..=crate::design::L + 2.0).contains(&gap)
                && add.left() - title.left() <= crate::design::READER_MEASURE + crate::design::L,
            format!("Marker actions align left with an L gap after the content: gap {gap}, Add {add:?}, added {added:?}"),
        );
    }

    fn check_duplicate_sessions(d: &mut Driver<'_>) {
        let rows: Vec<_> = d
            .nodes()
            .into_iter()
            .filter(|node| node.in_view() && node.name.starts_with("Session 1: The Bridge ·"))
            .collect();
        assert_eq!(rows.len(), 2, "Both equal session rows are visible");
        let campaign = d.app_mut().campaigns.active_campaign().unwrap();
        let folder = crate::sessions::folder(&campaign).unwrap();
        let notes = crate::sessions::list_notes(&folder);
        for note in notes.iter().filter(|note| note.number == 1) {
            let file = note.path.file_name().unwrap().to_string_lossy();
            d.observe(
                "iter-9-C-ST3-FT11",
                // iter-23-S FA11: a file name keeps its word spaces, so it wraps at one.
                rows.iter().any(|row| row.name.contains(file.as_ref())) && d.visible_text(&file),
                format!("Equal session row shows {file} and names it accessibly"),
            );
        }
    }

    fn check_destinations(d: &mut Driver<'_>) {
        let actions: Vec<_> = d
            .nodes()
            .into_iter()
            .filter(|node| {
                node.in_view()
                    && node.role == egui::accesskit::Role::Button
                    && node.name.starts_with("Process:")
            })
            .collect();
        assert_eq!(actions.len(), 2, "Two recording rows have Process");
        let destinations: Vec<_> = d
            .nodes()
            .into_iter()
            .filter(|node| {
                node.in_view()
                    && node.role == egui::accesskit::Role::Label
                    && node.name.starts_with("Process sends the audio")
            })
            .collect();
        d.observe(
            "iter-9-C-FT2",
            destinations.len() == 1,
            format!(
                "Two recordings share one visible destination: {}",
                destinations.len()
            ),
        );
        if let Some(destination) = destinations.first() {
            assert!(
                destination.rect.top() >= actions[0].rect.bottom()
                    && destination.rect.bottom() <= actions[1].rect.top()
            );
        }
    }

    /// Each party bar item's accessible node, and its first name with the words painted in it.
    pub(crate) fn party_bar_items(
        d: &mut Driver<'_>,
    ) -> (
        Vec<crate::visual_tests::driver::Found>,
        Vec<(String, String)>,
    ) {
        let items: Vec<_> = d
            .nodes()
            .into_iter()
            .filter(|node| node.name.contains("Passive Perception"))
            .collect();
        let painted = items
            .iter()
            .map(|item| {
                (
                    item.name
                        .split_whitespace()
                        .next()
                        .unwrap_or_default()
                        .to_owned(),
                    d.painted_lines_in(item.rect).join(" "),
                )
            })
            .collect();
        (items, painted)
    }

    /// Every party bar item paints its first name, and a state word that hover names stays.
    pub(crate) fn check_party_names(
        d: &mut Driver<'_>,
        id: &str,
        items: &[crate::visual_tests::driver::Found],
        painted: &[(String, String)],
    ) {
        let states = items.iter().zip(painted).all(|(item, (_, words))| {
            ["Bloodied", "0 HP"]
                .into_iter()
                .all(|state| !item.name.contains(state) || words.contains(state))
        });
        d.observe(
            id,
            items.len() >= 4
                && painted
                    .iter()
                    .all(|(first, words)| words.split_whitespace().any(|word| word == first))
                && states,
            format!("Every party bar item shows its first name and state word: {painted:?}"),
        );
    }

    /// The tops of Lookup, Notes and Settings in the rail, rounded to whole points.
    pub(crate) fn rail_targets(d: &Driver<'_>) -> Option<[i32; 3]> {
        let nodes = d.nodes();
        let top = |name: &str| {
            nodes
                .iter()
                .filter(|node| node.name == name)
                .min_by(|a, b| a.rect.left().total_cmp(&b.rect.left()))
                .map(|node| node.rect.top().round() as i32)
        };
        Some([top("Lookup")?, top("Notes")?, top("Settings")?])
    }

    /// FT-6: Lookup, Notes, Settings and the account line keep fixed places at the rail's
    /// bottom edge. The status slot grows upward above Lookup and never reaches under it.
    /// The account status in the rail: every node below Settings, together.
    pub(crate) fn account_rect(d: &Driver<'_>) -> Option<egui::Rect> {
        let nodes = d.nodes();
        let settings = nodes
            .iter()
            .filter(|node| node.name == "Settings")
            .min_by(|a, b| a.rect.left().total_cmp(&b.rect.left()))?
            .rect;
        nodes
            .iter()
            .filter(|node| {
                node.rect.top() >= settings.bottom() - 0.5
                    && node.rect.right() <= settings.right() + 1.0
                    && node.rect.height() < 100.0
            })
            .map(|node| node.rect)
            .reduce(|a, b| a.union(b))
    }

    pub(crate) fn check_rail_anchor(d: &mut Driver<'_>, id: &str) {
        let nodes = d.nodes();
        let (_, height) = d.window_size();
        let lookup = nodes
            .iter()
            .find(|node| node.name == "Lookup")
            .map(|node| node.rect);
        let Some(lookup) = lookup else {
            d.observe(id, false, "Lookup is in the rail");
            return;
        };
        let rail =
            |rect: &egui::Rect| rect.right() <= lookup.right() + 1.0 && rect.height() < 100.0;
        let settings = nodes
            .iter()
            .any(|node| node.name == "Settings" && rail(&node.rect));
        let slot_clear = nodes.iter().all(|node| {
            !rail(&node.rect)
                || node.rect.top() >= lookup.top() - 0.5
                || node.rect.bottom() <= lookup.top() + 0.5
        });
        // AD2: the account slot holds two `Small` lines and ends at the rail's bottom edge. The
        // status starts at the slot's top. One painted line leaves the second line empty.
        let expected = height - crate::design::S;
        let line = crate::design::Type::Small.line();
        let account = account_rect(d);
        // Painted rows at distinct heights: the icon's own column is not another line.
        let mut tops: Vec<f32> = Vec::new();
        if let Some(rect) = account {
            d.each_painted_text(|text, _| {
                for row in &text.galley.rows {
                    let row = row.rect().translate(text.pos.to_vec2());
                    if rect.contains(row.center())
                        && !tops.iter().any(|top| (top - row.top()).abs() < line / 2.0)
                    {
                        tops.push(row.top());
                    }
                }
            });
        }
        let lines = tops.len();
        let bottom = if lines >= 2 {
            expected
        } else {
            expected - line
        };
        d.observe(
            id,
            settings
                && slot_clear
                && (1..=2).contains(&lines)
                && account.is_some_and(|rect| {
                    (rect.top() - (expected - 2.0 * line)).abs() <= 0.5
                        && (rect.bottom() - bottom).abs() <= 0.5
                }),
            format!(
                "The account status ({lines} lines, {account:?}) starts at the top of its two-line \
                 slot ({}) and ends at {bottom}, below Settings; no status reaches under Lookup \
                 (top {})",
                expected - 2.0 * line,
                lookup.top()
            ),
        );
    }

    /// FT-3: a warned result is a status like its siblings, on the row's second line.
    /// FT3 (round 16): the recording row states an empty check once, in its warning line.
    fn check_empty_check_once(d: &mut Driver<'_>, id: &str) {
        let mut said = Vec::new();
        d.each_painted_text(|text, _| {
            let words = &text.galley.job.text;
            if words.contains("returned nothing") || words.contains("returned no result") {
                said.push(words.clone());
            }
        });
        d.observe(
            &format!("{id}-warning"),
            d.visible_text(WARNING),
            "The warning line still says the check returned nothing",
        );
        d.observe(
            id,
            said.len() == 1,
            format!("The empty check is said once on screen: {said:?}"),
        );
    }

    fn check_warned_row(d: &mut Driver<'_>) {
        let row = d.nodes().into_iter().find(|node| {
            node.in_view()
                && node.name.starts_with("Session 1: The Bridge ·")
                && node.name.contains("Processed, with a warning")
        });
        let Some(row) = row else {
            d.observe(
                "iter-10-C-FT3",
                false,
                "The warned session row names its warning",
            );
            return;
        };
        let lines = d.painted_lines_in(row.rect);
        let second = lines.get(1).map(|line| line.replace('\u{a0}', " "));
        d.observe(
            "iter-10-C-FT3",
            lines.len() == 2
                && second.as_deref().is_some_and(|line| {
                    line.starts_with(&format!("{} Processed · ", crate::design::icons::WARN))
                        && line.contains("Playing")
                        && !line.contains("with a warning")
                }),
            format!("The warned row has two lines: title, then the warning icon, Processed and Playing: {lines:?}"),
        );
        let hover = d.painted_tooltip_shows("Processed ·", "Processed, with a warning");
        d.observe(
            "iter-10-C-FT3-hover",
            hover,
            "Hover keeps the full words Processed, with a warning",
        );
    }

    /// FT-3, AT2: a clean result in the session list is a status: the check icon, then
    /// `Processed` in the success colour, as on the recording's own row.
    fn check_clean_row(d: &mut Driver<'_>, id: &str) {
        let rows: Vec<_> = d
            .nodes()
            .into_iter()
            .filter(|node| {
                node.in_view()
                    && node.name.starts_with("Session ")
                    && node.name.contains(" · ")
                    && node.name.contains("Processed")
                    && !node.name.contains("with a warning")
            })
            .collect();
        let success = crate::design::colors().result.success;
        let word = format!("{} Processed", crate::design::icons::OK);
        let mut seen = Vec::new();
        let clean = rows.iter().all(|row| {
            // The spoken name says Processed without the icon's private character.
            let spoken = !row.name.contains(crate::design::icons::OK)
                && (row.name.contains(" · Processed") || row.name.contains("\nProcessed"));
            let mut colored = false;
            d.each_painted_text(|text, _| {
                if !row.rect.contains(text.visual_bounding_rect().center()) {
                    return;
                }
                let job = &text.galley.job;
                let all = job.text.replace('\u{a0}', " ");
                for section in &job.sections {
                    let words = &job.text[section.byte_range.clone()];
                    if words.contains("Processed") {
                        seen.push((all.clone(), section.format.color));
                        colored |= all.contains(&word) && section.format.color == success;
                    }
                }
            });
            colored && spoken
        });
        let names: Vec<_> = rows.iter().map(|row| row.name.clone()).collect();
        d.observe(
            id,
            !rows.is_empty() && clean,
            format!(
                "Each clean Processed row shows the check icon and Processed in the success \
                 colour, and its name says Processed without the icon: {} rows, {seen:?}, \
                 {names:?}",
                rows.len()
            ),
        );
    }

    fn check_result_row(d: &mut Driver<'_>) {
        let rows: Vec<_> = d
            .nodes()
            .into_iter()
            .filter(|node| {
                node.in_view()
                    && node.name.starts_with("Session 1: The Bridge ·")
                    && node.name.contains("Processed")
            })
            .collect();
        assert_eq!(rows.len(), 1, "The processed session row is visible");
        let lines = d.painted_lines_in(rows[0].rect);
        assert!(!lines.is_empty(), "The processed row has laid-out text");
        if !rows[0].name.contains("with a warning") {
            check_clean_row(d, "iter-11-C-FT3");
        }
        d.observe(
            "iter-9-C2-FT4-lines",
            lines.iter().all(|line| !line.trim_start().starts_with('·')),
            format!("No laid-out session row line starts with a separator: {lines:?}"),
        );
        d.observe(
            "iter-9-C-FT4",
            !rows[0].name.contains("2026-01-01")
                && rows[0].name.contains("Playing")
                && !rows[0].name.contains("\n·"),
            "Processed state replaces the date; Playing has no leading separator on a new line",
        );
    }

    fn choose_view(d: &mut Driver<'_>, name: &str) {
        if d.has("Recording view") {
            d.click("Recording view");
        }
        d.click(name);
    }

    pub fn run(d: &mut Driver<'_>) {
        d.click(NAV_SESSIONS);
        new_session(d, TITLE);

        // A stale input stops here, rather than record from another microphone.
        d.app_mut().processing.simulate_input("gone.mic");
        d.settle();
        d.observe(
            "T11.5",
            d.is_disabled("Record this session"),
            "A chosen input that is gone blocks recording",
        );
        d.app_mut().processing.simulate_input("table.mic");
        d.settle();

        // T11.5: nothing records without a stated step.
        d.click("Record this session");
        d.expect("Start recording");
        check_choices_at_text(
            d,
            "iter-15-C-ST2",
            "Tell everyone at the table first.",
            &["Start recording", "Not now"],
            None,
            &[DEFAULT],
        );
        // iter-25-S FK2, SK1, AL2: Not now is a bordered secondary choice, as in Process again.
        let not_now = d.control_frame("Not now");
        d.observe(
            "iter-25-S-FK2",
            not_now
                .as_ref()
                .is_some_and(|frame| frame.stroke.width > 0.0),
            format!("Not now has the secondary border: {not_now:?}"),
        );
        d.state("record-consent", Sizes::Default);
        d.observe(
            "T11.5",
            d.has_containing(&format!("Record “Session 1: {TITLE}”.")),
            "The step names the session the audio goes to",
        );
        d.observe(
            "T11.5",
            d.has_containing("Tell everyone at the table first"),
            "Recording starts only after the DM is told to tell the table",
        );
        d.click("Start recording");
        d.expect("Stop recording");
        d.expect("Waiting for sound");
        d.state("recording-starting", Sizes::Default);
        d.app_mut()
            .sessions
            .simulate_recorder_health(Health::Hearing);
        d.settle();
        d.expect("Hearing sound");
        d.expect_same("Only the small sound-health label changes from Waiting to Hearing sound.");
        d.state_prepared("recording-hearing", Sizes::AllAndHiDpi, |d| {
            d.app_mut()
                .sessions
                .simulate_recorder_health(Health::Hearing);
        });
        // Capturing several sizes can outlast the five-second sound indicator.
        d.app_mut()
            .sessions
            .simulate_recorder_health(Health::Hearing);
        d.click_starting("Recordings");
        d.expect(&format!("{} Recording", crate::design::icons::RECORD));
        d.expect("0:00 recorded · Hearing sound");
        d.state_prepared("recording-listed", Sizes::Default, |d| {
            // AT1: with no fight open, the party bar keeps every first name at 960. Portraits give
            // way before names; HP and state words stay.
            let (items, painted) = party_bar_items(d);
            check_party_names(d, "iter-11-C-AT1", &items, &painted);
        });
        let tabs_top = |d: &Driver<'_>| {
            d.place_of("Plan")
                .or_else(|| d.place_of("Plan and notes"))
                .map(|rect| rect.top())
        };
        let hearing_top = tabs_top(d).expect("Session tabs are visible");
        d.app_mut()
            .sessions
            .simulate_recorder_health(Health::Silent);
        d.settle();
        assert_eq!(tabs_top(d), Some(hearing_top));
        d.expect("Silent for over a minute · 0:00 recorded");
        d.app_mut()
            .sessions
            .simulate_recorder_health(Health::Hearing);
        d.settle();
        assert_eq!(tabs_top(d), Some(hearing_top));
        write_note(d);

        d.app_mut()
            .sessions
            .simulate_recorder_health(Health::Silent);
        d.settle();
        d.expect("Silent for over a minute");
        // On the run, the same words must not paint over a long Undo.
        go_encounters(d);
        resume_run_from_cold_start(d);
        d.click_row("Ember Cultist");
        d.type_text("7");
        d.press(Key::Enter);
        d.observe(
            "T11.5",
            d.has_containing("Undo: 7 damage") && d.has_containing("Recording silence"),
            "A silent input shows on the run, beside a long Undo",
        );
        d.state("run-silent", Sizes::All);
        let silence = d.at_size(SMALL, |d| {
            d.visible_text("Recording silence") && d.visible_text("Sync off for the party")
        });
        d.observe(
            "R1",
            silence,
            "Silence and the full party sync state fit the run notice at 960x640",
        );
        d.click_containing("Undo: 7 damage");
        close_run(d);
        d.click_containing("Recording silence ·");
        d.expect("Silent for over a minute");
        d.expect("Check the input in Settings");
        d.state_prepared("recording-silent", Sizes::All, |d| {
            let detail = "Silent for over a minute · 0:00 recorded";
            assert!(d.visible_text(detail) && d.in_view("Stop recording"));
            d.observe(
                "iter-9-C-FT13",
                d.painted_color(detail) == Some(crate::design::colors().muted)
                    && d.painted_size(d.place_of(detail).unwrap(), detail)
                        == Some(crate::design::Type::Small.size()),
                "Silence detail is Small and muted; its status and Stop stay visible",
            );
        });
        d.observe_both(
            "iter-12-R2",
            "The silent row owns its detail and stop action",
            |d| {
                d.visible_text("Silent for over a minute · 0:00 recorded")
                    && d.in_view("Stop recording")
                    && d.in_view("Check the input in Settings")
            },
        );
        d.app_mut()
            .sessions
            .simulate_recorder_health(Health::Stalled);
        d.settle();
        d.expect("Stopped saving after");
        d.state("recording-stalled", Sizes::All);
        d.observe(
            "T11.5",
            d.has("Record again"),
            "A stall offers one press to record again",
        );
        d.observe(
            "T11.5",
            d.has_containing("Stopped saving after") && d.has_containing("What was saved is kept"),
            "The stalled row owns its reason and its recovery action",
        );

        // A failing recording reaches the DM on any other page.
        go_library(d);
        d.observe(
            "T11.5",
            d.has_containing("Not saving audio"),
            "A stall shows in the rail on other pages",
        );
        // Round 24 FJ7: the banner says what stopped and what is kept, in two sentences.
        d.observe(
            "iter-24-S-FJ7",
            d.has_containing("” stopped saving audio. The first ")
                && d.has_containing(" are kept. Record again starts a new file."),
            "The stall banner says the recording stopped saving and how much is kept",
        );
        d.state("recording-stalled-elsewhere", Sizes::Default);
        d.app_mut()
            .sessions
            .simulate_recorder_health(Health::Hearing);
        d.settle();
        d.observe(
            "T11.5",
            d.has_containing("Recording 0:00") && d.has_containing("Recording"),
            "Recording, with its time, stays in the rail on other pages",
        );
        d.state("recording-on-another-page", Sizes::Default);
        // A close does not end it by surprise.
        d.request_close();
        d.expect("Keep recording");
        assert_eq!(d.focused_name().as_deref(), Some("Keep recording"));
        d.state("recording-close-prompt", Sizes::Default);
        d.click("Keep recording");
        d.observe(
            "T11.5",
            d.page_title().as_deref() == Some("Library"),
            "Keep working leaves the DM on the page they were on",
        );
        d.click_containing("Recording 0:");
        d.expect("Stop recording");
        d.click("Stop recording");
        d.expect("saved: 2026-01-01 19:00");

        d.expect(RECORDING);
        d.observe(
            "T11.5",
            d.has_containing("Not processed"),
            "Stop keeps the recording in the session",
        );
        d.state("recording-saved-no-key", Sizes::Default);

        // T11.6: the default services are named, and what is missing is said.
        d.observe(
            "T11.6",
            d.has_containing("Save an OpenAI key in Settings"),
            "Process states what is missing before it can run",
        );
        d.click("Save an OpenAI key in Settings");
        d.expect("SPEECH SERVICE");
        d.expect("You cannot process yet. Save an OpenAI key under Speech service.");
        d.observe_both(
            "iter-18-A-key",
            "The speech key is focused and in view",
            |d| {
                d.focused_name().as_deref() == Some("Speech API key")
                    && d.in_view("Speech API key")
                    && d.in_view_containing(
                        "You cannot process yet. Save an OpenAI key under Speech service.",
                    )
            },
        );
        d.state("processing-settings", Sizes::All);
        // Round 24 FK6, FM1: help on this tab keeps the reader measure, as Account help does.
        let helps = [
            "Recording is your choice in each session.",
            "Turns your recording into text.",
            "Goes over the transcript twice:",
        ]
        .map(|start| {
            d.nodes()
                .iter()
                .find(|node| node.name.starts_with(start))
                .map(|node| node.rect.width())
        });
        d.observe(
            "iter-24-S-FK6",
            helps.iter().all(|width| {
                width.is_some_and(|width| width <= crate::design::READER_MEASURE + 1.0)
            }),
            format!("Recording help widths stay within the reader measure: {helps:?}"),
        );
        // Round 24 AM2, FM2, FK1, SK1: the DM's words for the second service, not the code's.
        d.observe(
            "iter-24-S-AM2",
            d.has("Transcript and summary instructions")
                && !d.has_containing("LLM")
                && !d.has_containing("sandbox"),
            "The instructions section and the Codex help use the DM's words",
        );
        d.observe(
            "T11.6",
            d.field_text("Transcript and summary model").as_deref() == Some("gpt-6.1-sol")
                && d.field_text("Speech model").as_deref() == Some("gpt-transcribe"),
            "The defaults are Sol through the Codex CLI, after OpenAI's gpt-transcribe",
        );
        d.scroll_until(pos2(600.0, 400.0), "Transcript and summary service", 12);
        d.click("Transcript and summary service");
        d.state_prepared("processing-llm-choices", Sizes::Default, |d| {
            if !d.has("Anthropic") {
                d.click("Transcript and summary service");
            }
            d.wait_for_text("Anthropic");
        });
        d.click("Anthropic");
        d.state_prepared("processing-settings-anthropic", Sizes::All, |d| {
            // iter-25-S FK4: `Save key` ends at the edge the other fields end at, and its reason
            // stands on its own line below the key field.
            let nodes = d.nodes();
            let edge = d
                .place_of("Transcript and summary model")
                .map(|rect| rect.right());
            let saves: Vec<egui::Rect> = nodes
                .iter()
                .filter(|node| node.name.starts_with("Save key:") && node.in_view())
                .map(|node| node.rect)
                .collect();
            let reasons: Vec<egui::Rect> = nodes
                .iter()
                .filter(|node| node.name == "Paste a key first." && node.in_view())
                .map(|node| node.rect)
                .collect();
            d.observe(
                "iter-25-S-FK4",
                !saves.is_empty()
                    && edge.is_some_and(|edge| {
                        saves.iter().all(|save| (save.right() - edge).abs() < 1.0)
                    })
                    && !reasons.is_empty()
                    && reasons
                        .iter()
                        .all(|reason| saves.iter().any(|save| reason.top() >= save.bottom())),
                format!("Save key {saves:?} ends at {edge:?}; reasons {reasons:?} sit below"),
            );
        });
        d.click("Transcript and summary service");
        d.click("Codex CLI, GPT-6.1 Sol");
        d.click("Account and campaigns");

        d.app_mut().processing.config.provider = crate::processing::Provider::OpenAiCompatible;
        d.app_mut().processing.config.llm_url = crate::processing::OPENAI_URL.to_owned();
        d.app_mut().processing.simulate_keys();
        d.settle();
        // A key saved for one service is not sent to another.
        d.click("Recording and processing");
        d.scroll_until(pos2(600.0, 400.0), "Transcript and summary service", 12);
        d.click("Transcript and summary service");
        d.click("Anthropic");
        d.observe(
            "T11.6",
            d.has_containing(
                "The saved key is for OpenAI, so Splat will not send it to Anthropic.",
            ),
            "A saved key does not follow the DM to a different service",
        );
        d.expect("Remove speech key (OpenAI)");
        d.state_prepared("key-for-another-service", Sizes::All, |d| {
            d.scroll_until(
                pos2(600.0, 400.0),
                "Remove transcript and summary key (OpenAI): Transcript and summary API key",
                12,
            );
            d.expect("Remove transcript and summary key (OpenAI)");
            d.observe(
                "iter-2-C-AT5",
                d.has_containing("Remove transcript and summary key (OpenAI)"),
                "The key removal action names the transcript and summary task",
            );
            // Round 24 FK3, SM1: a saved key says where it is in the DM's words.
            d.observe(
                "iter-24-S-FK3",
                d.has_containing("Saved on this computer.") && !d.has_containing("keyring"),
                "A saved key says it is saved on this computer",
            );
        });
        d.click("Transcript and summary service");
        d.click("Codex CLI, GPT-6.1 Sol");
        d.click("Account and campaigns");
        d.click(NAV_SESSIONS);
        d.expect("names to api.openai.com");
        d.state_prepared("recording-ready", Sizes::All, |d| {
            check_processing_action(d, "Process:", true)
        });
        d.observe_both(
            "iter-18-A-destination",
            "The enabled Process action owns its measured help below",
            |d| {
                let nodes = d.nodes();
                let button = nodes.iter().find(|n| n.name.starts_with("Process:"));
                let help = nodes.iter().find(|n| n.name.starts_with("Process sends"));
                button.zip(help).is_some_and(|(button, help)| {
                    help.rect.top() >= button.rect.bottom()
                        && help.rect.width() <= crate::design::READER_MEASURE
                })
            },
        );
        let recording_top = d.place_of("2026-01-01 19:00").unwrap().top() - tabs_top(d).unwrap();
        // Service help stays at the reader measure; recording rows keep the column.
        let notice = d.at_size(LARGE, |d| {
            d.nodes()
                .into_iter()
                .find(|node| node.name.starts_with("Process sends"))
                .map_or(0.0, |node| node.rect.width())
        });
        d.observe(
            "Column resize",
            notice > 0.0 && notice <= crate::design::READER_MEASURE,
            format!("The processing help stays at the reader measure, {notice} wide"),
        );
        d.observe(
            "T11.6",
            d.has_containing("events go to Codex (gpt-6.1-sol) twice"),
            "The page says where the audio and the words will go",
        );

        // Each state of a job, in words.
        d.app_mut()
            .sessions
            .simulate_job(Progress::Transcribing { part: 3, of: 24 });
        d.settle();
        d.expect("Transcribing part 3 of 24");
        assert_eq!(
            d.place_of("2026-01-01 19:00").unwrap().top() - tabs_top(d).unwrap(),
            recording_top
        );
        d.state("processing-speech", Sizes::Default);
        d.request_close();
        d.expect("Continue processing goes on from there");
        assert_eq!(d.focused_name().as_deref(), Some("Keep processing"));
        d.state("processing-close-prompt", Sizes::Default);
        d.click("Keep processing");
        d.app_mut()
            .sessions
            .simulate_job_end(Err(CANCELLED.to_owned()));
        d.settle();
        d.expect("Cancelled");
        d.expect("Continue processing sends the rest of the audio (parts 3 to 24)");
        d.state_prepared("processing-cancelled", Sizes::All, |d| {
            d.observe(
                "iter-9-C-FT10",
                d.rail_words().contains("Processing cancelled"),
                "The rail names what was cancelled",
            );
            check_rail_anchor(d, "iter-10-C-FT6-cancelled");
            let cancelled = d
                .nodes()
                .into_iter()
                .find(|node| node.name.starts_with("Session 1: The Bridge ·"))
                .map(|node| d.painted_lines_in(node.rect))
                .unwrap_or_default();
            d.observe(
                "iter-10-C-FT3-siblings",
                cancelled.len() == 2
                    && cancelled[1]
                        .starts_with(&format!("{}\u{a0}Cancelled", crate::design::icons::WARN)),
                format!("Other session states keep their icon and two lines: {cancelled:?}"),
            );
            assert!(
                d.has_containing("Session 1: The Bridge ·")
                    && d.visible_text("Continue processing")
            );
        });
        d.app_mut().sessions.simulate_job(Progress::Asking {
            pass: crate::processing::Pass::Review,
            seconds: 42,
        });
        d.settle();
        d.expect("Pass 2 of 2: checking the first pass, 42 s");
        d.observe(
            "iter-2-R1",
            !d.has_containing("Cancelled at part")
                && !d.has_containing("Processing it again goes on"),
            "A new processing pass hides the old cancellation reason",
        );
        d.state("processing-second-pass", Sizes::Default);
        d.observe(
            "iter-12-R1",
            !d.has_containing("Processing audio") && d.visible_text("Processing"),
            "LLM work says Processing",
        );
        d.app_mut().sessions.simulate_job_end(Err(
            "The transcript and summary service is busy, or your limit is reached. Wait, then select Continue processing."
                .to_owned(),
        ));
        d.settle();
        // A failure reaches the DM on any other page, and a routine message cannot hide it.
        go_library(d);
        d.observe(
            "T11.6",
            d.has_containing("Processing failed"),
            "A failed job stays in the rail on other pages",
        );
        d.state_prepared("processing-failed-elsewhere", Sizes::All, |d| {
            d.observe(
                "iter-7-C-ST1",
                d.visible_text("The transcript and summary service is busy"),
                "Processing failures use the configured service role from Settings",
            );
        });
        d.click(NAV_SESSIONS);
        d.expect("your limit is reached");
        d.expect("Continue processing sends the raw transcript");
        d.expect("The audio is not sent again.");
        d.state_prepared("processing-failed", Sizes::All, |d| {
            check_processing_action(d, "Continue processing:", true);
            // iter-23-S FL4: the failure's sentence wraps at the reader's line length, as the
            // destination below it does.
            let mut widths = Vec::new();
            d.each_painted_text(|text, _| {
                if text.galley.job.text.contains("your limit is reached") {
                    widths.push(text.galley.size().x);
                }
            });
            d.observe(
                "iter-23-S-FL4",
                !widths.is_empty()
                    && widths
                        .iter()
                        .all(|width| *width <= crate::design::READER_MEASURE + 1.0),
                format!(
                    "At {:?}, the failure reason is no wider than the reader measure: {widths:?}",
                    d.window_size()
                ),
            );
        });

        // The open recording row owns the retry and clears the old failure.
        d.click_containing("Continue processing:");
        d.expect("is processed");
        d.observe(
            "T11.6",
            !d.has_containing("Processing failed") && !d.has_containing("your limit is reached"),
            "A successful retry clears the failure",
        );

        // T11.7: a marker is a suggestion until the DM adds it.
        d.expect("EVENTS MARKED IN THIS RECORDING 2");
        d.state_prepared("recording-processed", Sizes::AllAndHiDpi, check_result_row);
        let marker_positions: Vec<_> = [SMALL, DEFAULT, LARGE]
            .into_iter()
            .map(|size| {
                let x = d.at_size(size, |d| {
                    assert!(d.in_view("Add to timeline: Wolves at the bridge (0)"));
                    d.place_of("Add to timeline: Wolves at the bridge (0)")
                        .unwrap()
                        .left()
                });
                (size, x)
            })
            .collect();
        d.click(NAV_TIMELINE);
        d.observe(
            "T11.7",
            !d.has_containing("Rest at the mill"),
            "Markers do not reach the timeline by themselves",
        );
        d.click(NAV_SESSIONS);
        d.click_containing("Add to timeline: Rest at the mill");
        d.expect("On the timeline");
        d.observe_both("H1 recording add", "The recording row owns one Undo", |d| {
            d.has("Undo add")
                && d.nodes()
                    .iter()
                    .filter(|node| node.in_view() && node.name.starts_with("Undo"))
                    .count()
                    == 1
        });
        let notice_seen = !d.app_mut().sessions.job_done();
        d.observe(
            "iter-4-C-ST1-AT3",
            d.rail_words().contains("Processed")
                && !d.rail_words().contains("Recording saved")
                && notice_seen,
            "Opening a processed session acknowledges the notice and keeps Processed in the rail",
        );
        d.expect("These events were suggested from the transcript. Only events you add go on the timeline.");
        d.state_prepared("marker-added", Sizes::All, |d| {
            // Round 24 FK5: the result is not held to the marker rows' width, so `Undo add`
            // follows its words on the message line.
            // iter-25-S FK7: the marker's Add result names the session, as every Add result does.
            let words = d.place_of("Rest at the mill added to Session 1.");
            let undo = d.place_of("Undo add");
            d.observe(
                "iter-24-S-FK5",
                words.zip(undo).is_some_and(|(words, undo)| {
                    undo.left() > words.right() && undo.top() < words.bottom()
                }),
                format!("`Undo add` sits on the message line: {words:?}, {undo:?}"),
            );
            check_events_gap(d, "iter-15-C-FT2");
            check_marker_columns(d);
            check_clean_row(d, "iter-11-C-FT3-marker");
            let size = d.window_size();
            let before = marker_positions
                .iter()
                .find(|(at, _)| *at == size)
                .unwrap()
                .1;
            let after = d
                .place_of("Add to timeline: Wolves at the bridge (0)")
                .unwrap()
                .left();
            d.observe(
                "iter-9-C-FT3",
                (after - before).abs() <= 1.0,
                format!("Adding a marker keeps the next Add at {before}, now {after}"),
            );
        });
        d.observe(
            "T11.7",
            d.has("Add to timeline: Wolves at the bridge (0)"),
            "Adding one marker leaves the other a suggestion",
        );

        // Each text reads in the whole pane, a part at a time, and a marker opens its place.
        d.click("00:10:00");
        d.expect("Part 2 of 2");
        d.expect("On the timeline");
        d.observe(
            "Transcript marker",
            d.visible_text("Long rest: Rest at the mill")
                && d.in_view_containing("On the timeline"),
            "The marker heading and its timeline state are visible together in the transcript",
        );
        d.state("marked-transcript", Sizes::All);
        d.click("On the timeline");
        d.expect("Copy link");
        d.observe(
            "T11.7",
            d.has_containing("Rest at the mill"),
            "The transcript's added marker opens its event on the timeline",
        );
        d.state("transcript-marker-on-timeline", Sizes::Default);
        d.observe_both(
            "iter-10-S3-wide",
            "Wide Timeline keeps its header and form edges aligned",
            |d| {
                let field = d
                    .nodes()
                    .into_iter()
                    .find(|node| {
                        node.name == "Title" && node.role == egui::accesskit::Role::TextInput
                    })
                    .map(|node| node.rect);
                let menu = d
                    .nodes()
                    .into_iter()
                    .find(|node| node.name.starts_with("More actions for "));
                field
                    .zip(menu)
                    .is_some_and(|(field, menu)| (field.right() - menu.rect.right()).abs() < 1.0)
            },
        );
        d.click(NAV_SESSIONS);
        d.expect("Earlier part");
        d.click("Earlier part");
        d.expect("Part 1 of 2");
        choose_view(d, "What the check changed");
        d.expect("Restored");
        d.state("second-pass-review", Sizes::Default);
        choose_view(d, "Summary");
        let viewer_layout = d.at_size(SMALL, |d| {
            let back = d.place_of("Back to recordings").unwrap();
            let add = d.place_of("Add to notes").unwrap();
            let views: Vec<_> = [
                "Summary",
                "Clean transcript",
                "What the check changed",
                "Raw transcript",
            ]
            .iter()
            .filter_map(|name| d.place_of(name))
            .collect();
            (back.center().y - add.center().y).abs() < 1.0
                && views.len() == 4
                && views.iter().all(|view| {
                    view.top() >= back.bottom() && (view.top() - views[0].top()).abs() < 1.0
                })
                && (views[0].left() - back.left()).abs() < 1.0
        });
        d.observe(
            "iter-2-R11",
            viewer_layout,
            "Navigation and Add to notes share a line; all four view choices stay on the next line",
        );
        d.expect("This draft was written automatically. Check it against your memory.");
        d.state_prepared("recording-summary", Sizes::All, |d| {
            // Round 24 AM1: the viewer is one block. Prose and its rules keep the reader
            // measure, and Add to notes ends at the block's edge, not the pane's far edge.
            let measure = crate::design::READER_MEASURE;
            let back = d.place_of("Back to recordings").unwrap();
            let add = d.place_of("Add to notes").unwrap();
            let views = d.place_of("Raw transcript").unwrap();
            let below = egui::Rect::from_min_max(
                pos2(back.left(), views.bottom()),
                pos2(f32::INFINITY, f32::INFINITY),
            );
            let rules = d.hairlines_in(below);
            let mut prose = Vec::new();
            d.each_painted_text(|text, clip| {
                let bounds = text.visual_bounding_rect();
                if below.contains(bounds.min) && clip.intersects(bounds) {
                    prose.push(bounds.width());
                }
            });
            let block = measure.max(views.right() - back.left());
            d.observe(
                "iter-24-S-AM1",
                !rules.is_empty()
                    && !prose.is_empty()
                    && rules.iter().all(|rule| rule.width() <= measure + 1.0)
                    && prose.iter().all(|width| *width <= measure + 1.0)
                    && add.right() - back.left() <= block + 1.0,
                format!(
                    "Prose and rules keep {measure} px and Add to notes ends at the block: rules {rules:?}, prose {prose:?}, add {add:?}, back {back:?}"
                ),
            );
        });
        d.click("Back to recordings");

        // A warning lasts as long as the result, past the message and the rail.
        d.app_mut().sessions.simulate_job(Progress::Asking {
            pass: crate::processing::Pass::Review,
            seconds: 9,
        });
        d.settle();
        go_party(d);
        d.app_mut().sessions.simulate_job_end_warned(WARNING);
        d.settle();
        d.observe(
            "T11.6",
            d.has_containing("Processed, with a warning") && d.rail_words().contains("Processed"),
            "A processed result keeps its warning in the rail and on other pages",
        );
        d.state("processing-warned-elsewhere", Sizes::All);
        d.click(NAV_SESSIONS);
        d.observe(
            "T11.6",
            d.has_containing("Processed, with a warning"),
            "The recording's own row keeps the warning",
        );
        let rail_warned = d.app_mut().sessions.job_warned();
        d.observe(
            "iter-4-C-ST1-AT3-warning",
            d.rail_words().contains("Processed") && rail_warned,
            "Opening a warned session keeps the processed warning in the rail",
        );
        d.state_prepared("recording-warned", Sizes::All, |d| {
            check_result_row(d);
            check_warned_row(d);
            check_empty_check_once(d, "iter-16-C-FT3");
        });

        d.click_containing("More actions for 2026-01-01");
        d.click_containing("Process again:");
        d.expect("Replace the results");
        d.state_prepared("process-again-confirm", Sizes::Default, |d| {
            // ST1: the confirmation keeps the reader's measure, with both choices together below.
            let mut message = None;
            d.each_painted_text(|text, _| {
                if text.galley.job.text.contains("The audio is not sent again") {
                    message = Some(text.visual_bounding_rect());
                }
            });
            let buttons = d.place_of("Not now").zip(d.place_of("Replace the results"));
            check_choices_at_text(
                d,
                "iter-15-C-AT4",
                "The audio is not sent again",
                &["Not now", "Replace the results"],
                Some("Replace the results"),
                &[DEFAULT],
            );
            // AT1: a check that ran and gave nothing back is not a check that never ran.
            d.observe(
                "iter-15-C-AT1",
                d.visible_text(WARNING) && !d.visible_text("No check ran"),
                "The warned recording says the check returned nothing, not No check ran",
            );
            check_empty_check_once(d, "iter-16-C-FT3-confirm");
            d.observe(
                "iter-11-C-ST1",
                message.zip(buttons).is_some_and(|(message, (not_now, replace))| {
                    message.width() <= crate::design::READER_MEASURE + 0.5
                        && not_now.top() >= message.bottom()
                        && (not_now.center().y - replace.center().y).abs() < 1.0
                        && replace.left() > not_now.right()
                        && replace.left() - not_now.right() <= crate::design::L + crate::design::S
                }),
                format!(
                    "The confirmation is at most {} wide, with Not now and Replace the results together below it: {message:?} {buttons:?}",
                    crate::design::READER_MEASURE
                ),
            );
        });
        d.click("Not now");

        // FT3 (round 16): processing again hides the warning line, so the label says it once.
        d.app_mut().sessions.simulate_job(Progress::Asking {
            pass: crate::processing::Pass::Review,
            seconds: 4,
        });
        d.settle();
        assert!(
            d.has_containing("Processing") && !d.visible_text(WARNING),
            "Processing again hides the warning line"
        );
        d.state_prepared("recording-processing-again", Sizes::All, |d| {
            let mut said = Vec::new();
            d.each_painted_text(|text, _| {
                let words = &text.galley.job.text;
                if words.contains("returned nothing") || words.contains("returned no result") {
                    said.push(words.clone());
                }
            });
            d.observe(
                "iter-16-C-FT3-processing",
                said == [crate::processing::NO_CHECK_LABEL],
                format!("While processing again, the label alone says the check returned nothing: {said:?}"),
            );
        });
        d.app_mut().sessions.simulate_job_end_warned(WARNING);
        d.settle();
        d.expect(WARNING);

        d.click_containing("More actions for 2026-01-01");
        d.expect("Delete recording:");
        check_events_gap(d, "iter-15-C-FT2-menu");
        d.state_prepared("recording-row-menu", Sizes::All, |d| {
            check_empty_check_once(d, "iter-16-C-FT3-menu");
        });
        d.click_containing("Delete recording:");
        d.expect("Keep the recording");
        assert_eq!(d.focused_name().as_deref(), Some("Keep the recording"));
        d.state("recording-delete", Sizes::Standard);
        d.click("Delete recording");
        d.expect("Recording deleted.");
        let warned_after_delete = d.app_mut().sessions.job_warned();
        d.observe(
            "iter-4-C-ST1-deleted",
            !d.rail_words().contains("Processed") && !warned_after_delete,
            "Deleting the processed recording takes Processed out of the rail",
        );

        // A recorder that ends by itself says so, wherever the DM is.
        d.click("Record this session");
        d.click("Start recording");
        go_library(d);
        d.app_mut()
            .sessions
            .simulate_recorder_health(Health::Stopped("no such device".to_owned()));
        d.settle();
        d.observe(
            "T11.5",
            d.has_containing("Recording stopped"),
            "A recording that stops by itself stays in the rail on other pages",
        );
        // iter-25-S AM1: Record again and Open the session are bordered; dismiss is the icon.
        for name in ["Record again", "Open the session"] {
            let frame = d.control_frame(name);
            d.observe(
                "iter-25-S-AM1",
                frame.as_ref().is_some_and(|frame| frame.stroke.width > 0.0),
                format!("{name} in the banner has a border: {frame:?}"),
            );
        }
        d.state("recording-stopped-elsewhere", Sizes::Default);
        d.expect("Choose an input");
        d.observe(
            "iter-10-C-FT5-banner",
            d.has_containing("stopped by itself: the microphone is no longer connected")
                && !d.visible_text("no such device"),
            "The banner puts the recorder's error in plain words",
        );
        let hover = d.painted_tooltip_shows(
            "stopped by itself: the microphone",
            "The recorder said: no such device.",
        );
        d.observe(
            "iter-15-C-FD5-hover",
            hover,
            "The banner's words keep the recorder's own words on hover",
        );
        d.click(NAV_SESSIONS);
        d.expect("Stopped by itself: the microphone is no longer connected.");
        d.expect("Choose an input");
        d.state_prepared("recording-stopped", Sizes::All, |d| {
            check_one_primary(d, "Choose an input", "iter-9-C-ST1-missing");
            check_processing_action(d, "Process:", false);
            check_rail_anchor(d, "iter-10-C-FT6-stopped");
        });
        d.observe(
            "iter-10-C-FT5",
            d.visible_text("Stopped by itself: the microphone is no longer connected.")
                && !d.visible_text("no such device"),
            "The stopped row puts the recorder's error in plain words",
        );
        let hover = d.painted_tooltip_shows(
            "Stopped by itself: the microphone",
            "The recorder said: no such device.",
        );
        d.observe(
            "iter-10-C-FT5-hover",
            hover,
            "Hover keeps the recorder's own words",
        );
        d.observe(
            "R3",
            d.has("Record again"),
            "The stopped row owns its state and offers Record again",
        );
        assert!(d.in_view("Record again"));
        d.click("Record again");
        d.expect("Stop recording");
        d.observe(
            "T11.5",
            !d.has_containing("stopped by itself"),
            "Recording again clears the error",
        );
        // An input chosen now cannot reach the recording that is running.
        d.app_mut().processing.focus_input = true;
        go_settings(d);
        d.click("Recording and processing");
        d.expect("A new input applies to the next recording");
        d.observe_both(
            "iter-13-R4",
            "Recording heading and input stay in view",
            |d| d.visible_text("RECORDING") && d.in_view("Recording input"),
        );
        d.state("input-during-recording", Sizes::Default);
        d.click_containing("Recording 0:");
        d.click("Stop recording");

        // A note renamed in Obsidian keeps the DM's words, and no two rows share one session.
        let plan_session = d.app_mut().sessions.current();
        write_note(d);
        d.type_into_editor("Words typed after the rename.\n");
        d.app_mut().sessions.simulate_file_renamed();
        d.press_ctrl(Key::S);
        d.expect("Notes not saved");
        d.expect("Your draft is still here.");
        for size in [SMALL, DEFAULT, LARGE] {
            let separated = d.at_size(size, |d| {
                let moved = d
                    .place_of("Save my draft to Renamed in Obsidian.md")
                    .expect("Moved-file recovery is present");
                let new = d
                    .place_of("Save my draft as a new file")
                    .expect("New-file recovery is present");
                let drop = d
                    .place_of(DISCARD)
                    .expect("Destructive recovery is present");
                assert!(
                    d.in_view("Save my draft to Renamed in Obsidian.md")
                        && d.in_view("Save my draft as a new file")
                        && d.in_view(DISCARD)
                );
                if size == SMALL {
                    (drop.top() - new.top()).abs() <= 1.0
                        && drop.left() - new.right() >= crate::design::L
                        || drop.top() - new.bottom() >= crate::design::M - 1.0
                } else {
                    (moved.top() - new.top()).abs() <= 1.0
                        && (new.top() - drop.top()).abs() <= 1.0
                        && moved.right() <= new.left()
                        && drop.left() - new.right() >= crate::design::L
                }
            });
            d.observe("iter-7-C-FT3", separated, format!("Missing-note safe choices share a line when they fit; Discard has an L or M gap at {size:?}"));
        }
        // iter-23-S FL2: the warning says Splat found the note under its new name, so the first
        // choice is explained.
        d.observe(
            "iter-23-S-FL2",
            d.visible_text("This note was moved or renamed. It is now Renamed in Obsidian.md.")
                && !d.visible_text("This file is gone from the folder."),
            "A renamed note's warning names where it went",
        );
        check_choices_at_text(
            d,
            "iter-15-C-FT1-gone",
            "This note was moved or renamed.",
            &[
                "Save my draft to Renamed in Obsidian.md",
                "Save my draft as a new file",
                DISCARD,
            ],
            Some(DISCARD),
            &[SMALL, DEFAULT, LARGE],
        );
        d.state("note-file-gone", Sizes::All);
        // FT4: Discard has one name. Its notice and Undo say discard, not drop or reload. These
        // steps prove the words and come back to the same choices, so they are not task cost.
        d.aside(|d| {
        // iter-25-S SM1: Discard follows the moved file. Its words, and later saves, are the
        // moved file's; the old path never comes back. The steps after this aside need the
        // moved-note state, so the aside puts the file and the draft back at its end.
        let snapshot = d.app_mut().sessions.draft_snapshot().expect("the open note");
        let old_path = snapshot.path.clone();
        let moved_path = snapshot.moved_to.clone().expect("the note was found renamed");
        let moved_before = std::fs::read_to_string(&moved_path).unwrap();
        d.click(DISCARD);
        // The folder scan, which follows the renamed file every few seconds, keeps the Undo.
        d.app_mut().sessions.simulate_folder_scan();
        d.settle();
        d.expect("Undo discard");
        d.observe(
            "iter-15-C-FT4",
            d.visible_text("Draft discarded.")
                && !d.has("Undo reload")
                && !d.has_containing("dropped")
                && d.editor_text()
                    .is_some_and(|text| !text.contains("Words typed after the rename.")),
            "Discard says Draft discarded. with Undo discard, and the draft's words leave the editor",
        );
        let bound = d.app_mut().sessions.draft_snapshot().map(|draft| draft.path);
        d.observe(
            "iter-25-S-SM1-follow",
            bound.as_deref() == Some(moved_path.as_path())
                && !d.visible_text("File gone")
                && !d.has("Save my draft as a new file")
                && d.editor_text().is_some_and(|text| {
                    moved_before.contains(text.trim()) && !text.trim().is_empty()
                }),
            format!("After Discard the editor holds the moved file {moved_path:?}: {bound:?}"),
        );
        d.state("note-discarded", Sizes::Default);
        d.click("Undo discard");
        d.observe(
            "iter-15-C-FT4-undo",
            !d.has("Undo discard")
                && d.editor_text()
                    .is_some_and(|text| text.contains("Words typed after the rename.")),
            "Undo discard brings the draft's words back",
        );
        d.press_ctrl(Key::S);
        d.expect("Notes saved");
        d.observe(
            "iter-25-S-SM1-save",
            !old_path.exists()
                && std::fs::read_to_string(&moved_path)
                    .is_ok_and(|text| text.contains("Words typed after the rename.")),
            "Discard, Undo discard and Save write the moved file, never the old path",
        );
        // Back to the moved-note state for the steps that follow.
        std::fs::write(&moved_path, &moved_before).unwrap();
        d.app_mut().sessions.restore_draft(snapshot);
        d.settle();
        d.expect("Your draft is still here.");
        });
        assert!(d.in_view("Save my draft as a new file"));
        d.click("Save my draft as a new file");
        d.expect("Notes saved");
        d.observe(
            "T11.4",
            d.editor_text()
                .is_some_and(|text| text.contains("Words typed after the rename.")),
            "A note whose file went keeps its words, saved under a new identity",
        );
        d.goal("record a session and process it");

        // A separate responsive check follows the measured workflow.
        d.click("Record this session");
        d.click("Start recording");
        d.click("Stop recording");
        d.click_starting("Recordings");
        d.click_containing("Process:");
        d.expect("Summary");
        d.click("Summary");
        d.press_ctrl(Key::K);
        d.expect("Recording view");
        choose_view(d, "Raw transcript");
        d.observe(
            "iter-2-R11",
            d.visible_text("Raw transcript") && !d.has("Add to notes"),
            "The narrow viewer's dropdown switches to the raw transcript",
        );
        choose_view(d, "Summary");
        let campaign = d.app_mut().campaigns.active_campaign().unwrap();
        let folder = crate::sessions::folder(&campaign).unwrap();
        let notes = crate::sessions::list_notes(&folder);
        let played = notes
            .iter()
            .find(|note| Some(note.id) == plan_session)
            .unwrap();
        let recovered = notes
            .iter()
            .find(|note| note.id != played.id && note.number == played.number)
            .unwrap();
        let expected = format!(
            "{} is not the session being played. The session being played is {}.",
            recovered.path.file_name().unwrap().to_string_lossy(),
            played.path.file_name().unwrap().to_string_lossy()
        );
        d.state_prepared("recording-summary-narrow", Sizes::All, |d| {
            d.observe(
                "iter-9-C-FT9",
                d.visible_text(&format!(
                    "What you write and add on this page goes into {}.",
                    recovered.path.file_name().unwrap().to_string_lossy()
                )),
                "The warning names the note where writing goes",
            );
            d.observe(
                "iter-8-C-ST2-files",
                d.visible_text(&expected),
                "Duplicate session numbers name both note files",
            );
            assert!(d.in_view("Return to the session being played"));
        });
        d.press_ctrl(Key::K);

        d.click(NAV_SETTINGS);
        d.click("Recording and processing");
        d.scroll_until(pos2(600.0, 400.0), "Speech service", 12);
        d.click("Speech service");
        d.click("Another service");
        d.expect("The custom service must support the OpenAI audio/transcriptions API.");
        // Round 24 FH4, AH1, SH1: after the choice, the notice shows whole, and the field that
        // blocks processing says why below itself.
        let notice = "You cannot process yet. Enter the speech service’s address.";
        d.observe(
            "iter-24-S-FH4",
            d.nodes()
                .iter()
                .any(|node| node.name.contains(notice) && node.fully_in_view()),
            "The notice is whole in view after the DM chooses another service",
        );
        let address = d.place_of("Speech address");
        let reason = d
            .nodes()
            .into_iter()
            .find(|node| node.name.contains("Enter the address.") && node.fully_in_view())
            .map(|node| node.rect);
        let model = d.place_of("Speech model");
        d.observe(
            "iter-24-S-AH1",
            address
                .zip(reason)
                .zip(model)
                .is_some_and(|((address, reason), model)| {
                    reason.top() >= address.bottom() && reason.bottom() <= model.top()
                }),
            format!("`Enter the address.` is in view below Address: {address:?}, {reason:?}"),
        );
        d.expect("The saved key is for OpenAI, so Splat will not send it to this service. Save a key for it.");
        d.state("processing-custom-speech-service", Sizes::All);

        // FT2: return to the original session. Saving the missing note made a new identity.
        d.click(NAV_SESSIONS);
        assert!(!d.in_view("Record again"));
        assert!(d.in_view("Return to the session being played"));
        d.click("Return to the session being played");
        d.observe(
            "iter-8-C-ST2-return",
            d.in_view("Record again")
                && d.visible_text("Stopped by itself: the microphone is no longer connected.")
                && !d.has("Return to the session being played") && !d.has_containing(&expected),
            "Return reveals the played session's stopped recording and clears the other-session warning",
        );
        show_plan(d);
        let resume = "Resume: Bridge Ambush";
        d.state_prepared("timeline-fight-resume", Sizes::All, |d| {
            assert!(
                d.in_view(resume),
                "A paused timeline fight has a visible Resume"
            );
            check_clean_row(d, "iter-11-C-AT2");
            // iter-25-S FM6: two notes share this title, so the Last time link names its file.
            // T12.6 keeps a unique title without a file name.
            let links: Vec<String> = d
                .nodes()
                .into_iter()
                .filter(|node| node.is_link() && node.name.starts_with("Session 1: The Bridge"))
                .map(|node| node.name)
                .collect();
            d.observe(
                "iter-25-S-FM6",
                d.has("Last time:")
                    && !links.is_empty()
                    && links.iter().all(|name| {
                        name.starts_with("Session 1: The Bridge (") && name.ends_with(".md)")
                    }),
                format!("The Last time link names its file: {links:?}"),
            );
        });
        d.begin_task();
        assert!(
            d.in_view(resume),
            "Resume is visible before its direct click"
        );
        d.click(resume);
        d.audit_goal_within(
            "iter-5-C-FT2",
            "resume a timeline fight from its plan row",
            1,
        );
        d.expect(NEXT_TURN);
        close_run(d);
        d.click(NAV_SESSIONS);

        // No Next turn is shown during setup or after ending, but both still own a run.
        // Use a fresh session: stopped recordings on the old one own its Record again action.
        d.click("New session");
        d.type_into("Session title", "Processing priority");
        d.click("Create session");
        d.app_mut().processing.config.speech_model =
            crate::processing::OPENAI_SPEECH_MODEL.to_owned();
        d.app_mut().processing.config.speech_url = crate::processing::OPENAI_URL.to_owned();
        d.app_mut().processing.simulate_keys();
        d.app_mut().processing.simulate_input("table.mic");
        d.settle();
        d.click("Record this session");
        d.click("Start recording");
        d.click("Stop recording");
        d.click_starting("Recordings");
        d.click("Record this session");
        d.state_prepared("processing-while-recording-consent", Sizes::All, |d| {
            assert!(d.in_view("Start recording"));
            check_processing_action(d, "Process:", false);
            check_one_primary(d, "Start recording", "iter-8-C-consent-primary");
            check_duplicate_sessions(d);
        });
        d.click("Start recording");
        d.click("Stop recording");
        assert_eq!(
            d.nodes()
                .iter()
                .filter(|node| node.in_view()
                    && node.name.starts_with("Process:")
                    && node.role == egui::accesskit::Role::Button)
                .count(),
            2
        );
        d.state_prepared("processing-with-no-run", Sizes::All, |d| {
            check_processing_action(d, "Process:", true);
            check_duplicate_sessions(d);
            check_destinations(d);
        });
        go_encounters(d);
        run_saved_encounter(d, "Shrine of Cinders");
        d.expect("Begin combat");
        d.click(NAV_SESSIONS);
        d.state_prepared("processing-with-setup-run", Sizes::All, |d| {
            check_processing_action(d, "Process:", false)
        });
        d.click(BACK_TO_RUN);
        d.click("Roll NPC initiative");
        for (name, total) in [
            ("Brannoch Vale", "18"),
            ("Iselle Marrowind", "15"),
            ("Kestrel Dunmoor", "12"),
            ("Oren Thistlewick", "9"),
            ("Sable Harrowgate", "6"),
        ] {
            d.type_into(&format!("Initiative for {name}"), total);
        }
        d.click("Begin combat");
        d.expect(NEXT_TURN);
        d.click(NAV_SESSIONS);
        d.state_prepared("processing-with-running-run", Sizes::All, |d| {
            check_rail_anchor(d, "iter-10-C-FT6-two-statuses");
            check_processing_action(d, "Process:", false);
            check_destinations(d);
            check_one_primary(d, NEXT_TURN, "iter-9-C-ST1-running");
        });
        d.click(BACK_TO_RUN);
        d.click("End encounter…");
        d.click("End encounter");
        d.click(NAV_SESSIONS);
        d.state_prepared("processing-with-ended-run", Sizes::All, |d| {
            check_processing_action(d, "Process:", false)
        });
        let process = d
            .nodes()
            .into_iter()
            .find(|node| {
                node.in_view()
                    && node.name.starts_with("Process:")
                    && node.role == egui::accesskit::Role::Button
            })
            .unwrap()
            .name;
        assert!(d.in_view(&process));
        d.click(&process);
        d.expect("Summary");
    }
}

/// W13: prepare encounter groups, and add one to a run when it arrives.
pub mod encounter_groups {
    use super::*;
    const LIST_HEADING: &str = "GROUPS";
    const NEW_GROUP: &str = "New group";
    const SAVE_GROUP: &str = "Save group";
    const SAVED_NOTE: &str = "Saved";
    const FIND: &str = FIND_CREATURE;
    const CHANGES: &str = "Changes for Ember Cultist";
    const FIRST_GOBLIN: &str = "Goblin Warrior 1";
    const RIVAL_NAME: &str = "Rival Captain";
    const TOTAL_FOR_RIVALS: &str = "Initiative total for Rival Team";

    pub fn run(d: &mut Driver<'_>) {
        d.expect(LIST_HEADING);
        d.click_starting(LIST_HEADING);
        d.state("groups-in-the-list", Sizes::All);
        // FT1: one fault, one tone, from its kind. The invalid maximum HP is an error here, as
        // in the editor and Arrivals; the member that is not in the packages stays a warning.
        let tones = (
            tones_beside(d, "would become −28"),
            tones_beside(d, "not in this campaign’s packages"),
        );
        d.observe(
            "iter-22-B-FT1-list",
            tones.0 == ["error"] && tones.1 == ["warning"],
            format!(
                "Encounters list: invalid maximum HP {:?}, missing member {:?}",
                tones.0, tones.1
            ),
        );
        d.check_tab_order("encounters, with groups");

        // An existing group: its changes are out in the open, with the values they replace.
        d.click(&format!("Edit group {GROUP_RIVALS}"));
        d.expect("MEMBERS 3");
        d.expect("XP 600");
        d.state("group-editor", Sizes::All);
        // FD3: where FORM_WIDTH ends first, the name and Belongs to still share one edge, and it
        // is clearly apart from the members column's.
        for size in [SMALL, DEFAULT, LARGE] {
            let edges = d.at_size(size, |d| group_field_edges(d, GROUP_RIVALS));
            d.observe(
                "iter-19-B-FD3-wide",
                edges.is_some_and(|(name, belongs, notes)| {
                    (name - belongs).abs() <= 0.5
                        && ((name - notes).abs() <= 0.5 || notes - name >= crate::design::XL)
                }),
                format!(
                    "At {size:?}, the group's field edges are one edge or clearly apart: {edges:?}"
                ),
            );
        }
        d.observe(
            "T13.3",
            d.has_containing("Changed · 1 to add by hand")
                && !d.has_containing("Maximum HP: 33 → 43 (+10)"),
            "Member changes stay in one summary until their editor opens",
        );
        d.click(CHANGES);
        d.expect("Maximum HP: how it changes");
        d.state("group-member-changes", Sizes::All);
        // The change form lists other effects as the run's Conditions tab does: one label, Body
        // text in the text colour, and one placeholder.
        let (label, color, placeholder) = d.at_size(LARGE, |d| {
            (
                d.visible_text("OTHER EFFECTS"),
                d.painted_color("Blessed"),
                d.visible_text("Other effect…"),
            )
        });
        d.observe(
            "iter-23-B-FI2",
            label && color == Some(crate::design::colors().text) && placeholder,
            format!(
                "Other effects: label {label}, Blessed colour {color:?}, placeholder {placeholder}"
            ),
        );
        // iter-25-R FH2, FJ2: an effect's row menu ends at the form's right edge, where the
        // Other effect field's Add ends, and sits on the effect's line.
        for size in [SMALL, DEFAULT, LARGE] {
            let (menu, add, word, shown) = d.at_size(size, |d| {
                let field = d.place_of("Other effect");
                let add = field.and_then(|field| {
                    d.nodes()
                        .into_iter()
                        .filter(|node| node.name == "Add")
                        .map(|node| node.rect)
                        .find(|rect| (rect.center().y - field.center().y).abs() < 1.0)
                });
                let menu = d.place_of("More actions for Blessed");
                let word = menu.and_then(|menu| {
                    let row = egui::Rect::from_min_max(
                        egui::pos2(0.0, menu.top() - crate::design::L),
                        egui::pos2(menu.left(), menu.bottom() + crate::design::L),
                    );
                    d.painted_word_rect("Blessed", row)
                });
                // At 960 the row is below the fold, so its words are not painted there: the
                // edge is checked at every size, the line where the words show.
                let shown = d.in_view("More actions for Blessed");
                (menu, add, word, shown)
            });
            d.observe(
                "iter-25-R-FH2",
                matches!((menu, add), (Some(menu), Some(add))
                    if (menu.right() - add.right()).abs() <= 1.0)
                    && (size == SMALL && !shown
                        || matches!((menu, word), (Some(menu), Some(word))
                            if (menu.center().y - word.center().y).abs() <= 1.0)),
                format!("At {size:?}, Blessed's menu {menu:?} ends with Add {add:?} on the word's line {word:?}"),
            );
        }
        // AI1: Starting conditions uses the run's grid: aligned columns at most `L` apart.
        let columns = d.at_size(LARGE, |d| {
            d.place_of("STARTING CONDITIONS")
                .map(|label| condition_columns(d, label.left(), label.bottom()))
                .unwrap_or_default()
        });
        d.observe(
            "iter-23-R-AI1",
            condition_columns_close(&columns),
            format!("At 1920 the starting-condition columns (left, right, count): {columns:?}"),
        );
        d.check_tab_order("group editor, changes open");
        d.click("Done with changes");
        d.observe(
            "F5",
            !d.has("Maximum HP: how it changes")
                && d.in_view_containing("Changes for Ember Cultist"),
            "Done closes the form and returns to its member",
        );
        d.expect_same("Done closes the changes form and restores the unchanged group editor.");
        d.assert_state_matches("group-changes-done", "group-editor");
        d.click("Back to encounters");

        // A group that cannot be added says why, in the editor, and its change says why.
        d.click(&format!("Edit group {GROUP_LOST}"));
        d.expect("Unknown creature");
        let (found, invalid) = d.at_size(SMALL, |d| {
            (
                d.in_view_containing("Unknown creature: not in this campaign’s packages."),
                d.in_view_containing("would become −28"),
            )
        });
        d.observe(
            "T13.3",
            found && invalid && d.has("Replace Unknown creature"),
            "The editor says why a creature is unavailable, and that a change cannot stand",
        );
        let tones = (
            tones_beside(d, "would become −28"),
            tones_beside(d, "Unknown creature: not in this campaign’s packages."),
        );
        // FA1 (Astra review of PR 259): the editor names the missing member in the one form,
        // and its help follows on a line of its own.
        d.observe(
            "iter-25-B-FA1-missing",
            d.has("Unknown creature: not in this campaign’s packages.")
                && d.has(
                    "Choose a replacement, check the packages it uses, or remove this member.",
                )
                && !d.has_containing("This campaign cannot find this creature"),
            "The group editor's missing member says the one member-fault form, then its help",
        );
        d.observe(
            "iter-22-B-FT1-editor",
            !tones.0.is_empty()
                && tones.0.iter().all(|tone| *tone == "error")
                && tones.1 == ["warning"],
            format!(
                "Group editor: invalid maximum HP {:?}, missing member {:?}",
                tones.0, tones.1
            ),
        );
        d.expect("Replace Unknown creature");
        // The repair is a notice action. FB5 (round 25): it is a bordered secondary button, and
        // its frame starts at the warning text, below it.
        for size in [SMALL, DEFAULT, LARGE] {
            let (aligned, detail) = d.at_size(size, |d| {
                let text = d.painted_rect_in(
                    "Unknown creature: not in this campaign’s packages.",
                    egui::Rect::EVERYTHING,
                );
                let words = d.painted_rect_in("Campaign → Packages", egui::Rect::EVERYTHING);
                let link = d.place_of("Campaign → Packages");
                let aligned = match (text, words, link) {
                    (Some(text), Some(words), Some(link)) => {
                        (link.left() - text.left()).abs() < 1.0
                            && words.left() > link.left() + 1.0
                            && link.top() >= text.bottom() - 0.5
                    }
                    _ => false,
                };
                (
                    aligned,
                    format!("text {text:?}, words {words:?}, link {link:?}"),
                )
            });
            d.observe(
                "iter-14-B-AT3",
                aligned,
                format!(
                    "Campaign → Packages starts its words at the warning text at {size:?}: {detail}"
                ),
            );
        }
        d.state("group-unavailable-creature", Sizes::All);
        d.click("Changes for Cinder Wolf");
        let in_view = d.at_size(SMALL, |d| d.in_view_containing("would become −28"));
        d.observe(
            "T13.3",
            in_view,
            "At the minimum window, an open form says in view that its change cannot stand",
        );
        d.state("group-invalid-change", Sizes::Default);
        // The fault sits under the Maximum HP field at its left edge, once, and the preview
        // beside the field takes the error colour.
        for size in [SMALL, DEFAULT] {
            let (placed, detail) = d.at_size(size, |d| {
                let field = d.place_of("Maximum HP: how it changes");
                let mut faults = Vec::new();
                let mut preview = None;
                d.each_painted_text(|text, clip| {
                    let rect = egui::Rect::from_min_size(text.pos, text.galley.size());
                    let words = &text.galley.job.text;
                    if !clip.intersects(rect) {
                        return;
                    }
                    if words.contains("Maximum HP would become −28") {
                        faults.push(rect);
                    }
                    if words.contains("22 → −28") {
                        preview = text.galley.job.sections.last().map(|s| s.format.color);
                    }
                });
                let placed = field.is_some_and(|field| {
                    faults.len() == 1
                        && faults[0].top() >= field.bottom() - 0.5
                        && faults[0].top() < field.bottom() + crate::design::XL
                        && faults[0].left() >= field.left() - 0.5
                }) && preview == Some(crate::design::colors().result.error);
                (
                    placed,
                    format!("field {field:?}, faults {faults:?}, preview {preview:?}"),
                )
            });
            d.observe(
                "iter-23-B-FI3",
                placed,
                format!("At {size:?}, the HP fault is under its field: {detail}"),
            );
        }
        d.click("Back to encounters");

        // A new group cannot be saved empty, and it says so.
        d.click(NEW_GROUP);
        d.type_text("Ash Pack");
        assert!(d.is_disabled(SAVE_GROUP));
        d.expect("Add a creature first");
        // AH1: Save refuses the empty group, so the state says Invalid, not Unsaved.
        d.observe(
            "iter-25-B-AH1-list",
            d.visible_text("Invalid") && !d.visible_text("Unsaved"),
            "An empty new group says Invalid beside its blocked Save",
        );
        d.state("group-rejected-empty", Sizes::Default);
        // FH3, SH2: a tall window shows every result and a short one scrolls them; each `Add`
        // keeps its x either way.
        // A proof aside: the search is empty again after it.
        let lanes = d.aside(|d| {
            d.type_into(FIND, "adult");
            let lanes = [(DEFAULT.0, 2000.0), (DEFAULT.0, SMALL.1)].map(|size| {
                d.at_size(size, |d| {
                    let adds = right_edges(d, ADD);
                    let bar = adds
                        .iter()
                        .copied()
                        .reduce(f32::max)
                        .is_some_and(|edge| scroll_bar_right_of(d, edge));
                    (adds, bar)
                })
            });
            d.clear_text_in(FIND);
            lanes
        });
        d.observe(
            "iter-25-B-FH3",
            !lanes[0].1
                && lanes[1].1
                && !lanes[0].0.is_empty()
                && !lanes[1].0.is_empty()
                && lanes
                    .iter()
                    .flat_map(|(adds, _)| adds)
                    .all(|x| (x - lanes[0].0[0]).abs() <= 0.5),
            format!("Add right edges and scroll bar, tall then short: {lanes:?}"),
        );
        d.type_into(FIND, "wolf");
        d.state("group-find-creature", Sizes::Default);
        d.click_beside_exact("Wolf", ADD);
        d.click(SAVE_GROUP);
        d.expect(SAVED_NOTE);
        d.state("group-saved", Sizes::Default);
        d.goal("make an encounter group");
        d.click("Back to encounters");
        d.expect("GROUPS");

        // Member recovery is separate from the unchanged build task.
        d.begin_task();
        d.click(&format!("Edit group {GROUP_RIVALS}"));
        d.click(&format!("More actions for {RIVAL_NAME}"));
        d.click("Remove");
        d.expect("Removed Rival Captain.");
        d.expect("Undo remove member");
        // ST1: a dirty group blocks the campaign switch, opened from Encounters too, and its
        // save row says why. Undo below must still match the clean editor.
        d.observe_both(
            "iter-18-B-ST1",
            "A dirty group opened in Encounters says why the campaign cannot change",
            |d| d.visible_text("Unsaved") && d.visible_text("Save or discard to switch campaign."),
        );
        // FD4 family: the group notes field ends at the members' right edge, as the encounter
        // editor's notes field ends at the roster's.
        for size in [SMALL, DEFAULT, LARGE] {
            let edges = d.at_size(size, |d| {
                (
                    d.place_of("More actions for Cinder Wolf")
                        .map(|rect| rect.right()),
                    d.place_of(&format!("Group notes for {GROUP_RIVALS}"))
                        .map(|rect| rect.right()),
                )
            });
            d.observe(
                "iter-18-B-FD4-group",
                matches!(edges, (Some(menu), Some(notes)) if (menu - notes).abs() <= 0.5),
                format!("At {size:?}, the member menu and the notes field end at {edges:?}"),
            );
        }
        // iter-24-B AK1, AB2: Save group ends at the members column's right edge (the member
        // menu's), on the save row with Unsaved and Discard, not across the Add creatures pane.
        for size in [SMALL, DEFAULT, LARGE] {
            let (save, column, unsaved) = d.at_size(size, |d| {
                (
                    d.place_of(SAVE_GROUP),
                    d.place_of("More actions for Cinder Wolf"),
                    d.painted_rect_in("Unsaved", egui::Rect::EVERYTHING),
                )
            });
            d.observe(
                "iter-24-B-AK1",
                save.zip(column).zip(unsaved).is_some_and(|((save, column), unsaved)| {
                    (save.right() - column.right()).abs() <= 0.5
                        && save.top() < unsaved.bottom()
                        && unsaved.top() < save.bottom()
                }),
                format!("At {size:?}: Save group {save:?}, column's menu {column:?}, Unsaved {unsaved:?}"),
            );
        }
        d.state("group-member-removed-with-undo", Sizes::All);
        d.click("Undo remove member");
        d.expect(CHANGES);
        d.observe_both(
            "iter4-B-AT1",
            "Undo restores the changed member and clears the removal notice",
            |d| d.has(CHANGES) && !d.has("Removed Rival Captain."),
        );
        d.observe(
            "iter-18-B-ST1-clean",
            !d.visible_text("Save or discard to switch campaign."),
            "A group with nothing to save gives no switch reason",
        );
        d.expect_same("Undo restores the complete group editor with the changed member.");
        d.assert_state_matches("group-member-restored", "group-editor");
        d.click("Back to encounters");

        // Repair is a separate workflow; it does not change the build goal's cost.
        d.click(&format!("Edit group {GROUP_LOST}"));
        d.click("Replace Unknown creature");
        d.type_into("Replacement creature", "Ember");
        d.expect("Use Ember Cultist");
        // iter-24-B FH3: at one width, a tall window shows the whole members column and a short
        // one gives it a scroll bar; the name field, Belongs to and every member menu keep x.
        let (before, fits) = d.at_size((DEFAULT.0, 2000.0), |d| {
            (member_column_xs(d, GROUP_LOST), !members_scroll_bar(d))
        });
        let (after, scrolls) = d.at_size((DEFAULT.0, SMALL.1), |d| {
            (member_column_xs(d, GROUP_LOST), members_scroll_bar(d))
        });
        let same: Vec<_> = before
            .iter()
            .filter_map(|(name, x)| {
                after
                    .iter()
                    .find(|(other, _)| other == name)
                    .map(|(_, y)| (name.clone(), *x, *y))
            })
            .collect();
        d.observe(
            "iter-24-B-FH3",
            fits && scrolls
                && same.len() >= 3
                && same.iter().any(|(name, ..)| name.starts_with("More actions for "))
                && same.iter().all(|(_, x, y)| (x - y).abs() <= 0.5),
            format!("Fits before {fits}, scrolls after {scrolls}; right edges before and after: {same:?}"),
        );
        d.state("group-member-replacement-search", Sizes::Default);
        d.click("Use Ember Cultist");
        d.expect("Changes for Ember Cultist");
        // Use gives a result and its Undo on the member result line, as Remove does.
        const REPLACED: &str = "Replaced Unknown creature with Ember Cultist.";
        d.observe(
            "iter-23-B-FI1",
            d.visible_text(REPLACED) && d.has("Undo replacement"),
            "Use names the replacement and offers Undo replacement",
        );
        // FH5, SH1: a finished change is a success message, with the success bar.
        let replaced = d.painted_rect_in(REPLACED, egui::Rect::EVERYTHING);
        d.observe(
            "iter-25-B-FH5",
            replaced
                .is_some_and(|words| d.has_tone_bar(words, crate::design::colors().result.success)),
            format!("The replacement result {replaced:?} has the success bar"),
        );
        d.state("group-member-replaced", Sizes::Default);
        // An unrelated edit keeps the result; Undo puts back only the creature, and its words
        // go with it.
        d.type_into(&format!("Group notes for {GROUP_LOST}"), " Arrive at dusk.");
        let typed = d.visible_text("Arrive at dusk.");
        d.click("Undo replacement");
        d.expect("Replace Unknown creature");
        d.observe(
            "iter-23-B-FI1-undo",
            typed
                && d.visible_text("Arrive at dusk.")
                && !d.has("Changes for Ember Cultist")
                && !d.visible_text(REPLACED)
                && !d.has("Undo replacement"),
            "Undo replacement keeps the notes typed after it and clears its own result",
        );
        d.click("Discard");
        d.click("Back to encounters");

        // Removal and recovery are their own task, outside the build and arrival costs.
        d.begin_task();
        d.click(&format!("More actions for {GROUP_GOBLINS}"));
        d.click("Remove…");
        d.expect("Undo remove group");
        d.expect("Lost Patrol draft discarded.");
        assert!(d.has("Undo discard of Lost Patrol"));
        d.state_prepared("group-removed", Sizes::All, |d| {
            // iter11-B-FT4: the discard notice is an Undo message, like the removal above it.
            let size = d.window_size();
            let window = egui::Rect::from_min_size(pos2(0.0, 0.0), egui::vec2(size.0, size.1));
            let discarded = d.painted_rect_in("Lost Patrol draft discarded.", window);
            let removed = d.painted_rect_in(&format!("Removed group {GROUP_GOBLINS}."), window);
            let text = crate::design::colors().text;
            let body = crate::design::Type::Body.size();
            let (Some(discarded), Some(removed)) = (discarded, removed) else {
                panic!("At {size:?}, both notices are painted: {discarded:?} and {removed:?}");
            };
            d.observe(
                "iter11-B-FT4",
                (discarded.left() - removed.left()).abs() < 1.0,
                format!(
                    "At {size:?}, the discard notice starts at the removal's left edge: {discarded:?} and {removed:?}"
                ),
            );
            let painted = d.painted_size(discarded, "Lost Patrol draft discarded.");
            let icon = |text: egui::Rect, icon: &str| {
                d.painted_rect_in(
                    icon,
                    egui::Rect::from_min_max(
                        pos2(text.left() - crate::design::XXL, text.top() - 2.0),
                        pos2(text.left(), text.bottom() + 2.0),
                    ),
                )
                .is_some()
            };
            let colour = d.painted_color("Lost Patrol draft discarded.");
            // Round 25 (FH5's family): the removal is a finished change, so it has the success
            // icon; the discard wrote nothing, so it keeps the info icon.
            let info = crate::design::icons::INFO;
            let success = crate::design::icons::SUCCESS;
            d.observe(
                "iter11-B-FT4",
                icon(discarded, info) && icon(removed, success) && colour == Some(text),
                format!("At {size:?}, the discard has the info icon, the removal the success icon, and the discard words use the text colour: {colour:?}"),
            );
            d.observe(
                "iter11-B-FT4",
                painted == Some(body),
                format!("At {size:?}, the discard notice uses Body text: {painted:?}, not {body}"),
            );
            check_undo_icon(d, "Undo discard of Lost Patrol", "iter11-B-FT4");
        });
        d.observe(
            "F15",
            !d.has(&format!("Edit group {GROUP_GOBLINS}")),
            "Remove drops the saved group and offers Undo",
        );
        d.click("Undo remove group");
        d.expect(&format!("Edit group {GROUP_GOBLINS}"));
        d.begin_task();

        // At the table: the panel, with members and XP.
        resume_run(d, RUN);
        open_arrivals(d);
        d.expect(&add_group(GROUP_GOBLINS));
        d.state_prepared("group-panel", Sizes::AllAndHiDpi, |d| {
            let groups = [GROUP_GOBLINS, GROUP_RIVALS, GROUP_LOST];
            let buttons = groups.map(|name| {
                d.place_of(&add_group(name))
                    .expect("Required group Add control")
            });
            let fields = groups.map(|name| {
                d.place_of(&format!("Initiative total for {name}"))
                    .expect("Required group Initiative field")
            });
            d.observe(
                "iter9-B-FT6-edge",
                buttons.iter().all(|b| {
                    (b.right() - buttons[0].right()).abs() <= 1.0
                        && (b.width() - buttons[0].width()).abs() <= 1.0
                }),
                format!("Arrivals Add and Add 1 fill one shared slot: {buttons:?}"),
            );
            d.observe(
                "iter9-B-FT6-neighbour",
                fields
                    .iter()
                    .all(|f| (f.left() - fields[0].left()).abs() <= 1.0),
                format!("Initiative fields keep one column: {fields:?}"),
            );
            // AH1: the groups share one block at the pane's left edge, no wider than a list,
            // so Add stays near the names it adds in a wide pane.
            let pane = d
                .place_of("Find or name a creature")
                .expect("Required Arrivals search");
            d.observe(
                "iter-23-R-AH1",
                buttons[0].right() - pane.left() <= crate::design::LIST_MEASURE + 1.0
                    && buttons[0].right() > pane.left(),
                format!(
                    "Add ends within LIST_MEASURE of the pane's left edge {}: {}",
                    pane.left(),
                    buttons[0].right()
                ),
            );
        });
        d.check_tab_order("run, group panel");
        d.observe(
            "T13.1",
            d.has_containing("3\u{a0}×\u{a0}Goblin\u{a0}Warrior")
                && d.visible_text("3 members · XP 150"),
            "The panel lists each group's members and total XP",
        );
        // T5.7 in Arrivals: after a group's XP, what the fight becomes when it joins. Bridge
        // Ambush's three creatures and the goblins are far below the party's Low 2,800.
        let joins = d.at_size(DEFAULT, |d| {
            let mut words = Vec::new();
            d.each_painted_text(|text, _| {
                if text.galley.job.text.starts_with(" → ") {
                    words.push(text.galley.job.text.clone());
                }
            });
            words
        });
        d.observe(
            "T5.7-arrivals",
            !joins.is_empty() && joins.iter().all(|word| word == " → Low"),
            format!("Each group heading with room says what the fight becomes: {joins:?}"),
        );
        // The words are worked out after a step, not on each frame of the panel.
        let words = crate::test_support::CachedWork::start("Arrivals words", d, |d| {
            let (reached, work) = d
                .app_mut()
                .encounters
                .run()
                .map_or((0, 0), |run| run.arrival_counts);
            crate::test_support::Counts { reached, work }
        });
        let steady = words.steady(d, |d| {
            d.settle();
            d.at_size(LARGE, |_| ());
            d.settle();
            d.visible_text(" → Low")
        });
        d.observe(
            "F1-arrivals-cache",
            steady.is_ok(),
            format!("Unchanged frames and a resize do not work the words out again: {steady:?}"),
        );
        d.observe(
            "T13.1",
            d.visible_text("XP 50")
                && d.has_containing(
                    "Unknown creature · line\u{a0}13: not in this campaign’s packages.",
                )
                // iter-26-R FI8: `Will add:` keeps its first member on its line.
                && d.visible_text("Will\u{a0}add:\u{a0}1\u{a0}×\u{a0}Goblin\u{a0}Warrior"),
            "The summary names the joining member; excluded members stay under Cannot join",
        );
        d.observe(
            "T13.1",
            !d.is_disabled(&add_group(GROUP_LOST)),
            "A group with one valid member has a working Add button",
        );
        d.observe(
            "T13.1",
            d.visible_text("1 of 3 can join")
                && d.has_containing("not in this campaign’s packages")
                && d.has_containing("Cinder Wolf · line\u{a0}14: Maximum HP would become −28."),
            "A group counts a creature it cannot find, and gives every reason it cannot be added",
        );
        let tones = (
            tones_beside(d, "would become −28"),
            tones_beside(d, "not in this campaign’s packages"),
        );
        d.observe(
            "iter-22-B-FT1-arrivals",
            tones.0 == ["error"] && tones.1 == ["warning"],
            format!(
                "Arrivals: invalid maximum HP {:?}, missing member {:?}",
                tones.0, tones.1
            ),
        );
        // iter-26-R FI5: every other group's field keeps its place when a total's reason comes;
        // the reason is in the footer below the list, named by its group.
        // Measured from Rival Team's own field, in the same scroll list: the driver scrolls a
        // control into view to press it, which moves every row alike and is not a fault.
        let other_fields = |d: &Driver<'_>| {
            let Some(rivals) = d.place_of(TOTAL_FOR_RIVALS) else {
                return Vec::new();
            };
            let mut fields: Vec<_> = d
                .nodes()
                .into_iter()
                .filter(|node| {
                    node.name.starts_with("Initiative total for ") && node.name != TOTAL_FOR_RIVALS
                })
                .map(|node| (node.name, (node.rect.top() - rivals.top()).round()))
                .collect();
            fields.sort_by(|a, b| a.0.cmp(&b.0));
            fields
        };
        let fields_before = other_fields(d);
        d.begin_task();
        d.type_into(TOTAL_FOR_RIVALS, "-");
        d.expect("Rival Team: Enter a whole number, or clear the field to roll.");
        assert!(d.is_disabled(&add_group(GROUP_RIVALS)));
        let fields_after = other_fields(d);
        d.observe(
            "iter-26-R-FI5",
            !fields_before.is_empty() && fields_before == fields_after,
            format!(
                "Other groups' fields before {fields_before:?}, with the reason {fields_after:?}"
            ),
        );
        d.state("invalid-arrival-total", Sizes::All);
        // AH1 at the captured sizes: one Add edge for every group, within the shared block.
        for size in [SMALL, DEFAULT, LARGE] {
            let (adds, pane) = d.at_size(size, |d| {
                let adds: Vec<_> = d
                    .nodes()
                    .into_iter()
                    .filter(|node| {
                        node.name.starts_with("Add ")
                            && node.name.ends_with(" to the initiative list")
                            && node.role == egui::accesskit::Role::Button
                            && !node.name.contains("creature")
                    })
                    .map(|node| node.rect.right())
                    .collect();
                (adds, d.place_of("Find or name a creature"))
            });
            d.observe(
                "iter-23-R-AH1",
                adds.len() >= 4
                    && pane.is_some_and(|pane| {
                        adds.iter().all(|right| {
                            (right - adds[0]).abs() <= 1.0
                                && right - pane.left() <= crate::design::LIST_MEASURE + 1.0
                        })
                    }),
                format!("At {size:?} every group's Add ends at one x inside the block: {adds:?} from {pane:?}"),
            );
            // FH2: the search field is as wide as the group block: one right edge.
            d.observe(
                "iter-24-R-FH2",
                pane.zip(adds.first())
                    .is_some_and(|(pane, add)| (pane.right() - add).abs() <= 1.0),
                format!(
                    "At {size:?} the search field ends at the Add column: {pane:?} and {adds:?}"
                ),
            );
        }
        d.clear_text_in(TOTAL_FOR_RIVALS);
        let before = roster_places(d).len();
        d.click(&add_group(GROUP_LOST));
        d.observe(
            "AT2",
            roster_places(d).len() == before + 1,
            "The valid member joins while the two unavailable members stay out",
        );
        d.state("available-member-added", Sizes::All);
        // FH1: a tied arrival is a finished change, so its message has the success bar.
        // FH6: its Move button names the group, as the message does, not one member.
        let tied = painted_paragraph(d, "Lost Patrol joined at");
        d.observe(
            "iter-24-R-FH1",
            tied.is_some_and(|words| d.has_tone_bar(words, crate::design::colors().result.success)),
            format!("The tied arrival's message has the success bar: {tied:?}"),
        );
        d.observe(
            "iter-24-R-FH6",
            d.in_view("Move Lost Patrol before Ember Cultist")
                && !d.has_containing("Move Goblin Warrior before"),
            "The Move button names Lost Patrol, as its message does",
        );
        d.click_containing("Undo: add Lost Patrol");
        d.begin_task();
        d.click(&add_group(GROUP_GOBLINS));
        d.expect(FIRST_GOBLIN);
        d.observe(
            "Arrival status",
            d.has_containing("Added in round 2"),
            "A group keeps Add and marks the round it joined",
        );
        d.state("group-added-by-rolling", Sizes::All);
        d.goal_within("T13.2", "add a group to the running fight, rolling once", 1);

        // Undo names it, and takes the whole group back.
        d.expect("Undo: add Goblin Wave");
        d.click_containing("Undo: add Goblin Wave");
        let gone = !roster_places(d)
            .iter()
            .any(|(name, _)| name.contains("Goblin Warrior"));
        d.observe("T13.2", gone, "Undo removes every member of an added group");

        // A typed total, as players give one.
        d.type_into(TOTAL_FOR_RIVALS, "14");
        d.observe(
            "Typed arrival total",
            d.field_text(TOTAL_FOR_RIVALS).as_deref() == Some("14"),
            "The typed initiative is kept before Add",
        );
        d.expect_same("Only the typed initiative differs from the group panel; the form and roster stay in place.");
        d.state("group-total-typed", Sizes::All);
        // iter-26-R FI5: a join puts `Added in round N` on the members' line: Rival Team's Add
        // and every other group's field keep their y.
        let add_y = |d: &Driver<'_>| {
            d.place_of(&add_group(GROUP_RIVALS))
                .zip(d.place_of(TOTAL_FOR_RIVALS))
                .map(|(add, field)| (add.top() - field.top()).round())
        };
        let (add_before, others_before) = (add_y(d), other_fields(d));
        d.click(&add_group(GROUP_RIVALS));
        d.expect(RIVAL_NAME);
        let (add_after, others_after) = (add_y(d), other_fields(d));
        d.observe(
            "iter-26-R-FI5-joined",
            d.has_containing("Added in round 2")
                && add_before.is_some()
                && add_before == add_after
                && others_before == others_after,
            format!(
                "Rival Team's Add {add_before:?} → {add_after:?}; others {others_before:?} → {others_after:?}"
            ),
        );
        d.observe(
            "T13.2",
            d.nodes()
                .iter()
                .any(|node| node.name.contains(RIVAL_NAME) && node.name.contains("Initiative 14")),
            "A typed total places the group, with no roll",
        );
        d.observe_both(
            "iter-5-B-FT3",
            "The arrival notice names only the group and total; the tie action stays",
            |d| {
                d.visible_text("Rival Team joined at\u{a0}14.")
                    && !d.visible_text("entered by the DM")
                    && d.in_view("Move Rival Team before Kestrel Dunmoor")
            },
        );
        d.expect("Tied with Kestrel Dunmoor at\u{a0}14, placed after.");
        // iter-26-R FI10: a row's hidden changes say how many: `1 more`, never a bare `more`.
        let words = d.at_size(DEFAULT, |d| d.row_words(RIVAL_NAME).replace('\u{a0}', " "));
        let counted = words.match_indices(" more").all(|(at, _)| {
            words[..at]
                .chars()
                .next_back()
                .is_some_and(|c| c.is_ascii_digit())
        });
        d.observe(
            "iter-26-R-FI10",
            words.contains(" more") && counted,
            format!("{RIVAL_NAME}'s row at 1200: {words}"),
        );
        d.state("group-added-with-a-total", Sizes::All);
        d.observe(
            "T13.2",
            d.has_containing("Undo: add Rival Team")
                && d.has_containing("Added in round 2")
                && d.nodes().iter().any(|node| {
                    node.name.contains(RIVAL_NAME) && node.name.contains("Initiative 14")
                }),
            "The group marks its arrival, Undo names it, and its members show the total",
        );

        // The card of a creature that arrived: where it came from, and its changes.
        d.click_row(RIVAL_NAME);
        d.click("Stat block");
        d.expect("Changed · arrived with Rival Team");
        d.state("card-of-an-added-member", Sizes::Default);
        open_arrivals(d);

        // The same group again, by dragging it onto the initiative list.
        let rows_before = roster_places(d).len();
        let list = roster_places(d)
            .first()
            .map(|(_, rect)| *rect)
            .expect("run rows");
        d.drag_hold(
            &format!("Drag {GROUP_GOBLINS} to the initiative list"),
            list,
        );
        d.state("group-held-over-the-list", Sizes::Default);
        d.drag_release(list);
        d.expect(FIRST_GOBLIN);
        d.observe(
            "T13.4",
            roster_places(d).len() == rows_before + 3,
            "Dragging a group onto the initiative list adds each member once",
        );
        d.state("group-added-by-dragging", Sizes::Default);

        // A side of its own, on the current row. The next step also ends the panel's line.
        d.click(NEXT_TURN);
        d.observe(
            "T13.2",
            !d.has_containing("Encounter group ") && !d.has_containing(" joined at"),
            "The panel's line about an addition ends with the next step",
        );
        for _ in 0..25 {
            if current_participant(d).as_deref() == Some(RIVAL_NAME) {
                break;
            }
            d.click(NEXT_TURN);
        }
        d.expect(RIVAL_NAME);
        d.state("a-named-side-on-its-turn", Sizes::All);
        d.goal("add groups by a total and by dragging");

        // A new arrival tie is checked outside the existing measured workflow.
        d.type_into(TOTAL_FOR_RIVALS, "14");
        d.click(&add_group(GROUP_RIVALS));
        d.expect("placed after");
        let before = current_participant(d);
        d.click("Move Rival Team (Rival Captain 2) before Rival Team");
        d.at_size(DEFAULT, |d| {
            d.expect("Rival Team (Rival Captain 2) moved before Rival Team at\u{a0}14")
        });
        d.observe(
            "F4",
            current_participant(d) == before
                && d.has_containing("Undo: move Rival Team (Rival Captain 2) before Rival Team"),
            "Move before keeps the current turn and offers Undo",
        );
        d.state("tied-arrival-moved-before", Sizes::All);
        d.click_containing("Undo: move Rival Team (Rival Captain 2) before Rival Team");
        d.click_containing("Undo: add Rival Team");
        d.begin_task();

        // Setup has the same panel.
        close_run(d);
        run_saved_encounter(d, "Shrine of Cinders");
        open_arrivals(d);
        d.state("group-panel-in-setup", Sizes::Default);
        // iter-25-R FB1, review 2: the first join, in setup, moves no Add and no row below it,
        // and neither does its Undo. Each group's Add, at 960, 1200 and 1920.
        let setup_rows = |d: &mut Driver<'_>| {
            [SMALL, DEFAULT, LARGE].map(|size| {
                d.at_size(size, |d| {
                    [
                        "Ash Pack",
                        GROUP_GOBLINS,
                        GROUP_LOST,
                        GROUP_RIVALS,
                        GROUP_WATCH,
                    ]
                    .map(|name| d.place_of(&add_group(name)).map(|rect| rect.top()))
                })
            })
        };
        let setup_before = setup_rows(d);
        d.click(&add_group(GROUP_GOBLINS));
        d.expect(FIRST_GOBLIN);
        let setup_added = setup_rows(d);
        d.state("group-added-in-setup", Sizes::All);
        let overflow = d.at_size((960.0, 640.0), |d| {
            d.visible_text("2 below") && !d.has_containing("2 ties") && !d.has_containing("2 tied")
        });
        d.observe(
            "iter-11-F3",
            overflow,
            "Setup names hidden shared-roll rows without a false tie warning at 960x640",
        );
        d.goal("add a group during setup");
        d.click_containing("Undo: add Goblin Wave");
        let setup_undone = setup_rows(d);
        d.observe(
            "iter-25-R-FB1-setup",
            setup_before.iter().flatten().all(Option::is_some)
                && setup_added == setup_before
                && setup_undone == setup_before,
            format!(
                "Group rows stay through a join in setup and its Undo: {setup_before:?}, added {setup_added:?}, undone {setup_undone:?}"
            ),
        );
        d.click("Edit group");
        d.expect("Group name: Lost Patrol");
        let run_open = d.app_mut().encounters.has_run();
        d.observe(
            "Group repair",
            run_open,
            "Edit group opens the broken group and keeps the run open",
        );
        d.expect("Replace Unknown creature");
        d.state("broken-group-open-for-repair", Sizes::All);
    }
}

pub mod encounter_history {
    use super::*;
    const OPEN_OLD: &str = "Open the run of Old Mill Raid";
    pub fn run(d: &mut Driver<'_>) {
        d.expect(READY_TO_RUN);
        d.expect(HISTORY);
        d.observe(
            "T4.6",
            d.has_label(RUN_PAST)
                && d.nodes()
                    .iter()
                    .filter(|node| node.name == RUN && node.role == egui::accesskit::Role::Label)
                    .count()
                    == 1,
            "Ended encounters return to Ready; an unfinished encounter appears only in In progress",
        );
        d.state("encounters-with-history", Sizes::All);
        d.click_starting(HISTORY);
        d.observe(
            "T4.6",
            d.has_containing(&PAST_SESSION.to_uppercase())
                && d.has_containing("NO SESSION")
                && d.has_label(RUN_PAST)
                && d.has_label(RUN_OLD),
            "History groups ended runs by session",
        );
        d.state("history-open", Sizes::All);
        d.check_tab_order("encounters, history open");
        delete_saved_run(d, RUN);
        d.expect("Undo delete run");
        d.observe(
            "T4.6",
            !d.has_containing(&format!("{RUN} · Round")),
            "Deleted run leaves the list with Undo",
        );
        d.state("run-deleted", Sizes::Default);
        d.click("Undo delete run");
        d.expect(&format!("{RUN} · Round"));
        let before = d.app_mut().timeline.events.len();
        d.click_starting(OPEN_OLD);
        d.expect("Read-only");
        let unchanged =
            d.app_mut().timeline.events.len() == before && !d.app_mut().encounters.has_run();
        d.observe(
            "T4.6",
            unchanged,
            "Reading History changes no story and leaves the live-run slot free",
        );
        d.state("ended-run-open", Sizes::All);
        resume_run(d, RUN);
        let live = d
            .app_mut()
            .encounters
            .run()
            .unwrap()
            .live_info()
            .unwrap()
            .id;
        go_encounters(d);
        d.click_starting(OPEN_OLD.replace("Open", "Close").as_str());
        d.click_starting(OPEN_OLD);
        d.expect("Read-only");
        let still_open = d
            .app_mut()
            .encounters
            .run()
            .unwrap()
            .live_info()
            .unwrap()
            .id
            == live;
        d.observe("T4.6", still_open, "History leaves the current fight open");
        d.observe(
            "T4.4",
            d.nodes().iter().filter(|node| node.name == RUN && node.role == egui::accesskit::Role::Label).count() == 1,
            "An open unfinished run has one row; the strip keeps Next turn primary and Back to run secondary",
        );
        d.state("history-beside-open-run", Sizes::All);

        open_saved_encounter(d, RUN);
        d.click("Keep in Ready during a run");
        d.click(SAVE_ENCOUNTER);
        d.click("Back to encounters");
        let nodes = d.nodes();
        d.observe(
            "T4.4",
            nodes
                .iter()
                .filter(|node| node.name == RUN && node.role == egui::accesskit::Role::Label)
                .count()
                == 2
                && nodes
                    .iter()
                    .filter(|node| node.name == "Round 2" && node.is_link())
                    .count()
                    == 1
                && !d.has(RESUME)
                && d.is_disabled(RUN_ENCOUNTER)
                && d.visible_text("Go back to the run"),
            "Keep in Ready adds only a blocked Start fight with a reason, never a second run link",
        );
        d.state("keep-in-ready-blocked", Sizes::All);
        d.click(NEW_ENCOUNTER);
        d.expect("Encounter name");
        d.observe(
            "P12",
            d.is_disabled(RUN_ENCOUNTER) && !d.is_disabled(SAVE_ENCOUNTER),
            "New encounter stays editable while starting a fight is blocked",
        );
        d.state("draft-during-run", Sizes::All);
        // iter-24-B FI7: Keep in Ready says what it changes, below the option, at each size.
        for size in [SMALL, DEFAULT, LARGE] {
            let placed = d.at_size(size, |d| {
                let option = d.place_of("Keep in Ready during a run")?;
                let help = d.painted_rect_in(
                    "Lists this encounter under Ready to run",
                    egui::Rect::EVERYTHING,
                )?;
                Some(
                    help.top() >= option.bottom() - 0.5
                        && help.top() < option.bottom() + crate::design::L,
                )
            });
            d.observe(
                "iter-24-B-FI7",
                placed == Some(true),
                format!("At {size:?}, the option's help sits below it: {placed:?}"),
            );
        }
    }
}

/// iter-19-B FD3, FT9: the right edges of a group editor's name field, `Belongs to` and
/// `Group notes`, which ends at the members column's right edge.
fn group_field_edges(d: &Driver<'_>, group: &str) -> Option<(f32, f32, f32)> {
    let name = d.place_of(&format!("Group name: {group}"))?.right();
    let belongs = d
        .nodes()
        .into_iter()
        .find(|node| node.name.starts_with("Belongs to: "))?
        .rect
        .right();
    let notes = d.place_of(&format!("Group notes for {group}"))?.right();
    Some((name, belongs, notes))
}

/// iter-24-B FH3: the right edges in a group editor that must not move when its members list
/// gains a scroll bar: the name field, `Belongs to` and each member's menu, by name.
fn member_column_xs(d: &Driver<'_>, group: &str) -> Vec<(String, f32)> {
    let mut xs: Vec<(String, f32)> = d
        .place_of(&format!("Group name: {group}"))
        .map(|name| ("Group name".to_owned(), name.right()))
        .into_iter()
        .collect();
    xs.extend(
        d.nodes()
            .into_iter()
            .filter(|node| {
                node.name.starts_with("More actions for ") || node.name.starts_with("Belongs to: ")
            })
            .map(|node| (node.name, node.rect.right())),
    );
    xs
}

/// Egui's scroll bar just right of `edge`: an unnamed thin node with no role.
fn scroll_bar_right_of(d: &Driver<'_>, edge: f32) -> bool {
    d.nodes().iter().any(|node| {
        node.role == egui::accesskit::Role::Unknown
            && node.name.is_empty()
            && node.rect.width() <= crate::design::M
            && node.rect.height() >= crate::design::XXL
            && node.rect.left() >= edge - 0.5
            && node.rect.left() <= edge + crate::design::XL
    })
}

/// The right edges of every control named `name`, top to bottom.
fn right_edges(d: &Driver<'_>, name: &str) -> Vec<f32> {
    d.nodes()
        .into_iter()
        .filter(|node| node.name == name && node.in_view())
        .map(|node| node.rect.right())
        .collect()
}

/// Egui's scroll bar beside a group editor's members column: an unnamed thin node with no
/// role, just right of the member menus.
fn members_scroll_bar(d: &Driver<'_>) -> bool {
    let nodes = d.nodes();
    let Some(column) = nodes
        .iter()
        .filter(|node| node.name.starts_with("More actions for "))
        .map(|node| node.rect.right())
        .reduce(f32::max)
    else {
        return false;
    };
    nodes.iter().any(|node| {
        node.role == egui::accesskit::Role::Unknown
            && node.name.is_empty()
            && node.rect.width() <= crate::design::M
            && node.rect.height() >= crate::design::XXL
            && node.rect.left() >= column - 0.5
            && node.rect.left() <= column + crate::design::XL
    })
}

/// The name field and `Belongs to` share one right edge, and it is the column's edge.
fn group_fields_end_at_column(edges: Option<(f32, f32, f32)>) -> bool {
    edges.is_some_and(|(name, belongs, notes)| {
        (name - belongs).abs() <= 0.5 && (name - notes).abs() <= 0.5
    })
}

/// iter-19-B FD4: from a notes field's frame to the start of its words (text or placeholder).
/// A field's label has the field's name too; the field is the wider node.
fn notes_inset(d: &Driver<'_>, field: &str, words: &str) -> Option<f32> {
    let frame = d
        .nodes()
        .into_iter()
        .filter(|node| node.name == field)
        .map(|node| node.rect)
        .max_by(|a, b| a.width().total_cmp(&b.width()))?;
    Some(d.painted_rect_in(words, frame)?.left() - frame.left())
}

/// W13: groups for one encounter beside groups for any encounter.
pub mod encounter_owned_groups {
    use super::*;
    const SHRINE: &str = "Shrine of Cinders";
    const BELONGS_DROPDOWN: usize = 2;
    const NEW_FOR_ENCOUNTER: &str = "New group for this encounter";
    const NEW_NAME: &str = "Ash Hounds";
    const SAVE_ENCOUNTER: &str = "Save encounter";

    fn edit_group(name: &str) -> String {
        format!("Edit group {name}")
    }

    pub fn run(d: &mut Driver<'_>) {
        // The page lists what can arrive anywhere. An encounter's own stay out of the way.
        d.expect("GROUPS");
        // FB2: opening Groups makes the page scroll at 960, and no row action moves.
        let actions = |d: &Driver<'_>| {
            let menus: Vec<f32> = d
                .nodes()
                .into_iter()
                .filter(|node| node.name.starts_with("More actions for ") && node.in_view())
                .map(|node| node.rect.right())
                .collect();
            let bar = menus
                .iter()
                .copied()
                .reduce(f32::max)
                .is_some_and(|edge| scroll_bar_right_of(d, edge));
            (right_edges(d, "Resume"), menus, bar)
        };
        let closed = d.at_size(SMALL, actions);
        d.state("groups-for-encounters-closed", Sizes::All);
        d.click_starting("GROUPS");
        let open = d.at_size(SMALL, actions);
        d.observe(
            "iter-25-B-FB2",
            !closed.2
                && open.2
                && !closed.0.is_empty()
                && closed.0.first() == open.0.first()
                && !closed.1.is_empty()
                && closed.1.first() == open.1.first(),
            format!("At 960, Resume, row menus and scroll bar, closed {closed:?}, open {open:?}"),
        );
        d.check_tab_order("encounters, with groups for encounters closed");
        d.observe(
            "T13.5",
            d.has(&edit_group(GROUP_GOBLINS))
                && d.has(&edit_group(GROUP_ORPHAN))
                && d.has(&edit_group(GROUP_SHRINE))
                && d.has(&edit_group(GROUP_ARENA))
                && d.has_containing("ENCOUNTER DELETED"),
            "Groups opens one flat list, with headings for each encounter and deleted encounters last",
        );
        d.expect(&edit_group(GROUP_SHRINE));
        d.state("groups-for-encounters-open", Sizes::All);
        d.observe(
            "T13.5",
            d.has_containing("FOR SHRINE OF CINDERS") && d.has_containing("FOR ARENA FINAL"),
            "Opened, the section says who each group is for",
        );
        // One look and one wording for a page's content issue count, as Campaign's.
        d.observe(
            "iter-23-B-FA7",
            d.has(&format!(
                "{} 2 content issues to repair",
                crate::design::icons::WARN
            )),
            "Encounters counts its content issues in Campaign's words",
        );

        // A group whose encounter is gone says so in its editor.
        d.click(&edit_group(GROUP_ORPHAN));
        d.expect("Deleted encounter");
        d.state_prepared("group-whose-encounter-is-gone", Sizes::Default, |d| {
            let edges = group_field_edges(d, GROUP_ORPHAN);
            d.observe(
                "iter-19-B-FD3",
                group_fields_end_at_column(edges),
                format!(
                    "The group name field and Belongs to end at the notes field's right edge: {edges:?}"
                ),
            );
            // Must stay true: the shorter choice still shows its whole value.
            d.observe(
                "iter-19-B-FD3-value",
                d.visible_text("Deleted encounter"),
                "Belongs to still shows Deleted encounter",
            );
            // A group no fight can offer says so as a warning, not as field help.
            let tones = tones_beside(d, "No fight offers this group now.");
            d.observe(
                "iter-23-B-FB2",
                tones == ["warning"],
                format!("The line under Belongs to is a warning: {tones:?}"),
            );
        });
        let inset = notes_inset(
            d,
            &format!("Group notes for {GROUP_ORPHAN}"),
            "When they arrive, what they want.",
        );
        d.observe(
            "iter-19-B-FD4",
            inset.is_some_and(|inset| (inset - crate::design::S).abs() <= 0.5),
            format!("Group notes starts its words S inside its frame: {inset:?}"),
        );
        d.click("Back to encounters");

        // An encounter's editor has a section for each of its own, with what is in it.
        open_saved_encounter(d, SHRINE);
        d.expect("GROUPS FOR THIS ENCOUNTER 1");
        d.expect("Adding to the roster");
        // Add participants says where party members come from in the DM's words.
        d.expect("Party members come from the sheets Splat saved last.");
        d.state_prepared("encounter-with-its-groups", Sizes::All, |d| {
            // FD3 family: a group's name in the encounter's frame starts at the frame's left edge,
            // where its notes start, at every width.
            let lefts = d
                .place_of(&format!("Group name: {GROUP_SHRINE}"))
                .zip(d.place_of(&format!("Group notes for {GROUP_SHRINE}")))
                .map(|(name, notes)| (name.left(), notes.left()));
            d.observe(
                "iter-19-B-FD3-frame",
                lefts.is_some_and(|(name, notes)| (name - notes).abs() <= 0.5),
                format!("The frame's name field starts where its notes start: {lefts:?}"),
            );
        });
        // The notes are below the roster; at the large size they are on screen.
        let inset = d.at_size(LARGE, |d| {
            notes_inset(d, "Read-aloud and notes", "The cultists retreat")
        });
        d.observe(
            "iter-19-B-FD4-encounter",
            inset.is_some_and(|inset| (inset - crate::design::S).abs() <= 0.5),
            format!("Read-aloud and notes starts its words S inside its frame: {inset:?}"),
        );
        d.check_tab_order("encounter editor, with its groups");
        d.observe(
            "T13.6",
            d.has(&format!("Group name: {GROUP_SHRINE}"))
                && d.has_containing("Goblin Warrior")
                && d.has("MEMBERS 2")
                && d.has_containing("XP 100"),
            "A section shows the group's name and its members",
        );

        // A new group is built right here, and saved with the encounter.
        d.begin_task();
        d.click(NEW_FOR_ENCOUNTER);
        d.type_text(NEW_NAME);
        d.observe(
            "T13.6",
            d.has(&format!("Adding to {NEW_NAME}"))
                && !d.has(&format!("Belongs to: {SHRINE}"))
                && d.has_containing("No members yet. Add creatures from the search."),
            "A new group belongs to this encounter, and takes what the search adds",
        );
        d.state("new-group-in-the-encounter", Sizes::All);
        // Two frames, one adding and one not: both name fields end at one edge at every size.
        for size in [SMALL, DEFAULT, LARGE] {
            let edges = d.at_size(size, |d| {
                d.place_of(&format!("Group name: {GROUP_SHRINE}"))
                    .zip(d.place_of(&format!("Group name: {NEW_NAME}")))
                    .map(|(shrine, new)| (shrine.right(), new.right()))
            });
            d.observe(
                "iter-23-B-FB4",
                d.has(&format!("Add creatures to {GROUP_SHRINE}"))
                    && d.has(&format!("Add creatures to {NEW_NAME}"))
                    && edges.is_some_and(|(shrine, new)| (shrine - new).abs() <= 0.5),
                format!("At {size:?}, the name fields end at one x: {edges:?}"),
            );
        }
        let roster_before = d.has_containing("ROSTER 7");
        d.type_into(FIND_CREATURE, "wolf");
        d.click_beside_exact("Wolf", ADD);
        d.expect("1. Wolf");
        d.observe(
            "T13.6",
            roster_before && d.has_containing("ROSTER 7"),
            "What the search adds goes to the group, and the roster stays as it was",
        );
        d.observe(
            "T13.6",
            d.in_view("1. Wolf"),
            "The member just added comes into view, in its group",
        );
        d.state("creature-added-to-a-group", Sizes::All);
        d.click(SAVE_ENCOUNTER);
        d.expect("Saved");
        d.observe(
            "T13.6",
            d.in_view("1. Wolf"),
            "After a save, the group being built is still in view",
        );
        d.state("encounter-and-group-saved", Sizes::Default);
        d.goal("build a group in an encounter's editor");
        d.begin_task();
        d.click_first(&format!("More actions for {NEW_NAME}"));
        d.click_containing("Move to…");
        d.expect(RUN_ARENA);
        assert!(!d.has("Any encounter"));
        // SD2/AD1: in each capture, the submenu is clear of the roster's scroll bar and the
        // splitter before Add participants.
        let clear = |d: &mut Driver<'_>| {
            let size = d.window_size();
            let clear = super::submenu_clear_of_chrome(d, &[RUN_ARENA, RUN], true)
                .map(|frame| (frame, d.place_of("Move to…")));
            d.observe(
                "iter-22-B-SD2",
                clear.is_ok(),
                format!("At {size:?}, the Move to… submenu and its menu: {clear:?}"),
            );
        };
        d.state_prepared("move-encounter-group-menu", Sizes::All, clear);
        // Astra review of PR 231: shrunk from 1920 with both menus open, the menu and its
        // submenu stay inside the editor, clear of its scroll bar and splitter.
        d.state_prepared(
            "move-encounter-group-menu-after-shrink",
            Sizes::Small,
            clear,
        );
        // Astra re-check of PR 231: only a resize brings the menu's button back. With the menu
        // open, the DM's wheel moves the editor, and the editor stays where the DM left it.
        let opener = format!("More actions for {NEW_NAME}");
        let editor = pos2(250.0, 200.0);
        let before = d.place_of(&opener);
        let mut notches = 0;
        while d.in_view(&opener) && notches < 10 {
            d.scroll_up_at(editor, 1);
            notches += 1;
        }
        let left = d.place_of(&opener);
        d.settle();
        let after = d.place_of(&opener);
        d.observe(
            "iter-22-B-SD2-wheel",
            !d.in_view(&opener)
                && before
                    .zip(left)
                    .is_some_and(|(before, left)| left.top() > before.top())
                && left == after
                && d.has("Make available to any encounter"),
            format!(
                "After {notches} wheel notches with the menu open, its button went from {before:?} to {left:?} and then {after:?}"
            ),
        );
        d.scroll_until(pos2(250.0, 400.0), &opener, 20);
        d.click("Make available to any encounter");
        d.expect("Undo move");
        let notice = format!("{NEW_NAME} moves to any encounter when you save.");
        d.expect(&notice);
        d.state_prepared("saved-group-removed", Sizes::All, |d| {
            d.scroll_until(egui::pos2(400.0, 400.0), "Undo move", 20);
            assert!(d.in_view_containing(&notice));
        });
        d.click("Undo move");
        d.expect(&format!("Group name: {NEW_NAME}"));
        d.begin_task();
        d.click("Back to encounters");
        d.expect("GROUPS");
        d.expect("FOR SHRINE OF CINDERS 2");

        // A group can change hands, in its editor.
        d.click(&edit_group(GROUP_WATCH));
        d.expect("A run of any encounter offers this group.");
        d.open_combo_at(BELONGS_DROPDOWN);
        // FB8: the open menu's painted frame is as wide as its control at 960, 1200 and 1920,
        // and back at 960, while it stays open through each resize.
        const BELONGS_TO_LABEL: &str = "Belongs to";
        const CONTROL: &str = "Belongs to: Any encounter";
        for size in [SMALL, DEFAULT, LARGE, SMALL] {
            let (control, frame) = d.at_size(size, |d| {
                let choices = ["Any encounter", RUN_ARENA]
                    .map(|name| d.place_of(name))
                    .into_iter()
                    .collect::<Option<Vec<_>>>()
                    .map(|rects| {
                        rects
                            .into_iter()
                            .fold(egui::Rect::NOTHING, |a, b| a.union(b))
                    });
                (
                    d.place_of(CONTROL),
                    choices.and_then(|choices| d.painted_frame_around(choices)),
                )
            });
            d.observe(
                "iter-25-B-FB8",
                control.zip(frame).is_some_and(|(control, frame)| {
                    (frame.left() - control.left()).abs() <= 1.0
                        && (frame.right() - control.right()).abs() <= 1.0
                }),
                format!(
                    "At {size:?} the Belongs to menu frame {frame:?} spans the control {control:?}"
                ),
            );
        }
        // A proof aside (revision 17 menu checks): the control owns its menu, and the focus
        // comes back to it when the menu closes. Mouse open (above), then Escape; Enter opens it
        // by the keys, then Escape; Tab leaves and Shift+Tab comes back; a click on an outside
        // field closes it and the focus stays on that field. The group stays unchanged, and the
        // menu is opened once more for the capture. A choice is checked after the capture.
        const NOTES: &str = "Group notes for Watch Detail";
        d.aside(|d| {
            d.press(Key::Escape);
            let closed = !d.has(RUN_ARENA) && d.has(CONTROL);
            let after_escape = d.focused_name();
            d.press(Key::Enter);
            let opened = d.has(RUN_ARENA);
            d.press(Key::Escape);
            let closed_again = !d.has(RUN_ARENA);
            let after_keys = d.focused_name();
            d.press(Key::Tab);
            let after_tab = d.focused_name();
            d.press_shift(Key::Tab);
            let back = d.focused_name();
            d.open_combo_at(BELONGS_DROPDOWN);
            let reopened = d.has(RUN_ARENA);
            d.click(NOTES);
            let outside = d.focused_name();
            let closed_by_click = !d.has(RUN_ARENA);
            let clean = !d.visible_text("Unsaved");
            d.observe(
                "iter-25-B-FB8-escape",
                closed
                    && after_escape.as_deref() == Some(CONTROL)
                    && opened
                    && closed_again
                    && after_keys.as_deref() == Some(CONTROL)
                    && after_tab.is_some()
                    && after_tab.as_deref() != Some(CONTROL)
                    && back.as_deref() == Some(CONTROL)
                    && reopened
                    && closed_by_click
                    && outside.as_deref() == Some(NOTES)
                    && clean,
                format!(
                    "Escape closes {closed} (focus {after_escape:?}); Enter opens {opened}, Escape closes {closed_again} (focus {after_keys:?}); Tab {after_tab:?}, Shift+Tab {back:?}; a click on the notes closes {closed_by_click} (focus {outside:?}); unchanged {clean}"
                ),
            );
            d.open_combo_at(BELONGS_DROPDOWN);
        });
        d.state("belongs-to-open", Sizes::Default);
        d.click(RUN_ARENA);
        d.expect("Only runs of this encounter offer this group.");
        // A choice closes the menu and gives the focus back to its control.
        let after_choice = d.focused_name();
        d.observe(
            "iter-25-B-FB8-choice",
            after_choice.as_deref() == Some(&format!("{BELONGS_TO_LABEL}: {RUN_ARENA}")[..]),
            format!("After a choice the focus is on {after_choice:?}"),
        );
        d.click("Save group");
        d.expect("Saved");
        d.click("Back to encounters");

        // At the table: its own first, then those for any encounter.
        run_saved_encounter(d, SHRINE);
        open_arrivals(d);
        d.expect(&add_group(GROUP_SHRINE));
        d.state_prepared("group-panel-for-one-encounter", Sizes::All, |d| {
            // SD5: every group's Initiative and Add block shares one column.
            let groups = [
                NEW_NAME,
                GROUP_SHRINE,
                GROUP_GOBLINS,
                GROUP_LOST,
                GROUP_RIVALS,
            ];
            let buttons = groups.map(|name| {
                d.place_of(&add_group(name))
                    .expect("Required group Add control")
            });
            let fields = groups.map(|name| {
                d.place_of(&format!("Initiative total for {name}"))
                    .expect("Required group Initiative field")
            });
            d.observe(
                "iter10-B-SD5-column",
                buttons
                    .iter()
                    .all(|b| (b.right() - buttons[0].right()).abs() <= 1.0)
                    && fields
                        .iter()
                        .all(|f| (f.left() - fields[0].left()).abs() <= 1.0),
                format!("Initiative and Add share one column: {fields:?} {buttons:?}"),
            );
        });
        d.check_tab_order("run, groups for one encounter");
        d.observe(
            "T13.5",
            d.has(&add_group(NEW_NAME))
                && d.has(&add_group(GROUP_GOBLINS))
                && !d.has(&add_group(GROUP_ARENA))
                && !d.has(&add_group(GROUP_ORPHAN))
                && !d.has(&add_group(GROUP_WATCH)),
            "A run offers its encounter's groups and those for any encounter, and no others",
        );
        d.begin_task();
        d.click(&add_group(GROUP_SHRINE));
        d.expect("Goblin Warrior 1");
        d.goal_within("T13.5", "add this encounter's own group", 1);

        // The same panel during combat, with a typed total and by dragging.
        d.click("Roll NPC initiative");
        for (name, total) in [
            ("Brannoch Vale", "18"),
            ("Iselle Marrowind", "15"),
            ("Kestrel Dunmoor", "12"),
            ("Oren Thistlewick", "9"),
            ("Sable Harrowgate", "6"),
        ] {
            d.type_into(&format!("Initiative for {name}"), total);
        }
        d.click("Begin combat");
        d.expect(NEXT_TURN);
        open_arrivals(d);
        d.expect(&add_group(NEW_NAME));
        d.state("own-groups-in-combat", Sizes::Default);
        // iter-25-R FB1: an arrival's result never moves a group row: not when it shows, not
        // when a tie adds its words and its Move button, not after Move or its Undo. Each
        // group's Add, at 960, 1200 and 1920.
        let group_rows = |d: &mut Driver<'_>| {
            [SMALL, DEFAULT, LARGE].map(|size| {
                d.at_size(size, |d| {
                    [
                        NEW_NAME,
                        GROUP_SHRINE,
                        GROUP_GOBLINS,
                        GROUP_LOST,
                        GROUP_RIVALS,
                    ]
                    .map(|name| d.place_of(&add_group(name)).map(|rect| rect.top()))
                })
            })
        };
        let rows_still = group_rows(d);
        d.type_into(&format!("Initiative total for {NEW_NAME}"), "14");
        d.click(&add_group(NEW_NAME));
        d.expect("Wolf");
        let row_after = group_rows(d);
        d.observe(
            "iter-25-R-FB1",
            d.visible_text(&format!("{NEW_NAME} joined at\u{a0}14."))
                && rows_still.iter().flatten().all(Option::is_some)
                && rows_still == row_after,
            format!("The result shows and the group rows stay: {rows_still:?} then {row_after:?}"),
        );
        d.state("own-group-added-in-combat", Sizes::Default);
        d.observe(
            "T13.5",
            d.has_containing("Initiative 14") && d.has_containing("Undo: add Ash Hounds"),
            "An encounter's own group joins the fight on a typed total, and Undo names it",
        );
        let rows_before = roster_places(d).len();
        let list = roster_places(d)
            .first()
            .map(|(_, rect)| *rect)
            .expect("run rows");
        d.drag_hold(&format!("Drag {GROUP_SHRINE} to the initiative list"), list);
        d.drag_release(list);
        d.observe(
            "T13.5",
            roster_places(d).len() == rows_before + 2,
            "Dragging an encounter's own group in combat adds each member",
        );
        // iter-25-R FB1, continued: the tied result, Move, and Undo of the move.
        let tie_action = |d: &Driver<'_>| {
            d.nodes()
                .into_iter()
                .find(|node| node.name.starts_with(&format!("Move {GROUP_SHRINE}")))
                .map(|node| node.name)
        };
        let tied = tie_action(d);
        let after_tie = group_rows(d);
        let mut after_move = None;
        let mut after_undo = None;
        if let Some(action) = &tied {
            d.click(action);
            d.wait_for_text("moved before");
            after_move = Some(group_rows(d));
            d.click_starting("Undo: ");
            d.expect(action);
            after_undo = Some(group_rows(d));
        }
        d.observe(
            "iter-25-R-FB1-tie",
            tied.is_some()
                && after_tie == rows_still
                && after_move.as_ref() == Some(&rows_still)
                && after_undo.as_ref() == Some(&rows_still),
            format!(
                "Group rows stay through a tie {tied:?}: {rows_still:?}, tie {after_tie:?}, move {after_move:?}, undo {after_undo:?}"
            ),
        );
        d.expect("Maximum HP would become −28. It must be 1 or more.");
        // The dragged group's initiative comes from the run's dice. A tie adds a notice at the
        // top of the panel, which can push the faults below a small window: the panel scrolls,
        // so bring the second fault's link into view first. With no tie, this scrolls nothing.
        d.painted_rect_in("Cannot join", egui::Rect::EVERYTHING)
            .expect("the panel names the groups that cannot join");
        // Scroll inside the list: a tie's result is pinned below it, where the wheel would
        // not reach the list.
        let list = d
            .place_of(&add_group(NEW_NAME))
            .expect("the list shows the group that joined");
        d.scroll_until(list.center(), "Edit group", 5);
        // iter-25-R FB5: a member fault's repair is a bordered secondary button. Below its
        // fault, its border starts at the warning text; in a wide panel, it stays beside the
        // fault on its line, after an `L` gap.
        for size in [SMALL, DEFAULT, LARGE] {
            let (placed, detail) = d.at_size(size, |d| {
                let mut placed = true;
                let mut detail = String::new();
                for (fault, repair) in [
                    ("Unknown creature · line\u{a0}13:", "Campaign → Packages"),
                    ("Cinder Wolf · line\u{a0}14: Maximum HP", "Edit group"),
                ] {
                    let text = d.painted_rect_in(fault, egui::Rect::EVERYTHING);
                    let buttons: Vec<_> = d
                        .nodes()
                        .into_iter()
                        .filter(|node| node.name == repair)
                        .map(|node| node.rect)
                        .filter(|rect| {
                            text.is_some_and(|text| {
                                rect.center().y >= text.top() - crate::design::XL
                                    && rect.center().y < text.bottom() + crate::design::XL
                            })
                        })
                        .collect();
                    let bordered = |rect: &egui::Rect| {
                        d.control_frame_at(*rect)
                            .is_some_and(|frame| frame.stroke.width > 0.0)
                    };
                    placed &= match (text, buttons.as_slice()) {
                        (Some(text), [button]) if size == LARGE => {
                            bordered(button)
                                && button.center().y < text.bottom()
                                && button.left() >= text.right() + crate::design::L - 0.5
                        }
                        (Some(text), [button]) => {
                            bordered(button)
                                && button.top() >= text.bottom() - 0.5
                                && (button.left() - text.left()).abs() < 1.0
                        }
                        _ => false,
                    };
                    detail += &format!("{repair}: text {text:?}, buttons {buttons:?}. ");
                }
                (placed, detail)
            });
            d.observe(
                "iter-25-R-FB5",
                placed,
                format!("Arrivals repairs are bordered message actions at {size:?}: {detail}"),
            );
        }
        // iter-26-R FA6 (replaces iter-25-R FA4): the tie button sits under the message that
        // names the arrival whole, so a cut button gives up the whole parenthesis; it never
        // shows a bare `(… 3)`. Its accessible name keeps the whole words.
        let tie = d
            .nodes()
            .into_iter()
            .find(|node| node.name.starts_with("Move Shrine Reserve"))
            .map(|node| node.name);
        d.observe(
            "iter-26-R-FA6-found",
            tie.as_deref()
                .is_some_and(|name| name.contains("(Goblin Warrior 3)")),
            format!("The tie button's accessible name: {tie:?}"),
        );
        for size in [SMALL, DEFAULT, LARGE] {
            let words = d.at_size(size, |d| {
                d.nodes()
                    .into_iter()
                    .find(|node| node.name.starts_with("Move Shrine Reserve"))
                    .map(|node| d.painted_lines_in(node.rect).join(" "))
            });
            d.observe(
                "iter-26-R-FA6",
                words.as_deref().is_some_and(|words| {
                    words.starts_with("Move Shrine")
                        && !words.contains("(…")
                        && (words.contains("(Goblin\u{a0}Warrior\u{a0}3)")
                            || words.contains("(Goblin Warrior 3)")
                            || !words.contains('('))
                }),
                format!("At {size:?} the tie button says: {words:?}"),
            );
        }
        d.state("own-group-dragged-in-combat", Sizes::All);
        close_run(d);

        // A save from before this cannot say which encounter it is, so it hides nothing.
        resume_run(d, RUN);
        open_arrivals(d);
        d.expect(&add_group(GROUP_GOBLINS));
        d.state("group-panel-of-an-older-save", Sizes::Default);
        d.observe(
            "T13.5",
            d.has(&add_group(GROUP_ARENA)) && d.has_containing("For Arena Final"),
            "A run from an older save lists every group, and says who each is for",
        );
    }
}

/// W13: changes built in to one creature, never hidden, in the editor and on the card.
pub mod run_overrides {
    use super::*;
    const RIVAL: &str = "Rival Captain";
    const FIRST_SLOT: &str = "6. Rival Captain";
    const BASE_AND_CHANGED: &str = "Maximum HP: 33 → 43 (+10)";

    pub fn run(d: &mut Driver<'_>) {
        // The card has both values, and marks what the run cannot apply.
        resume_run(d, RUN_ARENA);
        d.expect(&format!("{RUN_HEADING}{RUN_ARENA}"));
        d.click_row(RIVAL);
        d.state("overridden-participant", Sizes::AllAndHiDpi);
        d.observe_both(
            "iter-12-F3",
            "The run card keeps hand changes in one compact summary",
            |d| d.visible_text("Add by hand: speed 40 ft"),
        );
        // Splat's rolls include the attack and damage changes, so the card says them once:
        // the tiles show the numbers rolled, and nothing tells the DM to add them again.
        d.observe_both(
            "iter-19-R-FT3",
            "The card states the roll changes once, in the tiles' numbers",
            |d| {
                d.visible_text("+4 · 1d6+3\u{a0}slashing")
                    && !d.has_containing("Add by hand: attack")
                    && !d.has_containing("in the rolls")
                    && d.visible_text("+1 on each attack · in Splat’s rolls")
            },
        );
        // A number stays with its word on the row, by a space that does not wrap.
        let row = d.row_words(RIVAL).replace('\u{a0}', " ");
        d.observe(
            "T13.3",
            row.contains("Blessed")
                && row.contains("Changed")
                && ((row.contains("Attack +1") && row.contains("Damage +1"))
                    || row.contains("more")),
            format!("The row marks a change and carries its bonuses: {row}"),
        );
        d.observe(
            "T13.3",
            !d.has_containing(BASE_AND_CHANGED)
                && d.visible_text("printed 33")
                && d.visible_text("Add by hand: speed 40 ft")
                && d.has_containing("add by hand")
                && d.visible_text("Speed 30 ft → 40 ft · add by hand")
                && d.has_containing("Changed"),
            "The card shows the base and changed values, and says what to add by hand",
        );
        d.observe(
            "Changed stat values",
            d.visible_text("printed 12") && d.visible_text("printed 33"),
            "AC and HP show the run's values with their printed values below",
        );
        d.goal("read a changed creature's card in a run");
        close_run(d);

        // Start the next intention on the ledger, after closing the fight just read.
        d.begin_task();
        // The editor names the creature, and each change under it.
        open_saved_encounter(d, RUN_ARENA);
        d.expect(FIRST_SLOT);
        d.state("slot-with-changes", Sizes::All);
        // Must stay true (iter-23 FJ1): with no changes, Resume keeps the primary.
        let primary = super::session_recording::accent_filled(d);
        d.observe(
            "iter-23-B-FJ1-clean",
            d.visible_text("No changes") && primary == ["Resume"],
            format!("A clean editor's primary is Resume: {primary:?}"),
        );
        d.observe_both(
            "FT-10",
            "A changed roster slot keeps its base creature",
            |d| d.visible_text("as\u{a0}Ember\u{a0}Cultist"),
        );
        d.observe_both(
            "iter-18-A-changes",
            "The whole hand-change link stays in view",
            |d| d.in_view("Changed · 1 to add by hand"),
        );
        d.click_beside(
            FIRST_SLOT,
            "Options: starting HP, temporary HP, initiative group and changes",
        );
        d.observe(
            "T13.3",
            d.in_view("Maximum HP: how it changes"),
            "Opening a slot's overrides brings the form into view, however low the slot is",
        );
        d.state("slot-changes-open", Sizes::All);
        // This encounter already has a paused run. Starting a second fight is explicit.
        d.click(&format!("More actions for {RUN_ARENA}"));
        d.click("Start another fight");
        d.expect(&format!("{RUN_HEADING}{RUN_ARENA}"));
        d.state("setup-with-a-changed-row", Sizes::Default);
        d.observe(
            "T13.3",
            d.has_containing("Changed ·") && d.has_containing("Enemies · HP 43/43 · AC 15"),
            "A new run starts with the slot's changes, and its setup row says so",
        );
        d.goal("start a run with a changed creature");
    }
}

/// Loot: the editor builds a list from typed words and the campaign's tables, a creature that
/// is down says so on its row, its card lists what it carries, and the ended view lists all of
/// it with a button for each roll.
pub mod run_loot {
    use super::*;
    const ADD: &str = "Add loot to the encounter";
    const GUARD: &str = "Vault Guard";
    const ROLL_ENCOUNTER: &str = "Roll 2 × gems for Sunken Vault";
    const UNDO_ROLLS: &str = "Undo: 2 loot rolls on gems";

    pub fn run(d: &mut Driver<'_>) {
        resume_run(d, RUN_VAULT);
        let row = d.row_words(GUARD).replace('\u{a0}', " ");
        d.observe(
            "T14.8",
            row.contains("0 HP") && row.contains("Loot"),
            format!("A creature at 0 HP with loot says so on its row: {row}"),
        );
        d.click_row(GUARD);
        d.expect("Vault key");
        d.observe_both(
            "iter7-B-FT1",
            "A monster at zero HP keeps loot first without death-save controls",
            |d| {
                !d.has("Add a death save success")
                    && !d.has("Add a death save failure")
                    && d.in_view("Roll gems for Vault Guard")
            },
        );
        d.state("loot-on-the-card", Sizes::All);
        d.observe(
            "T14.8",
            d.has("Roll gems for Vault Guard"),
            "The card lists the creature's loot, with a button for its roll",
        );
        d.goal("see what a fallen creature carries");

        d.begin_task();
        d.click("End encounter…");
        d.click("End encounter");
        d.expect("Outcome: ");
        d.expect("120 gp in a locked coffer");
        d.state("ended-with-loot", Sizes::All);
        d.observe(
            "iter-23-R-FJ10",
            d.has_containing("Fight prepared from the saved encounter.")
                && !d.has_containing("DM run prepared")
                && !d.has_containing("Rules are resolved by the DM"),
            "The Log's first line speaks to the DM in the app's words",
        );
        d.observe(
            "T14.8",
            d.has(ROLL_ENCOUNTER) && d.has("Roll gems for Vault Guard"),
            "The ended view lists the encounter's loot and the loot of each creature that is down",
        );
        d.click(ROLL_ENCOUNTER);
        d.expect(" on 1d4");
        d.state("loot-rolled", Sizes::Default);
        // FI5: a count of rolls uses the roster's form, `2 × gems`, with a blank before `×`.
        d.observe(
            "iter-24-R-FI5",
            d.visible_text("2 × gems · ") && !d.has_containing("2× gems"),
            "The rolled loot heading says 2 × gems",
        );
        d.observe(
            "T14.8",
            d.has_containing(UNDO_ROLLS),
            "Each loot roll is one step that Undo takes back, after the fight has ended",
        );
        d.goal("roll the loot after a fight");
        // iter-25-R, Astra 3: `Loot copied.` belongs to that copy. Undo ends it, and so does the
        // next roll, even one that leaves the Log as long as it was when copied.
        d.click("Copy loot");
        d.wait_for_text("Loot copied.");
        d.click_starting(UNDO_ROLLS);
        let after_undo = d.visible_text("Loot copied.");
        d.click(ROLL_ENCOUNTER);
        d.expect(" on 1d4");
        let after_roll = d.visible_text("Loot copied.");
        d.observe(
            "iter-25-R-loot-copy-owner",
            !after_undo && !after_roll,
            format!("Loot copied ends on Undo ({after_undo}) and on the next roll ({after_roll})"),
        );
        // The same with no frame between the Undo and the next roll.
        d.click("Copy loot");
        d.wait_for_text("Loot copied.");
        d.app_mut()
            .encounters
            .run_mut()
            .expect("the ended run is open")
            .simulate_undo_and_roll_first_loot();
        d.settle();
        d.observe(
            "iter-25-R-loot-copy-owner-no-draw",
            d.visible_text(" on 1d4") && !d.visible_text("Loot copied."),
            "Loot copied ends when Undo and a roll come before the next frame",
        );

        // The editor: typed words are a line, and a table comes from the campaign's list.
        d.begin_task();
        close_run(d);
        open_saved_encounter(d, RUN);
        d.type_into(ADD, "120 gp in a locked coffer");
        d.press(Key::Enter);
        d.type_into(ADD, "roll");
        d.expect("gems · 1d4 · Treasure.md");
        d.state("loot-table-suggestion", Sizes::Default);
        // The DM has just typed loot into a paused fight's encounter: Save is the one primary.
        let primary = super::session_recording::accent_filled(d);
        d.observe(
            "iter-23-B-FJ1",
            d.has("Resume") && primary == ["Save encounter"],
            format!("An unsaved draft owns the primary over Resume: {primary:?}"),
        );
        d.observe(
            "T14.8",
            d.has_containing("the encounter loot line 1"),
            "Enter adds the typed words as a line, and the field offers the campaign's tables",
        );
        d.click_containing("gems · 1d4 · Treasure.md");
        d.type_into(ADD, "Roll coins");
        d.press(Key::Enter);
        d.type_into(ADD, "veil");
        d.click_containing("Veilglass Lantern ·");
        for size in [SMALL, DEFAULT, (1920.0, 1080.0)] {
            let fits = d.at_size(size, |d| {
                let roster = d.place_of("1. Brannoch Vale").unwrap();
                let notes = d
                    .nodes()
                    .into_iter()
                    .find(|node| {
                        node.name == "Read-aloud and notes"
                            && node.role == egui::accesskit::Role::MultilineTextInput
                    })
                    .unwrap();
                let actions: Vec<_> = d
                    .nodes()
                    .into_iter()
                    .filter(|node| {
                        node.name
                            == "Options: starting HP, temporary HP, initiative group and changes"
                    })
                    .collect();
                !actions.is_empty()
                    && actions.iter().all(|node| {
                        node.rect.right() - roster.left() <= crate::design::LIST_MEASURE
                    })
                    && notes.rect.width() <= crate::design::LIST_MEASURE
                    && d.place_of(FIND_CREATURE).is_some_and(|search| {
                        search.left() > notes.rect.right()
                            && search.left() - notes.rect.right() <= crate::design::XL * 2.0
                    })
            });
            d.observe("iter7-B-AT1", fits, format!("Roster actions and notes keep LIST_MEASURE with Add directly beside them at {size:?}"));
        }
        d.state("loot-in-the-editor", Sizes::All);
        d.observe(
            "T14.8",
            d.has("Rolls on gems, the encounter loot line 2")
                && d.has_containing("No roll table has the key “coins”.")
                && d.has("Library item"),
            "A roll names its table and count, a key with no table says so before play, and a Library item is a line of its own",
        );
        d.goal("add loot and a roll on a campaign table");
        // iter-25-R, Astra 4: a removed loot line is a finished change: a success message, with
        // its Undo, which puts the line back.
        let lines = |d: &Driver<'_>| {
            d.nodes()
                .into_iter()
                .filter(|node| {
                    node.name
                        .starts_with("More actions for the encounter loot line")
                })
                .count()
        };
        let before = lines(d);
        d.click("More actions for the encounter loot line 1");
        d.click("Remove");
        let removed =
            d.painted_rect_in("Removed 120 gp in a locked coffer.", egui::Rect::EVERYTHING);
        let success = removed
            .is_some_and(|words| d.has_tone_bar(words, crate::design::colors().result.success));
        let fewer = lines(d);
        d.click("Undo remove loot");
        d.observe(
            "iter-25-R-loot-removed",
            success && fewer + 1 == before && lines(d) == before && !d.visible_text("Removed 120 gp"),
            format!("Removed is a success message {success} ({removed:?}); lines {before} → {fewer} → {}", lines(d)),
        );
        // P1c: the add field is the design system's shared one. Beside Lookup: an arrow key and
        // Enter add a suggestion, and the field stays open for the next line. Escape clears the
        // highlight, then the words.
        d.click(NAV_LOOKUP);
        d.type_into(ADD, "roll ge");
        d.expect("gems · 1d4 · Treasure.md");
        d.press(Key::ArrowDown);
        d.state("loot-suggestion-chosen-beside-lookup", Sizes::Default);
        d.press(Key::Enter);
        d.observe(
            "P1c-add-field",
            d.has("Rolls on gems, the encounter loot line 5")
                && d.field_text(ADD).as_deref() == Some("")
                && d.focused_name().as_deref() == Some(ADD),
            "An arrow key and Enter add the suggestion, and the field stays open for the next line",
        );
        d.type_into(ADD, "roll");
        d.press(Key::ArrowDown);
        d.press(Key::Escape);
        let highlight = (d.field_text(ADD), d.focused_name());
        d.clear_text_in(ADD);
        d.type_into(ADD, "rope");
        d.press(Key::Escape);
        let words = (d.field_text(ADD), d.focused_name());
        d.observe(
            "P1c-add-field",
            highlight == (Some("roll".to_owned()), None)
                && words.0.as_deref() == Some("")
                && !d.has_containing("gems · 1d4 · Treasure.md"),
            format!("Escape with a chosen suggestion clears the choice and keeps the words; with none, it clears the words: {highlight:?}, then {words:?}"),
        );
        d.click(NAV_LOOKUP);

        // iter-23 FJ1 beside round 22 FD3: while a run is open, its `Next turn` is the only
        // primary, even with unsaved changes here. Closed, the draft's Save owns it; after Save,
        // and after Discard, Resume does. A draft that Save refused does not own it.
        let primaries = |d: &Driver<'_>| super::session_recording::accent_filled(d);
        let dirty = d.has("Discard");
        d.click_starting("Paused run ·");
        d.expect(NEXT_TURN);
        d.click(NAV_ENCOUNTERS);
        let open = primaries(d);
        d.click("Back to run");
        close_run(d);
        if !d.has("Save encounter") {
            d.click(NAV_ENCOUNTERS);
        }
        d.expect("Save encounter");
        let closed = primaries(d);
        d.observe(
            "iter-23-B-FJ1-run",
            dirty && open == [NEXT_TURN] && closed == ["Save encounter"],
            format!("Unsaved draft: run open {open:?}, run closed {closed:?}"),
        );
        d.click("Save encounter");
        d.settle();
        let saved = primaries(d);
        d.type_into(ADD, "A rope ladder");
        d.press(Key::Enter);
        d.expect("Discard");
        d.click("Discard");
        d.settle();
        let discarded = (primaries(d), d.has("Discard"));
        d.observe(
            "iter-23-B-FJ1-resume",
            saved == ["Resume"] && discarded == (vec!["Resume".to_owned()], false),
            format!("Resume after Save {saved:?}, after Discard {discarded:?}"),
        );
        d.type_into(&format!("Encounter name: {RUN}"), " ");
        d.press_ctrl(Key::A);
        d.press(Key::Delete);
        d.click("Save encounter");
        d.expect("Enter an encounter name.");
        let refused = primaries(d);
        d.observe(
            "iter-23-B-FJ1-refused",
            !refused.iter().any(|name| name == "Save encounter"),
            format!("A draft that Save refused: primaries {refused:?}"),
        );
    }
}

/// W13: a campaign with no groups says what to do, in the run.
pub mod run_no_groups {
    use super::*;

    pub fn run(d: &mut Driver<'_>) {
        resume_run(d, RUN);
        open_arrivals(d);
        d.expect("No groups yet. Make one on Encounters with");
        // iter-25-R FJ7: the sentence's full stop follows the link's last word.
        let link = d.place_of("New group");
        let stop = link.and_then(|link| {
            let area = egui::Rect::from_min_max(
                egui::pos2(link.right() - 1.0, link.top()),
                egui::pos2(link.right() + crate::design::L, link.bottom()),
            );
            d.painted_word_rect(".", area)
        });
        d.observe(
            "iter-25-R-FJ7",
            link.zip(stop)
                .is_some_and(|(link, stop)| (stop.left() - link.right()).abs() <= 2.0),
            format!("A full stop follows New group: link {link:?}, stop {stop:?}"),
        );
        d.state("no-groups", Sizes::Default);
        d.goal("see that a campaign has no groups");
    }
}

/// FT3 repair: a crowded party bar at the minimum window never cuts an item off.
pub mod party_bar_crowded {
    use super::*;
    const HURT: [(&str, &str); 2] = [("Iselle Marrowind", "8"), ("Oren Thistlewick", "23")];

    pub fn run(d: &mut Driver<'_>) {
        resume_run(d, RUN);
        // Kestrel is already Bloodied. Two more make three of five; two keep temporary HP.
        for (name, damage) in HURT {
            d.click_row(name);
            d.type_text(damage);
            d.press(Key::Enter);
            d.expect(&format!("Undo: {damage} damage to {name}"));
        }
        d.click(NAV_SESSIONS);
        d.state_prepared("three-bloodied", Sizes::Default, |d| {
            let (items, _) = session_recording::party_bar_items(d);
            let (width, _) = d.window_size();
            let hide = d.place_of("Hide the party");
            let more = d
                .nodes()
                .into_iter()
                .find(|node| node.name.contains(" more: "))
                .map(|node| node.name);
            let named = |part: &str| {
                items.iter().filter(|item| item.name.contains(part)).count()
                    + more.as_deref().map_or(0, |more| more.matches(part).count())
            };
            let shown = items.len()
                + more
                    .as_deref()
                    .and_then(|more| more.split_whitespace().next()?.parse::<usize>().ok())
                    .unwrap_or(0);
            d.observe(
                &format!("iter-12-C2-FT3-whole-{width}"),
                !items.is_empty()
                    && shown == 5
                    && named("Bloodied") == 3
                    && hide.is_some_and(|hide| {
                        hide.right() <= width
                            && items.iter().all(|item| item.rect.right() <= hide.left())
                    }),
                format!(
                    "At {width}, 3 of 5 Bloodied: every party item lies whole before Hide the \
                     party, and every player is named ({} items, {more:?})",
                    items.len()
                ),
            );
        });
    }
}

/// W13: one creature joins a fight with nothing prepared, made up or found.
pub mod run_add_creature {
    use super::*;
    const FIELD: &str = "Find or name a creature";

    fn check_form(d: &mut Driver<'_>) {
        d.observe_both(
            "iter-18-A-arrivals",
            "Arrivals rows start at the search edge and all fields fit",
            |d| {
                let search = d.place_of(FIELD).or_else(|| d.place_of("Wolf")).unwrap();
                ["Count", "Initiative"].iter().all(|name| {
                    d.place_of(name)
                        .is_some_and(|r| (r.left() - search.left()).abs() < 1.0)
                }) && [
                    "New creature count",
                    "New creature HP",
                    "New creature AC",
                    "New creature initiative total",
                    "Side of the new creature",
                ]
                .iter()
                .all(|name| d.in_view(name))
            },
        );
    }

    pub fn run(d: &mut Driver<'_>) {
        resume_run(d, RUN);
        open_arrivals(d);
        d.expect(FIELD);
        // iter-26-R FJ1: the search field's right edge, before typed words make the list long
        // enough to scroll.
        let search_right = |d: &mut Driver<'_>| {
            [SMALL, DEFAULT]
                .map(|size| d.at_size(size, |d| d.place_of(FIELD).map(|rect| rect.right().round())))
        };
        let short_list = search_right(d);
        d.state("arrivals-empty", Sizes::All);
        d.check_tab_order("run, arrivals panel");

        d.type_into(FIELD, "Ghost of the Mill");
        d.expect("Made up here");
        check_form(d);
        d.observe(
            "T13.7",
            d.has_containing("Type its HP"),
            "A creature made up with no HP says what it needs",
        );
        d.state("arrivals-made-up-needs-hp", Sizes::All);
        const ADD_GHOST: &str = "Add Ghost of the Mill to the initiative list";
        // FI4: `Count` and `Initiative` fields start at one x, and `Side` starts where `HP` does.
        let label_left = |d: &Driver<'_>, field: &str, word: &str| {
            let field = d.place_of(field)?;
            let area = egui::Rect::from_min_max(
                egui::pos2(field.left() - crate::design::XXL * 3.0, field.top()),
                egui::pos2(field.left(), field.bottom()),
            );
            d.painted_rect_in(word, area).map(|rect| rect.left())
        };
        let columns = (
            d.place_of("New creature count").map(|r| r.left()),
            d.place_of("New creature initiative total")
                .map(|r| r.left()),
            label_left(d, "New creature HP", "HP"),
            label_left(d, "Side of the new creature", "Side"),
        );
        d.observe(
            "iter-24-R-FI4",
            matches!(columns, (Some(count), Some(total), Some(hp), Some(side))
                if (count - total).abs() < 1.0 && (hp - side).abs() < 1.0),
            format!("Count and Initiative fields, HP and Side words at one x each: {columns:?}"),
        );
        // Blocked, the button's name says `this creature`.
        let add_before = d
            .place_of("Add this creature to the initiative list")
            .map(|r| r.top());
        d.type_into("New creature HP", "22");
        // FI2: the reason's line stays, so Add does not move when the DM types the HP.
        let add_after = d.place_of(ADD_GHOST).map(|r| r.top());
        d.observe(
            "iter-24-R-FI2",
            !d.has_containing("Type its HP") && add_before.is_some() && add_before == add_after,
            format!("The reason goes and Add stays: {add_before:?} then {add_after:?}"),
        );
        d.type_into("New creature AC", "14");
        d.type_into("New creature initiative total", "9");
        // iter-25-R SI2: one rule between Add and the groups below it.
        let rules = d.place_of(ADD_GHOST).map(|add| {
            d.horizontal_rules(
                add.left() + crate::design::S,
                add.bottom(),
                add.bottom() + 60.0,
            )
        });
        d.observe(
            "iter-25-R-SI2",
            rules.as_ref().is_some_and(|rules| rules.len() == 1),
            format!("One rule below Add, not two: {rules:?}"),
        );
        // iter-26-R FI8: in the narrow Arrivals column `Will add:` never ends a line: when it
        // does not fit beside Initiative with its first member, the summary has its own line.
        for size in [SMALL, DEFAULT] {
            let lines = d.at_size(size, |d| {
                d.painted_lines_in(egui::Rect::EVERYTHING)
                    .into_iter()
                    .map(|line| line.replace('\u{a0}', " "))
                    .filter(|line| line.contains("Will add:"))
                    .collect::<Vec<_>>()
            });
            d.observe(
                "iter-26-R-FI8",
                !lines.is_empty()
                    && lines
                        .iter()
                        .all(|line| !line.trim_end().ends_with("Will add:")),
                format!("At {size:?}, lines with Will add: {lines:?}"),
            );
        }
        d.state("arrivals-made-up-ready", Sizes::Default);
        d.click(ADD_GHOST);
        d.expect(
            "Ghost of the Mill joined at\u{a0}9. Tied with Ember Cultist at\u{a0}9, placed after.",
        );
        d.expect("Move Ghost of the Mill before Ember Cultist");
        d.observe(
            "iter-23-R-FH6",
            !d.has_containing("Added by hand: Ghost") && !d.has_containing("entered by the DM"),
            "A made-up creature's notice says it joined, as a group's does, and keeps the Log's words in the Log",
        );
        d.state("arrivals-made-up-added", Sizes::All);
        d.goal("add a creature made up at the table to a running fight");
        d.click_row("Ghost of the Mill");
        d.click("Stat block");
        d.expect("Made up in this fight. It has no stat block.");
        d.observe(
            "F1",
            d.visible_text("22 / 22") && d.visible_text("9") && !d.visible_text("printed +0"),
            "A made-up card shows typed HP and an unsigned initiative total",
        );
        // FK7: the trio keeps the stat block's columns when the DM selects one after the other.
        let trio = |d: &Driver<'_>| {
            ["ARMOR CLASS", "HIT POINTS", "INITIATIVE"].map(|word| d.painted_left(word))
        };
        let made_up = trio(d);
        d.state("made-up-creature-sheet", Sizes::Default);
        d.click_row("Ember Cultist");
        d.expect("Stat block");
        let stat_block = trio(d);
        d.observe(
            "iter-24-R-FK7",
            made_up
                .iter()
                .zip(&stat_block)
                .all(|(made_up, stat_block)| {
                    made_up
                        .zip(*stat_block)
                        .is_some_and(|(a, b)| (a - b).abs() <= 1.0)
                }),
            format!("The made-up trio {made_up:?} sits where the stat block's does {stat_block:?}"),
        );
        d.begin_task();
        d.click_row("Ghost of the Mill");
        d.type_text("5");
        d.press(Key::Enter);
        d.observe(
            "F1",
            d.visible_text("17 / 22"),
            "Damage keeps the made-up creature's maximum HP",
        );
        d.state("made-up-creature-after-damage", Sizes::Default);
        d.click_containing("Undo: 5 damage");
        d.begin_task();
        open_arrivals(d);
        d.expect(FIELD);
        d.observe(
            "T13.7",
            d.has_containing("Ghost of the Mill joined at\u{a0}9."),
            "A made-up creature joins the running fight, and the panel says where it went",
        );

        d.type_into(FIELD, "wolf");
        d.expect("Choose Wolf");
        check_form(d);
        // iter-26-R FJ2: the arrival is the newest step again (the damage was undone), so its
        // result and open Move choice stay while the DM types the next arrival.
        d.observe(
            "iter-26-R-FJ2-typing",
            d.in_view("Move Ghost of the Mill before Ember Cultist"),
            "A tie's Move choice stays in the footer while search words are typed",
        );
        // iter-26-R FJ1: the list keeps the scroll bar's lane, so a list long enough to scroll
        // moves no field.
        let long_list = search_right(d);
        d.observe(
            "iter-26-R-FJ1",
            short_list.iter().all(Option::is_some) && short_list == long_list,
            format!("Search right edge, short list {short_list:?}, long list {long_list:?}"),
        );
        d.state("arrivals-found", Sizes::Default);
        d.observe(
            "T13.7",
            d.has("Add Wolf to the initiative list"),
            "The button says which creature the words typed will add",
        );
        d.click("Choose Wolf");
        d.type_into("New creature count", "2");
        check_form(d);
        d.state("arrivals-found-chosen", Sizes::Default);
        d.click("Add Wolf to the initiative list");
        d.expect("Wolf 1 and Wolf 2 joined at\u{a0}");
        d.goal("add two of a campaign's creatures to a running fight");
        d.observe(
            "iter-23-R-FH2",
            d.has_containing("Undo: add 2 × Wolf") && !d.has_containing("Undo: add Wolf"),
            "Undo names both rows the step added, with the count its button said",
        );
        d.observe(
            "T13.7",
            d.has_containing("Wolf 1") && d.has_containing("Wolf 2"),
            "Creatures found in the campaign join with numbered names",
        );
        // iter-26-R FJ2: the result's owner is the newest step. Typed words do not end it.
        d.type_into(FIELD, "new arrival");
        d.observe(
            "iter-26-R-FJ2",
            d.in_view_containing("Wolf 1 and Wolf 2 joined at\u{a0}")
                && d.has_containing("Undo: add 2 × Wolf"),
            "The newest arrival's result stays while search words are typed",
        );
        d.clear_text_in(FIELD);
        d.expect("Wolf 1 and Wolf 2 joined at\u{a0}");
        // Revision 18: A (the wolves join), then an unrelated step B (damage to another row)
        // ends A's result; B really changed HP. Undo B gives A's result back with A's rows, and
        // B's damage is gone.
        d.click_row("Ember Cultist");
        let cultist_hp = d.row_hp("Ember Cultist");
        d.type_text("2");
        d.press(Key::Enter);
        let damaged = d.row_hp("Ember Cultist");
        if !d.has("This fight · Arrivals") {
            open_arrivals(d);
        }
        d.observe(
            "iter-26-R-FJ2-ends",
            cultist_hp
                .zip(damaged)
                .is_some_and(|(before, after)| after == before - 2)
                && !d.has_containing("Wolf 1 and Wolf 2 joined at")
                // Astra 1: what shows and what Undo reaches agree: Undo names B, not A.
                && d.has_containing("Undo: 2 damage")
                && !d.has_containing("Undo: add 2 × Wolf"),
            format!("A later step ends the arrival's result: HP {cultist_hp:?} → {damaged:?}"),
        );
        d.click_containing("Undo: 2 damage");
        if !d.has("This fight · Arrivals") {
            open_arrivals(d);
        }
        d.observe(
            "iter-26-R-FJ2-undo",
            d.row_hp("Ember Cultist") == cultist_hp
                && d.in_view_containing("Wolf 1 and Wolf 2 joined at\u{a0}")
                && d.row_node("Wolf 2").is_some(),
            "Undo of the later step gives the arrival's result back, with its rows",
        );
        d.begin_task();
        d.type_into(FIELD, "wolf");
        d.expect("Choose Wolf");
        // Show all lists every match in place: the words, the chosen match and the list stay.
        let choices = |d: &Driver<'_>| {
            d.nodes()
                .into_iter()
                .filter(|node| node.name.starts_with("Choose ") && node.in_view())
                .count()
        };
        let show_all = d
            .nodes()
            .into_iter()
            .find_map(|node| {
                node.name
                    .strip_prefix("Show all ")
                    .and_then(|count| count.parse::<usize>().ok())
            })
            .expect("Show all names how many creatures match");
        let before = choices(d);
        // FT2: Show all starts a line, so its words start at the result rows' words, and its
        // press area reaches into the gutter, never into the fight panel's splitter.
        let label = format!("Show all {show_all}");
        for size in [SMALL, DEFAULT, LARGE] {
            let (words, rows, target, splitter) = d.at_size(size, |d| {
                let row = d.place_of("Choose Wolf");
                let mut rows = None::<f32>;
                d.each_painted_text(|text, _| {
                    let rect = egui::Rect::from_min_size(text.pos, text.galley.size());
                    if row.is_some_and(|row| row.contains(rect.center())) {
                        rows = Some(rows.map_or(rect.left(), |left| left.min(rect.left())));
                    }
                });
                (
                    d.painted_left(&label),
                    rows,
                    d.place_of(&label),
                    d.place_of("Resize fight panel column"),
                )
            });
            d.observe(
                "iter-17-B-FT2",
                words
                    .zip(rows)
                    .is_some_and(|(words, rows)| (words - rows).abs() <= 1.0),
                format!(
                    "At {size:?}, {label} starts at {words:?}, the result rows' words at {rows:?}"
                ),
            );
            d.observe(
                "iter-17-B-FT2-splitter",
                target
                    .zip(splitter)
                    .is_some_and(|(target, splitter)| target.left() >= splitter.right()),
                format!(
                    "At {size:?}, {label}'s press area {target:?} starts right of the splitter {splitter:?}"
                ),
            );
        }
        d.click(&format!("Show all {show_all}"));
        d.observe(
            "iter-14-B-AT1",
            before == 5
                && show_all > before
                && choices(d) == show_all
                && !d.has_containing("Show all")
                && !d.has_containing("Keep typing")
                && d.in_view(FIELD)
                && d.in_view("Choose Wolf")
                && d.row_node("Wolf 2").is_some_and(|row| row.in_view()),
            format!(
                "Show all {show_all} lists every match ({before} before, {} after), with the \
                 words, the chosen match and the roster in view",
                choices(d)
            ),
        );
        // Must stay true: the button still adds the whole name typed.
        d.observe(
            "iter-14-B-AT1-add",
            d.has("Add Wolf to the initiative list"),
            "With every match listed, the button still adds the whole name typed",
        );
        d.state("arrivals-found-all", Sizes::All);
        // New words start a short list again, and the same words do not bring the long one back.
        d.replace_text_in(FIELD, "wol");
        d.replace_text_in(FIELD, "wolf");
        d.observe(
            "iter-14-B-AT1-reset",
            choices(d) == 5 && d.has(&format!("Show all {show_all}")),
            "Changed words close Show all",
        );
        // A match that only Show all lists can be chosen and added, with its own values.
        d.click(&format!("Show all {show_all}"));
        let listed = d.in_view("Choose Winter Wolf");
        if listed {
            d.click("Choose Winter Wolf");
        }
        let picked = listed
            && d.has("Choose another creature")
            && d.has_label("Winter Wolf")
            && d.nodes().iter().any(|node| {
                node.role == egui::accesskit::Role::Label
                    && node.name.contains("HP 75")
                    && node.name.contains("AC 13")
            })
            && d.has("Add Winter Wolf to the initiative list");
        d.observe(
            "iter-14-B-AT1-pick",
            picked,
            "Winter Wolf, listed only by Show all, can be chosen, and the form shows its HP 75 and AC 13",
        );
        if picked {
            d.click("Add Winter Wolf to the initiative list");
        }
        let row = d.nodes().into_iter().find(|node| {
            node.role == egui::accesskit::Role::Button
                && node.name.starts_with("Winter Wolf")
                && node.name.contains("HP 75/75")
                && node.name.contains("AC 13")
        });
        d.observe(
            "iter-14-B-AT1-added",
            picked && row.is_some(),
            format!(
                "Winter Wolf joins the roster with HP 75 and AC 13: {:?}",
                row.map(|row| row.name)
            ),
        );

        // A broad search: rows below the list's edge are reached by Tab, and the list follows.
        d.replace_text_in(FIELD, "dragon");
        let broad = d
            .nodes()
            .into_iter()
            .find_map(|node| {
                node.name
                    .strip_prefix("Show all ")
                    .and_then(|count| count.parse::<usize>().ok())
            })
            .expect("a broad search offers Show all");
        d.click(&format!("Show all {broad}"));
        let rows: Vec<_> = d
            .nodes()
            .into_iter()
            .filter(|node| node.name.starts_with("Choose "))
            .collect();
        let first = rows
            .first()
            .map(|row| row.name["Choose ".len()..].to_owned());
        let last = rows
            .last()
            .map(|row| row.name["Choose ".len()..].to_owned());
        let ninth = rows.get(8).map(|row| (row.name.clone(), row.rect));
        // The list's own visible area: the clip its first row's words are painted in.
        let mut list_clip = None;
        d.each_painted_text(|text, clip| {
            if first.as_deref() == Some(text.galley.job.text.as_str()) {
                list_clip = Some(clip);
            }
        });
        let inside =
            |rect: egui::Rect| list_clip.is_some_and(|clip| clip.expand(0.5).contains_rect(rect));
        // Rows out of view keep their control and name, but skip their words until shown.
        d.observe(
            "iter-14-B-AT1-visible-only",
            broad > 9
                && rows.len() == broad
                && rows.last().is_some_and(|row| !inside(row.rect))
                && first.as_deref().is_some_and(|name| d.has_label(name))
                && last.as_deref().is_some_and(|name| !d.has_label(name)),
            format!(
                "Show all {broad}: {} rows named; the first row's words are drawn ({first:?}), the last \
                 row's words are not drawn while it is out of view ({last:?})",
                rows.len()
            ),
        );
        if let Some((ninth, before)) = ninth {
            // Focus starts in the search field, above the list.
            d.type_into(FIELD, "");
            let mut tabs = 0;
            while d.focused_name().as_deref() != Some(ninth.as_str()) && tabs < 40 {
                d.press(Key::Tab);
                tabs += 1;
            }
            let after = d.place_of(&ninth);
            d.observe(
                "iter-14-B-AT1-tab",
                !inside(before)
                    && d.focused_name().as_deref() == Some(ninth.as_str())
                    && after.is_some_and(|after| after.top() < before.top() - 1.0 && inside(after)),
                format!(
                    "Tab reaches {ninth} after {tabs} presses, and the list scrolls it inside its clip \
                     {list_clip:?}: before {before:?}, after {after:?}"
                ),
            );
        } else {
            d.observe(
                "iter-14-B-AT1-tab",
                false,
                format!("Show all {broad} lists no ninth row to reach with Tab"),
            );
        }
        d.clear_text_in(FIELD);
        d.begin_task();
    }
}

/// W13 and W14: a group assembled in the session's plan is first in the run's panel.
pub mod session_plan_groups {
    use super::*;
    use crate::test_support::{CachedWork, Counts};

    pub fn run(d: &mut Driver<'_>) {
        // The Library has been read before the group exists, so it must read again after.
        go_library(d);
        d.expect(SEARCH_LIBRARY);
        d.click(NAV_SESSIONS);
        new_session(d, "The Crossing");
        show_plan(d);
        d.click("Add a group");
        d.click("New group");
        d.expect("Back to the plan");
        d.type_text("Ash Pack");
        d.state_prepared("plan-group-empty", Sizes::All, |d| {
            let layout = if d.has("Plan and notes") {
                d.place_of("Note text")
                    .zip(d.place_of("Save group"))
                    .is_some_and(|(note, save)| {
                        note.left() > save.right()
                            && note.width() >= crate::design::READER_MEASURE - crate::design::L
                    })
            } else {
                d.has("Note") && !d.has("Note text")
            };
            d.observe(
                "iter-4-C-AT2",
                layout,
                "A wide group editor keeps the note beside it; smaller pages keep the Note tab",
            );
        });
        d.observe(
            "T14.6",
            d.has_containing("Session 1: The Crossing"),
            "Assembling a group stays on the session's page",
        );
        // AK1: the empty group's reason has a reserved place, so adding the first creature moves
        // no field. AH1: Save refuses an empty group, so it says Invalid.
        let sizes = [SMALL, DEFAULT, LARGE];
        let empty =
            sizes.map(|size| d.at_size(size, |d| d.place_of(FIND_CREATURE).map(|r| r.top())));
        d.observe(
            "iter-25-B-AH1",
            d.visible_text("Invalid")
                && !d.visible_text("Unsaved")
                && d.has("Add a creature first")
                && d.visible_text(crate::encounters::blocked_draft_switch_reason())
                && !d.visible_text(crate::encounters::draft_switch_reason()),
            "An empty new group says Invalid beside its blocked Save, and Discard is the way to switch (FB6)",
        );
        d.type_into(FIND_CREATURE, "wolf");
        d.click_beside_exact("Wolf", ADD);
        d.click_beside_exact("Wolf", ADD);
        let assembled =
            sizes.map(|size| d.at_size(size, |d| d.place_of(FIND_CREATURE).map(|r| r.top())));
        d.observe(
            "iter-25-B-AK1",
            !d.has("Add a creature first")
                && empty.iter().all(Option::is_some)
                && empty == assembled
                && d.visible_text("Unsaved")
                && d.visible_text(crate::encounters::draft_switch_reason()),
            format!("Find a creature top, empty {empty:?}, with members {assembled:?}"),
        );
        d.state("plan-group-assembled", Sizes::All);
        for size in [DEFAULT, LARGE] {
            let edges = d.at_size(size, |d| group_field_edges(d, "Ash Pack"));
            d.observe(
                "iter-19-B-FT9",
                group_fields_end_at_column(edges),
                format!(
                    "At {size:?}, the plan group's name and Belongs to end at its notes' right edge: {edges:?}"
                ),
            );
        }
        // Must stay true: when the compact editor shows only Add creatures, the name keeps no
        // column edge from the wider layout; it runs on to the edge Save group ends at, on the
        // save row below it (iter-24-B AK1). That edge is the compact pane's own: the search
        // field's right edge, which no stored column edge sets (iter-24-B review).
        let gap = d.at_size(SMALL, |d| {
            let compact = d.has("Back to the plan") && !d.has("Group notes for Ash Pack");
            let gap = d
                .place_of("Group name: Ash Pack")
                .zip(d.place_of("Save group"))
                .zip(d.place_of(FIND_CREATURE))
                .map(|((name, save), search)| {
                    (
                        save.right() - name.right(),
                        save.top() - name.bottom(),
                        save.right() - search.right(),
                    )
                });
            (compact, gap)
        });
        d.observe(
            "iter-19-B-FT9-compact",
            gap.0
                && gap.1.is_some_and(|(edge, below, pane)| {
                    (0.0..=crate::design::XL).contains(&edge)
                        && below >= -0.5
                        && pane.abs() <= 1.0
                }),
            format!(
                "The compact name field ends near Save group's edge, above it, and Save ends at the pane's edge (Save minus name, Save top minus name bottom, Save minus search): {gap:?}"
            ),
        );
        let summary_visible = d.at_size(SMALL, |d| d.visible_text("2 × Wolf"));
        d.observe(
            "iter-18-B-PX3",
            summary_visible,
            "The compact Add creatures pane keeps a member summary",
        );
        d.click("Save group");
        d.expect("Edit group Ash Pack");
        d.state("plan-group-saved", Sizes::All);
        d.goal("assemble a group in the session's plan");
        d.observe(
            "T14.6",
            d.has_containing("2 × Wolf") && !d.has("Back to the plan"),
            "A group saved from the plan is linked in it, with what it holds",
        );
        go_library(d);
        d.type_into(SEARCH_LIBRARY, "Ash Pack");
        d.expect("Ash Pack");
        d.observe(
            "T14.6",
            d.in_view_containing("Ash Pack"),
            "A group saved from the plan is in the Library at once",
        );
        d.click(NAV_SESSIONS);
        show_plan(d);

        d.click("Add a group");
        d.click(&format!("Plan {GROUP_RIVALS}"));
        d.expect(&format!("Edit group {GROUP_RIVALS}"));
        d.state_prepared("plan-with-groups", Sizes::All, |d| {
            show_plan(d);
            let toolbar = d
                .place_of("Add a group")
                .zip(d.place_of("Edit group Ash Pack"))
                .is_some_and(|(add, row)| add.bottom() < row.top());
            d.observe("iter-3-C-ST2-FT2", toolbar, "Add stays above the plan rows");
        });
        d.state_prepared("plan-groups-read", Sizes::All, |d| {
            read_note(d);
            if d.has("Plan and notes") {
                let writes = d.nodes().iter().filter(|node| node.name == "Write").count();
                d.observe("iter-3-C-ST3", writes == 1, "The empty note has one Write action");
            }
            let compact = if d.has("Plan and notes") {
                !d.visible_text("Plan for tonight") && d.has("Edit group Ash Pack") && d.has(&format!("Edit group {GROUP_RIVALS}"))
            } else {
                d.visible_text("2 groups, in the plan")
            };
            d.observe("iter-2-C-ST3", compact, "Read compacts group links already shown in the working plan, and keeps the tab count");
            // Round 13 FT3: the Rival Team summary wraps between members, never inside
            // `1 × Rival Captain (as Ember Cultist)`.
            let rows: Vec<String> = d
                .painted_lines_in(egui::Rect::EVERYTHING)
                .into_iter()
                .map(|row| row.replace('\u{a0}', " "))
                .collect();
            // At 960 px, Read compacts the plan's group links, so there is no summary to read.
            // From 1200 px the summary must show.
            if d.window_size().0 >= 1200.0 || d.in_view_containing("Rival Captain (as Ember Cultist)") {
                let captain: Vec<_> = rows.iter().filter(|row| row.contains("Captain")).collect();
                let whole = !captain.is_empty()
                    && captain.iter().all(|row| row.contains("× Rival Captain (as Ember Cultist)"))
                    && !rows.iter().any(|row| row.trim_end().ends_with("× Rival"))
                    && rows.iter().any(|row| row.contains("2 × Cinder Wolf"));
                d.observe("iter-13-C-FT3", whole, format!("The plan summary keeps each member whole, and every member shows: {captain:?}"));
            }
        });
        show_plan(d);
        d.begin_task();
        d.click(&format!("More actions for {GROUP_RIVALS}"));
        d.click(&format!("Remove group {GROUP_RIVALS} from the plan"));
        d.expect("Undo remove");
        d.state("group-removed-from-plan", Sizes::Default);
        d.observe(
            "P1",
            d.in_view("Undo remove")
                && d.visible_text(&format!("Removed “{GROUP_RIVALS}” from tonight.")),
            "Remove group keeps its message and Undo in view after capture resize",
        );
        d.observe(
            "iter-25-S-success-group-removed",
            d.place_of(&format!("Removed “{GROUP_RIVALS}” from tonight."))
                .is_some_and(|words| d.has_tone_bar(words, crate::design::colors().result.success)),
            "Removing a group from the plan uses the tone a removed fight uses",
        );
        d.observe(
            "P17",
            !d.has(&format!("Edit group {GROUP_RIVALS}")),
            "The row menu removes a group from this plan",
        );
        d.click("Undo remove");
        d.expect(&format!("Edit group {GROUP_RIVALS}"));
        d.begin_task();

        d.click(&format!("Edit group {GROUP_RIVALS}"));
        d.expect("Back to the plan");
        d.state("plan-group-edit", Sizes::Default);
        // FK1: the first edit adds the campaign-switch reason in its reserved place, so no field
        // moves. A proof aside: Discard puts the clean group back.
        d.aside(|d| {
            let sizes = [SMALL, DEFAULT, LARGE];
            // The first member's menu: in the members column, or on the Members tab at 960.
            let member = |d: &Driver<'_>| {
                d.nodes()
                    .into_iter()
                    .find(|node| node.name.starts_with("More actions for ") && node.in_view())
                    .map(|node| node.rect.top())
            };
            let clean = sizes.map(|size| d.at_size(size, member));
            d.type_into(&format!("Group name: {GROUP_RIVALS}"), " 2");
            let changed = sizes.map(|size| d.at_size(size, member));
            d.observe(
                "iter-25-B-FK1",
                d.has_containing("Save or discard to switch campaign.")
                    && clean.iter().all(Option::is_some)
                    && clean == changed,
                format!("First member menu top, clean {clean:?}, changed {changed:?}"),
            );
            d.click("Discard");
            d.expect(&format!("Group name: {GROUP_RIVALS}"));
        });
        d.click("Back to the plan");
        d.expect(&format!("Edit group {GROUP_RIVALS}"));

        d.click(NAV_ENCOUNTERS);
        resume_run(d, RUN);
        open_arrivals(d);
        d.expect("IN TONIGHT’S PLAN");
        d.state("arrivals-tonight-first", Sizes::All);
        d.observe(
            "T14.6",
            d.has_containing("FOR ANY ENCOUNTER"),
            "The groups in tonight's plan come first in the run's panel",
        );

        // From the plan, with the fight open: one press to where the group can arrive.
        d.click("Stat block");
        d.click(NAV_SESSIONS);
        show_plan(d);
        let route = format!("Add {GROUP_RIVALS} to fight");
        d.expect(&route);
        d.observe_both(
            "iter-13-C-FT2",
            "With a fight open, a plan group's door uses the run's words and plus icon",
            |d| {
                d.in_view(&route)
                    && d.place_of(&route).is_some_and(|rect| {
                        let words = d.painted_in(rect);
                        words.contains("Add to fight…")
                            && words.contains(crate::design::icons::ADD)
                            && !words.contains("Arrivals")
                    })
            },
        );
        d.state("plan-groups-fight-open", Sizes::All);
        d.click(&route);
        d.expect("IN TONIGHT’S PLAN");
        d.observe(
            "T14.6",
            d.has(&format!("Add {GROUP_RIVALS} to the initiative list")),
            "With a fight open, a planned group's row goes to the Arrivals panel in one press",
        );

        // A group belongs to the plan it began in, whichever note is on screen when it is saved.
        d.click(NAV_SESSIONS);
        d.click("New session");
        d.type_into("Session title", "The Ford");
        d.click("Create session");
        d.expect("Session 2: The Ford");
        show_plan(d);
        d.click("Add a group");
        d.click("New group");
        d.expect("Back to the plan");
        d.type_text("Border Watch");
        d.type_into(FIND_CREATURE, "wolf");
        d.click_beside_exact("Wolf", ADD);
        d.click_starting("Session 1: The Crossing ·");
        d.expect("A group is being assembled in Session 2.");
        d.expect("Return to the session being played");
        d.observe(
            "iter-5-P8",
            !d.has("Go to it"),
            "One warning names the draft session and one link opens it",
        );
        d.state("plan-group-in-another-session", Sizes::Default);
        // iter-24-B FJ5: the plan's group summary draws the pencil before Changed, as Arrivals.
        let pencil = format!("{}\u{a0}Changed", crate::design::icons::CHANGED);
        d.observe(
            "iter-24-B-FJ5",
            d.visible_text(&pencil),
            "A changed group's plan row has the pencil before Changed",
        );
        let stays_out = !d.has("Save group");
        d.click("Return to the session being played");
        d.click("Save group");
        d.expect("Add Border Watch to fight");
        d.click_starting("Session 1: The Crossing ·");
        d.expect(&format!("Add {GROUP_RIVALS} to fight"));
        d.observe(
            "T14.6",
            stays_out && !d.has("Add Border Watch to fight"),
            "A group is linked in the plan it began in, with another note open on the way",
        );

        d.click(NAV_CAMPAIGN);
        d.expect("1 content issue to repair");
        d.click("1 content issue to repair");
        d.expect("Lost Patrol needs a creature that cannot be found.");
        let missing_id = splat::character::stable_id("visual:group:lost:creature").to_string();
        d.observe_both(
            "iter-2-C-missing-member",
            "The missing member names its owner and keeps its reference folded",
            |d| {
                d.visible_text("Lost Patrol needs a creature that cannot be found.")
                    && d.has("Repair Lost Patrol")
                    && !d.visible_text(&missing_id)
            },
        );
        // FA6: the dependency issue is a warning message, and its repair starts at its words,
        // as the Campaign.md flag's choice does. Round 24 FA2: the repair is a bordered
        // secondary button like that choice, not a link, so its border starts at the words.
        for size in [SMALL, DEFAULT] {
            let (tones, words, link, bordered) = d.at_size(size, |d| {
                let words = d
                    .painted_rect_in(
                        "Lost Patrol needs a creature that cannot be found.",
                        egui::Rect::EVERYTHING,
                    )
                    .map(|rect| rect.left());
                let link = d.place_of("Repair Lost Patrol").map(|rect| rect.left());
                let bordered = d
                    .nodes()
                    .iter()
                    .any(|node| node.name == "Repair Lost Patrol" && !node.is_link())
                    && d.control_frame("Repair Lost Patrol").is_some_and(|frame| {
                        frame.stroke.width > 0.0
                            && frame.stroke.color == crate::design::colors().field_border
                    });
                (
                    tones_beside(d, "Lost Patrol needs a creature that cannot be found."),
                    words,
                    link,
                    bordered,
                )
            });
            d.observe(
                "iter-23-Q-FA6",
                tones == ["warning"]
                    && words
                        .zip(link)
                        .is_some_and(|(words, link)| (words - link).abs() <= 1.0),
                format!("At {size:?}: tones {tones:?}, words at {words:?}, repair at {link:?}"),
            );
            d.observe(
                "iter-24-C-FA2",
                bordered,
                format!(
                    "At {size:?}, Repair Lost Patrol is a bordered secondary button: {bordered}"
                ),
            );
        }
        d.state("missing-group-member", Sizes::All);
        d.click("Technical details");
        d.expect(&missing_id);
        d.state("missing-member-technical-details", Sizes::All);
        // iter-24-B FJ6: the bare id has a label that says what it is.
        d.observe(
            "iter-24-B-FJ6",
            d.visible_text(&format!("Missing creature id: {missing_id}")),
            "Technical details names the missing creature's id",
        );
        // Lookalikes before the member row: the notes mention the missing id in words, and an
        // example table links it in another section.
        let lost = d
            .app_mut()
            .campaigns
            .active_campaign()
            .unwrap()
            .path
            .join("Encounter Groups/Lost Patrol.md");
        let lost_text = std::fs::read_to_string(&lost).unwrap();
        let with_note = lost_text.replacen(
            "\n## Members",
            &format!(
                "\nScouts last saw {missing_id} at the ford. They report to [[Rival Team]].\n\n## Example\n\n| Count | Creature | Name | Changes | Note |\n| --- | --- | --- | --- | --- |\n| 1 | [[{missing_id}]] |  |  |  |\n\n## Members"
            ),
            1,
        );
        assert_ne!(with_note, lost_text);
        std::fs::write(&lost, &with_note).unwrap();
        // The row lookup is counted inside `repair_row`; frames stand in for its guard.
        let repair_work = CachedWork::start("Repair row lookup", d, |d: &mut Driver<'_>| Counts {
            reached: d.passes(),
            work: d.app().library.repair_lookups(),
        });
        // Round 25 FH6: the broken-link scan is counted inside `broken_link_lines`.
        let broken_work = CachedWork::start("Broken link lines", d, |d: &mut Driver<'_>| Counts {
            reached: d.passes(),
            work: d.app().library.broken_line_scans(),
        });
        d.click("Repair Lost Patrol");
        d.expect("Markdown editor");
        d.observe(
            "iter-2-C-member-repair",
            d.editor_text()
                .is_some_and(|text| text.contains(&missing_id)),
            "Repair opens the referring member's file without changing it",
        );
        // Round 24 FH5: both links name the one missing creature, so the row has one mark and
        // one cause.
        let rows: Vec<String> = d
            .nodes()
            .into_iter()
            .filter(|node| node.name.starts_with("Lost Patrol · "))
            .map(|node| node.name)
            .collect();
        d.observe(
            "iter-24-L-FH5",
            !rows.is_empty()
                && rows.iter().all(|row| {
                    row.contains(&format!("{} 2 broken links", crate::icons::WARN))
                        && row.matches(crate::icons::WARN).count() == 1
                        && !row.contains("Needs fixing")
                }),
            format!("The Lost Patrol row says its broken links once: {rows:?}"),
        );
        d.state("missing-member-repair", Sizes::All);
        // Lines that name the id: the notes' words, the example row, and the member row, which
        // is the one after `## Members`.
        let lines = |d: &Driver<'_>| -> Vec<(usize, bool)> {
            let text = d.editor_text().unwrap_or_default();
            let members = text.lines().position(|line| line.trim() == "## Members");
            text.lines()
                .enumerate()
                .filter(|(_, line)| line.contains(&missing_id))
                .map(|(at, line)| {
                    (
                        at + 1,
                        line.trim_start().starts_with('|') && members.is_some_and(|m| at > m),
                    )
                })
                .collect()
        };
        let named_ids = lines(d);
        let row = named_ids
            .iter()
            .find(|(_, member)| *member)
            .map(|(at, _)| *at);
        let named = row
            .map(|line| format!("Line {line}: Lost Patrol needs a creature that cannot be found."));
        let caret = d.editor_caret_line();
        // Astra review of round 25: a message kept in the one message space is drawn and reached
        // by keyboard and by pointer at 960, 1200 and 1920, not only in the space's name.
        let reaches = |d: &mut Driver<'_>, words: Option<&str>| {
            words.is_some_and(|words| {
                super::world_build::space_reaches(d, words, None)
                    .iter()
                    .all(|(keyboard, pointer)| *keyboard && *pointer)
            })
        };
        // Round 25 FH6: the row counts two broken links; the editor names both lines. The
        // member row has the repair's words, and the example row its own warning.
        let example = named_ids
            .iter()
            .filter(|(_, member)| !*member)
            .map(|(at, _)| *at)
            .next_back();
        let example_words = example.map(|line| format!("Line {line}: this link names no entry."));
        let each_named = |d: &Driver<'_>| {
            example_words
                .as_deref()
                .is_some_and(|words| d.has_containing(words))
                && named
                    .as_deref()
                    .is_some_and(|named| d.has_containing(named))
        };
        d.observe(
            "iter-25-L-FH6",
            each_named(d)
                && row.is_some_and(|row| {
                    !d.has_containing(&format!("Line {row}: this link names no entry."))
                }),
            format!("Both broken links are named, each once: example {example_words:?}, member {named:?}"),
        );
        // The example's warning is drawn, not only named: the wheel over the message space
        // brings it into view (the repair's guidance comes first and fills the two lines).
        let space = d
            .nodes()
            .into_iter()
            .find(|node| {
                node.role == egui::accesskit::Role::GenericContainer
                    && example_words
                        .as_deref()
                        .is_some_and(|words| node.name.contains(words))
            })
            .map(|node| node.rect);
        let drawn = space.is_some_and(|space| {
            d.scroll_at(space.center(), 1);
            let drawn = example_words
                .as_deref()
                .is_some_and(|words| d.visible_text(words));
            d.scroll_up_at(space.center(), 1);
            drawn
        });
        d.observe(
            "iter-25-L-FH6-drawn",
            drawn,
            format!("The example row's warning is drawn in the message space {space:?}"),
        );
        let broken_warm = broken_work.warm(d, each_named(d), 1);
        let broken_steady = broken_work.steady(d, |d| {
            d.settle();
            d.at_size(LARGE, |_| ());
            each_named(d)
        });
        d.observe(
            "iter-25-L-FH6-cached",
            broken_warm.is_ok() && broken_steady.is_ok(),
            format!("Scans once, then not on unchanged frames: {broken_warm:?}, {broken_steady:?}"),
        );
        d.observe(
            "iter-23-B-FK1",
            named_ids.len() == 3
                && named_ids[..2].iter().all(|(_, member)| !member)
                && named.as_deref().is_some_and(|named| d.visible_text(named))
                && caret == row,
            format!(
                "The repair names the member row {named:?}; caret {caret:?}; lines naming the id {named_ids:?}"
            ),
        );
        // The guidance is in the message footer directly below the editor: no empty slot between
        // them, at 960 and 1200. Round 26: the editor's frame margin, the footer's `S` gap and
        // its inset come to about 20 px, so the bound is `XXL`; an empty reserved part (about
        // 44 px) fails it.
        const LINK_WORDS: &str = "needs a creature that cannot be found. Replace the link";
        let mut gaps = Vec::new();
        for size in [SMALL, DEFAULT] {
            gaps.push(d.at_size(size, |d| {
                // The editor's frame: its accessible rect follows its scrolled text.
                let editor = d.input_frame("Markdown editor")?.0;
                let text = d.painted_rect_in(LINK_WORDS, egui::Rect::EVERYTHING)?;
                Some(text.top() - editor.bottom())
            }));
        }
        let slot = crate::design::XXL;
        d.observe(
            "iter-23-B-FK1-slot",
            gaps.iter()
                .all(|gap| gap.is_some_and(|gap| (0.0..=slot).contains(&gap))),
            format!("Editor to guidance in the footer at 960 and 1200: {gaps:?}"),
        );
        // This repair looked the row up, and unchanged frames and a resize do not look it up
        // again.
        let shown = |d: &Driver<'_>| named.as_deref().is_some_and(|named| d.visible_text(named));
        let warm = repair_work.warm(d, shown(d), 1);
        d.observe(
            "iter-24-T-repair-warm",
            warm.is_ok(),
            format!("Repair lookups since the repair opened: {warm:?}"),
        );
        let steady = repair_work.steady(d, |d| {
            d.settle();
            d.at_size(LARGE, |_| ());
            shown(d)
        });
        d.observe(
            "iter-23-B-FK1-cached",
            steady.is_ok(),
            format!("Unchanged frames and a resize: {steady:?}"),
        );
        // Repair A stays through an unrelated edit B and a Save that fails on the disk: the
        // guidance, the draft and the file stay as they were. Edit B is new text, so the row is
        // looked up again, and the same row is named.
        let refresh = repair_work.refresh(d, 1, |d| {
            d.press_ctrl(Key::End);
            d.type_text("Arrive at dusk.");
            shown(d)
                && d.editor_text()
                    .is_some_and(|text| text.contains("Arrive at dusk."))
        });
        d.observe(
            "iter-24-T-repair-refresh",
            refresh.is_ok(),
            format!("An edit looks the row up again: {refresh:?}"),
        );
        let original = std::fs::metadata(&lost).unwrap().permissions();
        let mut read_only = original.clone();
        read_only.set_readonly(true);
        std::fs::set_permissions(&lost, read_only).unwrap();
        d.press_ctrl(Key::S);
        d.state("missing-member-save-failed", Sizes::Default);
        // iter-24-B FL1, AL1: Save found the file read only, so the owner line says so, and
        // Save's reason says what to do next.
        let read_only_shown = (
            d.visible_text("Campaign · Read only"),
            d.visible_text("Campaign · Editable"),
            d.visible_text("This file is read only. Make it writable, then Save again."),
        );
        // The editor's visible part: its accessible rect is its scrolled text.
        let editor_shown = |d: &Driver<'_>| {
            d.nodes()
                .into_iter()
                .find(|node| node.name == "Markdown editor")
                .map(|node| node.shown)
        };
        // Round 25 (one message space): Save's reason comes first, then the guidance. Round 26:
        // both sit in the pane's message footer, below the editor.
        let mut reason = None;
        d.each_painted_text(|text, clip| {
            let rect = egui::Rect::from_min_size(text.pos, text.galley.size());
            if text.galley.job.text.contains("This file is read only.")
                && clip.intersects(rect)
                && reason.is_none_or(|found: egui::Rect| rect.top() < found.top())
            {
                reason = Some(rect);
            }
        });
        let guidance = d
            .painted_rect_in(LINK_WORDS, egui::Rect::EVERYTHING)
            .filter(|rect| reason.is_some_and(|reason| rect.top() < reason.top()));
        d.observe(
            "iter-25-L-FK1-reason-first",
            guidance.is_none()
                && reason
                    .zip(editor_shown(d))
                    .is_some_and(|(reason, editor)| reason.top() >= editor.bottom()),
            format!(
                "Save's reason ({reason:?}) comes first, in the footer below the editor ({:?}); guidance above it: {guidance:?}",
                editor_shown(d)
            ),
        );
        let failed = (
            // The guidance stays in the message space, after the reason: drawn and reachable.
            reaches(d, named.as_deref()),
            d.editor_text()
                .is_some_and(|text| text.contains("Arrive at dusk.")),
            std::fs::read_to_string(&lost).is_ok_and(|text| text == with_note),
        );
        std::fs::set_permissions(&lost, original).unwrap();
        d.observe(
            "iter-23-B-FK1-save-failed",
            failed == (true, true, true),
            format!("After a failed Save (guidance kept, B kept, file unchanged): {failed:?}"),
        );
        // Replace only the member link: the notice ends though the notes and the example still
        // name the id.
        d.press_ctrl(Key::Home);
        for _ in 0..60 {
            if d.editor_caret_line() >= row {
                break;
            }
            d.press(Key::ArrowDown);
        }
        d.press(Key::Home);
        d.press_shift(Key::End);
        d.type_text("| 1 | [[Goblin Warrior]] |  |  |  |");
        // The next edit ends the failed Save, and the owner line with it.
        let editable_again = (
            d.visible_text("Campaign · Read only"),
            d.visible_text("Campaign · Editable"),
            d.visible_text("This file is read only."),
        );
        d.observe(
            "iter-24-B-FL1",
            read_only_shown == (true, false, true) && editable_again == (false, true, false),
            format!(
                "After the failed Save (Read only, Editable, next step): {read_only_shown:?}; after the next edit: {editable_again:?}"
            ),
        );
        let replaced = lines(d);
        let gone = replaced.len() == 2
            && replaced.iter().all(|(_, member)| !member)
            && !d.has_containing("needs a creature that cannot be found. Replace the link");
        d.click("Discard");
        d.expect("Markdown editor");
        // Round 25 lead ruling: Discard's result carries a live Undo, so it leads the one
        // message space and the guidance follows it there (reachable by scroll at 960). Presence
        // in the space is the owner's check; `visible_text` would test the order instead.
        let back = reaches(d, named.as_deref());
        d.observe(
            "iter-23-B-FK1-lifetime",
            gone && back,
            format!(
                "The notice goes with the member link, not the notes' or example's mention, and comes back with Discard: {replaced:?}"
            ),
        );
        // The guidance belongs to Lost Patrol while it stays open. Opening another entry ends it,
        // by a link and Back, and by the list: coming back to Lost Patrol does not bring it back.
        const GUIDANCE: &str = "needs a creature that cannot be found. Replace the link";
        let open_with_editor = |d: &mut Driver<'_>| {
            if !d.has("Markdown editor") {
                d.click("Edit Markdown");
            }
            d.expect("Markdown editor");
            (
                d.editor_text()
                    .is_some_and(|text| text.contains(&missing_id)),
                d.has_containing(GUIDANCE),
            )
        };
        let shown_before_back = reaches(d, Some(GUIDANCE));
        // A link opens another entry and records Lost Patrol on the trail; Back returns to it.
        d.click("Read");
        d.click("Rival Team (opens Rival Team)");
        d.wait_for_text("Rival Captain");
        d.click("Back");
        let after_back = open_with_editor(d);
        d.click(NAV_CAMPAIGN);
        d.expect("Repair Lost Patrol");
        d.click("Repair Lost Patrol");
        d.expect("Markdown editor");
        let shown_before_list = reaches(d, Some(GUIDANCE));
        d.click_starting("Rival Team ·");
        d.click_starting("Lost Patrol ·");
        let after_list = open_with_editor(d);
        d.observe(
            "iter-23-B-FK1-owner",
            shown_before_back
                && after_back == (true, false)
                && shown_before_list
                && after_list == (true, false),
            format!(
                "Guidance before Back {shown_before_back}, Lost Patrol again after Back (open, guidance) {after_back:?}; before the list {shown_before_list}, after the list {after_list:?}"
            ),
        );
        // A repair whose file is gone opens nothing; its guidance must not wait for the file.
        let away = lost.with_extension("md.away");
        std::fs::rename(&lost, &away).unwrap();
        d.click(NAV_CAMPAIGN);
        d.expect("Repair Lost Patrol");
        d.click("Repair Lost Patrol");
        d.settle();
        d.click_starting("Rival Team ·");
        std::fs::rename(&away, &lost).unwrap();
        d.click("Packages");
        d.click("Reload from disk");
        d.settle();
        d.click_starting("Lost Patrol ·");
        let after_missing = open_with_editor(d);
        d.observe(
            "iter-23-B-FK1-owner-missing",
            after_missing == (true, false),
            format!("After a repair on a missing file, Lost Patrol opened normally (open, guidance): {after_missing:?}"),
        );
        std::fs::write(&lost, &lost_text).unwrap();

        // Astra review of PR 231: a plan group none of whose members can join keeps each member
        // fault's own tone, then says so as a warning. In another group, a member named like
        // that fault stays plain words.
        let campaign = d.app_mut().campaigns.active_campaign().unwrap();
        let wolf = splat::character::stable_id("visual:module:cinder_wolf");
        for (name, member) in [
            (
                "Ash Remnant",
                format!("  - creature: {{id: {wolf}}}\n    overrides: {{max_hp: {{add: -50}}}}\n"),
            ),
            (
                "Mill Watch",
                "  - creature: {key: 'dnd2024:creatures/goblin_warrior'}\n    overrides: {name: 'Maximum HP would become −28'}\n".to_owned(),
            ),
            // iter-23 FH1: some members can join and one cannot.
            (
                "Split Watch",
                format!("  - creature: {{id: {wolf}}}\n  - creature: {{key: 'dnd2024:creatures/goblin_warrior'}}\n    overrides: {{armor_class: {{add: -50}}}}\n"),
            ),
        ] {
            let relative = format!("Encounter Groups/{name}.md");
            let text = splat::content::formats::fixture(
                &relative,
                &format!(
                    "splat_id: {}\nsplat_kind: encounter_group\nsplat_name: {name}\nsplat_encounter_group:\n  side: enemies\n  members:\n{member}",
                    splat::character::stable_id(&format!("visual:group:{name}"))
                ),
                "",
            )
            .unwrap();
            crate::library::write_note(&campaign.path, &relative, None, &text).unwrap();
        }
        go_encounters(d);
        d.click("Reload from disk");
        d.click(NAV_SESSIONS);
        d.click_starting("Session 1: The Crossing ·");
        show_plan(d);
        for name in ["Ash Remnant", "Mill Watch", "Split Watch"] {
            d.click("Add a group");
            d.click(&format!("Plan {name}"));
        }
        const FAULT: &str =
            "Cinder Wolf · line\u{a0}13: Maximum HP would become −28. It must be 1 or more.";
        d.expect(FAULT);
        d.state_prepared("plan-group-no-member-can-join", Sizes::All, |d| {
            show_plan(d);
            // At 960 the plan scrolls to show both new groups.
            if !d.in_view("Edit group Mill Watch") && !d.in_view_containing("Mill Watch to fight") {
                d.scroll_until(pos2(650.0, 500.0), "Mill Watch", 10);
                d.scroll_at(pos2(650.0, 500.0), 1);
            }
            let size = d.window_size();
            let tones = (
                tones_where(d, |text| text == FAULT),
                // FK5: `0 of 1 can join` and the member's fault say it; no third line does.
                tones_where(d, |text| text.starts_with("No members can join")),
                // The summary joins a member's words with no-break spaces.
                tones_where(d, |text| {
                    text.replace('\u{a0}', " ")
                        .contains("Maximum HP would become −28")
                        && text != FAULT
                }),
            );
            d.observe(
                "iter-22-B-FT1-plan",
                tones.0 == ["error"]
                    && tones.1.is_empty()
                    && d.has_containing("0 of 1 can join")
                    && !tones.2.is_empty()
                    && tones.2.iter().all(|tone| *tone == "none"),
                format!(
                    "At {size:?}: invalid maximum HP {:?}, group reason {:?}, lookalike name {:?}",
                    tones.0, tones.1, tones.2
                ),
            );
            // A group none of whose members can join offers its repair, not Add to fight; a
            // group that can join keeps Add to fight.
            d.observe(
                "iter-23-B-FH1",
                d.has("Edit group Ash Remnant")
                    && !d.has("Add Ash Remnant to fight")
                    && d.has("Add Mill Watch to fight"),
                format!("At {size:?}: Ash Remnant offers Edit group, Mill Watch Add to fight"),
            );
            // Nobody can join, so the summary ends at the count.
            let summary = d
                .nodes()
                .into_iter()
                .find(|node| node.name.contains("0 of 1 can join"))
                .map(|node| node.name);
            d.observe(
                "iter-23-B-FH3",
                summary
                    .as_deref()
                    .is_some_and(|name| !name.contains("XP") && !name.contains("Will add")),
                format!("At {size:?}: Ash Remnant's summary {summary:?}"),
            );
            // A member wider than the plan column breaks at its own spaces, never inside a word
            // or after an opening bracket.
            let mut rows = Vec::new();
            d.each_painted_text(|text, _| {
                let words = text.galley.job.text.replace('\u{a0}', " ");
                if words.contains("× Maximum HP would become −28 (as Goblin Warrior)") {
                    rows = text
                        .galley
                        .rows
                        .iter()
                        .map(|row| row.glyphs.iter().map(|glyph| glyph.chr).collect::<String>())
                        .collect();
                }
            });
            let whole = "Maximum HP would become −28 (as Goblin Warrior)"
                .split(' ')
                .all(|word| {
                    rows.iter()
                        .any(|row| row.split([' ', '\u{a0}']).any(|part| part == word))
                });
            d.observe(
                "iter-23-B-FH4",
                !rows.is_empty() && whole && rows.iter().all(|row| !row.trim_end().ends_with('(')),
                format!("At {size:?}: Mill Watch's summary rows {rows:?}"),
            );
        });
        // Some members can join: Add to fight keeps its place, and the row menu holds the
        // repair its fault names.
        d.click("More actions for Split Watch");
        d.observe(
            "iter-23-B-FH1-menu",
            d.has("Add Split Watch to fight") && d.in_view("Edit group Split Watch"),
            "A group that can partly join keeps Add to fight and offers Edit group in its menu",
        );
        d.click("Edit group Split Watch");
        d.expect("Group name: Split Watch");
    }
}

/// W14: an evening from the session's one page: the plan, the fights, and what they leave behind.
pub mod session_plan {
    use super::*;
    /// New events go to this session, by the Timeline page's own control.
    fn go_timeline_choose_session(d: &mut Driver<'_>, session: &str) {
        d.click(NAV_TIMELINE);
        d.click("Session for new events");
        d.click_starting(session);
    }

    /// The top edge of each main rail item, found at the rail's left edge, not on the page.
    fn rail_tops(d: &Driver<'_>) -> Vec<(&'static str, Option<f32>)> {
        let nodes = d.nodes();
        [
            NAV_ENCOUNTERS,
            NAV_SESSIONS,
            NAV_TIMELINE,
            NAV_WORLD,
            NAV_PARTY,
            NAV_LIBRARY,
            NAV_CAMPAIGN,
        ]
        .map(|name| {
            let top = nodes
                .iter()
                .find(|node| node.name == name && node.rect.left() < crate::design::XL)
                .map(|node| node.rect.top());
            (name, top)
        })
        .to_vec()
    }

    /// FT4: the played fight's `Log`, `Whole run` and `Timeline` start their words at the text
    /// edge of the HP grid above, with no spaces in the words, and each target is at least `XL`
    /// square from the link's own size.
    fn played_fight_links_at_the_text_edge(d: &mut Driver<'_>) {
        use egui::accesskit::Role;
        let size = d.window_size();
        let xl = crate::design::XL;
        let nodes = d.nodes();
        let links = ["Log", "Whole run", "Timeline"].map(|name| {
            nodes
                .iter()
                .find(|node| node.name == name && node.role == Role::Link && node.in_view())
                .map(|node| node.rect)
        });
        // The word as painted, alone in its galley: a padded ` Log ` does not match.
        let words: Vec<_> = ["Log", "Whole run", "Timeline"]
            .iter()
            .zip(&links)
            .map(|(name, link)| {
                link.and_then(|link| d.painted_word_rect(name, link.expand(crate::design::XS)))
            })
            .collect();
        // The grid's first cell, in the party bar's order (iter-23-S FK4).
        let edge = d.painted_left("Brannoch 58/62");
        let log = words[0];
        d.observe(
            "iter-14-C-FT4",
            links.iter().all(|link| {
                link.is_some_and(|link| link.width() >= xl - 0.5 && link.height() >= xl - 0.5)
            }) && words.iter().all(Option::is_some)
                && edge
                    .zip(log)
                    .is_some_and(|(edge, log)| (log.left() - edge).abs() <= 0.5),
            format!(
                "At {size:?}, Log starts at the HP grid's text edge {edge:?}: words {words:?}, \
                 targets {links:?}"
            ),
        );
        // Must stay true: the links stay on one line, in order.
        d.observe(
            "iter-14-C-FT4-order",
            words.iter().flatten().count() == 3
                && words.windows(2).all(|pair| {
                    pair[0].zip(pair[1]).is_some_and(|(a, b)| {
                        a.right() < b.left() && (a.center().y - b.center().y).abs() <= 0.5
                    })
                }),
            format!("At {size:?}, Log, Whole run and Timeline share one line: {words:?}"),
        );
    }

    /// FT1: the party bar says where its numbers come from whenever the room after the players
    /// holds it. Its words are the last fact of each player's accessible name.
    /// iter-23-S FK4: the plan's final HP grid reads in the party bar's order.
    fn hp_grid_follows_the_bar(d: &mut Driver<'_>) {
        let nodes = d.nodes();
        let mut bar: Vec<_> = nodes
            .iter()
            .filter(|node| {
                node.role == egui::accesskit::Role::Button
                    && node.name.contains(" · HP ")
                    && node.in_view()
            })
            .filter_map(|node| {
                let first = node.name.split_whitespace().next()?;
                Some((node.rect.left(), first.to_owned()))
            })
            .collect();
        bar.sort_by(|a, b| a.0.total_cmp(&b.0));
        let bar: Vec<_> = bar.into_iter().map(|(_, name)| name).collect();
        let mut grid: Vec<_> = nodes
            .iter()
            .filter(|node| node.in_view() && node.role != egui::accesskit::Role::Button)
            .filter_map(|node| {
                let (first, hp) = node.name.split_once(' ')?;
                (bar.iter().any(|name| name == first)
                    && hp.split_whitespace().next()?.contains('/'))
                .then(|| (node.rect.top().round(), node.rect.left(), first.to_owned()))
            })
            .collect();
        grid.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
        // A label can show twice in the tree at one place; one place is one cell.
        grid.dedup();
        let grid: Vec<_> = grid.into_iter().map(|(.., name)| name).collect();
        d.observe(
            "iter-23-S-FK4",
            grid.len() >= 2 && grid == bar,
            format!("The final HP grid reads as the party bar does: grid {grid:?}, bar {bar:?}"),
        );
    }

    fn party_bar_names_its_source(d: &mut Driver<'_>) {
        use egui::accesskit::Role;
        let size = d.window_size();
        let nodes = d.nodes();
        let hide = nodes
            .iter()
            .find(|node| node.name == "Hide the party")
            .map(|node| node.rect);
        let players: Vec<_> = nodes
            .iter()
            .filter(|node| {
                node.role == Role::Button
                    && node.name.contains(" · HP ")
                    && hide.is_some_and(|hide| {
                        (node.rect.center().y - hide.center().y).abs() <= crate::design::S
                    })
            })
            .collect();
        let kestrel = players
            .iter()
            .find(|node| node.name.starts_with("Kestrel Dunmoor · HP 7/41"));
        let source = kestrel.and_then(|node| node.name.rsplit(" · ").next());
        let last = players
            .iter()
            .map(|node| node.rect.right())
            .fold(0.0, f32::max);
        let status = source.and_then(|source| {
            nodes
                .iter()
                .find(|node| node.name.ends_with(source) && !node.name.contains(" · HP "))
                .map(|node| node.rect)
        });
        let placed = status
            .zip(hide)
            .is_some_and(|(status, hide)| status.left() >= last && status.right() <= hide.left());
        d.observe(
            "iter-14-C-FT1",
            kestrel.is_some() && (size == SMALL || placed),
            format!(
                "At {size:?}, the bar after {} players (to x {last}) shows its source {source:?} \
                 at {status:?}, before Hide the party at {hide:?}",
                players.len()
            ),
        );
    }

    const FIRST: &str = "Bridge Ambush";
    const SECOND: &str = "Shrine of Cinders";
    const PLAYERS: [(&str, &str); 5] = [
        ("Brannoch Vale", "15"),
        ("Iselle Marrowind", "19"),
        ("Kestrel Dunmoor", "21"),
        ("Oren Thistlewick", "11"),
        ("Sable Harrowgate", "13"),
    ];

    fn roll_and_begin(d: &mut Driver<'_>) {
        d.click("Roll NPC initiative");
        for (name, total) in PLAYERS {
            d.type_into(&format!("Initiative for {name}"), total);
        }
    }

    pub fn run(d: &mut Driver<'_>) {
        d.click(NAV_SESSIONS);
        new_session(d, "The Road");
        let body = d.editor_text().expect("new session has a note");
        d.replace_editor_text(&body.replace(
            "## What happened",
            "## What happened\n\nHobb the miller says the toll collectors have not been paid.",
        ));
        d.press_ctrl(Key::S);
        d.click("New session");
        d.type_into("Session title", "The Crossing");
        d.click("Create session");
        d.expect("Session 2: The Crossing");
        d.wait_for_text("Hobb the miller says");
        d.state("new-session-with-last-time", Sizes::All);
        d.observe_both(
            "iter-18-B-S4",
            "The new note's first source line is fully visible",
            |d| d.first_source_line_visible("# Session 2: The Crossing"),
        );
        d.observe(
            "T12.6",
            d.has("Last time:")
                && d.has("Session 1: The Road")
                && d.visible_text("Hobb the miller says"),
            "A new session shows what the last one left, with nothing to open",
        );

        // T12.1: from the plan to the fight, and back to the plan.
        show_plan(d);
        d.wait_for_text("Nothing planned yet");
        d.state_prepared("plan-fight-picker", Sizes::Default, |d| {
            if !d.has(&format!("Plan {FIRST}")) {
                d.click("Add a fight");
            }
            d.wait_for_text(FIRST);
        });
        d.click(&format!("Plan {FIRST}"));
        d.click("Add a fight");
        d.click(&format!("Plan {SECOND}"));
        d.expect(&format!("Resume: {FIRST}"));
        d.click(&format!("More actions for {SECOND}"));
        d.click("Remove from tonight");
        d.expect(&format!("Removed “{SECOND}” from tonight."));
        // iter-25-S family: a removal is a finished change, in the success tone.
        d.observe(
            "iter-25-S-success-fight-removed",
            d.place_of(&format!("Removed “{SECOND}” from tonight."))
                .is_some_and(|words| d.has_tone_bar(words, crate::design::colors().result.success)),
            "Removing a planned fight says so in the success tone",
        );
        d.state("planned-fight-removed", Sizes::All);
        d.click("Undo remove");
        d.expect(&format!("Start fight: {SECOND}"));
        // P1c: both plan acts go through the note's one act path, `Draft::apply`, with today's
        // behaviour: the change joins the draft as typed words do, and the note saves after its
        // pause. Beside Lookup as without it. Test builds have no timed save, so the file holds
        // only what an act would save at once.
        d.click(NAV_LOOKUP);
        let folder = d
            .app()
            .campaigns
            .active_campaign()
            .unwrap()
            .path
            .join(crate::sessions::DEFAULT_FOLDER);
        let note = crate::sessions::list_notes(&folder)
            .into_iter()
            .find(|note| note.title == "The Crossing")
            .expect("the session's note")
            .path;
        let line = format!("[[{SECOND}]]");
        let before = std::fs::read_to_string(&note).unwrap();
        d.click(&format!("More actions for {SECOND}"));
        d.click("Remove from tonight");
        d.expect(&format!("Removed “{SECOND}” from tonight."));
        d.state("planned-fight-removed-beside-lookup", Sizes::Default);
        d.click("Undo remove");
        d.expect(&format!("Start fight: {SECOND}"));
        let after = std::fs::read_to_string(&note).unwrap();
        d.observe(
            "P1c-plan-acts",
            !before.contains(&line) && after == before && d.has("Close Lookup"),
            format!("Beside Lookup, Remove from tonight and its Undo change the draft, and the file waits for the note's pause: {before:?}, then {after:?}"),
        );
        d.click(NAV_LOOKUP);

        d.state_prepared("plan-with-fights", Sizes::AllAndHiDpi, |d| {
            assert!(
                d.in_view(&format!("Resume: {FIRST}")),
                "iter-9-C-FT1: the paused plan row has Resume"
            );
            let frame = d.control_frame(&format!("Resume: {FIRST}"));
            let status = d.visible_text("Round 2 · Paused");
            d.observe(
                "iter-9-C-FT1",
                status
                    && frame.as_ref().is_some_and(|frame| {
                            frame.fill == crate::design::colors().accent_fill
                                || frame.fill == crate::design::colors().accent_hover
                        }),
                format!("A legacy paused run owns Resume and shows Round 2 · Paused on its plan row: status {status}, frame {frame:?}"),
            );
        });
        d.check_tab_order("session plan");
        let paused = d
            .app_mut()
            .encounters
            .saved_runs()
            .into_iter()
            .find(|run| run.encounter == FIRST && run.source.is_none())
            .unwrap();
        let paused_bytes = std::fs::read(&paused.path).unwrap();
        d.click(&format!("More actions for {FIRST}"));
        assert!(d.in_view("Start a new run") && d.in_view("Delete the saved run"));
        d.press(Key::Escape);
        assert!(d.in_view(&format!("Resume: {FIRST}")));
        d.click(&format!("Resume: {FIRST}"));
        d.expect(NEXT_TURN);
        assert_eq!(
            d.app_mut()
                .encounters
                .run()
                .and_then(|run| run.live_info())
                .and_then(|run| run.id),
            paused.id,
            "iter-9-C2-FT1-resume-id: Resume opens the paused run, not a new run"
        );
        close_run(d);
        d.click(NAV_SESSIONS);
        show_plan(d);
        d.click(&format!("More actions for {FIRST}"));
        assert!(d.in_view("Remove this event"));
        d.click("Remove this event");
        d.expect(&format!("Resume: {FIRST}"));
        // AT2: the notice says which Bridge Ambush went, and that the planned one stays.
        let notice = format!("Removed “{FIRST}” (Round 2 · Paused). The other “{FIRST}” stays.");
        d.observe(
            "iter-10-C-AT2",
            d.visible_text(&notice),
            format!("The removal notice names the removed fight by its state: {notice}"),
        );
        d.observe(
            "iter-10-C-AT2-undo",
            d.in_view("Undo remove")
                && d.place_of("Undo remove")
                    .zip(d.place_of(FIRST))
                    .is_some_and(|(undo, row)| undo.bottom() <= row.top()),
            "Undo remove stays with the notice, above the remaining row",
        );
        d.click(&format!("More actions for {FIRST}"));
        assert!(d.in_view("Delete the saved run"));
        d.click("Delete the saved run");
        d.expect(&format!("Start fight: {FIRST}"));
        assert!(!d.has(&format!("Resume: {FIRST}")));
        // Round 24 FL3: one step gives one result. Deleting the saved run replaces the earlier
        // removal's result in the plan, so the plan has one message and one Undo.
        let undos = d
            .nodes()
            .iter()
            .filter(|node| node.in_view() && node.name.starts_with("Undo"))
            .map(|node| node.name.clone())
            .collect::<Vec<_>>();
        d.observe(
            "iter-24-S-FL3",
            undos == ["Undo delete run"]
                && d.visible_text(&format!("Deleted the saved run of {FIRST}."))
                && !d.visible_text(&notice),
            format!("The plan shows one result and one Undo after two steps: {undos:?}"),
        );
        // iter-25-S family: deleting a saved run is a finished change, in the success tone.
        d.observe(
            "iter-25-S-success-run-deleted",
            d.place_of(&format!("Deleted the saved run of {FIRST}."))
                .is_some_and(|words| d.has_tone_bar(words, crate::design::colors().result.success)),
            "The deleted run's result has the success tone",
        );
        // Review of PR 245: Undo delete run ends only the deletion's result. The run comes back,
        // and the removal's message and Undo show again; then delete the run again.
        d.click("Undo delete run");
        d.expect(&format!("Resume: {FIRST}"));
        d.observe(
            "iter-24-S-FL3-undo",
            !d.has("Undo delete run")
                && d.visible_text(&notice)
                && d.in_view("Undo remove")
                && d.in_view(&format!("Resume: {FIRST}")),
            "Undo delete run brings the run back, and the removal's result returns",
        );
        d.click(&format!("More actions for {FIRST}"));
        d.click("Delete the saved run");
        d.expect(&format!("Start fight: {FIRST}"));
        d.observe(
            "iter-24-S-FL3",
            !d.has("Undo remove") && d.in_view("Undo delete run"),
            "Deleting the run again replaces the removal's result again",
        );
        // FT4: Timeline names the removed fight as the plan does.
        d.click(NAV_TIMELINE);
        d.observe(
            "iter-12-C-FT4",
            d.visible_text(&format!("Removed “{FIRST}” (Round 2 · Paused)."))
                && d.in_view("Undo remove"),
            "Timeline's removal notice names the removed fight, with its Undo",
        );
        d.click(NAV_SESSIONS);
        show_plan(d);
        write_note(d);
        d.observe(
            "T12.1",
            d.editor_text()
                .is_some_and(|text| text.contains(&format!("[[{FIRST}]]"))),
            "The plan is in the note's own words, so Obsidian sees it too",
        );
        read_note(d);
        d.observe(
            "T12.1",
            !d.visible_text("[[") && d.visible_text("2 fights, in the plan"),
            "The tab layout summarizes planned fights and keeps the links in the file",
        );
        d.state("note-read-with-links", Sizes::Default);
        show_plan(d);
        d.begin_task();
        d.click(&format!("Start fight: {FIRST}"));
        d.goal_within("T12.1", "start a planned fight from the session", 1);
        d.expect(&format!("{RUN_HEADING}{FIRST}"));
        d.state_prepared("setup-with-read-aloud", Sizes::All, |d| {
            if d.has("Technical details") {
                fight_tab(d, "Prep");
            }
            d.wait_for_text("Wolves hold the far bank");
        });
        d.observe(
            "T12.7",
            d.visible_text("Wolves hold the far bank") && !d.visible_text("roster:"),
            "Setup shows what to read to the table, and no YAML",
        );
        fight_tab(d, "Prep");
        d.state_prepared("prep-technical-details", Sizes::Default, |d| {
            if !d.has("Technical details") {
                fight_tab(d, "Prep");
            }
            if !d.visible_text("Phases, assumptions") {
                d.click("Technical details");
            }
            d.wait_for_text("Phases, assumptions");
        });
        d.click("Technical details");
        d.observe(
            "T12.7",
            d.has("Technical details") && !d.visible_text("roster:"),
            "Prep keeps the setup's YAML closed under Technical details",
        );
        d.observe(
            "S22",
            !d.has("Turn order"),
            "Setup shows Prep without a second roster",
        );
        roll_and_begin(d);
        d.click("Begin combat");
        d.expect(CURRENT_TURN);

        // T12.3: one amount on several targets, one step of undo.
        d.click_row("Cinder Wolf 1");
        d.ctrl_click_row("Cinder Wolf 2");
        d.ctrl_click_row("Ember Cultist");
        d.expect("Half damage: Ember Cultist");
        d.click("Half damage: Ember Cultist");
        d.state("several-targets", Sizes::All);
        d.begin_task();
        d.type_text("8");
        d.press(Key::Enter);
        let cost = d.goal("damage several targets, once they are chosen");
        let (wolf, cultist) = (d.row_hp("Cinder Wolf 2"), d.row_hp("Ember Cultist"));
        d.observe(
            "T12.3",
            cost <= 2 && wolf == Some(14) && cultist == Some(29),
            format!("Full and half from one amount, in {cost}: wolf {wolf:?}, cultist {cultist:?}"),
        );
        d.observe(
            "T12.3",
            d.has_containing("Undo: 8 damage to 3 participants, half for 1"),
            "One step of undo, and it says how many, and how many took half",
        );
        // SH1: at 960 the whole step does not fit beside Connect to sync, so Undo says the step
        // without its rows, not `Undo` alone; hover and the accessible name keep it whole.
        let shown = d.at_size(SMALL, |d| {
            (
                d.visible_text("Undo: 8 damage × 3"),
                d.visible_text("Undo: 8 damage to 3 participants"),
            )
        });
        d.observe(
            "iter-23-R-SH1",
            shown == (true, false) && d.has("Undo: 8 damage to 3 participants, half for 1"),
            format!("At 960 Undo says `Undo: 8 damage × 3` (compact, whole): {shown:?}"),
        );
        d.state("several-targets-applied", Sizes::Default);
        d.click_containing("Undo: 8 damage");
        d.observe(
            "T12.3",
            d.row_hp("Cinder Wolf 2") == Some(22) && d.row_hp("Ember Cultist") == Some(33),
            "Undo takes the whole step back",
        );

        // T12.4: dice beside the amount, with the stat block still open.
        d.click_row("Ember Cultist");
        d.begin_task();
        d.type_into("Dice", "1d1+3");
        d.press(Key::Enter);
        let cost = d.goal("roll dice beside the stat block");
        // iter-26-R SL1: the selected name keeps the dock's left edge at every size, and the
        // total never starts the name's line: at 960 it has its own place at the line's end.
        for size in [SMALL, DEFAULT, LARGE] {
            let (name, amount, rolled) = d.at_size(size, |d| {
                (
                    d.place_of("Selected: Ember Cultist"),
                    d.place_of("Amount"),
                    d.place_of("Rolled 4"),
                )
            });
            let painted = d.at_size(size, |d| {
                name.map(|name| d.painted_in(name)).unwrap_or_default()
            });
            d.observe(
                "iter-26-R-SL1",
                name.zip(amount)
                    .is_some_and(|(name, amount)| (name.left() - amount.left()).abs() <= 1.0)
                    && !painted.contains('=')
                    && (size != SMALL
                        || name.zip(rolled).is_some_and(|(name, rolled)| rolled.left() > name.right())),
                format!("At {size:?}: name {name:?} paints {painted:?}, Amount {amount:?}, total {rolled:?}"),
            );
        }
        d.state("dice-in-the-dock", Sizes::All);
        d.observe(
            "T12.4",
            cost <= 3 && d.visible_text("Scorching Sickle") && d.visible_text("4"),
            format!("The result shows in {cost}, and the stat block stays"),
        );
        // iter11-B-FT2: a roll is not an Undo step. Damage, a roll, then U takes back the damage.
        d.observe(
            "iter11-B-FT2",
            d.has_containing("Undo") && !d.has_containing("Undo: 1d1+3"),
            "The roll does not take an Undo step",
        );
        let whole = d.row_hp("Ember Cultist");
        d.type_text("2");
        d.press(Key::Enter);
        let hurt = d.row_hp("Ember Cultist");
        assert!(d.has_containing("Undo: 2 damage to Ember Cultist"));
        // Not T12.4's expression, so the Log line can only come from this roll.
        d.type_into("Dice", "1d1+5");
        d.press(Key::Enter);
        d.observe(
            "iter11-B-FT2",
            d.has_containing("Undo: 2 damage to Ember Cultist"),
            "After the roll, Undo still names the damage",
        );
        d.press(Key::U);
        d.observe(
            "iter11-B-FT2",
            hurt != whole && d.row_hp("Ember Cultist") == whole,
            format!(
                "U after the roll takes back the damage: {whole:?}, {hurt:?}, then {:?}",
                d.row_hp("Ember Cultist")
            ),
        );
        fight_tab(d, "Log");
        d.observe(
            "iter11-B-FT2",
            d.visible_text("DM roll 1d1+5"),
            "The roll stays in the Log after that Undo",
        );
        fight_tab(d, "Stat block");
        d.expect("Scorching Sickle");

        // T12.5: a look at someone else, and back.
        let turn = current_participant(d).unwrap_or_default();
        d.press(Key::Escape);
        d.observe(
            "T12.5",
            participant_is_selected(d, &turn),
            "Escape selects the current turn again",
        );

        // T12.8: a condition's rule, from its word on the row.
        d.click_row("Oren Thistlewick");
        d.click(CONDITIONS);
        d.click("Frightened");
        d.begin_task();
        d.click("Rule: Frightened");
        d.expect("Close Lookup");
        d.state("rule-from-the-row", Sizes::Default);
        d.press(Key::Escape);
        d.goal_within("T12.8", "read a condition's rule from its row", 2);

        // T12.9: words the DM wrote, found beside the fight.
        d.press_ctrl(Key::K);
        d.type_text("miller");
        // FT2 and FT4: `New NPC “Miller”` starts a line at the search field's edge, with curly
        // quotation marks. `New location “Miller”` follows it on its line, or starts the next
        // line at the same edge when it wraps.
        let field_left = |d: &Driver<'_>| d.place_of(SEARCH_LOOKUP).map(|field| field.left());
        lookup_action_at_text_edge(
            d,
            "iter-17-D-FT2-new",
            "New NPC “Miller”",
            &[SMALL, DEFAULT, LARGE],
            field_left,
        );
        // ST2: the words of `New NPC “Miller”` start at the result names. Outside a message, a
        // line-start action has no icon, so nothing of it sits left of the names.
        for size in [SMALL, DEFAULT, LARGE] {
            let (words, names, icon) = d.at_size(size, |d| {
                let target = d.place_of("New NPC “Miller”");
                (
                    target.and_then(|target| words_left_in(d, target)),
                    lookup_row_text_left(d),
                    target.is_some_and(|target| {
                        let mut icon = false;
                        d.each_painted_text(|text, _| {
                            let rect = egui::Rect::from_min_size(text.pos, text.galley.size());
                            icon |=
                                target.contains(rect.center()) && is_icon(&text.galley.job.text);
                        });
                        icon
                    }),
                )
            });
            d.observe(
                "iter-18-A-ST2",
                words
                    .zip(names)
                    .is_some_and(|(words, names)| (words - names).abs() <= 1.0)
                    && !icon,
                format!(
                    "At {size:?}, New NPC's words start at {words:?}, the result names at \
                     {names:?}; an icon in its target: {icon}"
                ),
            );
        }
        for size in [SMALL, DEFAULT, LARGE] {
            let (npc, location, edge, splitter, words) = d.at_size(size, |d| {
                d.expect("New location “Miller”");
                let location = d.place_of("New location “Miller”");
                (
                    d.place_of("New NPC “Miller”"),
                    location,
                    field_left(d),
                    d.place_of(LOOKUP_SPLITTER),
                    location.and_then(|location| words_left_in(d, location)),
                )
            });
            // On a line of its own, its words start at the edge and its icon hangs left of it.
            let placed = npc
                .zip(location)
                .zip(edge)
                .is_some_and(|((npc, location), edge)| {
                    if (location.top() - npc.top()).abs() < 1.0 {
                        location.left() >= npc.right()
                    } else {
                        words.is_some_and(|words| (words - edge).abs() <= 1.0)
                    }
                });
            d.observe(
                "iter-17-D-FT2-new-location",
                placed
                    && location
                        .zip(splitter)
                        .is_some_and(|(location, splitter)| location.left() >= splitter.right()),
                format!(
                    "At {size:?}, New location {location:?} follows New NPC {npc:?} or starts at the \
                     edge {edge:?}, right of the splitter {splitter:?}"
                ),
            );
        }
        d.observe(
            "iter-17-D-FT4",
            d.has("New NPC “Miller”")
                && d.has("New location “Miller”")
                && !d.has_containing("\"Miller\""),
            "Lookup's New actions name the new note in curly quotation marks",
        );
        d.state("lookup-finds-a-note", Sizes::All);
        d.observe(
            "T12.9",
            d.visible_text("Hobb the miller says") && d.in_view(NEXT_TURN),
            "Lookup finds last session's note, and the run stays in view",
        );
        // Reading a result closes the list. `N results`, which opens it again, starts a line at
        // the search field's edge.
        let first = lookup_results(d).first().map(|row| row.name.clone());
        if let Some(first) = &first {
            d.click(first);
        }
        lookup_action_at_text_edge(
            d,
            "iter-17-D-results",
            "Show the results",
            &[SMALL, DEFAULT, LARGE],
            |d| d.place_of(SEARCH_LOOKUP).map(|field| field.left()),
        );
        // The action names what a press does: `Show N results`, never a bare count, at every
        // captured size.
        for size in [SMALL, DEFAULT, LARGE] {
            let words = d.at_size(size, lookup_results_words);
            d.observe(
                "iter-19-D-show-results",
                words.as_deref().is_some_and(|words| {
                    words
                        .strip_prefix("Show ")
                        .and_then(|rest| rest.strip_suffix(" results"))
                        .is_some_and(|count| count.parse::<usize>().is_ok_and(|count| count > 1))
                }),
                format!("At {size:?}, the results action says {words:?}"),
            );
        }
        d.press(Key::Escape);

        // T14.2: a number typed wrong can be put right, and the turn stays.
        let turn = current_participant(d);
        d.click_row("Ember Cultist");
        d.click("Correct");
        d.state("correct-initiative", Sizes::Default);
        d.type_into("Initiative for Ember Cultist", "30");
        d.click("Set initiative");
        d.observe(
            "T14.2",
            roster_places(d).first().map(|row| row.0.as_str()) == Some("Ember Cultist")
                && current_participant(d) == turn,
            "A wrong initiative moves its row, and whose turn it is does not change",
        );
        d.click_containing("Undo: initiative for Ember Cultist");

        // T14.3: Exhaustion is one level, however many steps it takes.
        d.click_row("Oren Thistlewick");
        d.observe(
            "iter9-B-FT8-before",
            !d.row_words("Oren Thistlewick").contains("Sync off"),
            "Oren has no sync word before Correct",
        );
        d.click("Correct");
        d.click(CONDITIONS);
        d.observe(
            "iter9-B-FT8-no-side-effect",
            !d.row_words("Oren Thistlewick").contains("Sync off"),
            "Viewing Correct does not turn sync off",
        );
        d.click("Exhaustion");
        d.click("Increase Exhaustion level");
        d.click("Increase Exhaustion level");
        d.expect("exhaustion 2 → 3 on Oren Thistlewick");
        let shown = d.at_size(SMALL, |d| d.visible_text("Undo: exhaustion 2 → 3"));
        d.observe(
            "iter-23-R-SH1",
            shown,
            "At 960 Undo keeps both exhaustion levels, without the row when its name does not fit",
        );
        d.state_prepared("exhaustion-level", Sizes::All, |d| {
            let down = d
                .place_of("Decrease Exhaustion level")
                .expect("Required decrease control");
            let up = d
                .place_of("Increase Exhaustion level")
                .expect("Required increase control");
            let value = d
                .nodes()
                .into_iter()
                .find(|n| {
                    n.name == "Exhaustion level"
                        && n.rect.left() > down.right()
                        && n.rect.right() < up.left()
                })
                .expect("Required exhaustion value between the step buttons")
                .rect;
            d.observe(
                "iter9-B-FT12-scan",
                [value, down, up]
                    .iter()
                    .all(|r| (r.height() - crate::design::Density::Scan.control()).abs() <= 1.0),
                format!("Exhaustion uses one Scan stepper: {down:?}, {value:?}, {up:?}"),
            );
            d.observe(
                "iter9-B-FT8-stable",
                !d.row_words("Oren Thistlewick").contains("Sync off"),
                "Changing exhaustion does not turn sync off",
            );
        });
        let row = d.row_words("Oren Thistlewick");
        d.observe(
            "T14.3",
            row.contains("Exhaustion (3)") && !row.contains("Exhaustion (2)"),
            format!("Each step replaces the last level: {row}"),
        );
        d.click_containing("Undo: exhaustion 2 → 3 on Oren Thistlewick");
        d.observe(
            "iter9-B-FT12-undo",
            d.row_words("Oren Thistlewick").contains("Exhaustion (2)")
                && !d.row_words("Oren Thistlewick").contains("Exhaustion (3)"),
            "Undo restores the previous level",
        );
        d.click("Decrease Exhaustion level");
        d.click("Decrease Exhaustion level");

        // Kestrel ends this fight hurt, so the next one has something to carry.
        d.click_row("Kestrel Dunmoor");
        d.type_text("4");
        d.press(Key::Enter);
        d.click("End encounter…");
        d.type_text("Held the bridge");
        d.press(Key::Enter);
        d.expect("Outcome: Held the bridge");
        let with_run = rail_tops(d);
        let had_slot = d.has(&crate::no_run_name());
        d.begin_task();
        d.click(CLOSE_RUN);
        d.goal_within("T12.1", "go back from a finished fight", 1);
        // FT3: closing the last run leaves the quiet Run slot, and no main rail item moves.
        let without_run = rail_tops(d);
        d.observe(
            "iter-14-C-FT3-close",
            !had_slot
                && d.has(&crate::no_run_name())
                && without_run.iter().all(|(_, top)| top.is_some())
                && without_run == with_run,
            format!(
                "Closing the run leaves the empty Run slot, and the rail items keep their places: \
                 {with_run:?} then {without_run:?}"
            ),
        );
        d.observe(
            "T12.1",
            d.page_title().as_deref() == Some("Session"),
            "Close run goes back to the page the fight began on",
        );
        // FT3: the empty Run slot is a working target. It opens the Encounters list.
        assert!(
            d.in_view(&crate::no_run_name()),
            "the empty Run slot is in view"
        );
        d.click(&crate::no_run_name());
        d.observe(
            "iter-14-C-FT3-opens",
            d.page_title().as_deref() == Some("Encounters"),
            format!("The empty Run slot opens Encounters: {:?}", d.page_title()),
        );
        d.click(NAV_SESSIONS);

        // T12.5: how a fight ended, in place.
        show_plan(d);
        // iter-23-S FK2: a run that starts ends the removal's result and its Undo, so no notice
        // sits above rows it no longer names.
        let removal_gone = |d: &Driver<'_>| {
            !d.has_containing(&format!("Removed “{FIRST}”")) && !d.has("Undo remove")
        };
        d.state_prepared("plan-after-a-fight", Sizes::All, |d| {
            d.observe(
                "iter-23-S-FK2-plan",
                removal_gone(d),
                "After the fight ran, the plan shows no removal notice",
            );
        });
        d.begin_task();
        d.click(FIRST);
        d.observe(
            "S3",
            d.nodes().iter().any(|node| {
                node.name.replace('\u{a0}', " ") == "Held the bridge · Ended · Round 1"
            }),
            "A played fight gives its result and last round on one line",
        );
        d.goal_within("T12.5", "read how a fight ended", 1);
        d.state_prepared("fight-in-place", Sizes::All, |d| {
            played_fight_links_at_the_text_edge(d);
            party_bar_names_its_source(d);
            if d.window_size() == DEFAULT {
                hp_grid_follows_the_bar(d);
            }
        });
        d.observe(
            "T12.5",
            d.visible_text("Kestrel 7/41") && d.page_title().as_deref() == Some("Session"),
            "Final HP names each character in the plan, with no run to reopen",
        );
        d.click(FIRST);

        d.click(NAV_TIMELINE);
        assert!(d.has_containing("Held the bridge · Ended · Round 1"));
        d.state_prepared("timeline-after-fight", Sizes::All, |d| {
            d.observe(
                "iter-23-S-FK2-timeline",
                removal_gone(d) && d.has_containing("Held the bridge · Ended · Round 1"),
                "After the fight ran, Timeline shows no removal notice, and the played fight's row stays",
            );
        });
        d.click(NAV_SESSIONS);
        show_plan(d);

        // T14.4: a patrol can be planned again after it is played.
        d.click("Add a fight");
        d.click(&format!("Plan {FIRST}"));
        d.observe(
            "T14.4",
            d.has(&format!("Start fight: {FIRST}")),
            "A fight planned a second time can start a second time",
        );
        d.observe(
            "iter-23-S-FK2-again",
            removal_gone(d),
            "Planning the fight again shows no old removal notice",
        );
        d.state("plan-the-same-fight-again", Sizes::Default);

        // T12.2: the next fight remembers the last one.
        d.click(&format!("Start fight: {SECOND}"));
        let kestrel = |d: &Driver<'_>| {
            d.nodes()
                .into_iter()
                .find(|node| node.name.starts_with("Kestrel Dunmoor ·"))
                .map_or_else(String::new, |node| node.name)
        };
        d.state("setup-carries-hp", Sizes::All);
        // FT3: a run opening takes the slot in place, so the rail items do not move down.
        let opened = rail_tops(d);
        d.observe(
            "iter-14-C-FT3-open",
            !d.has(&crate::no_run_name()) && opened == without_run,
            format!(
                "Starting a fight keeps the rail items in place: {without_run:?} then {opened:?}"
            ),
        );
        d.observe(
            "T12.2",
            kestrel(d).contains("HP 7/41") && d.has_containing("Party HP carries over"),
            format!("Setup says where HP came from: {}", kestrel(d)),
        );
        d.begin_task();
        d.click("Use the sheets’ HP");
        d.goal_within("T12.2", "go back to the sheets' HP", 1);
        d.observe(
            "T12.2",
            kestrel(d).contains("HP 11/41"),
            format!("One press gives the sheet's value: {}", kestrel(d)),
        );
        close_run(d);

        // T12.2: a long rest is the DM's ruling, and the next fight follows it.
        show_plan(d);
        d.expect(&format!("Resume: {SECOND}"));
        d.state("plan-saved-run-resume", Sizes::All);
        d.click(&format!("More actions for {SECOND}"));
        d.click("Delete the saved run");
        d.observe(
            "S1",
            !d.has(&format!("Resume: {SECOND}")) && d.has("Undo delete run"),
            "Deleting the saved run offers one step of undo",
        );
        d.state("plan-saved-run-deleted", Sizes::Default);
        d.click("Undo delete run");
        d.expect(&format!("Resume: {SECOND}"));
        d.click(&format!("Resume: {SECOND}"));
        d.observe(
            "S1",
            kestrel(d).contains("HP 11/41"),
            "Resume keeps the saved setup's HP",
        );
        close_run(d);
        show_plan(d);
        d.click("Add Long rest");
        d.observe(
            "S4",
            d.has("Undo add") && !d.has("Undo everyone to full"),
            "The long rest has one Undo",
        );
        d.click("Undo add");
        d.observe(
            "S4",
            !d.has("Everyone at full"),
            "Undo removes both the rest and the full-HP ruling",
        );
        d.click("Add Long rest");
        assert!(d.has("Long rest added to Session 2. Everyone starts the next fight at full."));
        d.state("long-rest-restores", Sizes::All);
        // iter-25-S family (FB2 rule): the plan keeps the scroll bar's lane, so the row menu
        // keeps its x when the open rest and its result make the plan scroll at 960.
        let menu_right = |d: &Driver<'_>| {
            d.nodes()
                .into_iter()
                .find(|node| node.name.starts_with("More actions for Long rest"))
                .map(|node| node.rect.right())
        };
        let menu_before = d.at_size(SMALL, menu_right);
        d.click("Long rest");
        d.click("Mark not at full");
        d.expect("Undo ruling");
        let menu_after = d.at_size(SMALL, menu_right);
        d.observe(
            "iter-25-S-plan-lane",
            menu_before
                .zip(menu_after)
                .is_some_and(|(before, after)| (before - after).abs() < 1.0),
            format!("The row menu keeps its x at 960: {menu_before:?} then {menu_after:?}"),
        );
        // iter-23-S FL1: the result names the new ruling and its effect.
        d.observe(
            "iter-23-S-FL1-plan-off",
            d.visible_text("The next fight starts from the HP the party has now.")
                && !d.visible_text("Long rest ruling changed."),
            "Mark not at full says what the next fight starts from",
        );
        let session = d.app_mut().sessions.current();
        assert!(!crate::timeline::party_state(&d.app_mut().timeline.events, session).rested);
        d.state("plan-rest-ruling-off", Sizes::All);
        d.click("Long rest");
        assert!(!crate::timeline::party_state(&d.app_mut().timeline.events, session).rested);
        d.state_prepared("closed-plan-rest-ruling-off", Sizes::All, |d| {
            d.observe(
                "iter-3-C-ST1-plan",
                d.visible_text("Not\u{a0}at\u{a0}full") && !d.has("Mark everyone at full"),
                "The closed rest keeps its Not at full ruling in view",
            );
        });
        d.click("Long rest");
        let sizes = [(960.0, 640.0), (1200.0, 800.0)];
        let before = sizes.map(|size| {
            d.at_size(size, |d| {
                d.nodes()
                    .iter()
                    .find(|n| n.name == "Mark everyone at full")
                    .unwrap()
                    .rect
                    .top()
            })
        });
        d.click("Mark everyone at full");
        assert!(crate::timeline::party_state(&d.app_mut().timeline.events, session).rested);
        let after = sizes.map(|size| {
            d.at_size(size, |d| {
                d.nodes()
                    .iter()
                    .find(|n| n.name == "Mark not at full")
                    .unwrap()
                    .rect
                    .top()
            })
        });
        d.observe(
            "i20-S2",
            before.iter().zip(after).all(|(a, b)| (a - b).abs() < 1.0),
            "The open rest ruling stays at the same height at both sizes",
        );
        d.observe(
            "iter-23-S-FL1-plan-on",
            d.visible_text("Everyone starts the next fight at full.")
                && !d.visible_text("The next fight starts from the HP the party has now."),
            "Mark everyone at full says the next fight starts at full",
        );
        // iter-25-S FK6: the plan's ruling result is a finished change, in the success tone.
        d.observe(
            "iter-25-S-FK6",
            d.place_of("Everyone starts the next fight at full.")
                .is_some_and(|words| d.has_tone_bar(words, crate::design::colors().result.success)),
            "The plan's ruling result has the success tone",
        );
        d.expect_same(
            "The ruling and its message change words and colour; the open row stays in place.",
        );
        d.state("plan-rest-ruling-on", Sizes::All);
        d.click("Undo ruling");
        assert!(!crate::timeline::party_state(&d.app_mut().timeline.events, session).rested);
        d.click("Mark everyone at full");
        d.click("Long rest");
        d.click(&format!("Resume: {SECOND}"));
        d.expect("Party HP is full after the long rest.");
        d.expect("Use the saved HP");
        // iter-25-R FM1: Splat made the fill, so it is no Undo step. `Use the saved HP` is the
        // one control that gives the saved HP back.
        d.observe(
            "iter-25-R-FM1",
            // The Setup's own older step stays the newest: the fill added none.
            !d.has_containing("full HP after long rest")
                && !d.name_starting("Undo").contains("long rest"),
            format!(
                "The header offers no Undo of the fill: {}",
                d.name_starting("Undo")
            ),
        );
        d.observe(
            "iter-7-S1",
            kestrel(d).contains("HP 41/41"),
            "Resume restores a saved Setup after the long rest",
        );
        d.state("resumed-setup-after-long-rest", Sizes::All);
        const SAVED_HP_SET: &str = crate::encounter_run::SAVED_HP_USED;
        const SAVED_HP_UNDO: &str = "Undo use saved HP";
        let notice_top = d
            .place_of("Party HP is full after the long rest.")
            .map(|rect| rect.top());
        // iter-26-R FI2: the offer, its result and the offer back after Undo are in the setup's
        // message footer below the rows: no row moves through the three.
        // Setup rows, by their initiative fields.
        let row_tops = |d: &Driver<'_>| {
            d.nodes()
                .into_iter()
                .filter(|node| node.name.starts_with("Initiative for ") && node.in_view())
                .map(|node| (node.name, node.rect.top().round()))
                .collect::<Vec<_>>()
        };
        let rows_offered = row_tops(d);
        d.click("Use the saved HP");
        d.observe(
            "iter-7-S1",
            kestrel(d).contains("HP 11/41"),
            "Use the saved HP puts back the saved HP",
        );
        // FT2: the press is its own step, so the full HP stays one Undo away.
        let result_top = d.place_of(SAVED_HP_SET).map(|rect| rect.top());
        d.observe(
            "iter-16-B-FT2-message",
            notice_top
                .zip(result_top)
                .is_some_and(|(notice, result)| (notice - result).abs() < 1.0),
            format!(
                "The result replaces the notice in its place: {notice_top:?} then {result_top:?}"
            ),
        );
        d.observe(
            "iter-16-B-FT2-message-undo",
            d.in_view(SAVED_HP_UNDO)
                && d.place_of(SAVED_HP_UNDO)
                    .zip(d.place_of(SAVED_HP_SET))
                    .is_some_and(|(undo, words)| {
                        (undo.center().y - words.center().y).abs() < crate::design::S
                    }),
            "The result offers Undo on its own line",
        );
        d.observe(
            "iter-16-B-FT2-header",
            // iter-25-R FH4: the header and the message's Undo name one step: use saved HP.
            d.has("Undo: use saved HP") && !d.has_containing("Nothing to undo"),
            "The header names the new step, not an empty history",
        );
        // FJ4: the message's Undo names its step, as every other message Undo does.
        d.observe(
            "iter-24-R-FJ4",
            d.has(SAVED_HP_UNDO) && !d.has("Undo"),
            "The saved-HP result's Undo says Undo use saved HP, not a bare Undo",
        );
        let rows_result = row_tops(d);
        d.state("saved-hp-restored", Sizes::All);
        // Revision 18: A (use saved HP), then an unrelated step B (an initiative typed), then
        // Undo B and Undo A. B ends A's result and empties the footer, and no row moves; Undo B
        // gives A's result back with its Undo; Undo A still restores the full HP.
        let field = rows_result
            .first()
            .map(|(name, _)| name.clone())
            .expect("setup rows have initiative fields");
        d.type_into(&field, "12");
        d.press(Key::Enter);
        let rows_b = row_tops(d);
        d.observe(
            "iter-26-R-FI2-setup-b",
            d.field_text(&field).as_deref() == Some("12")
                && !d.visible_text(SAVED_HP_SET)
                // Astra 1: no result shows for a step Undo can no longer reach first.
                && !d.has(SAVED_HP_UNDO)
                && d.has_containing("Undo: initiative")
                && rows_b == rows_result,
            format!("B ends A's result and moves no row: {rows_result:?} then {rows_b:?}"),
        );
        d.click_containing("Undo: initiative");
        d.observe(
            "iter-26-R-FI2-setup-undo-b",
            d.field_text(&field).as_deref() != Some("12")
                && d.visible_text(SAVED_HP_SET)
                && d.in_view(SAVED_HP_UNDO)
                && kestrel(d).contains("HP 11/41")
                && row_tops(d) == rows_result,
            "Undo of B gives back A's result and its Undo, with A's HP",
        );
        // AT1: this Undo has the shape of every Undo after a message: quiet, with its icon.
        let saved_hp_undo = undo_shape(d, SAVED_HP_UNDO);
        d.observe(
            "iter-17-B-AT1",
            saved_hp_undo.is_some_and(|(stroke, _, _, icon)| stroke == egui::Stroke::NONE && icon),
            format!("The saved-HP result's Undo is quiet with the Undo icon: {saved_hp_undo:?}"),
        );
        d.click(SAVED_HP_UNDO);
        d.observe(
            "iter-16-B-FT2-undo",
            kestrel(d).contains("HP 41/41"),
            format!("Undo gives back the full HP: {}", kestrel(d)),
        );
        // Must stay true: the rest's own notice and its header step come back with it.
        d.observe(
            "iter-16-B-FT2-notice-back",
            d.has("Use the saved HP")
                && !d.has_containing("full HP after long rest")
                && !d.visible_text(SAVED_HP_SET),
            "After Undo the rest notice is back, with no Undo step of the fill, and the result is gone",
        );
        let rows_back = row_tops(d);
        d.observe(
            "iter-26-R-FI2-setup",
            !rows_offered.is_empty() && rows_offered == rows_result && rows_result == rows_back,
            format!("Setup row tops: offer {rows_offered:?}, result {rows_result:?}, after Undo {rows_back:?}"),
        );
        d.click("Use the saved HP");
        d.expect(SAVED_HP_SET);
        close_run(d);
        show_plan(d);
        d.click("Long rest");
        d.click("Mark not at full");
        d.click(NAV_TIMELINE);
        d.click("Add Long rest");
        d.expect("Long rest added. Everyone starts the next fight at full.");
        let undo_add = undo_shape(d, "Undo add");
        d.observe(
            "iter-17-B-AT1-pair",
            saved_hp_undo.is_some() && undo_add == saved_hp_undo,
            format!(
                "The saved-HP Undo {saved_hp_undo:?} has the same shape as Undo add {undo_add:?}"
            ),
        );
        d.click("Undo add");
        assert!(!crate::timeline::party_state(&d.app_mut().timeline.events, session).rested);
        d.click("Add Long rest");
        d.expect("Everyone at full");
        assert!(d.has("Long rest added. Everyone starts the next fight at full."));
        d.state("timeline-long-rest-restores", Sizes::All);
        // iter-25-S FM5: the ruling changes data, so its action is a button, not a link.
        d.observe(
            "iter-25-S-FM5",
            d.nodes()
                .iter()
                .find(|node| node.name == "Mark not at full")
                .is_some_and(|node| !node.is_link()),
            "Mark not at full is a quiet button",
        );
        let where_top = |d: &Driver<'_>| d.place_of("Where").map(|rect| rect.top());
        let where_before = [SMALL, DEFAULT, LARGE].map(|size| d.at_size(size, where_top));
        d.observe_both(
            "iter-18-B-S1",
            "The untitled rest header says Long rest once",
            |d| {
                let nodes = d.nodes();
                let left = nodes
                    .iter()
                    .find(|node| {
                        node.name == "Title" && node.role == egui::accesskit::Role::TextInput
                    })
                    .map(|node| node.rect.left());
                let area = nodes
                    .iter()
                    .find(|node| node.name == "Copy link")
                    .zip(left)
                    .map(|(node, left)| {
                        egui::Rect::from_min_max(egui::pos2(left, 0.0), node.rect.right_bottom())
                    });
                area.is_some_and(|area| d.painted_in(area).matches("Long rest").count() == 1)
            },
        );
        d.click("Mark not at full");
        assert!(!crate::timeline::party_state(&d.app_mut().timeline.events, session).rested);
        // iter-25-S FM2: the ruling's result sits in the list column, so Where keeps its place.
        d.expect("Undo ruling");
        let where_after = [SMALL, DEFAULT, LARGE].map(|size| d.at_size(size, where_top));
        d.observe(
            "iter-25-S-FM2",
            where_before
                .iter()
                .zip(where_after)
                .all(|(before, after)| before.zip(after).is_some_and(|(a, b)| (a - b).abs() < 1.0)),
            format!("Where keeps its y after the ruling: {where_before:?} then {where_after:?}"),
        );
        // iter-25-S FM3: the ruling's result is a finished change, in the success tone.
        const RULING_OFF: &str = "The next fight starts from the HP the party has now.";
        d.observe(
            "iter-25-S-FM3-off",
            d.place_of(RULING_OFF)
                .is_some_and(|words| d.has_tone_bar(words, crate::design::colors().result.success)),
            "The Timeline ruling result has the success tone",
        );
        d.state_prepared("timeline-rest-ruling-off", Sizes::All, |d| {
            let row = d.nodes().iter().any(|node| {
                node.name.starts_with("Long rest ·") && node.name.contains("Not at full")
            });
            d.observe(
                "iter-3-C-ST1-timeline",
                row,
                "The Timeline rest row keeps its Not at full ruling in view",
            );
        });
        d.click("Mark everyone at full");
        assert!(crate::timeline::party_state(&d.app_mut().timeline.events, session).rested);
        d.observe(
            "iter-23-S-FL1-timeline-on",
            d.visible_text("Everyone starts the next fight at full."),
            "On Timeline, Mark everyone at full says the next fight starts at full",
        );
        d.expect_same("Only the long-rest status, its action and its message change; the wide page stays in place.");
        d.state("timeline-rest-ruling-on", Sizes::All);
        d.click("Undo ruling");
        assert!(!crate::timeline::party_state(&d.app_mut().timeline.events, session).rested);
        d.click("Mark everyone at full");
        d.click(NAV_SESSIONS);
        show_plan(d);
        d.click(&format!("More actions for {SECOND}"));
        d.click("Start a new run");
        d.observe(
            "T12.2",
            kestrel(d).contains("HP 41/41"),
            format!(
                "A rest added on Timeline starts the next new fight at full: {}",
                kestrel(d)
            ),
        );
        close_run(d);

        // T14.5: a restart keeps the session being played, and where a run goes back to.
        d.click("New session");
        d.type_text("Later");
        d.click("Create session");
        d.expect("Session 3: Later");
        go_timeline_choose_session(d, "Session 2: The Crossing");
        d.click(NAV_SESSIONS);
        d.click_starting("Session 2: The Crossing ·");
        show_plan(d);
        d.click(&format!("Start fight: {FIRST}"));
        d.expect(&format!("{RUN_HEADING}{FIRST}"));
        d.app_mut().sessions.flush();
        d.app_mut().timeline.save();
        d.settle();
        d.restart();
        d.expect(&format!("{RUN_HEADING}{FIRST}"));
        close_run(d);
        d.observe(
            "T14.5",
            d.page_title().as_deref() == Some("Session")
                && d.has_containing("Session 2: The Crossing"),
            "After a restart, Close run goes to the page the run began on, in the session chosen",
        );

        // Party says what it shows, and lets the DM compare.
        go_party(d);
        d.state("party-from-tonight", Sizes::All);
        d.observe(
            "T12.2",
            d.has_containing("from tonight’s fights"),
            "Party says its HP is from tonight",
        );
        d.observe(
            "T12.0",
            d.has_containing("Per 14") && d.has_containing("Ins 11"),
            "Passive Perception and Insight compare without opening a sheet",
        );

        // The party stays in view above the Session page while the session is played.
        d.click(NAV_SESSIONS);
        if d.has("Show party") {
            d.click("Show party");
        }
        d.state_prepared("session-party-bar", Sizes::All, |d| {
            // FT-1: each item names its player before HP. Stats give way first, together.
            let (items, painted) = session_recording::party_bar_items(d);
            session_recording::check_party_names(d, "iter-10-C-FT1", &items, &painted);
            let stats = |words: &str| {
                ["AC", "Per", "Ins"]
                    .into_iter()
                    .filter(|label| words.split_whitespace().any(|word| word == *label))
                    .count()
            };
            d.observe(
                "iter-10-C-FT1-stats",
                items.len() >= 4
                    && painted
                        .windows(2)
                        .all(|pair| stats(&pair[0].1) == stats(&pair[1].1))
                    && items
                        .iter()
                        .all(|item| item.name.contains("HP ") && item.name.contains("AC ")),
                format!("All items show the same stats, and hover keeps every fact: {painted:?}"),
            );
        });
        d.observe(
            "T12.0",
            d.nodes()
                .iter()
                .any(|node| node.name.contains("Passive Perception") && node.name.contains("HP ")),
            "The Session page keeps each player's HP and stats in view, with the rest on hover",
        );
        let member = d
            .nodes()
            .into_iter()
            .find(|node| node.name.contains("Passive Perception"))
            .map(|node| node.name)
            .expect("a party bar item");
        d.state_with_tooltip("session-party-bar-hover", &member);

        // Keep the original independent paused copy for the existing duplicate-run checks.
        std::fs::write(&paused.path, paused_bytes).unwrap();
        let campaign = d.app_mut().campaigns.active_campaign().unwrap();
        d.app_mut().encounters.read_runs_again(&campaign);

        // What is unfinished can go on or go away. What is over is in the history.
        go_encounters(d);
        d.observe_both(
            "iter-2-C-FT4",
            "Duplicate paused runs show one save time",
            |d| {
                let nodes = d.nodes();
                let duplicates: Vec<_> = nodes
                    .iter()
                    .filter(|node| {
                        node.name.contains(" · Setup · Paused") && node.name.contains("2026-01-01")
                    })
                    .collect();
                !duplicates.is_empty()
                    && duplicates.iter().all(|node| !node.name.contains("Saved "))
            },
        );
        // iter-23-S FL3: a run that began after a restoring rest names the rest's step, in the
        // same lower-case words as every other `Last:` value.
        d.observe(
            "iter-23-S-FL3",
            d.has_containing("Last: full HP after long rest")
                && !d.has_containing("carried over from the long rest")
                && !d.has_containing("Last: Party"),
            "Last: after a long rest says full HP, in lower case",
        );
        // iter-25-S FK8: the run closed last says it in the DM's words.
        d.observe(
            "iter-25-S-FK8",
            d.nodes()
                .iter()
                .any(|node| node.name.contains("Played last"))
                && !d.has_containing("Closed last"),
            "The saved run's second line says Played last",
        );
        d.state("saved-runs-history", Sizes::All);
        d.observe(
            "T14.1",
            d.has(RESUME) && d.has_containing(HISTORY),
            "An interrupted run can be resumed, and ended runs are in the history",
        );
        let delete = format!("{SECOND} · Setup · Paused");
        let before = d.has_containing(&delete);
        delete_saved_run(d, SECOND);
        d.expect("Undo delete run");
        d.state("run-removed", Sizes::Default);
        d.click("Undo delete run");
        d.observe(
            "T14.1",
            before && d.has_containing(&delete),
            "A deleted run has one step of undo",
        );

        // T12.0: an encounter, whole, with no trip to the Library.
        d.click(NEW_ENCOUNTER);
        d.type_text("Ford of Ash");
        d.expect("5. ");
        d.press(Key::Enter);
        d.type_text("werewolf");
        d.click_first(ADD);
        d.expect("6. Werewolf");
        d.begin_task();
        d.click_first("More actions for Werewolf");
        d.state("roster-row-menu", Sizes::Standard);
        // FT2: a menu choice's reason and help start at its words, also after an icon.
        for size in [SMALL, DEFAULT] {
            let notes = d.at_size(size, |d| {
                [
                    ("Move down", "Already last."),
                    ("Make it yours", "Edit a copy in this campaign."),
                ]
                .map(|(choice, note)| note_starts_at_choice_words(d, choice, note))
            });
            d.observe(
                "iter-21-B-FT2",
                notes.iter().all(Result::is_ok),
                format!("At {size:?}, the roster row menu's notes: {notes:?}"),
            );
        }
        d.click("Make it yours");
        d.expect("6. Werewolf copy");
        d.state("make-it-yours", Sizes::All);
        // AK2, FK3: the Name field and Save creature end at one edge, at every size.
        for size in [SMALL, DEFAULT, LARGE] {
            let (name, save) = d.at_size(size, |d| {
                let field = d
                    .nodes()
                    .into_iter()
                    .find(|node| {
                        node.name == "Name" && node.role == egui::accesskit::Role::TextInput
                    })
                    .map(|node| node.rect);
                (field, d.place_of("Save creature"))
            });
            d.observe(
                "iter-25-B-AK2",
                name.zip(save)
                    .is_some_and(|(name, save)| (name.right() - save.right()).abs() <= 0.5),
                format!("At {size:?}: Name {name:?}, Save creature {save:?}"),
            );
        }
        d.observe(
            "T12.0",
            d.has("Save creature") && d.has_containing("Hit points"),
            "A creature that names itself copies, swaps in and opens beside the roster",
        );
        d.type_text("90");
        d.press_ctrl(Key::S);
        d.expect(FIND_CREATURE);
        d.goal_within("T12.0", "make a creature your own", 7);
        d.observe(
            "T12.0",
            d.has_containing("HP 90"),
            "The roster shows the new hit points",
        );
        d.type_into("Read-aloud and notes", "Ash falls on the ford.");
        d.state("read-aloud-field", Sizes::All);
        d.click(SAVE_ENCOUNTER);
        d.expect("Saved");
        d.click("Back to encounters");

        // A save on Encounters reaches the Library, so its next save is not a false conflict.
        go_library(d);
        d.click("Section");
        d.click("Library section: Encounters");
        d.click_containing("Ford of Ash");
        d.click(EDIT_MARKDOWN);
        d.type_into_editor("More tactics.\n");
        d.press_ctrl(Key::S);
        d.observe(
            "T12.0",
            !d.has("Overwrite with my draft"),
            "The Library holds the encounter's latest file",
        );
        d.begin_task();
    }
}

/// A named NPC joins a prepared encounter through its stat block, in one Add.
pub mod npc_in_encounter {
    use super::*;
    pub fn run(d: &mut Driver<'_>) {
        let campaign = d.app_mut().campaigns.active_campaign().unwrap();
        let catalogue = crate::content_workspace::Catalogue::load(&[], Some(&campaign)).unwrap();
        let guard = catalogue
            .entries
            .values()
            .find(|entry| entry.entry.kind == "creature" && entry.entry.name == "Guard")
            .unwrap()
            .entry
            .id;
        let text = splat::content::formats::fixture("Rook.md", &format!("splat_id: {}\nsplat_kind: world\nsplat_name: Rook Ambry\nsplat_world:\n  type: npc\n  creature: {{id: {guard}}}\n", crate::hermetic::new_uuid()), "").unwrap();
        std::fs::create_dir_all(campaign.path.join("NPCs")).unwrap();
        crate::library::write_note(&campaign.path, "NPCs/Rook Ambry.md", None, &text).unwrap();
        go_encounters(d);
        d.click("Reload from disk");
        d.wait_for_text(NEW_ENCOUNTER);
        d.begin_task();
        let started = d.interaction_count();
        d.click(NEW_ENCOUNTER);
        d.type_text("Rook's ambush");
        d.type_into(FIND_CREATURE, "Rook Ambry");
        d.wait_for_text("Rook Ambry");
        d.state("npc-result", Sizes::All);
        d.click_beside_exact("Rook Ambry", "Add");
        d.observe(
            "NPC encounter",
            d.has_containing("6. Rook Ambry"),
            "One Add uses Guard with the NPC's name",
        );
        d.state("npc-in-roster", Sizes::All);
        d.observe_both(
            "P2",
            "A named NPC already on the roster cannot be added again from search",
            |d| d.visible_text("In roster") && !d.has("Add"),
        );
        d.click(SAVE_ENCOUNTER);
        d.wait_for_text("Saved");
        d.click(RUN_ENCOUNTER);
        d.expect("Initiative for Rook Ambry");
        d.state("npc-in-setup", Sizes::All);
        d.observe_both("P5", "Setup names the NPC's creature once", |d| {
            d.painted_in(egui::Rect::EVERYTHING)
                .matches("as\u{a0}Guard")
                .count()
                == 1
        });
        let cost = d.interaction_count() - started;
        d.observe("W6", cost <= 8, format!("Prepare an encounter with a named NPC and start setup: {cost} interactions, target 8"));
        fight_tab(d, "Prep");
        d.expect("No prep notes yet");
        d.expect("Edit encounter");
        for size in [SMALL, DEFAULT, (1920.0, 1080.0)] {
            let aligned = d.at_size(size, |d| {
                let title = d.painted_left("No prep notes yet").unwrap();
                let action = d.painted_left("Edit encounter").unwrap();
                (title - action).abs() < 1.0 && d.in_view("Edit encounter")
            });
            d.observe(
                "iter6-B-FD12",
                aligned,
                format!("Prep action text aligns with its title at {size:?}"),
            );
        }
        // The action's press area reaches into the panel's gutter, never into its splitter.
        for size in [SMALL, DEFAULT, LARGE] {
            let (action, splitter) = d.at_size(size, |d| {
                (
                    d.place_of("Edit encounter"),
                    d.place_of("Resize fight panel column"),
                )
            });
            d.observe(
                "iter-17-B-splitter",
                action
                    .zip(splitter)
                    .is_some_and(|(action, splitter)| action.left() >= splitter.right()),
                format!(
                    "At {size:?}, Edit encounter's press area {action:?} starts right of the splitter {splitter:?}"
                ),
            );
        }
        assert!(d.in_view("Edit encounter"));
        d.click("Edit encounter");
        d.expect("Back to run");
        d.expect("Rook's ambush");
        d.state("edit-encounter-from-prep", Sizes::All);
        // iter-24-B AB1, FB1: the step Prep names works. Notes saved in the encounter reach this
        // open fight's Prep, and the fight keeps its setup (the NPC's initiative field).
        const NOTES: &str = "Rook waits under the bridge.";
        d.type_into("Read-aloud and notes", NOTES);
        d.click(SAVE_ENCOUNTER);
        d.wait_for_text("Saved");
        d.click("Back to run");
        d.expect("Initiative for Rook Ambry");
        fight_tab(d, "Prep");
        d.expect(NOTES);
        d.observe(
            "iter-24-B-AB1",
            d.visible_text(NOTES)
                && !d.visible_text("No prep notes yet")
                && d.has("Initiative for Rook Ambry"),
            "Saved notes show in the open fight's Prep, and its setup stays",
        );
        d.state("prep-after-encounter-save", Sizes::Default);
    }
}

/// New package content reaches the same campaign's encounter without selecting it again.
pub mod package_to_table {
    use super::*;
    pub fn run(d: &mut Driver<'_>) {
        go_encounters(d);
        d.click(NEW_ENCOUNTER);
        go_library(d);
        d.click("Packages");
        d.click("Manage packages…");
        d.begin_task();
        d.type_text("Copper Coast");
        d.expect("Choose for this campaign");
        d.state("create-package-location", Sizes::All);
        d.click("Create package");
        d.expect("Copper Coast");
        d.type_text("Copper Drake");
        d.expect("Choose for this campaign");
        d.state("new-entry-used-by-default", Sizes::All);
        d.click("Create entry");
        d.expect("Copper Drake");
        d.state("created-entry", Sizes::Default);
        go_encounters(d);
        d.type_into(FIND_CREATURE, "Copper Drake");
        d.expect("Copper Drake");
        d.click("Add");
        d.audit_goal_within(
            "W18a",
            "create a package and entry and add it to the open encounter",
            8,
        );
        d.state("new-package-at-the-table", Sizes::All);
        // iter-24-B FC1, must stay true: a changed draft Save would write still says Unsaved,
        // and its Save is not blocked.
        d.observe(
            "iter-24-B-FC1-ready",
            d.visible_text("Unsaved")
                && !d.visible_text("Invalid")
                && !d.is_disabled(SAVE_ENCOUNTER),
            "A draft that Save accepts says Unsaved, with Save open",
        );
        // FD1: one fault, one tone. The search result and the roster row paint `HP not set`,
        // icon and words, in the warning colour and nothing else.
        for size in [SMALL, DEFAULT, LARGE] {
            let colours = d.at_size(size, hp_not_set_colours);
            d.observe(
                "iter-21-B-FD1",
                colours.len() >= 2
                    && colours.iter().all(|colours| {
                        !colours.is_empty()
                            && colours
                                .iter()
                                .all(|colour| *colour == crate::design::colors().result.warning)
                    }),
                format!("At {size:?}, HP not set is painted in {colours:?}"),
            );
        }
        go_library(d);
        let text = d.editor_text().expect("new creature editor");
        let edited = text
            .replace("| Hit Points |  |", "| Hit Points | 22 |")
            .replace("| Armor Class |  |", "| Armor Class | 15 |")
            .replace("| Str | 10 | +0 |", "| Str | 18 | +4 |")
            .replace("| Dex | 10 | +0 |", "| Dex | 14 | +2 |");
        assert_ne!(text, edited);
        d.replace_editor_text(&edited);
        d.click("Save");
        d.click("Read");
        d.expect("ARMOR CLASS");
        d.expect("HIT POINTS");
        assert!(
            d.visible_text("STR"),
            "the creature's ability table is painted"
        );
        d.state("new-creature-read", Sizes::All);
        d.click("New entry");
        d.type_into("Entry name", "Copper Spark");
        d.click("Kind of entry");
        d.click("Spell");
        d.click("Create entry");
        let spell = d.editor_text().expect("new spell table");
        assert!(spell.contains("| Field | Value |"));
        assert!(spell.contains("| Level | 1 |"));
        d.state("new-spell-table-editor", Sizes::All);
        d.click("Read");
        d.expect("Describe the spell.");
        d.state("new-spell-table-reader", Sizes::All);
        go_encounters(d);
        // FD1: the fault is the creature's data, never its words. Copper Drake now has hit points;
        // with its package named like the fault, nothing on screen takes the warning colour.
        let package = d
            .app_mut()
            .campaigns
            .modules()
            .iter()
            .find(|module| module.name == "Copper Coast")
            .map(|module| module.path.clone())
            .expect("the new package");
        let (entries, _) = splat::content::read_directory(&package).unwrap();
        let manifest = package.join(
            &entries
                .iter()
                .find(|entry| entry.kind == "package")
                .expect("the package manifest")
                .path,
        );
        let original = std::fs::read_to_string(&manifest).unwrap();
        let rename = |d: &mut Driver<'_>, text: &str| {
            std::fs::write(&manifest, text).unwrap();
            let app = d.app_mut();
            app.library.content_changed =
                Some(app.campaigns.active_campaign().unwrap().path.clone());
            go_library(d);
            go_encounters(d);
        };
        let fault = format!("{} HP not set", crate::design::icons::WARN);
        rename(
            d,
            &crate::content_workspace::patch_fields(
                &original,
                &[("splat_name", Some(fault.clone().into()))],
            )
            .unwrap(),
        );
        // The search result paints the package with plain spaces once the reload arrives.
        d.wait_for_text(&fault);
        let mut coast = Vec::new();
        d.each_painted_text(|text, clip| {
            if text.galley.job.text.contains("Coast")
                && clip.intersects(text.visual_bounding_rect())
            {
                coast.push((text.galley.job.text.clone(), text.pos));
            }
        });
        assert!(
            coast.is_empty(),
            "the roster and search show the renamed package: {coast:?}"
        );
        for size in [SMALL, DEFAULT, LARGE] {
            let colours = d.at_size(size, hp_not_set_colours);
            d.observe(
                "iter-21-B-FD1-data",
                !colours.is_empty()
                    && colours.iter().all(|colours| {
                        !colours.is_empty()
                            && colours
                                .iter()
                                .all(|colour| *colour == crate::design::colors().muted)
                    }),
                format!(
                    "At {size:?}, a healthy creature from a package named {fault:?} paints it in {colours:?}"
                ),
            );
        }
        rename(d, &original);
        d.wait_for_text("Copper Coast");
        d.replace_text_in("Encounter name", "Copper Coast patrol");
        d.click("Start fight");
        d.click_starting("Copper Drake · ");
        d.click("Stat block");
        assert!(
            d.visible_text("STR"),
            "the creature's ability table is painted"
        );
        d.state("authored-creature-beside-run", Sizes::All);
    }
}

/// A private required dependency blocks sharing; an independent copy can use a new folder.
pub mod share_to_package {
    use super::*;
    pub fn run(d: &mut Driver<'_>) {
        let root = d.root.join("campaigns/emberfall");
        let (entries, _) = splat::content::read_directory(&root).unwrap();
        let varra = entries
            .iter()
            .find(|entry| entry.name == "Captain Varra")
            .unwrap();
        let text = crate::library::note_text("Watch Orders", "rule", None);
        let text = splat::content::formats::patch_model(
            &text,
            &[(
                "splat_requires",
                Some(
                    serde_yaml_ng::to_value(vec![crate::content_workspace::EntryRef::id(varra.id)])
                        .unwrap(),
                ),
            )],
        )
        .unwrap();
        std::fs::write(root.join("Watch Orders.md"), text).unwrap();
        go_library(d);
        d.open_combo_at(LIBRARY_PACKAGE_DROPDOWN);
        d.click("Campaign entries");
        d.type_into(SEARCH_LIBRARY, "Watch Orders");
        d.click_starting("Watch Orders · ");
        d.click(COPY_TO);
        d.click("Destination");
        d.click(&format!("Package · {MODULE}"));
        d.wait_for_text("Ashen Marches/Captain Varra.md");
        d.expect("Also copies Captain Varra, because Watch Orders links to it.");
        assert!(!d.visible_text("Required entries: 1"));
        d.state("private-dependency-shared-together", Sizes::All);
        d.wait_for_text("Ashen Marches/Watch Orders.md");
        d.type_into("Folder", "Watch/Captains");
        d.wait_for_text("Watch/Captains/Watch Orders.md");
        d.click("Review Markdown");
        d.state("share-to-new-folder", Sizes::All);
        d.click("Copy both");
        d.wait_for_text(
            "Copied Watch Orders and Captain Varra to Ashen Marches, in Watch/Captains.",
        );
        let package = d.root.join("modules/ashen-marches");
        let (entries, errors) = splat::content::read_directory(&package).unwrap();
        assert!(errors.is_empty());
        let orders = entries.iter().find(|e| e.name == "Watch Orders").unwrap();
        let captain = entries.iter().find(|e| e.name == "Captain Varra").unwrap();
        d.observe(
            "W1",
            splat::content::required_references(orders).unwrap()
                == vec![splat::content::Reference::Id(captain.id)]
                && captain.id != varra.id,
            "Copy both copies the dependency and reconnects the new entry",
        );
        d.state("shared-copy-created", Sizes::All);
        d.click("Close");
        d.open_combo_at(LIBRARY_PACKAGE_DROPDOWN);
        d.click(MODULE);
        d.clear_text_in(SEARCH_LIBRARY);
        d.type_into(SEARCH_LIBRARY, "Watch Orders");
        d.click_starting("Watch Orders · ");
        d.wait_for_text("Watch Orders");
        d.observe(
            "W1",
            !d.visible_text("is in this campaign only"),
            "The shared entry has no campaign dependency warning",
        );
        d.state("shared-entry-ready", Sizes::All);
    }
}

pub mod remove_package_in_use {
    use super::*;
    pub fn run(d: &mut Driver<'_>) {
        let package = d.app_mut().campaigns.modules()[0].id;
        go_library(d);
        d.click("Packages");
        d.click("Manage packages…");
        d.click(&format!("More actions for {MODULE}"));
        d.click(&format!("Unregister {MODULE}"));
        d.expect("Unregister Ashen Marches?");
        // Round 24 FC5, must stay true: an existing caller of the shared dialog keeps its place
        // at the top anchor, where Remove campaign now sits too.
        d.observe_both(
            "iter-24-C-FC5-unregister",
            "The Unregister dialog's title stays at the shared top anchor",
            |d| {
                d.painted_rect_in("Unregister Ashen Marches?", egui::Rect::EVERYTHING)
                    .is_some_and(|title| {
                        (title.top() - (crate::design::XXL + crate::design::L)).abs() <= 2.0
                    })
            },
        );
        d.observe_both(
            "FD-12",
            "Unregister stays left of Keep the package on one line",
            |d| {
                let nodes = d.nodes();
                let rect = |name: &str| {
                    nodes
                        .iter()
                        .find(|node| {
                            node.name == name && node.role == egui::accesskit::Role::Button
                        })
                        .unwrap()
                        .rect
                };
                let (remove, keep) = (rect("Unregister"), rect("Keep the package"));
                remove.right() < keep.left() && (remove.center().y - keep.center().y).abs() < 1.0
            },
        );
        d.state("package-in-use-warning", Sizes::All);
        // Round 25 FG4: the scope, the search and the list keep their place when the result
        // comes and when Undo takes it away, at every size: the result sits in the reader's place.
        let places = |d: &mut Driver<'_>| {
            [SMALL, DEFAULT, LARGE].map(|size| {
                d.at_size(size, |d| {
                    let scope = d
                        .nodes()
                        .into_iter()
                        .find(|node| node.name.starts_with("What the Library shows:"))?;
                    Some((d.place_of(SEARCH_LIBRARY)?.top(), scope.rect.top()))
                })
            })
        };
        let before = places(d);
        d.click("Unregister");
        d.observe(
            "W18c",
            d.root.join("modules/ashen-marches/Module.md").is_file(),
            "Unregistering keeps the package files",
        );
        d.expect("Undo unregister");
        let with_result = places(d);
        // Round 25 FG2: a finished change is a success message.
        let words = d.painted_rect_in("Ashen Marches is unregistered.", egui::Rect::EVERYTHING);
        d.observe(
            "iter-25-L-FG2",
            words
                .is_some_and(|words| d.has_tone_bar(words, crate::design::colors().result.success)),
            format!("The unregister result has the success bar: {words:?}"),
        );
        d.state("package-unregistered-with-undo", Sizes::All);
        d.click("Undo unregister");
        d.expect(&format!("More actions for {MODULE}"));
        let registered = d
            .app_mut()
            .campaigns
            .modules()
            .iter()
            .any(|module| module.id == package);
        let undone = places(d);
        d.observe(
            "iter-25-L-FG4",
            before.iter().all(Option::is_some)
                && with_result == before
                && undone == before
                && registered
                && !d.has_containing("is unregistered."),
            format!(
                "Search and list tops before {before:?}, with the result {with_result:?}, after Undo {undone:?}; registered again: {registered}"
            ),
        );
        // Unregister again, for the Campaign steps below.
        d.click(&format!("More actions for {MODULE}"));
        d.click(&format!("Unregister {MODULE}"));
        d.click("Unregister");
        d.expect("Undo unregister");
        go_campaign(d);
        d.click("Reload from disk");
        d.expect("Ashen Marches is not registered.");
        d.expect("Register it again");
        d.expect("Remove from this campaign");
        d.expect("Find the folder…");
        d.observe(
            "Missing package name",
            !d.visible_text(&package.to_string()),
            "The missing package keeps its name; identifiers stay in Technical details",
        );
        d.state_prepared(
            "campaign-keeps-missing-package-choice",
            Sizes::All,
            check_link_spaces,
        );
        d.click("Technical details");
        d.expect(&package.to_string());
        d.state("missing-package-technical-details", Sizes::Default);
        d.click("Technical details");
        d.click("Remove from this campaign");
        d.wait_for_text("now have creatures Splat cannot find.");
        d.expect("Bridge Ambush");
        d.expect("Shrine of Cinders");
        d.state_prepared("missing-package-removed-with-undo", Sizes::All, |d| {
            check_link_spaces(d);
            check_undo_icon(d, "Undo package removal", "iter-6-C-FD3-Undo");
            let first = d
                .place_of("Bridge Ambush")
                .expect("First affected encounter link");
            d.observe(
                "iter-6-C-FD3",
                d.has_message_bar(first, crate::design::colors().result.warning),
                "The linked missing-creatures warning keeps the shared warning bar and icon",
            );
            // Round 23 FF4: the warning icon centres on the first line, whose words are the
            // links'. A link's press area is centred on its words.
            let band = egui::Rect::from_min_max(
                egui::pos2(first.left() - crate::design::XXL, first.top() - crate::design::S),
                egui::pos2(first.left(), first.bottom() + crate::design::S),
            );
            let icon = d.painted_word_rect(crate::design::icons::WARN, band);
            d.observe(
                "iter-23-L-FF4",
                icon.is_some_and(|icon| (icon.center().y - first.center().y).abs() <= 1.0),
                format!(
                    "The warning icon {icon:?} centres on the first line of the message, beside {first:?}"
                ),
            );
        });
        // Round 25 review: the removal's words are prepared once, at the removal. Unchanged
        // frames at two sizes prepare nothing.
        let prepared = d.app_mut().campaign_content.removal_text_preparations();
        d.at_size(SMALL, |_| ());
        d.at_size(LARGE, |_| ());
        d.settle();
        let unchanged = d.app_mut().campaign_content.removal_text_preparations();
        d.observe(
            "iter-25-C-removal-words-once",
            prepared == 1 && unchanged == prepared,
            format!("Removal words prepared {prepared} times at the removal, {unchanged} after unchanged frames"),
        );
        d.click("Undo package removal");
        d.wait_for_text("Find the folder…");
        d.expect_same("Undo restores the missing-package choice shown before removal.");
        d.assert_state_matches(
            "package-removal-undone",
            "campaign-keeps-missing-package-choice",
        );
        d.click("Remove from this campaign");
        let campaign = d.app_mut().campaigns.active_campaign().unwrap();
        let document = crate::content_workspace::CampaignDocument::load(&campaign).unwrap();
        d.observe(
            "Remove unavailable package",
            !document
                .selections
                .iter()
                .any(|selection| selection.package.id == Some(package)),
            "Removing the unavailable package saves the other campaign choices",
        );
        go_encounters(d);
        resume_run_from_cold_start(d);
        d.wait_for_text(NEXT_TURN);
        d.expect(
            "This stat block uses the copy from the start of the fight. Ashen Marches is not used in this campaign now.",
        );
        d.observe(
            "i19 removed package",
            !d.visible_text("was deleted"),
            "A package removal does not say its creature file was deleted",
        );
        d.state("paused-run-after-package-removal", Sizes::All);
    }
}

/// T16: rolls as in a tactics game. A tile in the stat block arms an action; each row shows
/// the chance to hit it; a click on a row rolls against it and puts the damage in Amount, and
/// Enter applies it. A save adds each clicked row and rolls its save. A player's attack only
/// targets, and the DM types the player's damage. Outside a run, a tile previews.
pub mod run_rolls {
    use super::super::driver::Found;
    use super::*;
    use egui::accesskit::Role;
    const ATTACKER: &str = "Cinder Wolf 1";
    const TARGET: &str = "Cinder Wolf 2";
    const CASTER: &str = "Ember Cultist";
    const PLAYER: &str = "Iselle Marrowind";
    const HERO: &str = "Brannoch Vale";
    const BITE: &str = "Bite · +4";
    const BURST: &str = "Ember Burst · DC 12";
    const ROWS: [&str; 8] = [
        "Brannoch Vale",
        ATTACKER,
        TARGET,
        PLAYER,
        "Kestrel Dunmoor",
        "Oren Thistlewick",
        "Sable Harrowgate",
        CASTER,
    ];

    /// Each row's height and its AC label's place. Roll words must change neither.
    fn row_geometry(d: &Driver<'_>) -> Vec<Option<(f32, egui::Rect)>> {
        ROWS.iter()
            .map(|row| {
                let node = d.row_node(row)?;
                let ac = d.painted_word_rect("AC", node.shown)?;
                Some((node.rect.height(), ac))
            })
            .collect()
    }

    /// iter-20-R-FT4: a row's roll words have one place. They start on the line under the
    /// name, left of the AC column (never the state column), after the condition line when
    /// the row has one (or on their own line under it when the result cannot fit whole
    /// there), else at the name's edge. They are whole or cut by whole words, and
    /// the row's accessible name holds them whole. `start` is how they begin. What is wrong,
    /// or `None`.
    fn roll_words_fault(
        d: &Driver<'_>,
        row: &str,
        start: &str,
        condition: Option<&str>,
    ) -> Option<String> {
        let Some(node) = d.row_node(row) else {
            return Some(format!("{row}: no row"));
        };
        let Some((_, full)) = node
            .name
            .rsplit_once(", ")
            .filter(|(_, words)| words.contains(start))
        else {
            return Some(format!("{row}: its name {:?} has no {start:?}", node.name));
        };
        let first = full.split(' ').next().unwrap_or(full);
        let (Some(words), Some(name), Some(ac)) = (
            d.painted_rect_in(first, node.shown),
            d.painted_rect_in(row, node.shown),
            d.painted_word_rect("AC", node.shown)
                .or_else(|| d.painted_word_rect("DEX", node.shown)),
        ) else {
            return Some(format!("{row}: its words, name or AC are not painted"));
        };
        let shown = d.painted_in(words.expand(1.0)).replace('\u{a0}', " ");
        let kept = shown.trim_end_matches('…');
        let whole_words = shown == full
            || (shown.ends_with('…')
                && full.starts_with(kept)
                && full[kept.len()..].starts_with(' '));
        let after = match condition {
            // iter-25-R SJ1: words whose result cannot fit whole after the conditions take a
            // line of their own under them, at the name's edge.
            Some(condition) => d
                .painted_rect_in(condition, node.shown)
                .is_some_and(|line| {
                    (words.left() >= line.right() && (words.top() - line.top()).abs() < 2.0)
                        || ((words.left() - name.left()).abs() < 1.0
                            && words.top() >= line.bottom() - 1.0)
                }),
            None => (words.left() - name.left()).abs() < 1.0,
        };
        let wrong = [
            // Under the name's last line: a name's galley holds its line spacing below.
            (
                words.top() >= name.top() + name.height() * 0.75,
                "under the name",
            ),
            (words.left() < ac.left(), "in the name column"),
            (after, "after the conditions, or at the name's edge"),
            (whole_words, "whole or cut by whole words"),
        ]
        .into_iter()
        .filter(|(holds, _)| !holds)
        .map(|(_, what)| what)
        .collect::<Vec<_>>();
        (!wrong.is_empty()).then(|| {
            format!("{row}: {shown:?} at {words:?} (name {name:?}, AC {ac:?}) is not {wrong:?}")
        })
    }

    /// `roll_words_fault` for each row, at 960, 1200 and 1920.
    fn roll_words_faults(d: &mut Driver<'_>, rows: &[(&str, &str, Option<&str>)]) -> Vec<String> {
        let mut faults = Vec::new();
        for size in [SMALL, DEFAULT, LARGE] {
            faults.extend(d.at_size(size, |d| {
                rows.iter()
                    .filter_map(|(row, start, condition)| {
                        roll_words_fault(d, row, start, *condition)
                    })
                    .map(|fault| format!("{}x{}: {fault}", size.0, size.1))
                    .collect::<Vec<_>>()
            }));
        }
        faults
    }

    /// A row's result from its accessible name: whether it saved, and its damage.
    fn save_said(d: &Driver<'_>, row: &str) -> Option<(bool, i64)> {
        let name = d.row_node(row)?.name;
        let (saved, rest) = ["Saved · ", "Failed · "]
            .into_iter()
            .find_map(|word| Some((word == "Saved · ", name.split_once(word)?.1.to_owned())))?;
        Some((saved, rest.split(' ').next()?.parse().ok()?))
    }

    /// A tile's control, by the start of its name.
    fn tile(d: &Driver<'_>, start: &str) -> Option<Found> {
        d.nodes()
            .into_iter()
            .find(|node| node.name.starts_with(start) && node.role == Role::Button)
    }

    /// A roll row's first quiet action starts its words at the tiles' left edge, the panel's
    /// text edge (FT4, AT5).
    fn action_at_text(d: &mut Driver<'_>, id: &str, tile_name: &str, action: &str) {
        let tile_left = tile(d, tile_name).map(|tile| tile.rect.left());
        let words = d.painted_left(action);
        d.observe(
            id,
            tile_left.zip(words).is_some_and(|(tile, words)| (tile - words).abs() < 1.0),
            format!("`{action}` starts its words at the tiles' edge: tile {tile_left:?}, words {words:?}"),
        );
    }

    pub fn run(d: &mut Driver<'_>) {
        resume_run_from_cold_start(d);
        d.click_row(ATTACKER);
        d.observe_both(
            "iter-19-R-ST3",
            "A tile shows where its action lands, on its own line",
            |d| d.visible_text("Reach 5 ft"),
        );
        // Keyboard focus reads a tile's rules as hover does, and its description holds them.
        let bite = tile(d, BITE).map(|tile| tile.name).unwrap_or_default();
        let focused = d.tab_to(&bite);
        d.observe(
            "iter-19-R-ST3",
            focused && d.visible_text("Melee Attack Roll: +4, reach 5 ft."),
            format!("Tab reaches the Bite tile ({focused}) and shows its rules text"),
        );
        let target = tile(d, BITE).map(|tile| tile.rect.size());
        d.observe(
            "iter-19-R-ST3",
            target.is_some_and(|size| size.x >= 24.0 && size.y >= 24.0),
            format!("The focused tile stays a full target: {target:?}"),
        );
        d.click_row(ATTACKER);
        d.observe(
            "iter-19-R-ST3",
            !d.visible_text("Melee Attack Roll: +4, reach 5 ft."),
            "The rules text goes when the tile loses focus",
        );
        d.state("tiles-in-the-stat-block", Sizes::All);

        let geometry = row_geometry(d);
        d.begin_task();
        d.click_starting(BITE);
        d.observe(
            "T16.1",
            d.has_containing("Click a target") && d.has_containing("% to hit"),
            "An armed tile says what to do, and each row shows the chance to hit it",
        );
        d.goal_within("T16.1", "arm the selected creature's attack", 1);
        action_at_text(d, "iter-19-R-FT4", BITE, "Stop");
        // Must stay true (iter-20-R-FT4): roll words move no row's AC and change no height.
        let armed = row_geometry(d);
        d.observe(
            "iter-20-R-FT4",
            geometry.iter().all(Option::is_some) && armed == geometry,
            format!("Rows keep their height and AC place when roll words show: {geometry:?} then {armed:?}"),
        );
        d.state("attack-armed", Sizes::All);

        d.begin_task();
        let before = d.row_hp(TARGET);
        d.click_row(TARGET);
        d.observe(
            "T16.2",
            d.has_containing(" vs AC 13") && d.has_containing("Click a target"),
            "A click on a row rolls against it, and the action stays armed",
        );
        // The attack's total and the AC stay in view beside the damage, as a plain
        // comparison with no hit or miss verdict.
        d.observe_both(
            "iter-19-R-ST2",
            "The rolled row paints `<total> vs AC 13 · <damage> damage`, and no Hits or Misses",
            |d| {
                let shown = d.painted_in(egui::Rect::EVERYTHING);
                let number_before = |text: &str| {
                    let digits =
                        text.len() - text.trim_end_matches(|c: char| c.is_ascii_digit()).len();
                    digits > 0
                };
                let compared = shown.match_indices(" vs AC 13 · ").any(|(at, found)| {
                    let rest = &shown[at + found.len()..];
                    let digits =
                        rest.len() - rest.trim_start_matches(|c: char| c.is_ascii_digit()).len();
                    number_before(&shown[..at])
                        && digits > 0
                        && rest[digits..].starts_with(" damage")
                });
                compared && !shown.contains("Hits") && !shown.contains("Misses")
            },
        );
        d.state("attack-rolled", Sizes::All);
        d.press(Key::Enter);
        let after = d.row_hp(TARGET);
        d.observe(
            "T16.2",
            before != after,
            format!("Enter applies the rolled damage to the target: {before:?} then {after:?}"),
        );
        d.goal_within("T16.2", "roll against a target and apply the damage", 2);

        d.begin_task();
        d.click_row(TARGET);
        d.press(Key::Enter);
        d.goal_within("T16.3", "make a second attack and apply its damage", 2);

        d.press(Key::Escape);
        d.begin_task();
        d.press_ctrl(Key::Num1);
        d.observe(
            "T16.5",
            d.has_containing("Click a target"),
            "Ctrl+1 arms the first action, as a game's action bar does",
        );
        d.goal_within("T16.5", "arm an action by its number", 1);
        d.press(Key::Escape);

        // Right-click a tile: how its d20 rolls.
        d.click_row(ATTACKER);
        // iter-26-R FK4: Stop's y with the plain hint, to compare with advantage below.
        let stop_y = |d: &mut Driver<'_>| {
            [SMALL, DEFAULT, LARGE]
                .map(|size| d.at_size(size, |d| d.place_of("Stop").map(|rect| rect.top().round())))
        };
        d.click_starting(BITE);
        d.expect("Stop");
        let plain_stop = stop_y(d);
        d.press(Key::Escape);
        d.begin_task();
        d.right_click_starting(BITE);
        // iter-20-R-FT3: one roll menu. It opens under its tile, headed in muted Small with the
        // action's name, and its choices start at the heading's words.
        let tile_rect = tile(d, BITE).map(|tile| tile.rect);
        let below = tile_rect.map(|tile| {
            egui::Rect::from_min_max(
                egui::pos2(tile.left() - 48.0, tile.bottom()),
                egui::pos2(f32::INFINITY, f32::INFINITY),
            )
        });
        let heading = below.and_then(|below| d.painted_rect_in("Bite", below));
        let choice = d.painted_left("Roll with disadvantage");
        let first = below.and_then(|below| d.painted_rect_in("Roll with advantage", below));
        d.observe(
            "iter-20-R-FT3",
            tile_rect.zip(heading).zip(first).is_some_and(|((tile, heading), first)| {
                heading.top() >= tile.bottom() && first.top() >= heading.bottom() - 1.0
            }) && heading.zip(choice).is_some_and(|(heading, choice)| (heading.left() - choice).abs() < 1.0)
                && heading.is_some_and(|heading| {
                    d.painted_size(heading, "Bite") == Some(crate::design::Type::Small.size())
                }),
            format!("The tile menu opens under the tile with its heading: tile {tile_rect:?}, heading {heading:?}, choices at {choice:?}, first choice {first:?}"),
        );
        // Review repair 1: the tile menu stays inside the panel, clear of its splitter, at
        // every size: its heading and each choice start at least `S` right of the splitter.
        let mut faults = Vec::new();
        for size in [SMALL, DEFAULT, LARGE] {
            faults.extend(d.at_size(size, |d| {
                let splitter = d
                    .nodes()
                    .into_iter()
                    .find(|node| node.name == "Resize fight panel column")
                    .map(|node| node.rect);
                let tile = tile(d, BITE).map(|tile| tile.rect);
                let below = tile.map(|tile| {
                    egui::Rect::from_min_max(
                        egui::pos2(0.0, tile.bottom()),
                        egui::pos2(f32::INFINITY, f32::INFINITY),
                    )
                });
                let heading = below.and_then(|below| d.painted_rect_in("Bite", below));
                let choices: Vec<egui::Rect> = d
                    .nodes()
                    .into_iter()
                    .filter(|node| node.name.starts_with("Roll with"))
                    .filter(|node| below.is_some_and(|below| below.contains(node.rect.center())))
                    .map(|node| node.rect)
                    .collect();
                let clear = splitter.zip(heading).is_some_and(|(splitter, heading)| {
                    let edge = splitter.right() + crate::design::S;
                    choices.len() == 2
                        && heading.left() >= edge
                        && choices.iter().all(|choice| choice.left() >= edge)
                });
                (!clear).then(|| {
                    format!(
                        "{}x{}: splitter {splitter:?}, heading {heading:?}, choices {choices:?}",
                        size.0, size.1
                    )
                })
            }));
        }
        d.observe(
            "iter-20-R-review-1",
            faults.is_empty(),
            format!("The tile menu stays clear of the panel's splitter: {faults:?}"),
        );
        d.state("tile-menu", Sizes::Default);
        d.click("Roll with advantage");
        d.observe(
            "T16.8",
            d.has_containing("Click a target"),
            "A tile's right-click menu arms it with advantage",
        );
        d.goal_within("T16.8", "arm an attack with advantage from its menu", 2);

        // Right-click a target row: roll against that one row another way.
        d.begin_task();
        // The way the armed tile rolls is in words, in the hint and on each row.
        d.observe(
            "iter-19-R-FT2",
            d.visible_text("With advantage. Click a target")
                && d.visible_text("% to hit · advantage"),
            "An attack armed with advantage says so in its hint and on the rows",
        );
        // iter-26-R FK4: the hint keeps the lines of the longest hint, so Stop keeps its y
        // when the words grow with the roll mode.
        let advantage_stop = stop_y(d);
        d.observe(
            "iter-26-R-FK4",
            plain_stop.iter().all(Option::is_some) && plain_stop == advantage_stop,
            format!("Stop y plain {plain_stop:?}, with advantage {advantage_stop:?}"),
        );
        // iter-20-R-FT7: the wrapped hint keeps its crosshair on its first line.
        d.observe_both(
            "iter-20-R-FT7",
            "The armed hint's crosshair is centred on the hint's first line",
            |d| {
                let tile_bottom = tile(d, BITE).map_or(f32::INFINITY, |tile| tile.rect.bottom());
                let below = egui::Rect::from_min_max(
                    egui::pos2(0.0, tile_bottom),
                    egui::pos2(f32::INFINITY, f32::INFINITY),
                );
                let hint = d.painted_rect_in("With advantage. Click a target", below);
                let icon = d.painted_rect_in(crate::design::icons::ARMED, below);
                hint.zip(icon).is_some_and(|(hint, icon)| {
                    let first_line = hint.top() + crate::design::Type::Small.line() / 2.0;
                    hint.height() > crate::design::Type::Small.line()
                        && (icon.center().y - first_line).abs() <= 3.0
                })
            },
        );
        // iter-20-R-FT4: with advantage, the chance sits under the name, after `Conc`.
        let faults = roll_words_faults(
            d,
            &[
                ("Brannoch Vale", "% to hit · advantage", None),
                (TARGET, "% to hit · advantage", Some("Conc")),
                ("Kestrel Dunmoor", "% to hit · advantage", Some("Poisoned")),
            ],
        );
        d.observe(
            "iter-20-R-FT4",
            faults.is_empty(),
            format!("Each row's chance sits in its one place at every size: {faults:?}"),
        );
        d.right_click_row(TARGET);
        d.observe(
            "iter-19-R-FT4",
            d.visible_text("Bite against Cinder Wolf 2")
                && d.painted_left("Bite against Cinder Wolf 2")
                    == d.painted_left("Roll with disadvantage"),
            "The row menu's choices start at its heading's words",
        );
        d.state("row-menu", Sizes::Default);
        d.click("Roll with disadvantage");
        d.observe(
            "T16.9",
            d.has_containing(" vs AC 13"),
            "A row's right-click menu rolls against it with disadvantage",
        );
        d.goal_within("T16.9", "roll against one target with disadvantage", 2);
        d.press(Key::Escape);

        // Damage that depends on what only the DM knows: the click asks before it rolls.
        d.click_row(CASTER);
        d.begin_task();
        d.click_starting("Scorching Sickle");
        d.click_row(TARGET);
        d.observe(
            "T16.10",
            d.has_containing("The cultist moved 10+ feet")
                && !d.has_containing("If the cultist moved")
                && d.has_containing("No · 1d6+2 slashing"),
            "The question names the condition, with the damage for yes and for no",
        );
        d.observe_both(
            "iter-19-R-FT7",
            "A tile alone on its line keeps every damage type",
            |d| d.visible_text("or\u{a0}2d6+2\u{a0}slashing"),
        );
        // Two tiles do not fit beside each other in the panel, so each takes the line.
        d.observe_both(
            "iter-19-R-FT7",
            "A tile alone on its line takes the line's width",
            |d| {
                tile(d, "Scorching Sickle")
                    .zip(tile(d, BURST))
                    .is_some_and(|(sickle, burst)| {
                        sickle.rect.top() < burst.rect.top()
                            // Wider than the 208 px of a tile that shares its line.
                            && sickle.rect.width() > 208.0 + crate::design::S
                            && (sickle.rect.width() - burst.rect.width()).abs() < 1.0
                    })
            },
        );
        // FJ2: the question wraps, and its lines are left aligned, not justified.
        let mut question = None;
        d.each_painted_text(|text, _| {
            if text.galley.job.text.starts_with("The cultist moved") {
                question = Some((text.galley.job.justify, text.galley.rows.len()));
            }
        });
        d.observe(
            "iter-23-R-FJ2",
            question.is_some_and(|(justify, rows)| !justify && rows >= 2),
            format!("The damage question wraps left aligned (justified, rows): {question:?}"),
        );
        d.state("damage-question", Sizes::Default);
        d.click_starting("Yes · 2d6+2 slashing");
        d.observe(
            "T16.10",
            d.has_containing(" vs AC 13"),
            "The chosen damage rolls against the row",
        );
        d.goal_within("T16.10", "attack when its damage depends on a condition", 3);
        d.press(Key::Escape);

        // A tile's number is its place among all of the creature's actions. Spellcasting, which
        // is prose, parts the cultist's third tile from the first two, and takes no number.
        d.click_row(CASTER);
        d.observe(
            "T16.5",
            d.name_starting("Scorching Sickle").contains("Ctrl+1.")
                && d.name_starting(BURST).contains("Ctrl+2.")
                && d.name_starting("Cinder Dart").contains("Ctrl+3."),
            format!(
                "The tile after the Spellcasting text says Ctrl+3, not Ctrl+1 again: {:?}",
                d.name_starting("Cinder Dart")
            ),
        );
        d.press_ctrl(Key::Num3);
        d.observe(
            "T16.5",
            d.has_containing("Click a target to roll against it"),
            "Ctrl+3 arms the tile that says Ctrl+3, the attack after the prose",
        );
        d.press(Key::Escape);

        // A save: each clicked row is a target, and its save stands where its AC is.
        d.click_row(CASTER);
        d.begin_task();
        d.click_starting(BURST);
        d.click_row(ATTACKER);
        d.click_row(TARGET);
        d.observe(
            "T16.4",
            d.has_containing(", DEX save "),
            "Each creature's save shows on its row",
        );
        d.observe_both(
            "iter-19-R-AT2",
            "A save tile shows its area and what a success does",
            |d| d.visible_text("10-ft emanation · half on a success"),
        );
        action_at_text(d, "iter-19-R-AT5", BURST, "Roll damage");
        // iter-26-R FK5: the dock shows the mark that the rows paint, and leaves the count to
        // the panel's hint, so `2 targets.` shows once.
        d.observe_both(
            "iter-26-R-FK5",
            "The dock names the target's mark, and the count shows once",
            |d| {
                let hint = format!(
                    "Press a target’s {} to give it half, rounded down.",
                    crate::design::icons::OK
                );
                d.visible_text(&hint)
                    && d.painted_in(egui::Rect::EVERYTHING)
                        .matches("2 targets.")
                        .count()
                        == 1
            },
        );
        d.state("save-targets", Sizes::All);
        if d.has("Half for those at or above DC") {
            d.click("Half for those at or above DC");
        }
        d.click("Roll damage");
        // iter-20-R-FT4: each save result sits in the one place too, and a long result
        // (`Saved · 10 damage (half)`) is whole or cut by whole words at every size.
        let rows = [
            (ATTACKER, " damage", Some("Prone")),
            (TARGET, " damage", Some("Conc")),
        ];
        let faults = roll_words_faults(d, &rows);
        d.observe(
            "iter-20-R-FT4",
            faults.is_empty(),
            format!("Each save result sits in its one place at every size: {faults:?}"),
        );
        let rolled = [ATTACKER, TARGET].map(|row| save_said(d, row));
        let hp_before = [d.row_hp(ATTACKER), d.row_hp(TARGET)];
        d.press(Key::Enter);
        d.goal_within(
            "T16.4",
            "an area save against two creatures, then its damage",
            6,
        );
        // Each target says its own result, and its HP fell by that row's own amount.
        let hp_after = [d.row_hp(ATTACKER), d.row_hp(TARGET)];
        let said = [ATTACKER, TARGET].map(|row| {
            // The row's whole name; a narrow row can cut its painted words.
            let words = d.row_node(row).map(|node| node.name).unwrap_or_default();
            let saved = if words.contains("Saved · ") {
                Some(true)
            } else if words.contains("Failed · ") {
                Some(false)
            } else {
                None
            };
            let amount = ["Saved · ", "Failed · "]
                .into_iter()
                .find_map(|word| words.split_once(word))
                .and_then(|(_, rest)| rest.split(' ').next())
                .and_then(|amount| amount.parse::<i64>().ok());
            (saved, amount, words.contains("(half)"))
        });
        let lost = [0, 1].map(|at| hp_before[at].zip(hp_after[at]).map(|(was, is)| was - is));
        d.observe(
            "iter-19-R-FT1",
            said.iter().all(|(saved, amount, _)| saved.is_some() && amount.is_some())
                && said.iter().any(|(saved, _, half)| *saved == Some(true) && *half)
                && said.iter().any(|(saved, _, half)| *saved == Some(false) && !*half)
                && (0..2).all(|at| said[at].1.is_some_and(|amount| {
                    lost[at] == Some(amount.min(hp_before[at].unwrap_or(0)))
                })),
            format!("Each save target says its own result and lost that much: said {said:?}, lost {lost:?}"),
        );
        d.observe_both("iter-19-R-FT1", "The rows name each result in words", |d| {
            d.visible_text("Saved · ") && d.visible_text("Failed · ")
        });
        // iter-20-R-FT1: a finished save is finished. The tile disarms, and the rows show AC.
        let finished = |d: &Driver<'_>| {
            let rows_show_ac = [ATTACKER, TARGET].map(|row| {
                d.row_node(row).is_some_and(|node| {
                    !node.name.contains(" save ")
                        && d.painted_word_rect("AC", node.shown).is_some()
                        && d.painted_word_rect("DEX", node.shown).is_none()
                })
            });
            !d.has("Roll damage")
                && !d.has_containing("target. Click more")
                && !d.has_containing("targets. Click more")
                && !d.has_containing("Click each target")
                && rows_show_ac == [true, true]
                // Must stay true: the panel stays on the caster's tiles, with the results.
                && tile(d, BURST).is_some()
                && d.visible_text("Saved · ")
                && d.visible_text("Failed · ")
        };
        d.observe_both(
            "iter-20-R-FT1",
            "After its damage applies, the save is disarmed, its rows show AC with their results",
            finished,
        );
        // SJ1: at 960 Undo names the step in its short form; the accessible name keeps it whole.
        // iter-26-R FI4: the count follows `×`, so it is not read as Cinder Wolf 2.
        let shown = d.at_size(SMALL, |d| {
            (
                d.visible_text("Undo: 3 damage × 2"),
                d.visible_text("Undo: 3 damage to 2 participants"),
            )
        });
        d.observe(
            "iter-23-R-SJ1",
            shown == (true, false) && d.has("Undo: 3 damage to 2 participants, half for 1"),
            format!("At 960 the save's Undo says `Undo: 3 damage × 2` (compact, whole): {shown:?}"),
        );
        // iter-25-R SJ1: a save's result keeps its `(half)` at every size; a row whose
        // conditions leave no room for it gives it a line of its own.
        for size in [SMALL, DEFAULT, LARGE] {
            let whole = d.at_size(size, |d| {
                (d.visible_text("damage (half)"), d.visible_text("damage…"))
            });
            d.observe(
                "iter-25-R-SJ1-half",
                whole == (true, false),
                format!("At {size:?} the saved result shows whole, `(half)` included: {whole:?}"),
            );
        }
        d.state("save-applied", Sizes::Default);
        // Undo gives each target back its own HP.
        d.press(Key::U);
        let hp_undone = [d.row_hp(ATTACKER), d.row_hp(TARGET)];
        d.observe(
            "iter-19-R-FT1",
            hp_undone == hp_before && hp_before != hp_after,
            format!(
                "Undo restores each row's HP: {hp_before:?} then {hp_after:?}, undone {hp_undone:?}"
            ),
        );
        // iter-20-R-FT1: Undo arms the save again as it stood: its saves where AC is, Roll
        // damage, and each row's result from the damage in Amount again (iter-19-R2-1: the
        // applied results went with their step; the unit test asserts the step's results).
        let armed_again = [ATTACKER, TARGET].map(|row| {
            d.row_node(row)
                .is_some_and(|node| node.name.contains(", DEX save "))
        });
        d.observe(
            "iter-20-R-FT1",
            d.has("Roll damage")
                && armed_again == [true, true]
                && [ATTACKER, TARGET].map(|row| save_said(d, row)) == rolled,
            format!(
                "Undo arms the save again with its results: {armed_again:?}, {:?}",
                [ATTACKER, TARGET].map(|row| save_said(d, row))
            ),
        );
        let full = rolled
            .iter()
            .flatten()
            .find(|(saved, _)| !saved)
            .map(|(_, amount)| *amount);
        // Outside the task's count: another amount gives longer words.
        d.replace_text_in("Amount", "20");
        let said =
            [ATTACKER, TARGET].map(|row| d.row_node(row).map(|node| node.name).unwrap_or_default());
        let faults = roll_words_faults(d, &rows);
        d.observe(
            "iter-20-R-FT4",
            said.iter()
                .any(|name| name.ends_with("Saved · 10 damage (half)"))
                && said.iter().any(|name| name.ends_with("Failed · 20 damage"))
                && faults.is_empty(),
            format!(
                "`Saved · 10 damage (half)` is whole or cut by whole words: {said:?}, {faults:?}"
            ),
        );
        d.replace_text_in("Amount", &full.unwrap_or_default().to_string());
        d.observe(
            "iter-20-R-FT4",
            full.is_some() && [ATTACKER, TARGET].map(|row| save_said(d, row)) == rolled,
            format!("Amount has the rolled damage again: {rolled:?}"),
        );
        // Apply again: the same amounts, and the save is finished again.
        d.press(Key::Enter);
        let hp_again = [d.row_hp(ATTACKER), d.row_hp(TARGET)];
        d.observe(
            "iter-20-R-FT1",
            hp_again == hp_after && finished(d),
            format!(
                "Applying again gives the same HP, {hp_after:?} then {hp_again:?}, and disarms"
            ),
        );
        // Next turn ends the results; Undo of the turn brings back those the step applied.
        d.press(Key::N);
        let next_clears = !d.visible_text("Saved · ") && !d.visible_text("Failed · ");
        d.press(Key::U);
        d.observe(
            "iter-20-R-FT1",
            next_clears
                && d.visible_text("Saved · ")
                && d.visible_text("Failed · ")
                && !d.has("Roll damage"),
            format!("Next turn clears the save's results ({next_clears}), and Undo of it brings them back, disarmed"),
        );

        // A player rolls their own dice: the tile targets, and the DM types the damage.
        d.click_row(PLAYER);
        d.click("Actions");
        d.begin_task();
        d.click_starting("Dagger");
        let before = d.row_hp(ATTACKER);
        d.click_row(ATTACKER);
        d.observe(
            "T16.6",
            d.has_containing("Player rolls"),
            "A player's attack only targets the row",
        );
        d.state("player-attack", Sizes::Default);
        d.type_text("5");
        d.press(Key::Enter);
        let after = d.row_hp(ATTACKER);
        d.observe(
            "T16.6",
            before != after,
            format!("The player's typed damage applies to the target: {before:?} then {after:?}"),
        );
        d.goal_within("T16.6", "a player's attack with their own damage", 4);
        d.press(Key::Escape);

        // Inspiration, by the campaign's rule: one die of a roll made for a player rolls again.
        d.click_row(HERO);
        d.click("Actions");
        d.click_starting("Longsword");
        d.click_row(ATTACKER);
        d.observe(
            "T16.11",
            d.has("Roll for them"),
            "The panel keeps the player's sheet and its tiles while their attack has a target",
        );
        d.click("Roll for them");
        // AJ1: the same menu opens from the keyboard. Shift+F10 on the focused row puts the
        // focus on its first choice; Escape closes it, gives the focus back to the row, and
        // keeps the armed action and the reroll.
        let row = d.row_node(ATTACKER).map(|node| node.name);
        let reached = row.as_deref().is_some_and(|row| d.tab_to(row));
        d.press_shift(Key::F10);
        let opened = d.has_containing("Inspiration: reroll one die");
        let first = d.focused_name();
        // The panel's `Roll for them` and the menu's.
        let in_menu = d
            .nodes()
            .iter()
            .filter(|node| node.name == "Roll for them")
            .count()
            == 2;
        d.press(Key::Escape);
        let back = d.focused_name();
        d.observe(
            "iter-23-R-AJ1",
            reached
                && opened
                // The menu's first section rolls for the player; its first choice has the focus.
                && first.as_deref() == Some("Roll for them")
                && in_menu
                && !d.has_containing("Inspiration: reroll one die")
                && back.as_deref().is_some_and(|back| back.starts_with(&format!("{ATTACKER},")))
                && d.has("Roll for them"),
            format!(
                "Shift+F10 on the focused row opens its menu with the focus on {first:?}; Escape gives it back to {back:?} and keeps the action armed"
            ),
        );
        d.press_shift(Key::F10);
        let again = d.has_containing("Inspiration: reroll one die");
        d.press(Key::Escape);
        d.observe(
            "iter-23-R-AJ1",
            again,
            "With the focus back on the row, Shift+F10 opens its menu again",
        );
        // AJ1, the menu focus model: a keyboard close sends the focus to where it last was,
        // inside the menu (the row) or outside it (that control), and never leaves it empty.
        let menu_row = |d: &Driver<'_>| d.row_node(ATTACKER).map(|node| node.name);
        // (a, b): Tab out to a button or a text field, then Escape. The focus stays there,
        // and the next Tab goes on from there, as it does with no menu.
        for away in [NEXT_TURN, "Amount"] {
            let reached = d.tab_to(away);
            d.press(Key::Tab);
            let after = d.focused_name();
            d.harness_focus_nothing();
            let row = menu_row(d);
            let on_row = row.as_deref().is_some_and(|row| d.tab_to(row));
            d.press_shift(Key::F10);
            let opened = d.has_containing("Inspiration: reroll one die");
            let left = d.tab_to(away);
            d.press(Key::Escape);
            let focus = d.focused_name();
            d.press(Key::Tab);
            let next = d.focused_name();
            d.observe(
                "iter-23-R-AJ1",
                reached
                    && on_row
                    && opened
                    && left
                    && !d.has_containing("Inspiration: reroll one die")
                    && focus.as_deref() == Some(away)
                    && after.is_some()
                    && next == after
                    && d.has("Roll for them"),
                format!(
                    "Tab from the open menu to {away}, then Escape: the focus is {focus:?}, and Tab then goes to {next:?} (with no menu: {after:?})"
                ),
            );
            d.harness_focus_nothing();
        }
        // (c, d): Tab out, then back inside, to a menu choice with Shift+Tab or to the row
        // itself, then Escape: the focus goes to the row.
        for back_to_row in [false, true] {
            let row = menu_row(d);
            let on_row = row.as_deref().is_some_and(|row| d.tab_to(row));
            let before: std::collections::BTreeSet<String> =
                d.nodes().into_iter().map(|node| node.name).collect();
            d.press_shift(Key::F10);
            let choices: std::collections::BTreeSet<String> = d
                .nodes()
                .into_iter()
                .map(|node| node.name)
                .filter(|name| !before.contains(name))
                .collect();
            // The menu's own choices; its `Roll for them` shares a name with the panel's, so it
            // counts as neither inside nor outside.
            let in_menu =
                |d: &Driver<'_>| d.focused_name().is_some_and(|name| choices.contains(&name));
            let mut out = false;
            for _ in 0..20 {
                d.press(Key::Tab);
                if !in_menu(d) && d.focused_name().is_some_and(|name| name != "Roll for them") {
                    out = true;
                    break;
                }
            }
            let outside = d.focused_name();
            let back = if back_to_row {
                row.as_deref().is_some_and(|row| d.tab_to(row))
            } else {
                d.press_shift(Key::Tab);
                in_menu(d)
            };
            let open = d.has_containing("Inspiration: reroll one die");
            d.press(Key::Escape);
            let focus = d.focused_name();
            let how = if back_to_row {
                "the row"
            } else {
                "a menu choice"
            };
            d.observe(
                "iter-23-R-AJ1",
                on_row
                    && out
                    && back
                    && open
                    && !d.has_containing("Inspiration: reroll one die")
                    && focus
                        .as_deref()
                        .is_some_and(|focus| focus.starts_with(&format!("{ATTACKER},"))),
                format!(
                    "Tab out to {outside:?}, back to {how}, then Escape: the focus is {focus:?}, on the row"
                ),
            );
            d.harness_focus_nothing();
        }
        d.begin_task();
        d.right_click_row(ATTACKER);
        d.observe(
            "T16.11",
            d.has_containing("Inspiration: reroll one die") && d.has_containing(" on the d20"),
            "The row's menu offers each die of the roll made for the player",
        );
        d.state("reroll-menu", Sizes::Default);
        d.click_containing(" on 1d8");
        d.right_click_row(ATTACKER);
        d.observe(
            "T16.11",
            !d.has_containing("Inspiration: reroll one die"),
            "The inspiration is spent, so the menu offers no second reroll",
        );
        d.goal_within(
            "T16.11",
            "reroll one die of a player's roll with inspiration",
            3,
        );
        d.press(Key::Escape);
        d.press(Key::Escape);

        // AJ1 (review): the whole reroll by keyboard, on a check, whose menu holds only the
        // reroll. Choosing it empties the menu, and the focus still comes back to the row. The
        // sheet shows none, then a new grant, so the character has inspiration again.
        {
            let run = d.app_mut().encounters.run_mut().expect("an open run");
            run.simulate_sheet_inspiration(HERO, false);
            run.simulate_sheet_inspiration(HERO, true);
        }
        d.settle();
        d.click_row(HERO);
        d.click("Overview");
        d.click_containing(": Strength check.");
        let row = d.row_node(HERO).map(|node| node.name);
        let reached = row.as_deref().is_some_and(|row| d.tab_to(row));
        d.press_shift(Key::F10);
        let first = d.focused_name();
        let lines = d.app_mut().encounters.run().map_or(0, |run| run.log_len());
        d.press(Key::Enter);
        let rerolled = d.app_mut().encounters.run().map_or(0, |run| run.log_len()) > lines;
        let back = d.focused_name();
        d.press_shift(Key::F10);
        let spent = !d.has_containing("Inspiration: reroll one die");
        d.observe(
            "iter-23-R-AJ1",
            reached
                && first.as_deref().is_some_and(|first| first.ends_with(" on the d20"))
                && rerolled
                && back.as_deref().is_some_and(|back| back.starts_with(&format!("{HERO},")))
                && spent,
            format!(
                "By keyboard: the menu's only choice {first:?} rerolls ({rerolled}); the menu empties and the focus is back on {back:?}; no second reroll ({spent})"
            ),
        );
        d.harness_focus_nothing();

        // Outside a run, a tile previews and changes nothing.
        go_library(d);
        open_local_package(d, MODULE);
        d.click_containing("Cinder Wolf");
        d.begin_task();
        d.click_starting(BITE);
        d.observe(
            "T16.7",
            d.has_containing("to hit"),
            "A tile in the Library previews its attack and damage",
        );
        d.goal_within("T16.7", "preview an attack in the Library", 1);
        // iter-20-R-ST2: a lone tile takes the reader's line in the Library too, not the
        // 208 px of a tile that shares its line.
        let widths = [DEFAULT, LARGE]
            .map(|size| d.at_size(size, |d| tile(d, BITE).map(|tile| tile.rect.width())));
        d.observe(
            "iter-20-R-ST2",
            widths
                .iter()
                .all(|width| width.is_some_and(|width| width > 2.0 * 208.0)),
            format!("The Library's lone Bite tile takes the line at 1200 and 1920: {widths:?}"),
        );
        // iter-21-T-FT3: the preview's totals read `11 to hit · 5 damage: …`, with one muted
        // `·` between them, on the line of the damage words. At 960 the line is below the fold;
        // `rolls::a_preview_has_one_muted_separator…` checks narrow lines.
        let muted = crate::design::colors().muted;
        let totals = [DEFAULT, LARGE].map(|size| {
            d.at_size(size, |d| {
                let below = tile(d, BITE)?.rect.bottom();
                let mut texts = Vec::new();
                d.each_painted_text(|text, clip| {
                    let Some(row) = text.galley.rows.first() else {
                        return;
                    };
                    let rect = row
                        .rect_without_leading_space()
                        .translate(text.pos.to_vec2());
                    if rect.top() > below && clip.intersects(rect) {
                        let color = text.galley.job.sections.first().map(|s| s.format.color);
                        texts.push((text.galley.job.text.clone(), rect, color));
                    }
                });
                let hit = texts
                    .iter()
                    .position(|(words, ..)| words.ends_with(" to hit"))?;
                let [_, (dot, dot_rect, color), (damage, damage_rect, _)] =
                    texts.get(hit..hit + 3)?
                else {
                    return None;
                };
                let joined = dot == "·"
                    && *color == Some(muted)
                    && damage.contains(" damage")
                    && dot_rect.right() < damage_rect.left()
                    && (damage_rect.top()..damage_rect.bottom()).contains(&dot_rect.center().y);
                Some((format!("{} {dot} {damage}", texts[hit].0), joined))
            })
        });
        d.observe(
            "iter-21-T-FT3",
            totals
                .iter()
                .all(|found| found.as_ref().is_some_and(|(_, joined)| *joined)),
            format!(
                "At 1200 and 1920, the preview has one muted `·` between its totals, on the \
                 damage's line: {totals:?}"
            ),
        );
        d.state("library-preview", Sizes::All);

        // Two tiles share a line where the reader has room, each with its facts line, and a
        // narrow window cuts none of their words.
        d.click_containing("Ember Cultist");
        let paired = d.at_size(DEFAULT, |d| {
            tile(d, "Scorching Sickle")
                .zip(tile(d, BURST))
                .map(|(sickle, burst)| (sickle.rect, burst.rect))
        });
        d.observe(
            "iter-19-R-ST3",
            paired.is_some_and(|(sickle, burst)| {
                (sickle.top() - burst.top()).abs() < 1.0 && sickle.right() < burst.left()
            }),
            format!("At 1200 the two tiles share one line: {paired:?}"),
        );
        d.observe_both(
            "iter-19-R-ST3",
            "Every tile word shows whole, at 1200 and at 960",
            |d| {
                d.visible_text("or\u{a0}2d6+2\u{a0}slashing")
                    && d.visible_text("Reach 5 ft")
                    && d.visible_text("10-ft emanation · half on a success")
            },
        );
        d.state("library-two-tiles", Sizes::All);
    }
}

/// Lookup's result rows in view order: buttons named `<title> · <detail>` below the search field.
fn lookup_results(d: &Driver<'_>) -> Vec<crate::visual_tests::driver::Found> {
    let Some(field) = d.place_of(SEARCH_LOOKUP) else {
        return Vec::new();
    };
    d.nodes()
        .into_iter()
        .filter(|node| {
            node.role == egui::accesskit::Role::Button
                && node.name.contains(" · ")
                && node.rect.left() >= field.left() - 2.0
                && node.rect.top() >= field.bottom()
        })
        .collect()
}

/// The `Show all N` count, if Lookup or Arrivals offers it.
fn show_all_count(d: &Driver<'_>) -> Option<usize> {
    d.nodes().into_iter().find_map(|node| {
        node.name
            .strip_prefix("Show all ")
            .and_then(|count| count.parse::<usize>().ok())
    })
}

/// The painted words of Lookup's action that opens the closed result list.
pub(super) fn lookup_results_words(d: &Driver<'_>) -> Option<String> {
    d.place_of("Show the results")
        .map(|place| d.painted_in(place).trim().to_owned())
}

/// FT2: a quiet Lookup action that starts a line uses `line_start`. See `action_at_text_edge`.
fn lookup_action_at_text_edge(
    d: &mut Driver<'_>,
    id: &str,
    action: &str,
    sizes: &[(f32, f32)],
    edge: impl Fn(&Driver<'_>) -> Option<f32>,
) {
    action_at_text_edge(d, id, action, LOOKUP_SPLITTER, sizes, edge);
}

/// A quiet action that starts a line uses `line_start`: its words start at `edge`, the text edge
/// of what is above it, with any icon hanging in the gutter, and its press area stays right of
/// the nearest column splitter. It runs at each size given, the sizes its state is captured at.
fn action_at_text_edge(
    d: &mut Driver<'_>,
    id: &str,
    action: &str,
    splitter: &str,
    sizes: &[(f32, f32)],
    edge: impl Fn(&Driver<'_>) -> Option<f32>,
) {
    for &size in sizes {
        let (target, edge, splitter, start) = d.at_size(size, |d| {
            d.expect(action);
            d.expect(splitter);
            let target = d.place_of(action);
            let start = target.and_then(|target| words_left_in(d, target));
            (target, edge(d), d.place_of(splitter), start)
        });
        d.observe(
            id,
            start
                .zip(edge)
                .is_some_and(|(start, edge)| (start - edge).abs() <= 1.0)
                && target
                    .zip(splitter)
                    .is_some_and(|(target, splitter)| target.left() >= splitter.right()),
            format!(
                "At {size:?}, {action:?} starts at {start:?}, the text edge is at {edge:?}; its press \
                 area {target:?} is right of the splitter {splitter:?}"
            ),
        );
    }
}

/// The left of the first Lookup result's painted name: the text edge of the result rows.
fn lookup_row_text_left(d: &Driver<'_>) -> Option<f32> {
    let row = lookup_results(d).into_iter().next()?;
    let name = row.name.split(" · ").next()?.to_owned();
    d.painted_rect_in(&name, row.rect).map(|rect| rect.left())
}

const LOOKUP_SPLITTER: &str = "Resize Lookup column";

/// How a link with an icon paints `<icon> <words>`: whether the icon is underlined, whether the
/// words are, and the icon's colour. An underline counts whether the text paints it or the link
/// paints it as a line under the text.
fn icon_link_parts(d: &Driver<'_>, icon: &str, words: &str) -> Option<(bool, bool, egui::Color32)> {
    let whole = format!("{icon} {words}");
    let mut found = None;
    d.each_painted_text(|text, clip| {
        let job = &text.galley.job;
        if job.text != whole || !clip.intersects(text.visual_bounding_rect()) {
            return;
        }
        let section = |part: &str| {
            job.sections
                .iter()
                .find(|section| job.text[section.byte_range.clone()].contains(part))
        };
        if let Some((icon, name)) = section(icon).zip(section(words)) {
            let bounds = egui::Rect::from_min_size(text.pos, text.galley.size());
            let start = whole.chars().count() - words.chars().count();
            let words_left = text.pos.x
                + text
                    .galley
                    .pos_from_cursor(egui::text::CCursor::new(start))
                    .left();
            found = Some((bounds, words_left, icon.format.clone(), name.format.clone()));
        }
    });
    let (bounds, words_left, icon, name) = found?;
    // A line just under the text, across the icon or across the words.
    let lines = d.line_segments_in(bounds.expand2(egui::vec2(0.0, 4.0)));
    let line_under = |from: f32, to: f32| {
        lines
            .iter()
            .any(|line| line.left() < to - 0.5 && line.right() > from + 0.5)
    };
    Some((
        icon.underline.width > 0.0 || line_under(bounds.left(), words_left),
        name.underline.width > 0.0 || line_under(words_left, bounds.right()),
        icon.color,
    ))
}

/// FT1: Lookup's `Show all N` is quiet, lists every match in place in a list of at most eight
/// rows, and the list follows Tab and the arrow keys. Ends with `words` typed again.
fn lookup_show_all(d: &mut Driver<'_>, words: &str) {
    let before = lookup_results(d).len();
    let count = show_all_count(d);
    let label = format!("Show all {}", count.unwrap_or_default());
    // FT2: Show all starts a line, so its word starts at the result rows' text.
    lookup_action_at_text_edge(
        d,
        "iter-17-D-FT2-show-all",
        &label,
        &[SMALL, DEFAULT, LARGE],
        lookup_row_text_left,
    );
    let frame = d.control_frame(&label);
    let quiet = frame
        .as_ref()
        .is_none_or(|frame| frame.stroke.width == 0.0 && frame.fill.a() == 0);
    if count.is_some() {
        d.click(&label);
    }
    let rows = lookup_results(d);
    d.observe(
        "iter-16-D-FT1",
        before == 5
            && quiet
            && count.is_some_and(|count| count > before && rows.len() == count)
            && rows.iter().all(|row| row.fully_in_view())
            && show_all_count(d).is_none()
            && d.in_view(SEARCH_LOOKUP),
        format!(
            "Quiet {label} (frame {frame:?}) lists every match in place: {before} rows before, {} after, \
             all in view",
            rows.len()
        ),
    );
    d.state("lookup-show-all", Sizes::All);
    // Must stay true: new words return to five, as in Arrivals.
    d.replace_text_in(SEARCH_LOOKUP, &words[..words.len() - 1]);
    d.replace_text_in(SEARCH_LOOKUP, words);
    d.observe(
        "iter-16-D-FT1-reset",
        lookup_results(d).len() == 5 && show_all_count(d) == count,
        "Changed words return Lookup to five results and its Show all",
    );
    // A broad search scrolls past eight rows. Tab reaches a row below the edge, and so do the
    // arrow keys from the search field; the list follows each.
    d.replace_text_in(SEARCH_LOOKUP, "dragon");
    let broad = show_all_count(d);
    if let Some(broad) = broad {
        d.click(&format!("Show all {broad}"));
    }
    let rows = lookup_results(d);
    // The list's own visible area now: the clip its top row's words are painted in. Opening a
    // result can move the list, so each check measures it again.
    let list_clip = |d: &Driver<'_>| {
        let top = lookup_results(d).into_iter().find(|row| row.in_view())?;
        let title = top.name.split(" · ").next()?.to_owned();
        let mut found = None;
        d.each_painted_text(|text, clip| {
            if title == text.galley.job.text
                && top.rect.contains(text.pos + text.galley.size() / 2.0)
            {
                found = Some(clip);
            }
        });
        found
    };
    let clip = list_clip(d);
    let inside_of = |clip: Option<egui::Rect>, rect: egui::Rect| {
        clip.is_some_and(|clip| clip.expand(0.5).contains_rect(rect))
    };
    let inside = |rect: egui::Rect| inside_of(clip, rect);
    let place = |d: &Driver<'_>, row: &Option<crate::visual_tests::driver::Found>| {
        row.as_ref().and_then(|row| d.place_of(&row.name))
    };
    let shown = rows.iter().filter(|row| inside(row.rect)).count();
    // The whole row shows, not only the part inside egui's clip margin.
    let whole = |d: &Driver<'_>, row: &Option<crate::visual_tests::driver::Found>| {
        row.as_ref().is_some_and(|row| {
            lookup_results(d)
                .iter()
                .any(|now| now.name == row.name && now.fully_in_view())
        })
    };
    let ninth = rows.get(8).cloned();
    let tenth = rows.get(9).cloned();
    if let Some(ninth) = &ninth {
        let mut tabs = 0;
        while d.focused_name().as_deref() != Some(ninth.name.as_str()) && tabs < 40 {
            d.press(Key::Tab);
            tabs += 1;
        }
    }
    let reached = place(d, &ninth);
    let reached_clip = list_clip(d);
    d.observe(
        "iter-16-D-FT1-tab",
        broad.is_some_and(|broad| broad > 9 && rows.len() == broad)
            && shown == crate::lookup::RESULTS_LISTED
            && ninth.as_ref().is_some_and(|ninth| !inside(ninth.rect))
            && ninth
                .as_ref()
                .is_some_and(|ninth| d.focused_name().as_deref() == Some(ninth.name.as_str()))
            && reached.is_some_and(|rect| inside_of(reached_clip, rect))
            && whole(d, &ninth),
        format!(
            "Show all {broad:?}: {} rows named, {shown} inside the list clip {clip:?}; Tab \
             reaches the ninth ({:?}) and the list scrolls it inside {reached_clip:?}: {reached:?}",
            rows.len(),
            ninth.as_ref().map(|row| (&row.name, row.rect)),
        ),
    );
    d.replace_text_in(SEARCH_LOOKUP, "drago");
    d.replace_text_in(SEARCH_LOOKUP, "dragon");
    if let Some(broad) = broad {
        d.click(&format!("Show all {broad}"));
    }
    d.type_into(SEARCH_LOOKUP, "");
    let before = place(d, &tenth);
    let before_clip = list_clip(d);
    for _ in 0..9 {
        d.press(Key::ArrowDown);
    }
    let after = place(d, &tenth);
    let after_clip = list_clip(d);
    d.observe(
        "iter-16-D-FT1-arrows",
        before.is_some_and(|before| !inside_of(before_clip, before))
            && after.is_some_and(|after| inside_of(after_clip, after))
            && whole(d, &tenth),
        format!(
            "Nine Down presses from the search field bring the tenth row ({:?}) inside the list: \
             before {before:?} in {before_clip:?}, after {after:?} in {after_clip:?}",
            tenth.as_ref().map(|row| &row.name)
        ),
    );
    // Crowded: Enter reads the highlighted match and closes the long list. The action that
    // opens it again names every match, whole, at the smallest size.
    d.press(Key::Enter);
    let reopen = d.at_size(SMALL, lookup_results_words);
    d.observe(
        "iter-19-D-show-results-crowded",
        broad.is_some_and(|broad| reopen == Some(format!("Show {broad} results")))
            && show_all_count(d).is_none(),
        format!("With {broad:?} matches and the list closed, the action says {reopen:?}"),
    );
    d.replace_text_in(SEARCH_LOOKUP, words);
}
