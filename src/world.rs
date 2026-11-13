//! The campaign's world: plain notes Splat reads and does not own, and the NPCs and locations
//! made from them. Obsidian stays a plain note's editor; an entry also has Edit here. A plain
//! note becomes a Lookup item, not an entry, so nothing depends on it.
//! See `docs/design/world-and-house-rules.md`.
mod checks;
pub(crate) mod edit;
mod fields;
pub(crate) mod file_fault;
pub mod lines;
pub mod page;
mod related;
pub(crate) mod repair;

use crate::{
    content_workspace::{EntryRef, patch_fields},
    design::{self, Action, Density, Tone, Type, colors, reader as style},
    encounter_run::{GroupMember, RunGroup},
    library::{note_path, view, write_note},
    lookup::{ContentIndex, Item},
    markdown_view::{FILE_LINK, NOTE_LINK},
    persistence::Campaign,
    sessions, timeline,
};
use eframe::egui;
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use related::related;
use serde_yaml_ng::{Mapping, Value};
use splat::character::stable_id;
use splat::content::{
    Entry, Reference,
    art::FORMATS,
    layout::{Field, FlagKind},
    overrides::Overrides,
};
use std::{
    collections::HashMap,
    fs,
    hash::{Hash, Hasher},
    path::{Component, Path, PathBuf},
    sync::Arc,
};
use uuid::Uuid;

/// The kind of a plain note's Lookup item. No file holds this kind.
pub(crate) const NOTE_KIND: &str = "world_note";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Role {
    World,
    Session,
    Transcript,
}

#[derive(Debug)]
pub struct PlainNote {
    /// Relative to the campaign root, with forward slashes.
    pub path: String,
    pub name: String,
    pub role: Role,
    pub body: String,
}

#[derive(Default)]
pub struct Scan {
    pub notes: Vec<PlainNote>,
    /// By lower-case file name, as Obsidian finds an embedded image.
    pub images: HashMap<String, PathBuf>,
}

fn walk(current: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(current) else {
        return;
    };
    let mut paths: Vec<_> = entries.flatten().map(|entry| entry.path()).collect();
    paths.sort();
    for path in paths {
        // A vault's `.obsidian` and `.trash` hold Markdown that is nobody's world.
        if path
            .file_name()
            .is_some_and(|name| name.to_string_lossy().starts_with('.'))
        {
            continue;
        }
        let Ok(meta) = fs::symlink_metadata(&path) else {
            continue;
        };
        if meta.file_type().is_symlink() {
            continue;
        }
        if meta.is_dir() {
            walk(&path, files);
        } else {
            files.push(path);
        }
    }
}

/// What a note is for. The test is its role and not its folder, because a session folder can be
/// the campaign root, and a DM can keep world notes beside sessions.
fn role_of(front: Option<&str>, inside_sessions: Option<&Path>) -> Option<Role> {
    let has = |name: &str| front.is_some_and(|front| front.lines().any(|l| l.starts_with(name)));
    if has("splat_")
        || has("timeline_version:")
        || has("trackers_version:")
        || has(crate::sheet_writer::KEY)
    {
        return None;
    }
    if has("session_id:") {
        return Some(Role::Session);
    }
    if let Some(inside) = inside_sessions
        && let Some(Component::Normal(first)) = inside.components().next()
        && (first == sessions::TEMPLATES || first == sessions::RECORDINGS)
    {
        return None;
    }
    Some(Role::World)
}

/// The best words of a recording, and the name to show them under.
fn transcript_name(path: &Path) -> Option<String> {
    let name = path.file_name()?.to_string_lossy();
    if let Some(stem) = name.strip_suffix(".marked.md") {
        return Some(format!("{stem}, transcript"));
    }
    let stem = name.strip_suffix(".transcript.md")?;
    (!path.with_file_name(format!("{stem}.marked.md")).exists())
        .then(|| format!("{stem}, transcript"))
}

pub fn scan(root: &Path, sessions_folder: &Path) -> Scan {
    let mut scan = Scan::default();
    let mut files = Vec::new();
    walk(root, &mut files);
    // A session folder can be an absolute path outside the campaign.
    if !sessions_folder.starts_with(root) {
        walk(sessions_folder, &mut files);
    }
    let recordings = sessions_folder.join(sessions::RECORDINGS);
    for path in files {
        let extension = path
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase())
            .unwrap_or_default();
        let file_name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_lowercase())
            .unwrap_or_default();
        if FORMATS.contains(&extension.as_str()) {
            scan.images.entry(file_name).or_insert(path);
            continue;
        }
        if extension != "md"
            || (file_name.eq_ignore_ascii_case(timeline::FILE)
                && path.parent() == Some(sessions_folder))
        {
            continue;
        }
        // A recording leaves several files. Only one is worth reading.
        let transcript = if path.starts_with(&recordings) {
            match transcript_name(&path) {
                Some(name) => Some(name),
                None => continue,
            }
        } else {
            None
        };
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let text = text.trim_start_matches('\u{feff}').replace("\r\n", "\n");
        let (front, body) =
            sessions::split_note(&text).map_or((None, text.as_str()), |(f, b)| (Some(f), b));
        // The tracker ledger is Splat's own file, not a world note. A note of the DM's that has
        // that name and is no ledger stays a world note.
        if path.parent() == Some(root) && crate::trackers::ledger::is_ledger(&file_name, front) {
            continue;
        }
        let (name, role) = match transcript {
            Some(name) => (name, Role::Transcript),
            None => {
                let Some(role) = role_of(front, path.strip_prefix(sessions_folder).ok()) else {
                    continue;
                };
                let stem = path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned();
                (stem, role)
            }
        };
        scan.notes.push(PlainNote {
            path: path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/"),
            name,
            role,
            body: body.to_owned(),
        });
    }
    scan
}

/// Changes when a note is edited, so returning from Obsidian can refresh what Splat shows.
pub fn fingerprint(root: &Path) -> u64 {
    let mut files = Vec::new();
    walk(root, &mut files);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for path in files {
        if path.extension().is_some_and(|e| e == "md" || e == "json")
            && let Ok(meta) = fs::metadata(&path)
        {
            path.hash(&mut hasher);
            meta.len().hash(&mut hasher);
            if let Ok(modified) = meta.modified() {
                modified.hash(&mut hasher);
            }
        }
    }
    hasher.finish()
}

/// A note has no UUID. This one is stable for its path, and it never reaches a file.
pub fn note_id(path: &str) -> Uuid {
    stable_id(&format!("splat:world_note:{path}"))
}

pub fn note_entry(path: &str, name: &str, body: String) -> Entry {
    Entry {
        path: path.to_owned(),
        id: note_id(path),
        key: None,
        name: name.to_owned(),
        kind: NOTE_KIND.to_owned(),
        fields: Mapping::new().into(),
        body,
        body_line: 1,
        inventory: Vec::new(),
        inventory_containers: Vec::new(),
        flags: Vec::new(),
    }
}

fn outside_code(body: &str, mut plain: impl FnMut(&str, &mut String)) -> String {
    let mut output = String::with_capacity(body.len());
    let mut fence = false;
    for line in body.split_inclusive('\n') {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            fence = !fence;
            output.push_str(line);
            continue;
        }
        if fence {
            output.push_str(line);
            continue;
        }
        for (index, part) in line.split('`').enumerate() {
            if index > 0 {
                output.push('`');
            }
            if index % 2 == 0 {
                plain(part, &mut output);
            } else {
                output.push_str(part);
            }
        }
    }
    output
}

/// Every `[[link]]` in a text, as (is an embed, target, words).
pub(crate) fn each_link(text: &str) -> Vec<(std::ops::Range<usize>, bool, String, Option<String>)> {
    let mut found = Vec::new();
    let mut from = 0;
    while let Some(start) = text[from..].find("[[").map(|at| at + from) {
        let inner = start + 2;
        let Some(end) = text[inner..].find("]]").map(|at| at + inner) else {
            break;
        };
        let embed = text[..start].ends_with('!');
        let mut parts = text[inner..end].splitn(2, '|');
        let target = parts.next().unwrap_or("").trim_end_matches('\\').trim();
        let words = parts.next().map(|words| words.trim().to_owned());
        if !target.is_empty() && !target.contains('\n') {
            let at = if embed { start - 1 } else { start };
            found.push((at..end + 2, embed, target.to_owned(), words));
        }
        from = end + 2;
    }
    found
}

/// The file or note each link in a body names, in lower case, without its heading.
pub fn links_in(body: &str) -> Vec<String> {
    let mut links = Vec::new();
    outside_code(body, |part, _| {
        for (_, _, target, _) in each_link(part) {
            let name = target.split('#').next().unwrap_or("").trim().to_lowercase();
            if !name.is_empty() {
                links.push(name);
            }
        }
    });
    links
}

/// Redirect explicit links when their named dependency is copied. Keep their display words.
pub(crate) fn redirect_links(text: &str, old: &Entry, new_id: Uuid) -> String {
    outside_code(text, |part, output| {
        let mut copied = 0;
        for (range, embed, target, words) in each_link(part) {
            output.push_str(&part[copied..range.start]);
            let (name, heading) = target
                .split_once('#')
                .map_or((target.as_str(), String::new()), |(name, heading)| {
                    (name, format!("#{heading}"))
                });
            let matches = name.eq_ignore_ascii_case(&old.name)
                || name == old.id.to_string()
                || old.key.as_deref() == Some(name)
                || name.trim_end_matches(".md") == old.path.trim_end_matches(".md");
            if matches && !embed {
                output.push_str(&format!(
                    "[[{new_id}{heading}|{}]]",
                    words.as_deref().unwrap_or(&target)
                ));
            } else {
                output.push_str(&part[range.clone()]);
            }
            copied = range.end;
        }
        output.push_str(&part[copied..]);
    })
}

/// Rename note links outside code. Keep headings, aliases, and unrelated words.
pub(crate) fn rename_note_links(text: &str, old: &str, new: &str) -> (String, usize) {
    let mut count = 0;
    let (front, body) = crate::sessions::split_note(text).unwrap_or(("", text));
    let after = outside_code(body, |part, output| {
        let mut copied = 0;
        for (range, embed, target, words) in each_link(part) {
            output.push_str(&part[copied..range.start]);
            let (name, heading) = target
                .split_once('#')
                .map_or((target.as_str(), String::new()), |(name, heading)| {
                    (name, format!("#{heading}"))
                });
            if !embed && name.eq_ignore_ascii_case(old) {
                output.push_str(&format!(
                    "[[{new}{heading}{}]]",
                    words.map_or(String::new(), |words| format!("|{words}"))
                ));
                count += 1;
            } else {
                output.push_str(&part[range.clone()]);
            }
            copied = range.end;
        }
        output.push_str(&part[copied..]);
    });
    (format!("{front}{after}"), count)
}

/// Obsidian's links, written as links the Markdown reader understands.
pub fn with_links(body: &str, images: &HashMap<String, PathBuf>) -> String {
    outside_code(body, |part, output| {
        let mut copied = 0;
        for (range, embed, target, words) in each_link(part) {
            output.push_str(&part[copied..range.start]);
            copied = range.end;
            let file = target.rsplit('/').next().unwrap_or(&target).to_lowercase();
            let image = FORMATS
                .iter()
                .any(|extension| file.ends_with(&format!(".{extension}")));
            if embed && image {
                match images.get(&file) {
                    Some(path) => {
                        output.push_str(&format!("[image](<{FILE_LINK}{}>)", path.display()));
                    }
                    None => output.push_str(&format!("(Image not found: {target})")),
                }
                continue;
            }
            let shown = words.unwrap_or_else(|| {
                target
                    .rsplit('#')
                    .next()
                    .unwrap_or(&target)
                    .trim_start_matches('^')
                    .to_owned()
            });
            let shown = shown.replace('[', "\\[").replace(']', "\\]");
            let target = target.replace(['<', '>'], "");
            output.push_str(&format!("[{shown}](<{NOTE_LINK}{target}>)"));
        }
        output.push_str(&part[copied..]);
    })
}

/// Each heading's words, as the reader shows them.
pub fn headings(body: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut heading: Option<String> = None;
    for event in Parser::new_ext(body, Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH) {
        match event {
            Event::Start(Tag::Heading { .. }) => heading = Some(String::new()),
            Event::End(TagEnd::Heading(_)) => found.extend(heading.take()),
            Event::Text(text) | Event::Code(text) => {
                if let Some(heading) = &mut heading {
                    heading.push_str(&text);
                }
            }
            _ => {}
        }
    }
    found.retain(|text| !text.trim().is_empty());
    found
}

// --- World entries.

fn world_field<'a>(entry: &'a Entry, name: &str) -> Option<&'a Value> {
    entry.fields.get("splat_world")?.get(name)
}

/// Empty optional values have no row in Read mode. The editor keeps the source value.
fn reader_field<'a>(entry: &'a Entry, key: &str) -> Option<&'a Value> {
    world_field(entry, key).filter(|value| !empty_optional(value))
}

/// The line under a duplicate's repair action. A duplicate of another kind can leave a World
/// note with only its own path, so the first action may have no other file to name.
fn duplicate_consequence(paths: &[String], at: usize) -> Option<String> {
    if at == 0 {
        paths
            .get(1)
            .map(|other| format!("Existing links keep {other}."))
    } else {
        paths
            .first()
            .map(|first| format!("Existing links will then use {first}."))
    }
}

/// Which group a Part of value names. Two spellings of one link (an alias, a folder path, a
/// different case) are one group.
#[derive(PartialEq)]
enum GroupKey {
    /// The entry the value resolves to.
    Found(usize),
    /// A link that resolves to no single entry: its target as Obsidian reads it.
    Link(String),
    /// Any other value, compared as written.
    Text(Value),
}

fn group_key(index: &ContentIndex, value: &Value) -> GroupKey {
    if let Some(at) = index.world_value_target(value) {
        return GroupKey::Found(at);
    }
    value
        .as_str()
        .or_else(|| value.get("link").and_then(Value::as_str))
        .and_then(|raw| raw.trim().strip_prefix("[[")?.strip_suffix("]]"))
        .and_then(|link| link.split(['|', '#']).next())
        .map_or_else(
            || GroupKey::Text(value.clone()),
            |target| GroupKey::Link(target.trim().trim_end_matches(".md").to_lowercase()),
        )
}

/// Each group's key, found once per list: a link to no note can scan the whole catalogue.
fn group_keys(index: &ContentIndex, groups: &[Value]) -> Vec<GroupKey> {
    groups.iter().map(|group| group_key(index, group)).collect()
}

/// True when `groups` has a value that names the same group as `group`.
#[cfg(test)]
fn has_group(index: &ContentIndex, groups: &[Value], group: &Value) -> bool {
    group_keys(index, groups).contains(&group_key(index, group))
}

/// The disk list with only the groups that changed from `before` to `after` added or removed.
/// Groups added outside Splat stay. Groups compare by target, so a link Obsidian respelled is
/// still the same group, and an unchanged group keeps its disk spelling. The Read picker and
/// Edit's Overwrite both use this.
fn merge_groups(
    index: &ContentIndex,
    before: &[Value],
    after: &[Value],
    disk: &[Value],
) -> Vec<Value> {
    let before = group_keys(index, before);
    let after_keys = group_keys(index, after);
    let (mut groups, mut keys): (Vec<_>, Vec<_>) = disk
        .iter()
        .zip(group_keys(index, disk))
        .filter(|(_, key)| !before.contains(key) || after_keys.contains(key))
        .map(|(group, key)| (group.clone(), key))
        .unzip();
    for (group, key) in after.iter().zip(after_keys) {
        if !before.contains(&key) && !keys.contains(&key) {
            groups.push(group.clone());
            keys.push(key);
        }
    }
    groups
}

/// The name part of a missing link: the warning icon and the name, in `Body` as a set link is.
fn missing_name(name: &str) -> String {
    format!("{} {name}", design::icons::WARN)
}

/// The state part of a missing link, in `Small` after the name.
const NOT_FOUND: &str = " · Not found";

/// The width a missing link takes as `missing_link` draws it: one line, or with `stacked` its
/// name line above `Not found`.
fn missing_width(ui: &egui::Ui, name: &str, stacked: bool) -> f32 {
    let name = design::number_width(ui, &missing_name(name), &Type::Body.font(ui));
    let small = Type::Small.font(ui);
    if stacked {
        name.max(design::number_width(
            ui,
            NOT_FOUND.trim_start_matches(" · "),
            &small,
        ))
    } else {
        name + design::number_width(ui, NOT_FOUND, &small)
    }
}

/// A link to a note that is not there: icon and name in `Body`, as a set link, in the warning
/// tone, then ` · Not found` in `Small`. When the row is too narrow, or its label stands above it
/// (`stacked`, Lookup beside World), the name wraps by whole words and `Not found` takes its own
/// line below it, whole. The name's first line is as tall as a row's control and centred in it,
/// so a `Change` beside the value lines up with the name, not between the two lines.
fn missing_link(ui: &mut egui::Ui, name: &str, stacked: bool) {
    let warning = colors().result.warning;
    let words = missing_name(name);
    if !stacked && missing_width(ui, name, false) <= ui.available_width() {
        let mut job = egui::text::LayoutJob::default();
        let style = ui.style().clone();
        for text in [
            Type::Body.rich(ui, &words).color(warning),
            Type::Small.rich(ui, NOT_FOUND).color(warning),
        ] {
            text.append_to(
                &mut job,
                &style,
                egui::FontSelection::Style(egui::TextStyle::Body),
                egui::Align::Center,
            );
        }
        ui.label(job);
    } else {
        ui.vertical(|ui| {
            // The name's first line sits where a one-line value is centred in the row's
            // control, so `Change` beside it is level with the name; `Not found` follows it.
            ui.add_space(((Density::Scan.control() - Type::Body.line()) / 2.0).max(0.0));
            // `Not found` sits directly under the name, as one value's two lines.
            ui.spacing_mut().item_spacing.y = 0.0;
            ui.add(egui::Label::new(Type::Body.rich(ui, &words).color(warning)).wrap());
            ui.label(
                Type::Small
                    .rich(ui, NOT_FOUND.trim_start_matches(" · "))
                    .color(warning),
            );
        });
    }
}

/// A compact reader's missing link: its muted `Small` label, then `⚠ <name> · Not found` in the
/// warning tone, as the place that waits for a choice reads.
fn compact_missing(ui: &mut egui::Ui, label: &str, name: &str) {
    ui.horizontal(|ui| {
        ui.label(Type::Small.rich(ui, label).color(colors().muted));
        missing_link(ui, name, false);
    });
}

/// `line_start` for a quiet action that starts a line: the first in its row, or a later one that
/// wraps to a line of its own. A later one on the same line keeps its normal target, so the two
/// targets never overlap.
pub(crate) fn starts_line<'a>(ui: &egui::Ui, action: Action<'a>, first: bool) -> Action<'a> {
    if first || action.natural_width(ui) > ui.available_size_before_wrap().x {
        action.line_start()
    } else {
        action
    }
}

/// The words of a place link that matches more than one note.
fn place_choices_words(name: &str, count: usize) -> String {
    format!("{name} · {count} matches")
}

/// A link that a warning introduces, such as `⚠ Watch House · 2 matches`. The icon is its own
/// glyph before the link words, in the warning colour, as in `status_link`, so the underline
/// covers only the words. The press target and the accessible name keep the icon, so the name
/// still says it is a warning. The text wraps and has the same size as `wrapped_link`.
fn warning_link(ui: &mut egui::Ui, words: &str) -> egui::Response {
    let c = colors();
    // The icon has its own column, so wrapped words start each line at the first word, as
    // every wrapped status does (round 27 FH3 family).
    let icon = ui.painter().layout_no_wrap(
        format!("{} ", design::icons::WARN),
        Type::Body.font(ui),
        c.result.warning,
    );
    let mut job = egui::text::LayoutJob::default();
    let style = ui.style().clone();
    Type::Body.rich(ui, words).color(c.accent).underline().append_to(
        &mut job,
        &style,
        egui::FontSelection::Style(egui::TextStyle::Body),
        egui::Align::Center,
    );
    job.wrap.max_width = (ui.available_width() - icon.size().x).max(1.0);
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(
            icon.size().x + galley.size().x,
            galley.size().y.max(design::XL),
        ),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Link,
            ui.is_enabled(),
            format!("{} {words}", design::icons::WARN),
        )
    });
    if ui.is_rect_visible(rect) {
        let top = rect.center().y - galley.size().y / 2.0;
        ui.painter()
            .galley(egui::pos2(rect.left(), top), icon.clone(), c.result.warning);
        ui.painter().galley(
            egui::pos2(rect.left() + icon.size().x, top),
            galley,
            c.accent,
        );
        if response.has_focus() {
            design::focus_ring(ui, rect);
        }
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn empty_optional(value: &Value) -> bool {
    value.is_null() || value.as_str().is_some_and(|text| text.trim().is_empty())
}

/// The message's action starts at its words, below them, as every message action does.
fn unread_groups_message(
    ui: &mut egui::Ui,
    campaign: &Campaign,
    path: &str,
    reason: &str,
    whole: &str,
) {
    // `design::message_with_actions`'s layout, with the message's own response kept: its words
    // carry the whole sentence's hover and description (round 25 FD4).
    let open = ui
        .scope(|ui| {
            ui.set_max_width(ui.available_width().min(design::LIST_MEASURE));
            let response = design::message(ui, Tone::Warning, reason);
            whole_value(ui, &response, reason, whole);
            let inset = response.rect.left() - ui.cursor().left();
            ui.horizontal_top(|ui| {
                ui.add_space(inset);
                design::link(ui, "Edit in Obsidian").clicked()
            })
            .inner
        })
        .inner;
    if open {
        ui.ctx().open_url(egui::OpenUrl::new_tab(obsidian_url(
            &campaign.path.join(path),
        )));
    }
}

/// A warning that quotes an unread value cut around `…` (round 25 FD4): its hover and its
/// accessible description keep the sentence with the value whole.
fn whole_value(ui: &egui::Ui, response: &egui::Response, shown: &str, whole: &str) {
    if shown != whole {
        response.clone().on_hover_text(whole);
        ui.ctx()
            .accesskit_node_builder(response.id, |node| node.set_description(whole));
    }
}

/// An unread-value warning, cut as `whole_value` says.
fn unread_warning(ui: &mut egui::Ui, shown: &str, whole: &str) {
    let response = design::message(ui, Tone::Warning, shown);
    whole_value(ui, &response, shown, whole);
}

fn reference(value: &Value) -> Option<Reference> {
    serde_yaml_ng::from_value::<EntryRef>(value.clone())
        .ok()?
        .reference()
        .ok()
}

pub fn world_type(entry: &Entry) -> &str {
    world_field(entry, "type")
        .and_then(Value::as_str)
        .unwrap_or("")
}

pub fn status(entry: &Entry) -> &str {
    world_field(entry, "status")
        .and_then(Value::as_str)
        .unwrap_or("")
}

pub fn aliases(entry: &Entry) -> Vec<&str> {
    world_field(entry, "aliases")
        .and_then(Value::as_sequence)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect()
}

/// What this entry belongs to, as its file wrote each group.
pub fn part_of(entry: &Entry) -> Vec<&Value> {
    world_field(entry, "part_of")
        .and_then(Value::as_sequence)
        .into_iter()
        .flatten()
        .collect()
}

/// An event's date, as its file wrote it: `120 years ago`.
pub fn when(entry: &Entry) -> Option<&str> {
    world_field(entry, "when").and_then(Value::as_str)
}

pub fn until(entry: &Entry) -> Option<&str> {
    world_field(entry, "until").and_then(Value::as_str)
}

/// How long before Day 1 a date is, in minutes of the campaign clock. A date counts from
/// Day 1, so it does not move as the campaign runs.
pub fn ago(date: &str) -> Option<u64> {
    let date = date.trim().to_lowercase();
    let date = date.strip_prefix("about ").unwrap_or(&date);
    timeline::read_duration(date.strip_suffix(" ago")?.trim())
}

/// An event's date and its end, for a list: `120 to 90 years ago`.
pub fn date_words(entry: &Entry) -> Option<String> {
    let when = when(entry)?.trim();
    let Some(until) = until(entry)
        .map(str::trim)
        .filter(|until| !until.is_empty())
    else {
        return Some(when.to_owned());
    };
    // The same unit is said once.
    let short = when
        .strip_suffix(" ago")
        .and_then(|start| start.rsplit_once(' '))
        .filter(|(_, unit)| until.ends_with(&format!(" {unit} ago")))
        .map(|(number, _)| number);
    Some(match short {
        Some(number) => format!("{number} to {until}"),
        None => format!("{when} to {until}"),
    })
}

/// Any entry with a stat block can join a fight. An NPC can choose one when it does.
pub fn fights(entry: &Entry) -> bool {
    world_type(entry) == "npc" || world_field(entry, "creature").is_some_and(|v| !v.is_null())
}

/// A fight that nobody planned: what a reader's or a scene's `Start fight` asks the app for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StartFight {
    /// The NPC's or the creature's name, for the encounter `Fight with <name>`.
    pub name: String,
    /// The stat block that joins: the creature itself, or the NPC's. An NPC with none joins
    /// through the run's form for a creature made up at the table.
    pub creature: Option<Uuid>,
    /// The name is an NPC's, which its stat block's row takes.
    pub npc: bool,
}

/// Whether a Lookup item is a World entry: a world note, or a note of kind `world`.
pub fn is_world(item: &Item) -> bool {
    item.entry.kind == "world" || item.is_world_note()
}

/// Whether `Add to Log` can add a line to this entry's Log: a World entry of the campaign,
/// as Edit can change it. A package's entry has no action: its Log would belong to every
/// campaign that uses the package.
pub fn takes_log(item: &Item, index: &ContentIndex) -> bool {
    edit::editable(item, index)
}

/// A creature or an NPC that a fight can start with. A party member's copy is neither.
pub fn start_fight(item: &Item, index: &ContentIndex) -> Option<StartFight> {
    let entry = &item.entry;
    if is_world(item) {
        if !fights(entry) {
            return None;
        }
        return Some(StartFight {
            name: entry.name.clone(),
            creature: index
                .world_target(entry, "creature")
                .filter(|at| index.items[*at].entry.kind == "creature")
                .map(|at| index.items[at].entry.id),
            npc: true,
        });
    }
    (entry.kind == "creature"
        && crate::lookup::entry_origin(Some(entry), item.is_private())
            != crate::lookup::Origin::Sheet)
        .then(|| StartFight {
            name: entry.name.clone(),
            creature: Some(entry.id),
            npc: false,
        })
}

pub(crate) fn link_name(value: &Value, fallback: &str) -> String {
    value
        .as_str()
        .or_else(|| value.get("link").and_then(Value::as_str))
        .map(|link| {
            link.trim_matches(['[', ']'])
                .split('|')
                .next_back()
                .unwrap_or(link)
                .to_owned()
        })
        .unwrap_or_else(|| fallback.to_owned())
}

fn creature(entry: &Entry) -> Option<Reference> {
    reference(world_field(entry, "creature")?)
}

/// A world entry's type, as a heading and a list say it.
pub fn type_group(kind: &str) -> String {
    match kind {
        "npc" => "NPCs".into(),
        "location" => "Locations".into(),
        // These have no plural.
        "lore" | "history" | "news" | "equipment" | "gear" => type_label(kind),
        other => {
            let label = type_label(other);
            if label.ends_with('y')
                && !label
                    .chars()
                    .rev()
                    .nth(1)
                    .is_some_and(|c| "aeiou".contains(c))
            {
                format!("{}ies", label.trim_end_matches('y'))
            } else if label.ends_with('s') {
                format!("{label}es")
            } else {
                format!("{label}s")
            }
        }
    }
}

pub fn type_label(kind: &str) -> String {
    match kind {
        "npc" => "NPC".to_owned(),
        "" => "World note".to_owned(),
        other => style::human_label(other),
    }
}

/// The fields every new world entry starts with.
fn entry_fields(id: Uuid, name: &str, kind: &str) -> Vec<(&'static str, Value)> {
    let mut fields = vec![
        ("splat_schema", 2.into()),
        ("splat_id", id.to_string().into()),
        ("splat_kind", "world".into()),
        ("splat_name", name.into()),
        ("splat_type", kind.into()),
    ];
    if kind == "npc" {
        fields.push(("splat_status", "alive".into()));
    }
    fields
}

fn frontmatter(id: Uuid, name: &str, kind: &str) -> Result<String, String> {
    let fields: Mapping = entry_fields(id, name, kind)
        .into_iter()
        .map(|(key, value)| (key.into(), value))
        .collect();
    let yaml = serde_yaml_ng::to_string(&fields).map_err(|e| e.to_string())?;
    Ok(format!("---\n{yaml}---\n"))
}

/// A new NPC or location from a name typed at the table. Nothing else is required.
pub fn create_entry(campaign: &Campaign, name: &str, kind: &str) -> Result<Uuid, String> {
    create_with_details(campaign, name, kind, "")
}

/// A link on a world note that the DM must fix: its note is missing, or it waits for a choice.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum LinkProblem {
    PlaceNotFound,
    /// The place link matches this many notes, and waits for the DM to choose one.
    PlaceMatches(usize),
    StatBlockNotFound,
    GroupNotFound,
    /// The place's `In` chain leads back to this note.
    PlaceLoop,
}

impl std::fmt::Display for LinkProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PlaceNotFound => f.write_str("Place not found"),
            Self::PlaceMatches(count) => write!(f, "Place has {count} matches"),
            Self::StatBlockNotFound => f.write_str("Stat block not found"),
            Self::GroupNotFound => f.write_str("Group not found"),
            Self::PlaceLoop => f.write_str("Place loop"),
        }
    }
}

/// A list row's words for the links that need fixing. One problem names itself. Several are
/// `broken links` only when every target is missing; a place that waits for a choice, or one in a
/// loop, is not broken.
pub(crate) fn link_problem_words(problems: &[LinkProblem]) -> String {
    match problems {
        [one] => one.to_string(),
        many if many.iter().any(|problem| {
            matches!(
                problem,
                LinkProblem::PlaceMatches(_) | LinkProblem::PlaceLoop
            )
        }) =>
        {
            design::count(many.len(), "link needs fixing", "links need fixing")
        }
        many => design::count(many.len(), "broken link", "broken links"),
    }
}

/// The links on this note that need fixing, in the order the reader shows them.
pub(crate) fn broken_links(item: &Item, index: &ContentIndex) -> Vec<LinkProblem> {
    let mut problems = Vec::new();
    if reader_field(&item.entry, "in").is_some() && index.place_of(item.entry.id).is_none() {
        problems.push(match index.place_choices(item.entry.id).len() {
            0 => LinkProblem::PlaceNotFound,
            count => LinkProblem::PlaceMatches(count),
        });
    }
    if index.in_place_loop(item.entry.id) {
        problems.push(LinkProblem::PlaceLoop);
    }
    if reader_field(&item.entry, "creature").is_some()
        && index.world_target(&item.entry, "creature").is_none()
    {
        problems.push(LinkProblem::StatBlockNotFound);
    }
    // The index found each group when it loaded. Its list follows the file's order; a value it
    // did not see (two files with one id) is looked up again.
    let found = index.groups_of(&item.entry);
    for (at, value) in part_of(&item.entry).into_iter().enumerate() {
        if empty_optional(value) {
            continue;
        }
        let target = match found.get(at) {
            Some((name, target)) if *name == link_name(value, "Group") => *target,
            _ => index.world_value_target(value),
        };
        if target.is_none() {
            problems.push(LinkProblem::GroupNotFound);
        }
    }
    problems
}

/// A new NPC from the generator, with its attributes as the note's text.
pub fn create_generated(
    campaign: &Campaign,
    npc: &crate::npc_generator::Generated,
) -> Result<Uuid, String> {
    create_with_details(campaign, &npc.name, "npc", &npc.details())
}

/// The folder that holds most entries of this type, from each entry's path and type. Folders
/// belong to the DM, so a new entry goes where its kind already is.
pub(crate) fn common_folder<'a>(
    entries: impl Iterator<Item = (&'a str, &'a str)>,
    kind: &str,
) -> Option<String> {
    let mut folders: std::collections::BTreeMap<&str, usize> = Default::default();
    for (path, _) in entries.filter(|(_, other)| !kind.is_empty() && *other == kind) {
        *folders
            .entry(path.rsplit_once('/').map_or("", |(folder, _)| folder))
            .or_default() += 1;
    }
    // Of two equal folders, the first by name, so the answer does not change between loads.
    folders
        .into_iter()
        .max_by_key(|&(folder, count)| (count, std::cmp::Reverse(folder)))
        .map(|(folder, _)| folder.to_owned())
}

/// The relative path of a new entry with this name in this folder. An empty folder is the
/// campaign's root.
fn path_in(folder: &str, name: &str) -> String {
    let stem = sessions::file_safe(name.trim());
    if folder.is_empty() {
        format!("{stem}.md")
    } else {
        format!("{folder}/{stem}.md")
    }
}

/// Where a new entry of this kind and name will be written, from the campaign's notes as the
/// index read them. It uses the folder rule of the write, so a card can promise the real path.
pub(crate) fn new_entry_path(index: Option<&ContentIndex>, name: &str, kind: &str) -> String {
    let folder = index
        .and_then(|index| index.type_folder(kind))
        .unwrap_or_else(|| default_folder(kind));
    path_in(&folder, name)
}

/// Where a new entry of this kind is written when the campaign has none yet.
fn default_folder(kind: &str) -> String {
    match kind {
        "" => "Notes".into(),
        kind => sessions::file_safe(&type_group(kind)),
    }
}

/// Where a new entry of this kind is written, from the files as they are now.
fn folder_for(campaign: &Campaign, kind: &str) -> String {
    let entries = splat::content::read_directory(&campaign.path)
        .map(|(entries, _)| entries)
        .unwrap_or_default();
    common_folder(
        entries
            .iter()
            .filter(|entry| entry.kind == "world")
            .map(|entry| (entry.path.as_str(), world_type(entry))),
        kind,
    )
    .unwrap_or_else(|| default_folder(kind))
}

/// The link to an entry just made by `create_entry` or `create_generated`, before the index
/// reads it. Its bare name serves when that is the file's name and nothing else has it.
pub(crate) fn new_entry_link(index: Option<&ContentIndex>, name: &str, kind: &str) -> String {
    let name = name.trim();
    let stem = sessions::file_safe(name);
    let taken = index.is_some_and(|index| {
        index.has_file_named(&stem) || !matches!(index.resolve_link(name, None), Go::Missing(_))
    });
    if stem == name && !taken {
        format!("[[{name}]]")
    } else {
        let shown: String = name
            .chars()
            .filter(|c| !matches!(c, '|' | '[' | ']'))
            .collect();
        let folder = index
            .and_then(|index| index.type_folder(kind))
            .unwrap_or_else(|| default_folder(kind));
        if folder.is_empty() {
            format!("[[{stem}|{shown}]]")
        } else {
            format!("[[{folder}/{stem}|{shown}]]")
        }
    }
}

fn create_with_details(
    campaign: &Campaign,
    name: &str,
    kind: &str,
    details: &str,
) -> Result<Uuid, String> {
    create_at(campaign, name, kind, details).map(|(id, _)| id)
}

/// The new entry's identity and its path in the campaign.
fn create_at(
    campaign: &Campaign,
    name: &str,
    kind: &str,
    details: &str,
) -> Result<(Uuid, String), String> {
    let name = name.trim();
    let stem = sessions::file_safe(name);
    if stem.is_empty() {
        return Err("Type a name first.".to_owned());
    }
    let folder = folder_for(campaign, kind);
    let relative = path_in(&folder, name);
    let path = note_path(&campaign.path, &relative)?;
    if path.exists() {
        return Err(format!("{relative} already exists."));
    }
    let id = crate::hermetic::new_uuid();
    let mut text = if kind.is_empty() {
        format!("# {name}\n")
    } else {
        format!("{}\n# {name}\n", frontmatter(id, name, kind)?)
    };
    if !details.is_empty() {
        text.push('\n');
        text.push_str(details);
    }
    let failed = file_fault::write_error(&relative);
    fs::create_dir_all(campaign.path.join(&folder)).map_err(&failed)?;
    if kind.is_empty() {
        use std::io::Write;
        let mut temp =
            tempfile::NamedTempFile::new_in(path.parent().ok_or("Missing parent folder")?)
                .map_err(&failed)?;
        temp.write_all(text.as_bytes())
            .and_then(|_| temp.as_file().sync_all())
            .map_err(&failed)?;
        temp.persist_noclobber(&path)
            .map_err(|error| failed(error.error))?;
    } else {
        write_note(&campaign.path, &relative, None, &text)?;
    }
    drop(failed);
    let id = if kind.is_empty() {
        note_id(&relative)
    } else {
        id
    };
    Ok((id, relative))
}

/// What giving a note a type gives the DM, in the DM's words. It does not repeat the section
/// heading.
pub(crate) const TYPE_HELP: &str = "A type adds the fields for that kind of note, such as an NPC’s place and stat block. Its text stays the same.";
/// How Splat stores the type, for Technical details only.
pub(crate) const TYPE_STORAGE: &str = "Splat adds fields that start with splat_ (the type, name and an ID) to the front matter \
     at the top of the file.";

/// Give a plain note an identity. Its body does not change.
pub fn make_entry(campaign: &Campaign, item: &Item, kind: &str) -> Result<Uuid, String> {
    let relative = &item.entry.path;
    let text = file_fault::read_note(&campaign.path, relative)?;
    let id = crate::hermetic::new_uuid();
    let name = &item.entry.name;
    let updated = if sessions::split_note(&text).is_some() {
        let changes = entry_fields(id, name, kind)
            .into_iter()
            .map(|(key, value)| (key, Some(value)))
            .collect::<Vec<_>>();
        patch_fields(&text, &changes)?
    } else {
        format!("{}\n{text}", frontmatter(id, name, kind)?)
    };
    write_note(&campaign.path, relative, Some(&text), &updated)?;
    Ok(id)
}

/// What a note keeps to point at an entry: its key, or a link Obsidian can follow too.
fn stored_link(entry: &Entry) -> String {
    entry
        .key
        .clone()
        .unwrap_or_else(|| format!("[[{}|{}]]", entry.path.trim_end_matches(".md"), entry.name))
}

/// Change one field of a world entry's `splat_world`, keeping the rest of the note.
fn set_world_field(
    campaign: &Campaign,
    entry: &Entry,
    name: &str,
    value: Value,
) -> Result<Change, String> {
    let text = file_fault::read_note(&campaign.path, &entry.path)?;
    Entry::parse(&entry.path, &text)?;
    let updated = fields::patch(&text, name, value)?;
    write_note(&campaign.path, &entry.path, Some(&text), &updated)?;
    Ok(Change {
        campaign: campaign.id,
        path: entry.path.clone(),
        before: text,
        after: updated,
        word: "World value saved.".into(),
        kind: ChangeKind::of_key(name),
    })
}

/// What a Log line says about one changed value. A place and a group are links, so the line
/// also shows on their pages.
fn log_words(index: &ContentIndex, key: &str, before: Option<&Value>, after: &Value) -> String {
    let label = match key {
        "in" => "In",
        "part_of" => "Part of",
        _ => "Status",
    };
    let one = |value: &Value| -> String {
        if key == "status" {
            return match value.as_str().map(str::trim) {
                Some(word) if !word.is_empty() => word.to_owned(),
                _ => "not set".into(),
            };
        }
        if value.is_null() {
            return "not set".into();
        }
        index.world_value_target(value).map_or_else(
            || format!("[[{}]]", link_name(value, label)),
            |at| index.link_to(at),
        )
    };
    let all = |value: &Value| match value.as_sequence() {
        Some(groups) if groups.is_empty() => "not set".into(),
        Some(groups) => groups.iter().map(&one).collect::<Vec<_>>().join(", "),
        None => one(value),
    };
    format!(
        "{label} {} → {}.",
        before.map_or_else(|| "not set".into(), all),
        all(after)
    )
}

/// A value change's result as a sentence (round 26 FE3): `Stat block set to Guard.`, `Place set
/// to North Gate. It was Watch House.`, `Place cleared. It was Ashfall Keep.` The arrow form stays in
/// the note's Log lines only (`log_words`). `None` is a value that is not set.
fn change_words(label: &str, before: Option<&str>, after: Option<&str>) -> String {
    match (before, after) {
        (None, Some(after)) => format!("{label} set to {after}."),
        (Some(before), Some(after)) => format!("{label} set to {after}. It was {before}."),
        (Some(before), None) => format!("{label} cleared. It was {before}."),
        (None, None) => format!("{label} cleared."),
    }
}

/// One NPC, as an encounter group of one, so a run adds them the way it adds any group.
pub fn npc_group(npc: &Entry, stat_block: &Entry, side: &str) -> Arc<RunGroup> {
    let overrides = Overrides {
        name: Some(npc.name.clone()),
        ..Default::default()
    };
    let problem = match overrides.resolve_for(stat_block, side) {
        Err(error) => Some(error),
        Ok(resolved) if resolved.max_hp.is_none() => Some(format!(
            "{} has no maximum HP, so choose another stat block.",
            stat_block.name
        )),
        Ok(_) => None,
    };
    let mut frozen = stat_block.clone();
    frozen
        .fields
        .insert("splat_run_npc_name".into(), npc.name.clone().into());
    Arc::new(RunGroup {
        id: npc.id,
        path: npc.path.clone(),
        name: npc.name.clone(),
        side: side.to_owned(),
        encounter: None,
        encounter_name: None,
        members: vec![GroupMember {
            entry: Arc::new(frozen),
            count: 1,
            overrides,
        }],
        notes: String::new(),
        xp: 0,
        xp_unknown: Vec::new(),
        unavailable: 0,
        unavailable_members: Vec::new(),
        warnings: Vec::new(),
        problem,
    })
}

/// Always offer both routes: Obsidian for editing and the system file handler.
pub(crate) fn file_actions(ui: &mut egui::Ui, campaign: &Campaign, relative: &str) {
    entry_file_actions(ui, &campaign.path, relative, true);
}

pub(crate) fn entry_file_actions(ui: &mut egui::Ui, root: &Path, relative: &str, obsidian: bool) {
    // When even `…/<file name>` does not fit beside the links, the path takes its own row and
    // the links the row below, as in Library: the file name is never cut (FA4).
    let budget = ui.available_width() - entry_file_links_width(ui, obsidian) - design::L;
    let filename = relative.rsplit('/').next().unwrap_or(relative);
    if design::number_width(ui, &format!("…/{filename}"), &Type::Small.font(ui)) > budget {
        let path = root.join(relative);
        let font = Type::Small.font(ui);
        let shown = if design::number_width(ui, relative, &font) <= ui.available_width() {
            relative.to_owned()
        } else {
            format!("…/{filename}")
        };
        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), Density::Read.control()),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                let response = ui.add(
                    egui::Label::new(Type::Small.rich(ui, shown).color(colors().muted)).wrap(),
                );
                design::named(ui, &response, &path.display().to_string());
                response.on_hover_text(path.display().to_string());
            },
        );
        ui.horizontal(|ui| entry_file_links(ui, &path, obsidian));
    } else {
        ui.horizontal_wrapped(|ui| entry_file_parts(ui, root, relative, obsidian));
    }
}

pub(crate) fn entry_file_parts(ui: &mut egui::Ui, root: &Path, relative: &str, obsidian: bool) {
    let path = root.join(relative);
    let action_width = entry_file_links_width(ui, obsidian);
    let budget = (ui.available_width() - action_width - design::L).max(0.0);
    let font = Type::Small.font(ui);
    let mut shown = relative.to_owned();
    if design::number_width(ui, &shown, &font) > budget {
        let filename = relative.rsplit('/').next().unwrap_or(relative);
        shown = format!("…/{filename}");
        if design::number_width(ui, &shown, &font) > budget {
            shown = design::words_that_fit(ui, filename, &font, budget);
            shown = shown.trim_end_matches('…').to_owned();
            while !shown.is_empty()
                && design::number_width(ui, &format!("{shown}…"), &font) > budget
            {
                shown.pop();
            }
            shown.push('…');
        }
    }
    let response = ui.label(Type::Small.rich(ui, shown).color(colors().muted));
    design::named(ui, &response, &path.display().to_string());
    response.on_hover_text(path.display().to_string());
    entry_file_links(ui, &path, obsidian);
}

/// The width of a file line's links, with the gap between them.
pub(crate) fn entry_file_links_width(ui: &egui::Ui, obsidian: bool) -> f32 {
    design::number_width(ui, "Show file", &Type::Body.font(ui))
        + if obsidian {
            design::number_width(ui, "Edit in Obsidian", &Type::Body.font(ui)) + design::S
        } else {
            0.0
        }
}

/// `Edit in Obsidian` and `Show file` for a file at this full path.
pub(crate) fn entry_file_links(ui: &mut egui::Ui, path: &Path, obsidian: bool) {
    if obsidian && design::link(ui, "Edit in Obsidian").clicked() {
        ui.ctx()
            .open_url(egui::OpenUrl::new_tab(obsidian_url(path)));
    }
    if design::link(ui, "Show file").clicked() {
        ui.ctx().open_url(egui::OpenUrl::new_tab(format!(
            "file://{}",
            encode_path(path)
        )));
    }
}

fn encode_path(path: &Path) -> String {
    path.to_string_lossy()
        .bytes()
        .map(|byte| {
            if byte.is_ascii_alphanumeric() || b"-._~/".contains(&byte) {
                (byte as char).to_string()
            } else {
                format!("%{byte:02X}")
            }
        })
        .collect()
}

fn obsidian_url(path: &Path) -> String {
    format!("obsidian://open?path={}", encode_path(path))
}

pub(crate) fn make_npc_note(campaign: &Campaign, entry: &Entry) -> Result<Uuid, String> {
    let (id, path) = create_at(campaign, &entry.name, "npc", "")?;
    let text = file_fault::read_note(&campaign.path, &path)?;
    let npc = Entry::parse(&path, &text)?;
    set_world_field(
        campaign,
        &npc,
        "creature",
        serde_yaml_ng::to_value(EntryRef::id(entry.id)).map_err(|e| e.to_string())?,
    )?;
    Ok(id)
}

#[derive(Default)]
struct Picker {
    key: &'static str,
    search: String,
    new_location: String,
    focus: bool,
}

#[derive(Clone)]
pub(crate) struct Change {
    /// The campaign whose folder holds the note. The result shows, and Undo acts, only there:
    /// Lookup keeps its reader across a campaign change.
    campaign: Uuid,
    path: String,
    before: String,
    after: String,
    word: String,
    kind: ChangeKind,
}

/// What a change touched, which decides its Undo words and the row that shows it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum ChangeKind {
    Place,
    PartOf,
    StatBlock,
    Status,
    Name,
    Identity(IdentityNoun),
    /// Everything the edit form saved at once.
    Edit,
    /// One line that `Add to Log` added, in the DM's words.
    Log,
}

impl ChangeKind {
    /// The kind for one `splat_world` key.
    fn of_key(key: &str) -> Self {
        match key {
            "in" => Self::Place,
            "part_of" => Self::PartOf,
            "status" => Self::Status,
            _ => Self::StatBlock,
        }
    }

    fn undo_label(self) -> &'static str {
        match self {
            Self::Place => "Undo place change",
            Self::PartOf => "Undo group change",
            Self::Status => "Undo status change",
            Self::StatBlock => "Undo stat block change",
            Self::Name => "Undo name change",
            Self::Identity(noun) => noun.undo_label(),
            Self::Edit => "Undo changes",
            Self::Log => "Undo Log line",
        }
    }
}

impl Change {
    pub(crate) fn identity_undo_label(&self) -> &'static str {
        self.kind.undo_label()
    }

    pub(crate) fn message(&self) -> &str {
        &self.word
    }

    /// Whether this change's result belongs on this campaign's note at `path`.
    pub(crate) fn shows_on(&self, campaign: &Campaign, path: &str) -> bool {
        self.campaign == campaign.id && self.path == path
    }

    pub(crate) fn undo(&self, campaign: &Campaign) -> Result<(), String> {
        if campaign.id != self.campaign {
            return Err("This change belongs to another campaign. Nothing was changed.".into());
        }
        write_note(&campaign.path, &self.path, Some(&self.after), &self.before).map(|_| ())
    }
}

/// A note's file renamed to match its name (FD5). Undo moves it back. Neither move replaces a
/// file, and each checks that the file still holds the text the DM saw.
pub(crate) struct FileRename {
    /// The campaign whose folder holds the file. Undo acts only there.
    campaign: Uuid,
    from: String,
    to: String,
    text: String,
}

impl FileRename {
    /// `seen` is the file's text that produced the reader: the move refuses unless the file
    /// still holds it.
    fn apply(campaign: &Campaign, from: &str, to: &str, seen: &str) -> Result<Self, String> {
        move_note(&campaign.path, from, to, seen, || {})?;
        Ok(Self {
            campaign: campaign.id,
            from: from.into(),
            to: to.into(),
            text: seen.into(),
        })
    }

    /// Whether this rename's result belongs on this campaign's note at `path`.
    fn shows_on(&self, campaign: &Campaign, path: &str) -> bool {
        self.campaign == campaign.id && self.to == path
    }

    fn undo(&self, campaign: &Campaign) -> Result<(), String> {
        if campaign.id != self.campaign {
            return Err("This rename belongs to another campaign. Nothing was renamed.".into());
        }
        move_note(&campaign.path, &self.to, &self.from, &self.text, || {})
    }

    fn message(&self) -> String {
        let file = self.to.rsplit('/').next().unwrap_or(&self.to);
        format!("The file is now {file}.")
    }
}

/// The file beside `path` for a note named `name`, when the name can be a file name.
fn renamed_path(path: &str, name: &str) -> Option<String> {
    let name = name.trim();
    if name.is_empty()
        || name.starts_with('.')
        || name.contains([
            '/', '\\', ':', '*', '?', '"', '<', '>', '|', '#', '^', '[', ']',
        ])
    {
        return None;
    }
    let to = match path.rsplit_once('/') {
        Some((folder, _)) => format!("{folder}/{name}.md"),
        None => format!("{name}.md"),
    };
    (to != path && crate::library::note_path_rule(&to).is_none()).then_some(to)
}

/// How a note file moves. `rename` never replaces a file; `link` makes a second name and fails
/// when the target exists. Tests give other ones to stand in for a file system.
struct Moves {
    rename: fn(&Path, &Path) -> std::io::Result<()>,
    link: fn(&Path, &Path) -> std::io::Result<()>,
    remove: fn(&Path) -> std::io::Result<()>,
}

const SYSTEM_MOVES: Moves = Moves {
    #[cfg(target_os = "linux")]
    rename: |source, target| crate::sessions::rename_no_replace(source, target),
    #[cfg(not(target_os = "linux"))]
    rename: |source, target| std::fs::rename(source, target),
    link: |source, target| std::fs::hard_link(source, target),
    remove: |path| std::fs::remove_file(path),
};

/// The words when this folder's file system has no move that is safe for an existing file.
const NO_SAFE_MOVE: &str = "Splat cannot rename files safely in this folder, so nothing was moved.";

/// The words when a move left one file with two names and Splat could remove neither.
fn two_names(from: &str, to: &str) -> String {
    format!(
        "{from} and {to} are now two names for one note, and Splat could remove neither. Delete {to} in your file manager to keep the note as it was."
    )
}

/// The file system does not offer this kind of move (not a fault of these files).
fn move_unsupported(error: &std::io::Error) -> bool {
    error.kind() == std::io::ErrorKind::Unsupported
        || matches!(
            error.raw_os_error(),
            Some(libc::EINVAL | libc::ENOSYS | libc::EOPNOTSUPP)
        )
}

/// Moves a note file within the campaign. It refuses when `from` no longer holds `expected` or
/// a file is at `to`, so it never replaces or loses a file. The move itself never replaces a
/// file either (`RENAME_NOREPLACE`), so a file that arrives at `to` after the check keeps its
/// bytes. Where the file system does not offer that move, a hard link (which also refuses an
/// existing target) and then the removal of the old name move it; where it offers neither, it
/// refuses and nothing moves. `before_move` runs between the checks and the move, for tests.
fn move_note(
    root: &Path,
    from: &str,
    to: &str,
    expected: &str,
    before_move: impl FnOnce(),
) -> Result<(), String> {
    move_note_with(&SYSTEM_MOVES, root, from, to, expected, before_move)
}

fn move_note_with(
    moves: &Moves,
    root: &Path,
    from: &str,
    to: &str,
    expected: &str,
    before_move: impl FnOnce(),
) -> Result<(), String> {
    let source = crate::library::note_path(root, from)?;
    let target = crate::library::note_path(root, to)?;
    if file_fault::read_note(root, from)? != expected {
        return Err(format!("{from} changed on disk. Nothing was renamed."));
    }
    let exists = || format!("{to} already exists. Nothing was renamed.");
    if std::fs::symlink_metadata(&target).is_ok() {
        return Err(exists());
    }
    before_move();
    let failed = |error: std::io::Error| {
        if error.kind() == std::io::ErrorKind::AlreadyExists {
            exists()
        } else {
            file_fault::FileFault::write(from, &error).message()
        }
    };
    match (moves.rename)(&source, &target) {
        Ok(()) => Ok(()),
        Err(error) if move_unsupported(&error) => match (moves.link)(&source, &target) {
            // Both names now hold one file. The old name goes; if it cannot, the new one goes,
            // so nothing moved; if neither can, the DM is told both names and what to do.
            Ok(()) => match (moves.remove)(&source) {
                Ok(()) => Ok(()),
                Err(error) => match (moves.remove)(&target) {
                    Ok(()) => Err(format!(
                        "{} Nothing was renamed.",
                        file_fault::FileFault::write(from, &error).headline()
                    )),
                    Err(_) => Err(two_names(from, to)),
                },
            },
            Err(error) if move_unsupported(&error) || error.raw_os_error() == Some(libc::EPERM) => {
                Err(NO_SAFE_MOVE.into())
            }
            Err(error) => Err(failed(error)),
        },
        Err(error) => Err(failed(error)),
    }
}

/// World notes and Library entries share identity repair, but keep their own nouns.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum IdentityNoun {
    Note,
    Entry,
}

impl IdentityNoun {
    pub(crate) fn of(entry: &Entry) -> Self {
        if entry.kind == "world" {
            Self::Note
        } else {
            Self::Entry
        }
    }

    pub(crate) fn word(self) -> &'static str {
        match self {
            Self::Note => "note",
            Self::Entry => "entry",
        }
    }

    pub(crate) fn duplicate_warning(self) -> &'static str {
        match self {
            Self::Note => "These two files claim to be the same note.",
            Self::Entry => DUPLICATE_ENTRY,
        }
    }

    pub(crate) fn action(self, filename: &str) -> String {
        format!("Make {filename} a separate {}", self.word())
    }

    fn result(self, filename: &str) -> String {
        format!("{filename} is now a separate {}.", self.word())
    }

    fn undo_label(self) -> &'static str {
        match self {
            Self::Note => "Undo separate note",
            Self::Entry => SEPARATE_ENTRY_UNDO,
        }
    }
}

pub(crate) const DUPLICATE_ENTRY: &str = "These two files claim to be the same entry.";
pub(crate) const SEPARATE_ENTRY_UNDO: &str = "Undo separate entry";

pub(crate) fn separate_entry_action(filename: &str) -> String {
    IdentityNoun::Entry.action(filename)
}

/// Long filenames keep the complete repair action inside its secondary border.
pub(crate) fn separate_entry_button(ui: &mut egui::Ui, filename: &str) -> bool {
    let label = IdentityNoun::Note.action(filename);
    if Action::secondary(&label).natural_width(ui) <= ui.available_width() {
        return Action::secondary(&label).show(ui).clicked();
    }
    let response = ui.add(
        egui::Button::new(Type::Strong.rich(ui, &label))
            .wrap()
            .min_size(egui::vec2(0.0, Density::Scan.control()))
            .fill(egui::Color32::TRANSPARENT)
            .stroke(egui::Stroke::new(
                design::HAIRLINE_WIDTH,
                colors().field_border,
            )),
    );
    design::named(ui, &response, &label);
    response.clicked()
}

pub(crate) fn separate_entry_result(filename: &str) -> String {
    IdentityNoun::Entry.result(filename)
}

pub(crate) fn new_identity(campaign: &Campaign, relative: &str) -> Result<(Change, Uuid), String> {
    let before = file_fault::read_note(&campaign.path, relative)?;
    let noun = IdentityNoun::of(&Entry::parse(relative, &before)?);
    let id = crate::hermetic::new_uuid();
    let after = crate::content_workspace::new_note_identity(&before, relative, id)?;
    write_note(&campaign.path, relative, Some(&before), &after)?;
    Ok((
        Change {
            campaign: campaign.id,
            path: relative.into(),
            before,
            after,
            word: noun.result(
                Path::new(relative)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .as_ref(),
            ),
            kind: ChangeKind::Identity(noun),
        },
        id,
    ))
}

pub(crate) fn repair_in_world(path: &Path) -> bool {
    fs::read_to_string(path)
        .ok()
        .and_then(|text| Entry::parse("note.md", &text).ok())
        .is_some_and(|entry| entry.kind == "world")
}

// --- Reading.

/// What a run can tell a reader. `None` means that no run is open.
pub type RunSides = Option<Vec<String>>;

pub struct Join {
    pub group: Arc<RunGroup>,
    pub total: Option<i64>,
    /// The NPC just saved a stat block, so the index is out of date.
    pub saved: bool,
}

/// What pressing something in a reader asks for.
pub enum Go {
    Item(usize, Option<String>),
    Many(Vec<usize>),
    Missing(String),
    Timeline(String),
    Reload(Uuid),
    Join(Join),
}

/// What only the page holding a reader can do.
pub enum Nav {
    Item(usize),
    /// A link to a note that does not exist. The DM may want to create it.
    Missing(String),
    Reload(Uuid),
}

#[derive(Default)]
struct Adding {
    stat_block: Option<usize>,
    search: String,
    save: bool,
    side: String,
    initiative: String,
}

enum FormEnd {
    Cancel,
    Add,
}

/// One index generation's source-line lookup, and the entry without the flags drawn below the
/// file line.
struct GroupFault {
    id: Uuid,
    generation: u64,
    /// The Part of list on disk that could not be read.
    unread: Option<fields::Unread>,
    /// Every other value not read, as the reader words it, with a long value cut, then whole.
    /// They sit with the Part of reason.
    notices: Vec<(String, String)>,
    /// The entry for the shared reader, without its flags. None when it has none.
    entry: Option<Entry>,
}

/// The kind of the Part of flag, which World shows with its Change action.
const PART_OF: FlagKind = FlagKind::Unread(Field::PartOf);

/// True when the note's Status value was not read.
fn status_unread(entry: &Entry) -> bool {
    entry
        .flags
        .iter()
        .any(|flag| flag.kind == FlagKind::Unread(Field::Status))
}

/// A value the reader could not read, as the shared reader says it (`src/library/view.rs`):
/// the sentence with a long value cut, then whole.
fn flag_words(flag: &splat::content::layout::Flag) -> (String, String) {
    (copy_words(flag.short_message()), copy_words(&flag.message))
}

#[cfg(test)]
thread_local! {
    /// `copy_words` calls on this (the UI) thread.
    static WORD_COPIES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// How often an unread warning's words were copied out of a flag on this thread.
#[cfg(test)]
pub(crate) fn word_copies() -> usize {
    WORD_COPIES.with(std::cell::Cell::get)
}

/// The one copy of an unread warning's words, made when its fault is cached. The reader draws
/// from a borrow, so an unchanged frame makes none.
pub(super) fn copy_words(words: &str) -> String {
    #[cfg(test)]
    WORD_COPIES.with(|count| count.set(count.get() + 1));
    words.to_owned()
}

impl GroupFault {
    /// The flags of `entry` other than the Part of one, and the entry without any flags.
    fn flags(
        entry: &Entry,
        unread: Option<&fields::Unread>,
    ) -> (Vec<(String, String)>, Option<Entry>) {
        if entry.flags.is_empty() {
            return (Vec::new(), None);
        }
        let notices = entry
            .flags
            .iter()
            // The Part of flag is the one with its line and kind; another flag can share a line.
            .filter(|flag| {
                unread.is_none_or(|unread| unread.line != flag.line || flag.kind != PART_OF)
            })
            .map(flag_words)
            .collect();
        let mut entry = entry.clone();
        entry.flags.clear();
        (notices, Some(entry))
    }
}

/// When a result was made: one count for every World and Lookup result, so a footer lists its
/// results with a live Undo newest first, whatever their kind (round 26 review 3).
pub(crate) fn next_order() -> u64 {
    static ORDER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    ORDER.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1
}

/// Lookup's and the World page's reader, and its controls for the item on show.
#[derive(Default)]
pub struct Reader {
    /// When each kept result was made (`next_order`), by the action its Undo takes.
    orders: HashMap<FooterAct, u64>,
    showing: Option<Uuid>,
    status: String,
    status_editing: bool,
    /// `Change` on Status was pressed: the field takes the keyboard, its words selected, once
    /// the frame of the press is over.
    status_select: bool,
    status_drafts: HashMap<Uuid, String>,
    /// Edit forms by entry. A form with changes stays while the DM reads other entries.
    edits: HashMap<Uuid, edit::EditDraft>,
    /// What Discard dropped, until the DM undoes it or edits again.
    discarded: Option<(Uuid, edit::EditDraft)>,
    /// A field of the form had the keyboard when it last drew.
    form_focused: bool,
    pub compact: bool,
    other_type: String,
    adding: Option<Adding>,
    picker: Option<Picker>,
    undo: Option<Change>,
    /// The form closed with nothing to write: the note's path and its text, while it reads so.
    kept: Option<(String, String)>,
    identity_undo: Option<(Uuid, Change)>,
    name_undo: Option<Change>,
    /// The last `Rename the file to …` on this page, until it is undone or another replaces it.
    file_undo: Option<FileRename>,
    error: Option<String>,
    /// Source lookup is cached by note and index generation, including repairs and moves.
    group_fault: Option<GroupFault>,
    group_error: bool,
    /// The refused Part of write could not read the note at all (no permission, not text), so
    /// no editor can open it either: its error has no `Edit in Obsidian` link.
    group_error_unread_file: bool,
    /// A link that matches several files, waiting for the DM to choose.
    many: Vec<usize>,
    many_heading: Option<String>,
    place_choices_open: bool,
    place_choices_document: Option<Uuid>,
    heading_document: Option<Uuid>,
    heading: Option<String>,
    /// What only the app can do, for it to take after this reader draws.
    pub join: Option<Join>,
    /// `Start fight` in the header, for the app to take after this reader draws.
    pub start_fight: Option<StartFight>,
    /// Why no fight can start now, as the encounter editor's `Start fight` says it.
    pub start_blocked: Option<&'static str>,
    /// The encounter editor is the page beside this reader: its own primary `Start fight` is the
    /// one there, so the header offers none.
    pub beside_editor: bool,
    pub open_timeline: Option<String>,
    pub sides: RunSides,
    /// A run is in live play and owns `Next turn`, the screen's primary action.
    pub live: bool,
    pub in_run: std::collections::BTreeMap<Uuid, (usize, Option<i64>)>,
    pub select_run: Option<usize>,
    /// The session being played, as a Log line names it: `[[Session 1]]`. None when no
    /// session is being played (`played_session`).
    pub played: Option<String>,
    /// The campaign's day on the Timeline's clock, when the clock has one.
    pub day: Option<u64>,
    /// Each open `Add to Log` field, by its entry and the session it was opened in. Its words
    /// wait while the reader shows another entry; a restart loses them, as every held form.
    log_fields: HashMap<LogKey, LogField>,
    /// The Log lines whose entry is gone, for the footer, until the DM copies or dismisses each.
    gone_log_lines: Vec<GoneLog>,
    /// The entries whose edit form is open in the other reader (Lookup's, or the World
    /// page's). Their Log field waits too, so Splat never makes its own disk conflict.
    pub edited_elsewhere: Vec<Uuid>,
}

/// A result that the World page owns, drawn by the reader in the one place for results.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PageResult {
    Repaired,
    /// A generated NPC that Discard let go, by name, with `Undo discard`.
    Discarded(String),
}

/// The tone of a finished change to a note: a save, a value change, a name change, a repair or
/// a separate note. Library's finished changes use it too. A result that wrote nothing (the note
/// already held the changes, a discarded draft) keeps the info tone.
pub(crate) const CHANGED: Tone = Tone::Success;

impl PageResult {
    /// The page's result as a footer line. Its Undo is the page's (`FooterAct::Page`).
    pub(crate) fn line(&self) -> FooterLine {
        match self {
            Self::Repaired => FooterLine::new(
                FooterRank::Notice,
                CHANGED,
                "Note repaired.",
                FooterKind::Plain,
            ),
            Self::Discarded(name) => FooterLine::discarded(name, FooterAct::Page),
        }
    }
}

/// Whose an `Add to Log` field is: the campaign and the entry's ID, nothing else. A copied note
/// in another campaign has the same entry ID, so the campaign is part of the key, and the words
/// never reach another campaign's note. The session is not: the stamp is the session being
/// played at Enter, so a renamed, closed or changed session cannot hide typed words.
pub(crate) type LogKey = (Uuid, Uuid);

/// The typed words of a Log line whose entry is gone: the footer says them, with `Copy text`,
/// until the DM copies or dismisses them.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct GoneLog {
    /// This notice's own number, so two drafts lost from one entry stay two notices.
    id: u64,
    /// The campaign of the entry: the notice shows only there.
    campaign: Uuid,
    name: String,
    /// The words as the DM typed them, byte for byte, for `Copy text`.
    words: String,
}

/// One open `Add to Log` field: its words, and whether its next draw selects them.
#[derive(Default)]
struct LogField {
    field: design::AddField,
    /// The entry's name, for the warnings that list unsaved work.
    name: String,
    /// The words came from a done scene, so typing replaces them and Enter keeps them.
    select: bool,
}

/// Why an open `Add to Log` field waits: an edit form of its entry is open.
pub(crate) const LOG_WAITS: &str = "Save or discard your edit first.";

/// The least room that `Add to Log`'s field keeps for a sentence when its stamp shares its line.
/// It also keeps the words it holds whole, with the field's margins.
const LOG_FIELD_BESIDE: f32 = design::XXL * 7.0;

/// Where a reader's title-line actions go (round 27, one rule for World, Lookup and the
/// encounter editor's header): `None` when the measured title, the `L` gap and every action
/// with its word fit in `room`, so they follow the title on its line, `XS` apart. Else the row,
/// under the title, of each action: all of them move together, in order, and a row takes as
/// many as fit in `line`, the width under the title. An action never goes away and never loses
/// its word, whatever the length of the name. `title` is the title's measured width, `widths`
/// each action's own.
pub(crate) fn title_line_rows(
    room: f32,
    line: f32,
    title: f32,
    widths: &[f32],
) -> Option<Vec<usize>> {
    let all = widths.iter().sum::<f32>() + design::XS * widths.len().saturating_sub(1) as f32;
    // The first target starts `S` before its icon, which sits `L` after the title.
    if widths.is_empty() || title + design::L - design::S + all <= room {
        return None;
    }
    let mut rows = Vec::with_capacity(widths.len());
    let mut row = 0;
    let mut used: Option<f32> = None;
    for width in widths {
        // The first target of a row starts at the text edge: Lookup's reader clips at it.
        let next = used.map_or(*width, |used| used + design::XS + width);
        if used.is_some() && next > line {
            row += 1;
            used = Some(*width);
        } else {
            used = Some(next);
        }
        rows.push(row);
    }
    Some(rows)
}

/// Draw a reader's title-line actions by `title_line_rows`, just after `cell`, which holds the
/// title and its kind line; `title` is the title's measured width and `room` the width from the
/// cell's left edge that the actions may use beside the title (less a portrait). Under the
/// title, the first action's target starts at the text edge, so no part of it is clipped where
/// a pane clips at its text edge (Lookup). Returns which actions were pressed, in order.
pub(crate) fn title_line_actions(
    ui: &mut egui::Ui,
    cell: egui::Rect,
    title: f32,
    room: f32,
    actions: Vec<Action<'_>>,
) -> Vec<bool> {
    let widths: Vec<f32> = actions
        .iter()
        .map(|action| action.natural_width(ui))
        .collect();
    let height = Density::Read.control();
    // The cell's rect is as wide as its words; the line under it has the pane's width.
    let line_width = ui.available_width();
    let rows = title_line_rows(room, line_width, title, &widths);
    let mut places = Vec::with_capacity(widths.len());
    match &rows {
        None => {
            // The line under the title takes one auto ID; so does this layout, so the field
            // and form IDs below stay the same when a resize moves the actions, and the
            // keyboard stays in the field the DM types in.
            ui.skip_ahead_auto_ids(1);
            let mut next = cell.left() + title + design::L - design::S;
            for width in &widths {
                places.push(egui::pos2(next, cell.top()));
                next += width + design::XS;
            }
        }
        Some(rows) => {
            let count = rows.last().map_or(0, |row| row + 1);
            let (line, _) = ui.allocate_exact_size(
                egui::vec2(
                    line_width,
                    height * count as f32 + design::XS * count.saturating_sub(1) as f32,
                ),
                egui::Sense::hover(),
            );
            let mut next = line.left();
            let mut last = 0;
            for (width, row) in widths.iter().zip(rows) {
                if *row != last {
                    next = line.left();
                    last = *row;
                }
                places.push(egui::pos2(
                    next,
                    line.top() + *row as f32 * (height + design::XS),
                ));
                next += width + design::XS;
            }
        }
    }
    actions
        .into_iter()
        .zip(places.into_iter().zip(widths))
        .map(|(action, (at, width))| {
            in_rect(
                ui,
                egui::Rect::from_min_size(at, egui::vec2(width, height)),
                |ui| action.show(ui).clicked(),
            )
        })
        .collect()
}

/// Draw one action in its target, which the caller has already allocated or which lies on a
/// line already allocated: a child that takes no room from `ui`, so a top-down layout never
/// moves its cursor back up to the target.
pub(crate) fn in_rect(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    add: impl FnOnce(&mut egui::Ui) -> bool,
) -> bool {
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    add(&mut child)
}

/// Where a footer line stands: a result with a live Undo, a reason, or another notice.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum FooterRank {
    Undo,
    Reason,
    Notice,
}

/// What a footer line's action does. The pane takes it after the footer draws.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum FooterAct {
    /// Undo the value, edit-form or row change (`Reader::undo`).
    Change,
    FileRename,
    Name,
    Identity,
    /// Bring back the edit draft that Discard let go.
    Discarded,
    /// The page result's Undo: the page owns it.
    Page,
    /// The DM copied or dismissed the words of a Log line whose entry is gone, by its notice.
    GoneLog(u64),
    /// `Copy text` of a draft whose note is gone, by the note's ID.
    LostCopy(Uuid),
    /// `Discard draft` of a draft whose note is gone, by the note's ID.
    LostDiscard(Uuid),
    /// `Undo tick` of an open question, by its note and its line: the World page's.
    Untick(Uuid, usize),
    /// `Undo remove` of a row of a tracker's history: the trackers own it, and the pane takes it.
    Trackers,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum FooterKind {
    Plain,
    Undo {
        label: &'static str,
        act: FooterAct,
    },
    /// `<name> draft discarded.` with `Undo discard`, named for its draft.
    Discarded {
        name: String,
        act: FooterAct,
    },
    /// An action's error, with `Edit in Obsidian` for this file when an editor can open it.
    Error {
        editor: Option<std::path::PathBuf>,
    },
    /// A Log line whose entry is gone, with `Copy text` and `Dismiss`.
    GoneLog(GoneLog),
    /// A draft whose note is gone, with `Copy text` and `Discard draft`.
    LostDraft { id: Uuid, name: String },
}

/// One World or Lookup message for its pane's footer (round 26): a result, reason or notice
/// that comes and goes. It sits below the pane's scroll area, so it never moves the note.
#[derive(Clone)]
pub(crate) struct FooterLine {
    /// When its result was made (`next_order`): newer results come first within a rank.
    pub order: u64,
    pub rank: FooterRank,
    pub tone: Tone,
    pub words: String,
    pub kind: FooterKind,
}

impl FooterLine {
    pub(crate) fn new(rank: FooterRank, tone: Tone, words: &str, kind: FooterKind) -> Self {
        Self {
            order: 0,
            rank,
            tone,
            words: words.to_owned(),
            kind,
        }
    }

    /// The line, made at `order`.
    pub(crate) fn at(mut self, order: u64) -> Self {
        self.order = order;
        self
    }

    pub(super) fn discarded(name: &str, act: FooterAct) -> Self {
        Self::new(
            FooterRank::Undo,
            Tone::Info,
            &format!("{name} draft discarded."),
            FooterKind::Discarded {
                name: name.to_owned(),
                act,
            },
        )
    }

    fn draw(&self, ui: &mut egui::Ui) -> Option<FooterAct> {
        match &self.kind {
            FooterKind::Plain => {
                design::message(ui, self.tone, &self.words);
                None
            }
            FooterKind::Undo { label, act } => {
                design::undo_message(ui, self.tone, &self.words, label).then_some(*act)
            }
            FooterKind::Discarded { name, act } => {
                design::draft_discarded(ui, name).then_some(*act)
            }
            FooterKind::GoneLog(gone) => gone.draw(ui, &self.words, self.tone),
            FooterKind::LostDraft { id, name } => {
                edit::lost_draft(ui, &self.words, self.tone, *id, name)
            }
            FooterKind::Error { editor } => {
                ui.horizontal_wrapped(|ui| {
                    design::message(ui, self.tone, &self.words);
                    if let Some(path) = editor
                        && design::link(ui, "Edit in Obsidian").clicked()
                    {
                        ui.ctx()
                            .open_url(egui::OpenUrl::new_tab(obsidian_url(path)));
                    }
                });
                None
            }
        }
    }
}

/// A pane's footer lines with more from another owner, in the footer's order: results with a
/// live Undo newest first, then reasons, then notices.
pub(crate) fn with_lines(mut lines: Vec<FooterLine>, more: Vec<FooterLine>) -> Vec<FooterLine> {
    lines.extend(more);
    lines.sort_by_key(|line| (line.rank, std::cmp::Reverse(line.order)));
    lines
}

/// Draw a pane's footer lines in its message footer (`design::Footer`), pinned at the bottom of
/// what is left of `ui`, in the footer's order: results with a live Undo, then reasons, then
/// notices. `id` is the pane instance's own. Returns the action pressed and the y where the body
/// drawn next ends.
pub(crate) fn draw_footer(
    ui: &mut egui::Ui,
    id: impl std::hash::Hash,
    lines: &[FooterLine],
) -> (Option<FooterAct>, f32) {
    let pressed = std::cell::Cell::new(None);
    let mut footer = design::Footer::new(id, ui.available_width());
    for line in lines {
        let pressed = &pressed;
        let draw = move |ui: &mut egui::Ui| {
            if let Some(act) = line.draw(ui) {
                pressed.set(Some(act));
            }
        };
        match line.rank {
            FooterRank::Undo => footer.result(line.words.as_str(), true, draw),
            FooterRank::Reason => footer.reason(line.words.as_str(), draw),
            FooterRank::Notice => footer.notice(line.words.as_str(), draw),
        };
    }
    let bottom = footer.show(ui);
    (pressed.get(), bottom)
}

impl Reader {
    /// The stamp that a change's Log line begins with: the session being played and the day,
    /// `[[Session 1]], Day 1`. With no session, a change writes no Log line.
    fn log_as(&self) -> Option<String> {
        let session = self.played.as_ref()?;
        Some(match self.day {
            Some(day) => format!("{session}, Day {day}"),
            None => session.clone(),
        })
    }

    /// The stamp of an `Add to Log` line: a change's stamp, or the day alone when no session
    /// is being played, because the DM asked for a line.
    fn line_stamp(&self) -> Option<String> {
        self.log_as()
            .or_else(|| self.day.map(|day| format!("Day {day}")))
    }

    /// Open `Add to Log` for this entry of this campaign, as a done scene's `Add to Log` and the
    /// header's do. The field starts with `words`, selected, unless the DM already typed words
    /// there.
    pub fn open_log(&mut self, campaign: Uuid, id: Uuid, name: &str, words: &str) {
        let open = self.log_fields.entry((campaign, id)).or_default();
        open.field.focus = true;
        name.clone_into(&mut open.name);
        if open.field.words.trim().is_empty() {
            words.clone_into(&mut open.field.words);
            open.select = !words.is_empty();
        }
    }

    /// Offer the same short chooser to links opened outside the World page.
    pub fn choose_link(&mut self, positions: Vec<usize>, target: &str) {
        self.many = positions;
        self.many_heading = target
            .split('|')
            .next()
            .and_then(|target| target.split_once('#'))
            .map(|(_, heading)| {
                heading
                    .rsplit('#')
                    .next()
                    .unwrap_or(heading)
                    .trim()
                    .to_owned()
            });
    }

    /// Show this heading the next time the item is drawn.
    pub fn show_heading(&mut self, heading: Option<String>) {
        self.heading_document = None;
        self.heading = heading;
    }

    fn settle(&mut self, go: Go) -> Option<Nav> {
        match go {
            Go::Item(position, heading) => {
                self.many.clear();
                self.heading = heading;
                return Some(Nav::Item(position));
            }
            Go::Many(positions) => self.many = positions,
            Go::Missing(words) => return Some(Nav::Missing(words)),
            Go::Timeline(id) => self.open_timeline = Some(id),
            Go::Reload(id) => return Some(Nav::Reload(id)),
            Go::Join(join) => self.join = Some(join),
        }
        None
    }

    /// The choice a link to several files is waiting for. It stays above the scrolling text.
    pub fn choices(&mut self, ui: &mut egui::Ui, index: &ContentIndex) -> Option<Nav> {
        if self.many.is_empty() {
            return None;
        }
        ui.label(
            Type::Small
                .rich(ui, "This link matches more than one note. Choose one.")
                .color(colors().muted),
        );
        let mut chosen = None;
        for &position in &self.many {
            if design::link(ui, &index.items[position].entry.path).clicked() {
                chosen = Some(position);
            }
        }
        chosen.and_then(|position| {
            let heading = self.many_heading.take();
            self.heading_document = Some(index.items[position].entry.id);
            self.settle(Go::Item(position, heading))
        })
    }

    /// Draw any item: a plain note, a world entry, or an entry of another kind.
    /// A newly opened note starts with its own state: the last note's change, kept form,
    /// error and pickers end here. Both the footer and the body call it first.
    fn prepare(&mut self, item: &Item) {
        if self.showing != Some(item.entry.id) {
            self.undo = None;
            self.kept = None;
            if let Some(previous) = self.showing {
                self.status_drafts.insert(previous, self.status.clone());
            }
            self.showing = Some(item.entry.id);
            self.group_fault = None;
            self.group_error = false;
            self.group_error_unread_file = false;
            self.status = self
                .status_drafts
                .remove(&item.entry.id)
                .unwrap_or_else(|| status(&item.entry).to_owned());
            self.status_editing = self.status != status(&item.entry);
            self.status_select = false;
            // Only a form with changes is worth keeping. It opens again where it was left.
            self.edits.retain(|_, draft| draft.dirty());
            self.other_type.clear();
            self.adding = None;
            self.picker = None;
            self.error = None;
        }
    }

    #[cfg(test)]
    pub fn read(
        &mut self,
        ui: &mut egui::Ui,
        item: &Item,
        index: &ContentIndex,
        campaign: Option<&Campaign>,
    ) -> Option<Nav> {
        self.read_with(ui, item, index, campaign, None)
    }

    /// `read`, with the trackers: a house rule note in play shows its problems and each
    /// tracker's history after its text.
    pub fn read_with(
        &mut self,
        ui: &mut egui::Ui,
        item: &Item,
        index: &ContentIndex,
        campaign: Option<&Campaign>,
        trackers: Option<&mut crate::trackers::Trackers>,
    ) -> Option<Nav> {
        self.prepare(item);
        if self.undo.as_ref().is_some_and(|change| {
            campaign.is_none_or(|campaign| {
                fs::read_to_string(campaign.path.join(&change.path))
                    .ok()
                    .as_deref()
                    != Some(&change.after)
            })
        }) {
            self.undo = None;
        }
        if self.kept.as_ref().is_some_and(|(path, text)| {
            campaign.is_none_or(|campaign| {
                fs::read_to_string(campaign.path.join(path)).ok().as_deref() != Some(text)
            })
        }) {
            self.kept = None;
        }
        let heading = if self.heading_document.is_none_or(|id| id == item.entry.id) {
            self.heading_document = None;
            self.heading.take()
        } else {
            None
        };
        crate::markdown_view::want_heading(ui.ctx(), heading);
        let go = self.draw(ui, item, index, campaign, trackers);
        // A request lasts one draw, so it cannot reach a heading on another page.
        crate::markdown_view::want_heading(ui.ctx(), None);
        go.and_then(|go| self.settle(go))
    }

    fn draw(
        &mut self,
        ui: &mut egui::Ui,
        item: &Item,
        index: &ContentIndex,
        campaign: Option<&Campaign>,
        trackers: Option<&mut crate::trackers::Trackers>,
    ) -> Option<Go> {
        let mut go = None;
        if self.place_choices_document != Some(item.entry.id) {
            self.place_choices_open = false;
            self.place_choices_document = Some(item.entry.id);
        }
        ui.ctx()
            .data_mut(|data| data.insert_temp(egui::Id::new("reader_document"), item.entry.id));
        let title = item
            .entry
            .reading()
            .ok()
            .flatten()
            .map_or_else(|| item.entry.name.clone(), |reading| reading.title);
        let id = item.entry.id;
        let edit =
            campaign.filter(|campaign| edit::editable(item, index) && !self.editing(id, campaign));
        // Lookup has no file line. While the repair message names the fault, the kind line
        // names the open file in place of the fault.
        let kind_line = if self.compact
            && campaign.is_some()
            && index.duplicate_world.contains_key(&item.entry.path)
        {
            format!(
                "{} · {} · {}",
                crate::library::presentation::subtitle(&item.entry),
                item.owner,
                item.entry.path
            )
        } else {
            format!(
                "{} · {}",
                if !self.compact && world_type(&item.entry) == "npc" {
                    "NPC"
                } else {
                    &item.subtitle
                },
                item.owner
            )
        };
        // While a run is open, an NPC has Add to fight below, and Arrivals adds a creature.
        let start = (self.sides.is_none()
            && !self.beside_editor
            && !campaign.is_some_and(|c| self.editing(id, c)))
        .then(|| start_fight(item, index))
        .flatten();
        {
            let width = ui.available_width();
            let cell = ui
                .allocate_ui_with_layout(
                    egui::vec2(width, 0.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        view::title_with_art(ui, &title, &kind_line, Some(&item.entry));
                    },
                )
                .response
                .rect;
            let art = crate::art::entry_art(ui.ctx(), &item.entry).is_some();
            let art_width = if art {
                design::XXL * 2.0 + design::M
            } else {
                0.0
            };
            let title_width = ui
                .painter()
                .layout(
                    title.clone(),
                    Type::Display.font(ui),
                    colors().text,
                    (width - art_width).max(design::XXL),
                )
                .size()
                .x;
            // The title line's actions, each with its icon and its word (round 27): Edit, then
            // `Start fight`, where it is for a DM who never adds to a Log, then `Add to Log`. Edit's
            // icon sits one `L` after the measured title, so a kind line wider than a short title
            // never pushes it away. When the title and all of them do not fit on one line, all
            // of them move together to a line under the title: none goes away or loses its word.
            let edit_name = format!("Edit {}", item.entry.name);
            let start_name = start
                .as_ref()
                .map(|start| format!("Start a fight with {}", start.name));
            let mut actions = Vec::with_capacity(3);
            if edit.is_some() {
                actions.push(
                    Action::quiet("Edit")
                        .icon(design::icons::EDIT)
                        .density(Density::Read)
                        .name(&edit_name),
                );
            }
            if let Some(name) = start_name.as_deref() {
                actions.push(
                    Action::quiet("Start fight")
                        .icon(design::icons::RUN)
                        .density(Density::Read)
                        .name(name)
                        .blocked(self.start_blocked),
                );
            }
            if edit.is_some() {
                // Its own glyph, a list with a plus, so it is not a bare `+` above `Add to fight`.
                actions.push(
                    Action::quiet("Add to Log")
                        .icon(design::icons::ADD_TO_LOG)
                        .density(Density::Read),
                );
            }
            // A portrait keeps the right of the title's line.
            let pressed = title_line_actions(ui, cell, title_width, width - art_width, actions);
            let mut pressed = pressed.into_iter();
            if let Some(campaign) = edit
                && pressed.next() == Some(true)
            {
                self.begin_edit(campaign, item);
            }
            if let Some(start) = start
                && pressed.next() == Some(true)
            {
                self.start_fight = Some(start);
            }
            if let Some(campaign) = edit
                && pressed.next() == Some(true)
            {
                self.open_log(campaign.id, id, &item.entry.name, "");
            }
        }
        self.form_focused = false;
        let editing = campaign.is_some_and(|campaign| self.editing(id, campaign));
        if let Some(campaign) = campaign
            && takes_log(item, index)
            && let Some(go) = self.log_field(ui, item, campaign, editing)
        {
            return Some(go);
        }
        if !editing && let Some(campaign) = campaign {
            self.refresh_group_fault(item, index.generation(), || {
                note_path(&campaign.path, &item.entry.path)
                    .ok()
                    .and_then(|path| fs::read_to_string(path).ok())
            });
        }
        // The reader draws the unread warning from a borrow of the cached fault: no frame copies
        // its words (round 25 review).
        let has_unread = self
            .group_fault
            .as_ref()
            .filter(|fault| !editing && fault.id == id)
            .is_some_and(|fault| fault.unread.is_some());
        // The error, the results and their Undo are in the pane's footer (`footer_lines`).
        let _ = has_unread;
        if let Some(campaign) = campaign
            && editing
        {
            return self.edit_form(ui, item, index, campaign);
        }

        let file_line = !self.compact
            && (item.is_world_note() || item.entry.kind == "world")
            && item.is_private();
        if file_line && let Some(campaign) = campaign {
            // A note that a Part of change found unreadable gets no editor link here either.
            let editor = !(self.group_error && self.group_error_unread_file);
            entry_file_actions(ui, &campaign.path, &item.entry.path, editor);
        }
        // Every warning sits below the file line, so the file line keeps its place. Each wraps
        // at the reader measure, the right edge of the rows below.
        design::reader::measure(ui, |ui| {
            let fault = self.group_fault.as_ref().filter(|fault| fault.id == id);
            if let Some(unread) = fault
                .filter(|_| !editing)
                .and_then(|fault| fault.unread.as_ref())
                && let Some(campaign) = campaign
                && (self.error.is_none() || self.group_error)
            {
                if file_line {
                    // The file line just above has the one Edit in Obsidian link.
                    unread_warning(ui, &unread.reason, &unread.whole);
                } else {
                    unread_groups_message(
                        ui,
                        campaign,
                        &item.entry.path,
                        &unread.reason,
                        &unread.whole,
                    );
                }
            }
            // Every other value not read sits here too, not below the values.
            if let Some(fault) = fault {
                for (notice, whole) in &fault.notices {
                    unread_warning(ui, notice, whole);
                }
            }
        });
        if let Some(campaign) = campaign {
            if item.entry.kind == "world" && !index.duplicate_world.contains_key(&item.entry.path) {
                let stem = Path::new(&item.entry.path)
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy();
                if stem != item.entry.name {
                    let message = format!(
                        "The file is {stem}.md, but the name is {}.",
                        item.entry.name
                    );
                    // Two safe choices: the note takes the file's name, or the file takes the
                    // note's name (FD5). An open edit form keeps its file, so the file keeps
                    // its name while one is open.
                    let file = renamed_path(&item.entry.path, &item.entry.name)
                        .filter(|_| !self.editing(id, campaign));
                    let (note, rename_file) =
                        design::message_with_actions(ui, Tone::Warning, &message, |ui| {
                            // A choice whose words do not fit the message's width says only
                            // its verb; the warning above names the name, and the accessible
                            // name keeps the whole choice.
                            let room = ui.available_width();
                            let choice = |full: String, short: &'static str| {
                                let action = Action::secondary(&full).density(Density::Read);
                                if action.natural_width(ui) <= room {
                                    (full.clone(), full)
                                } else {
                                    (short.to_owned(), full)
                                }
                            };
                            let (note_words, note_name) =
                                choice(format!("Rename the note to {stem}"), "Rename the note");
                            let file_words = file.as_ref().map(|to| {
                                choice(
                                    format!(
                                        "Rename the file to {}",
                                        to.rsplit('/').next().unwrap_or(to)
                                    ),
                                    "Rename the file",
                                )
                            });
                            ui.horizontal_wrapped(|ui| {
                                let note = Action::secondary(&note_words)
                                    .name(&note_name)
                                    .density(Density::Read)
                                    .show(ui)
                                    .clicked();
                                let rename_file =
                                    file_words.as_ref().is_some_and(|(words, name)| {
                                        // The layout's own gap after a control, with this, is `L`.
                                        ui.add_space(design::L - ui.spacing().item_spacing.x);
                                        Action::secondary(words)
                                            .name(name)
                                            .density(Density::Read)
                                            .show(ui)
                                            .clicked()
                                    });
                                (note, rename_file)
                            })
                            .inner
                        });
                    if rename_file && let Some(to) = &file {
                        // The whole file must still be the whole file the index read, which
                        // produced the reader: a changed comment, line end or byte order mark
                        // counts as a change. The move checks the same text again.
                        let result = file_fault::read_note(&campaign.path, &item.entry.path)
                            .and_then(|text| {
                                if item.source_text() == Some(crate::lookup::text_hash(&text)) {
                                    FileRename::apply(campaign, &item.entry.path, to, &text)
                                } else {
                                    Err(format!(
                                        "{} changed on disk. Nothing was renamed.",
                                        item.entry.path
                                    ))
                                }
                            });
                        match result {
                            Ok(rename) => {
                                self.file_undo = Some(rename);
                                self.stamp(FooterAct::FileRename);
                                return Some(Go::Reload(item.entry.id));
                            }
                            Err(error) => self.error = Some(error),
                        }
                    }
                    if note {
                        let result = (|| {
                            let before = file_fault::read_note(&campaign.path, &item.entry.path)?;
                            let after = crate::content_workspace::patch_fields(
                                &before,
                                &[("splat_name", Some(stem.to_string().into()))],
                            )?;
                            let after = crate::content_workspace::rename_first_heading(
                                after,
                                &item.entry.name,
                                &stem,
                            );
                            write_note(&campaign.path, &item.entry.path, Some(&before), &after)?;
                            Ok::<_, String>(Change {
                                campaign: campaign.id,
                                path: item.entry.path.clone(),
                                before,
                                after,
                                word: format!("Name changed to {stem}."),
                                kind: ChangeKind::Name,
                            })
                        })();
                        match result {
                            Ok(change) => {
                                self.name_undo = Some(change);
                                self.stamp(FooterAct::Name);
                                return Some(Go::Reload(item.entry.id));
                            }
                            Err(error) => self.error = Some(error),
                        }
                    }
                }
            }
            if let Some(paths) = index.duplicate_world.get(&item.entry.path) {
                let mut paths = paths.clone();
                let first = crate::content_workspace::duplicate_repair_path(
                    paths.iter().map(|path| campaign.path.join(path)),
                );
                paths.sort_by_key(|path| Some(campaign.path.join(path)) != first);
                let spacing = ui.spacing().item_spacing.y;
                // The choices and their consequences start at the message text, as in Sessions.
                let chosen = design::message_with_actions(
                    ui,
                    Tone::Warning,
                    IdentityNoun::Note.duplicate_warning(),
                    |ui| {
                        let mut chosen = None;
                        for (at, path) in paths.iter().enumerate() {
                            // An `L` gap separates the choices, as in Library.
                            if at > 0 {
                                ui.add_space((design::L - spacing).max(0.0));
                            }
                            let filename = Path::new(path)
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy();
                            if separate_entry_button(ui, &filename) {
                                chosen = Some(path);
                            }
                            if let Some(consequence) = duplicate_consequence(&paths, at) {
                                ui.label(Type::Small.rich(ui, consequence).color(colors().muted));
                            }
                        }
                        chosen
                    },
                );
                if let Some(path) = chosen {
                    match new_identity(campaign, path) {
                        Ok((undo, id)) => {
                            self.identity_undo = Some((campaign.id, undo));
                            self.stamp(FooterAct::Identity);
                            return Some(Go::Reload(id));
                        }
                        Err(error) => self.error = Some(error),
                    }
                }
                // An `M` gap ends the choices, so the fields below do not read as a consequence.
                ui.add_space((design::M - spacing).max(0.0));
            }
        }
        let warnings: Vec<_> = index
            .errors
            .iter()
            .filter(|error| {
                error.starts_with(&format!("{}:", item.entry.path))
                    || error.starts_with(&format!("{} is in ", item.entry.name))
            })
            .collect();
        // Only a warning gets the scope: an empty one would still add an item gap.
        if !warnings.is_empty() {
            design::reader::measure(ui, |ui| {
                for warning in warnings {
                    design::message(ui, Tone::Warning, warning);
                }
            });
        }
        // A value block that draws no row adds no gap. In World, the prose then starts where a
        // plain note's starts, below the file line. Lookup's compact reader has no file line:
        // there the prose keeps the reader's gap below the kind line, as an entry with no value
        // block (a spell) has.
        let mut empty_value_block = false;
        if item.entry.kind == "world" && !index.duplicate_world.contains_key(&item.entry.path) {
            let top = ui.cursor().top();
            let block = ui.scope(|ui| {
                ui.spacing_mut().item_spacing.y =
                    design::Density::Scan.row() - design::Density::Scan.control();
                self.entry_controls(ui, item, index, campaign)
            });
            go = block.inner;
            // An empty block still takes its own item gap. That gap goes now; the reader's
            // gap before the prose goes through `empty_value_block`.
            if block.response.rect.height() <= 0.0 {
                ui.add_space(top - ui.cursor().top());
                empty_value_block = !self.compact;
            }
        }
        if item.entry.kind == "world" && index.duplicate_world.contains_key(&item.entry.path) {
            go = go.or(self.duplicate_values(ui, item, index));
        }
        let has_prose =
            !crate::library::presentation::reading_sections(&item.entry.body, &item.entry.name)
                .is_empty();
        if (item.entry.kind == "world" || item.is_world_note()) && !has_prose {
            ui.label(
                Type::Small
                    .rich(ui, "Nothing written yet.")
                    .color(colors().muted),
            );
            // The reader's gap now follows these words, not an empty value block.
            empty_value_block = false;
        }
        if item.is_note() {
            // In Lookup, a plain note's prose has the reader's gap below the kind line, as every
            // entry's has. World's plain note follows its file line directly.
            if self.compact && has_prose {
                ui.add_space(design::L);
            }
            let followed = style::measure(ui, |ui| {
                style::markdown(ui, &item.entry.body);
                related(ui, item, index)
            });
            go = go.or(followed);
            if !self.compact && item.is_world_note() {
                go = go.or(self.make_entry_control(ui, item, campaign));
            }
        } else {
            let entry = self
                .group_fault
                .as_ref()
                .filter(|fault| fault.id == item.entry.id)
                .and_then(|fault| fault.entry.as_ref())
                .unwrap_or(&item.entry);
            let followed = view::show_with_related(
                ui,
                &item.entry.name,
                Some(entry),
                &item.entry.body,
                &view::Origin {
                    path: &item.entry.path,
                    owner: &item.owner,
                    builtin: item.is_builtin(),
                },
                index,
                empty_value_block,
                |ui| {
                    if let Some(trackers) = trackers {
                        crate::trackers::history::show(ui, trackers, &item.entry);
                    }
                    related(ui, item, index).and_then(|go| {
                        if let Go::Item(at, _) = go {
                            Some(at)
                        } else {
                            None
                        }
                    })
                },
            );
            go = go.or(followed.map(|position| Go::Item(position, None)));
        }

        if let Some(target) = crate::markdown_view::take_note_link(ui.ctx()) {
            let followed = index.follow(&target, item);
            if let Go::Many(positions) = &followed {
                self.choose_link(positions.clone(), &target);
            }
            go = go.or(Some(followed));
        }
        go
    }

    fn make_entry_control(
        &mut self,
        ui: &mut egui::Ui,
        item: &Item,
        campaign: Option<&Campaign>,
    ) -> Option<Go> {
        let campaign = campaign?;
        let mut chosen = None;
        ui.add_space(design::M);
        // The edit form's fields end at `FORM_WIDTH` from the reader's edge; the section's
        // indent must not move this field's right edge past them (round 27 FF7).
        let form_edge = ui.max_rect().left() + design::FORM_WIDTH;
        design::quiet_section(ui, "Give this note a type", |ui| {
            // The help says what the choice does for the DM; how Splat stores it waits in
            // Technical details at the end of the section.
            // Help under a section keeps the reader measure (round 25 FG1, AG1).
            design::help(ui, TYPE_HELP);
            ui.horizontal_wrapped(|ui| {
                for (at, (label, kind)) in [("NPC", "npc"), ("Location", "location")]
                    .into_iter()
                    .enumerate()
                {
                    let action = Action::quiet(label).density(Density::Read);
                    if starts_line(ui, action, at == 0).show(ui).clicked() {
                        chosen = Some(kind.to_owned());
                    }
                }
            });
            ui.scope(|ui| {
                ui.set_max_width((form_edge - ui.max_rect().left()).min(ui.available_width()));
                design::text_field(
                    ui,
                    "Another type",
                    &mut self.other_type,
                    Some("A word of your own, such as faction or quest."),
                    None,
                );
            });
            let other = self.other_type.trim().to_lowercase();
            if !other.is_empty()
                && Action::quiet(&format!("Make {other}"))
                    .density(Density::Read)
                    .line_start()
                    .show(ui)
                    .clicked()
            {
                chosen = Some(other);
            }
            design::quiet_section(ui, "Technical details", |ui| {
                design::help(ui, TYPE_STORAGE);
            });
        });
        match make_entry(campaign, item, &chosen?) {
            Ok(id) => Some(Go::Reload(id)),
            Err(error) => {
                self.error = Some(error);
                None
            }
        }
    }

    /// No file read, source scan or entry copy occurs before the cache lookup.
    fn refresh_group_fault(
        &mut self,
        item: &Item,
        generation: u64,
        read: impl FnOnce() -> Option<String>,
    ) {
        if self
            .group_fault
            .as_ref()
            .is_some_and(|fault| fault.id == item.entry.id && fault.generation == generation)
        {
            return;
        }
        let source = if item.entry.flags.is_empty() && !self.group_error {
            None
        } else {
            read()
        };
        let unread = source
            .as_ref()
            .and_then(|file| fields::unread_groups(&item.entry, file));
        if self.group_error
            && unread.is_none()
            && source.as_ref().is_some_and(|file| {
                Entry::parse(&item.entry.path, file)
                    .is_ok_and(|entry| fields::unread_groups(&entry, file).is_none())
            })
        {
            self.error = None;
            self.group_error = false;
            self.group_error_unread_file = false;
        }
        // Draw each unread value once, below the file line.
        let (notices, entry) = GroupFault::flags(&item.entry, unread.as_ref());
        self.group_fault = Some(GroupFault {
            id: item.entry.id,
            generation,
            unread,
            notices,
            entry,
        });
    }

    fn change(
        &mut self,
        campaign: &Campaign,
        item: &Item,
        index: &ContentIndex,
        key: &str,
        value: Value,
    ) -> Option<Go> {
        // A picker can outlive an outside edit. Re-read before deriving a replacement or Log.
        let fresh = if key == "part_of" {
            // The last value says whether the file itself could not be read.
            let result = (|| {
                let file = file_fault::read_note(&campaign.path, &item.entry.path)
                    .map_err(|error| (error, None, true))?;
                let entry =
                    Entry::parse(&item.entry.path, &file).map_err(|error| (error, None, false))?;
                if let Some(unread) = fields::unread_groups(&entry, &file) {
                    return Err((
                        unread.reason.clone(),
                        Some((unread, Box::new(entry))),
                        false,
                    ));
                }
                Ok((file, entry))
            })();
            match result {
                Ok(fresh) => Some(fresh),
                Err((error, unread, unread_file)) => {
                    self.picker = None;
                    self.error = Some(error);
                    self.group_error = true;
                    self.group_error_unread_file = unread_file;
                    // The file on disk has an unread list now. Show that state at once, and
                    // read the note again so its list row and the header count it too.
                    let (unread, entry) = unread?;
                    if let Some(fault) = &mut self.group_fault
                        && fault.id == item.entry.id
                    {
                        (fault.notices, fault.entry) = GroupFault::flags(&entry, Some(&unread));
                        fault.unread = Some(unread);
                    }
                    return Some(Go::Reload(item.entry.id));
                }
            }
        } else {
            None
        };
        let current_entry = fresh.as_ref().map_or(&item.entry, |(_, entry)| entry);
        let value = if key == "part_of" {
            // Apply only the picker's add/remove to the fresh disk list.
            let previous: Vec<_> = part_of(&item.entry).into_iter().cloned().collect();
            let disk: Vec<_> = part_of(current_entry).into_iter().cloned().collect();
            Value::Sequence(merge_groups(index, &previous, value.as_sequence()?, &disk))
        } else {
            value
        };
        let label = match key {
            "in" => "In",
            "part_of" => "Part of",
            "status" => "Status",
            _ => "Stat block",
        };
        // Two notes with one name: the one a value opens is named with its folder, so the
        // result tells the DM which file the change chose (AD2, FD3, SD2).
        let one_name = |value: &Value| match edit::value_target(index, value) {
            Some(at) => index.shown_name(at),
            None => edit::value_name(index, value, label),
        };
        // `None` is a value that is not set: no words, an empty list or blank text.
        let name = |value: &Value| match value.as_sequence() {
            Some(groups) if groups.is_empty() => None,
            // Each value by the name the reader shows: the note its link opens, else the words
            // written in the file.
            Some(groups) => Some(groups.iter().map(one_name).collect::<Vec<_>>().join(", ")),
            None if empty_optional(value) => None,
            // A status is a word, not a link: the words in the file, as its row shows them.
            None if key == "status" => value.as_str().map(|word| word.trim().to_owned()),
            None => Some(one_name(value)),
        };
        // Choosing what is already there changes nothing, and writes no Log line.
        let current = world_field(current_entry, key).filter(|value| !value.is_null());
        let same = match (key, current) {
            ("part_of", _) => false,
            ("status", _) => value.as_str().map(str::trim) == Some(status(current_entry)),
            (_, None) => value.is_null(),
            (_, Some(now)) => {
                let target = index.world_value_target(now);
                target.is_some() && target == index.world_value_target(&value)
            }
        };
        if same {
            self.picker = None;
            self.place_choices_open = false;
            return None;
        }
        // The result names the place as the Undo (`Undo place change`) and the list (`Place not
        // found`) do; the field's label `In` is not a noun (round 27 FA5).
        let words = change_words(
            if key == "in" { "Place" } else { label },
            world_field(current_entry, key).and_then(name).as_deref(),
            name(&value).as_deref(),
        );
        // What play changes goes in the entry's Log, in the same write, so one Undo takes
        // both back.
        let log = self
            .log_as()
            .filter(|_| matches!(key, "in" | "part_of" | "status"))
            .map(|stamp| {
                let words = log_words(index, key, world_field(current_entry, key), &value);
                format!("{stamp}: {words}")
            });
        let value = edit::stored_value(index, value);
        let result = (|| {
            let before = match &fresh {
                Some((file, _)) => file.clone(),
                None => file_fault::read_note(&campaign.path, &item.entry.path)?,
            };
            let patched = fields::patch(&before, key, value)?;
            let after = match &log {
                Some(line) => lines::add_log(&patched, line),
                None => patched,
            };
            write_note(&campaign.path, &item.entry.path, Some(&before), &after)?;
            Ok::<_, String>(Change {
                campaign: campaign.id,
                path: item.entry.path.clone(),
                before: before.clone(),
                after: after.clone(),
                word: words,
                kind: ChangeKind::of_key(key),
            })
        })();
        let go = self.written(result, item.entry.id);
        if go.is_some() {
            self.picker = None;
            self.place_choices_open = false;
        }
        go
    }

    /// `Add to Log`'s field under the header, while it is open for this entry in this session:
    /// the line's stamp in muted words, then the field, both ending at the reader's measure.
    /// Enter adds the line and closes the field; Escape clears and closes it. An open edit form
    /// blocks it.
    fn log_field(
        &mut self,
        ui: &mut egui::Ui,
        item: &Item,
        campaign: &Campaign,
        editing: bool,
    ) -> Option<Go> {
        // With no field open, or a field that waits for an edit form, no scope: an empty one
        // still takes an item gap, and would move the form (round 27 FC2).
        if !self.log_fields.contains_key(&(campaign.id, item.entry.id))
            || editing
            || self.edited_elsewhere.contains(&item.entry.id)
        {
            return None;
        }
        design::reader::measure(ui, |ui| self.log_field_in(ui, item, campaign))
    }

    fn log_field_in(&mut self, ui: &mut egui::Ui, item: &Item, campaign: &Campaign) -> Option<Go> {
        let key = (campaign.id, item.entry.id);
        let stamp = self
            .line_stamp()
            .map(|stamp| format!("{}:", sessions::plain_links(&stamp)));
        let open = self.log_fields.get_mut(&key)?;
        let select = std::mem::take(&mut open.select);
        let field = &mut open.field;
        let name = format!("Log line for {}", item.entry.name);
        // The field ends at the reader's measure, also when the stamp shares its line: a row of
        // a wrapping layout does not hold a nested row to its parent's width (round 27 AF2, FF1).
        let edge = ui.max_rect().right();
        let draw = |ui: &mut egui::Ui, field: &mut design::AddField| {
            let width = (edge - ui.cursor().left()).max(design::XXL);
            ui.allocate_ui_with_layout(
                egui::vec2(width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.set_max_width(width);
                    design::add_field(
                        ui,
                        field,
                        &name,
                        "What happened?",
                        0.0,
                        |_| None::<()>,
                        |_| Vec::new(),
                    )
                    .field
                },
            )
            .inner
        };
        let response = match &stamp {
            None => draw(ui, field),
            Some(stamp) => {
                let width = ui
                    .painter()
                    .layout_no_wrap(stamp.clone(), Type::Small.font(ui), colors().muted)
                    .size()
                    .x;
                let words = Type::Small.rich(ui, stamp).color(colors().muted);
                // The stamp shares the field's line while the field keeps room for a sentence.
                // One layout for both, so the field keeps its ID, and the keyboard, when the
                // window changes size.
                let room = ui.available_width() - width - design::S;
                let typed = ui
                    .painter()
                    .layout_no_wrap(
                        field.words.clone(),
                        egui::TextStyle::Body.resolve(ui.style()),
                        colors().text,
                    )
                    .size()
                    .x;
                let beside = room >= LOG_FIELD_BESIDE.max(typed + 3.0 * design::S);
                ui.horizontal_wrapped(|ui| {
                    ui.label(words);
                    if !beside {
                        ui.end_row();
                    }
                    draw(ui, field)
                })
                .inner
            }
        };
        if select {
            design::focus_and_select(ui, response.id, &field.words);
        }
        // While the field has the keyboard, Escape is the field's, not Lookup's.
        if response.has_focus() || response.lost_focus() {
            self.form_focused = true;
        }
        let enter = response.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
        if ui.input(|input| input.key_pressed(egui::Key::Escape))
            && (response.has_focus() || response.lost_focus())
        {
            self.log_fields.remove(&key);
        } else if enter {
            let words = field.words.trim().to_owned();
            if words.is_empty() {
                self.log_fields.remove(&key);
            } else {
                return self.add_log_line(campaign, item, &words);
            }
        }
        None
    }

    /// `Add to Log`: one line in the entry's own note, at the end of its Log, stamped as a
    /// change is at this moment. It reads the file now, so an edit made a moment before stays,
    /// and writes only when the file still holds what it read. A failed write keeps the field
    /// and its words, and names the file.
    fn add_log_line(&mut self, campaign: &Campaign, item: &Item, words: &str) -> Option<Go> {
        let line = match self.line_stamp() {
            Some(stamp) => format!("{stamp}: {words}"),
            None => words.to_owned(),
        };
        let result = (|| {
            let before = file_fault::read_note(&campaign.path, &item.entry.path)?;
            let after = lines::add_log(&before, &line);
            write_note(&campaign.path, &item.entry.path, Some(&before), &after)?;
            Ok::<_, String>(Change {
                campaign: campaign.id,
                path: item.entry.path.clone(),
                before,
                after,
                word: format!("Added to {}’s Log.", item.entry.name),
                kind: ChangeKind::Log,
            })
        })();
        let go = self.written(result, item.entry.id);
        if go.is_some() {
            self.log_fields.remove(&(campaign.id, item.entry.id));
        }
        go
    }

    /// A change to one World note, written or not: its result and Undo, or its error, in the
    /// reader's one place for each.
    fn written(&mut self, result: Result<Change, String>, id: Uuid) -> Option<Go> {
        match result {
            Ok(change) => {
                self.error = None;
                self.group_error = false;
                self.group_error_unread_file = false;
                self.undo = Some(change);
                self.stamp(FooterAct::Change);
                self.kept = None;
                Some(Go::Reload(id))
            }
            Err(error) => {
                self.error = Some(error);
                None
            }
        }
    }

    /// Below this width a row's label stands above its value, not beside it.
    fn stacked(ui: &egui::Ui) -> bool {
        ui.available_width() < design::XXL * 12.0
    }

    fn relationship_width(ui: &egui::Ui) -> f32 {
        let width = design::XXL * 7.0 + design::L;
        let change = Action::quiet("Change")
            .density(Density::Scan)
            .natural_width(ui);
        // A stacked label stands above its value, so the value keeps the label column's room.
        let room = if Self::stacked(ui) {
            ui.available_width() - change - design::L
        } else {
            ui.available_width()
                - (design::XXL * 3.0 + design::S)
                - change
                - ui.spacing().item_spacing.x * 2.0
        };
        width.min(room.max(design::XXL))
    }

    /// The Part of reason while this note's list could not be read.
    fn unread_groups(&self, id: Uuid) -> Option<String> {
        self.group_fault
            .as_ref()
            .filter(|fault| fault.id == id)
            .and_then(|fault| fault.unread.as_ref())
            .map(|unread| unread.reason.clone())
    }

    fn relationship_value_width(&self, ui: &egui::Ui, item: &Item, index: &ContentIndex) -> f32 {
        let stacked = Self::stacked(ui);
        let cap = Self::relationship_width(ui);
        let mut values = Vec::new();
        // Missing links, measured as `missing_link` draws them.
        let mut missing = Vec::new();
        for (key, label) in [("creature", "Stat block"), ("in", "In")] {
            if key == "creature" && !fights(&item.entry) {
                continue;
            }
            let target = if key == "in" {
                index.place_of(item.entry.id)
            } else {
                index.world_target(&item.entry, key)
            };
            let value = if let Some(at) = target {
                index.items[at].entry.name.clone()
            } else if key == "in" && !index.place_choices(item.entry.id).is_empty() {
                let choices = index.place_choices(item.entry.id);
                let name = reader_field(&item.entry, key)
                    .map(|value| link_name(value, label))
                    .unwrap_or_default();
                format!(
                    "{} {}",
                    design::icons::WARN,
                    place_choices_words(&name, choices.len())
                )
            } else if let Some(value) = reader_field(&item.entry, key) {
                missing.push(missing_width(ui, &link_name(value, label), stacked));
                continue;
            } else if key == "creature" {
                "not set".into()
            } else {
                continue;
            };
            let linked =
                target.is_some() || (key == "in" && !index.place_choices(item.entry.id).is_empty());
            values.push((value, if linked { Type::Body } else { Type::Small }));
        }
        // Part of is measured as drawn: each group in its own type, a missing one with its
        // icon and ` · Not found`, with the gaps between them. The groups wrap within the
        // column, so the widest of their drawn lines counts, not all of them on one line (FG5).
        let groups = index.groups_of(&item.entry);
        let gap = ui.spacing().item_spacing.x;
        let (mut groups_width, mut line) = (0.0_f32, 0.0_f32);
        for (name, target) in groups.iter() {
            // A found group shows its note's name, not the words written in the file.
            let width = match target {
                Some(at) => {
                    design::number_width(ui, &index.items[*at].entry.name, &Type::Body.font(ui))
                }
                None => missing_width(ui, name, stacked),
            };
            line = if line > 0.0 && line + gap + width > cap {
                groups_width = groups_width.max(line);
                width
            } else if line > 0.0 {
                line + gap + width
            } else {
                width
            };
        }
        // A pixel of room, so a line that fits by its measure does not wrap on rounding.
        groups_width = groups_width.max(line) + if groups.is_empty() { 0.0 } else { 1.0 };
        let groups_width = missing.into_iter().fold(groups_width, f32::max);
        if self.unread_groups(item.entry.id).is_some() {
            values.push(("not read".into(), Type::Small));
        }
        let status = if status_unread(&item.entry) {
            ("not read", Type::Small)
        } else if status(&item.entry).is_empty() {
            ("not set", Type::Small)
        } else {
            (status(&item.entry), Type::Body)
        };
        values.push((status.0.into(), status.1));
        // The place choices' files share the value column, so Change and each choice's action
        // stand in one column.
        let choices = index.place_choices(item.entry.id);
        if !self.compact && reader_field(&item.entry, "in").is_some() && !choices.is_empty() {
            for &at in &choices {
                values.push((index.items[at].entry.path.clone(), Type::Body));
            }
        }
        let width = values
            .iter()
            .map(|(value, role)| design::number_width(ui, value, &role.font(ui)))
            .fold(groups_width, f32::max)
            .min(cap);
        if !self.compact && !choices.is_empty() {
            // The choice action is wider than Change. Both keep the column the narrower room allows.
            width.min(
                (ui.available_width() - Self::chooser_indent(ui) - Self::choice_room(ui))
                    .max(design::XXL),
            )
        } else {
            width
        }
    }

    /// The room a place choice's action takes after its file: the action and an `L` gap.
    fn choice_room(ui: &egui::Ui) -> f32 {
        Action::secondary("Always use this one").natural_width(ui) + design::L
    }

    /// The place choices start below the values: below the label column on a wide pane.
    fn chooser_indent(ui: &egui::Ui) -> f32 {
        if !Self::stacked(ui) {
            design::XXL * 3.0 + design::S + ui.spacing().item_spacing.x
        } else {
            0.0
        }
    }

    /// The files and their actions. `column` is the World reader's value column, which the files
    /// share so each action stands under Change.
    fn place_chooser(
        &self,
        ui: &mut egui::Ui,
        item: &Item,
        index: &ContentIndex,
        column: Option<f32>,
    ) -> (Option<Go>, Option<usize>) {
        let mut go = None;
        let mut chosen = None;
        let indent = if self.compact {
            design::number_width(ui, "In", &Type::Small.font(ui)) + ui.spacing().item_spacing.x
        } else {
            Self::chooser_indent(ui)
        };
        egui::Frame::new()
            .inner_margin(egui::Margin {
                left: indent as i8,
                ..Default::default()
            })
            .show(ui, |ui| {
                ui.label(
                    Type::Small
                        .rich(ui, "This link matches more than one note. Choose one.")
                        .color(colors().muted),
                );
                let choices = index.place_choices(item.entry.id);
                let file_width = column.unwrap_or_else(|| {
                    choices
                        .iter()
                        .map(|&at| {
                            design::number_width(
                                ui,
                                &index.items[at].entry.path,
                                &Type::Body.font(ui),
                            )
                        })
                        .fold(0.0_f32, f32::max)
                        .min((ui.available_width() - Self::choice_room(ui)).max(design::XXL))
                });
                for at in choices {
                    let path = &index.items[at].entry.path;
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = design::L;
                        let name = format!("Always use {path}");
                        let action = Action::secondary("Always use this one").name(&name);
                        // The link centres on the action's row, as a value row's link and
                        // Change share one line (FG4).
                        ui.allocate_ui_with_layout(
                            egui::vec2(file_width, Density::Read.control()),
                            egui::Layout::left_to_right(egui::Align::Center).with_main_wrap(true),
                            |ui| {
                                ui.set_width(file_width);
                                if design::wrapped_link(ui, path).clicked() {
                                    go = Some(Go::Item(at, None));
                                }
                            },
                        );
                        if action.show(ui).clicked() {
                            chosen = Some(at);
                        }
                    });
                }
            });
        (go, chosen)
    }

    fn relationship_row(
        ui: &mut egui::Ui,
        label: &str,
        width: f32,
        value: impl FnOnce(&mut egui::Ui),
        blocked: Option<&str>,
    ) -> bool {
        let name = format!("Change {label}");
        let change = Action::quiet("Change")
            .density(design::Density::Scan)
            .name(&name)
            .blocked(blocked);
        Self::value_row(ui, label, width, value, Some(change))
    }

    /// A duplicate note's values, read only until its repair is done. Each reads as every
    /// reader's rows do: a muted label, then the value as a link. Only Change is left out.
    fn duplicate_values(&self, ui: &mut egui::Ui, item: &Item, index: &ContentIndex) -> Option<Go> {
        let mut go = None;
        let rows: Vec<_> = [
            ("creature", "Stat block"),
            ("in", "In"),
            ("status", "Status"),
        ]
        .into_iter()
        .filter_map(|(key, label)| {
            let value = world_field(&item.entry, key).filter(|value| !empty_optional(value))?;
            if key == "status" {
                return Some((label, value.as_str()?.trim().to_owned(), None));
            }
            let target = edit::value_target(index, value);
            let name = target.map_or_else(
                || link_name(value, label),
                |at| index.items[at].entry.name.clone(),
            );
            Some((label, name, target))
        })
        .collect();
        if rows.is_empty() {
            return None;
        }
        let stacked = Self::stacked(ui);
        let value = |ui: &mut egui::Ui, label: &str, name: &str, target: Option<usize>| match target
        {
            Some(at) => design::wrapped_link(ui, name)
                .clicked()
                .then_some(Go::Item(at, None)),
            None if label == "Status" => {
                ui.label(Type::Body.rich(ui, name).color(colors().text));
                None
            }
            None => {
                missing_link(ui, name, stacked);
                None
            }
        };
        if self.compact {
            // Lookup's compact line, as a note with one file has.
            ui.horizontal_wrapped(|ui| {
                for (n, (label, name, target)) in rows.iter().enumerate() {
                    if n > 0 {
                        ui.label(Type::Small.rich(ui, "·").color(colors().muted));
                    }
                    ui.label(Type::Small.rich(ui, *label).color(colors().muted));
                    if let Some(next) = value(ui, label, name, *target) {
                        go.get_or_insert(next);
                    }
                }
            });
            return go;
        }
        let width = rows
            .iter()
            .map(|(label, name, target)| {
                let (words, role) = match target {
                    Some(_) => (name.clone(), Type::Body),
                    None if *label == "Status" => (name.clone(), Type::Body),
                    None => return missing_width(ui, name, stacked),
                };
                design::number_width(ui, &words, &role.font(ui))
            })
            .fold(0.0_f32, f32::max)
            .min(Self::relationship_width(ui));
        ui.scope(|ui| {
            ui.spacing_mut().item_spacing.y =
                design::Density::Scan.row() - design::Density::Scan.control();
            for (label, name, target) in &rows {
                Self::value_row(
                    ui,
                    label,
                    width,
                    |ui| {
                        if let Some(next) = value(ui, label, name, *target) {
                            go.get_or_insert(next);
                        }
                    },
                    None,
                );
            }
        });
        go
    }

    /// A labelled value row. With no `change`, the value is read only: the row keeps its label
    /// and value column, and leaves out Change.
    fn value_row(
        ui: &mut egui::Ui,
        label: &str,
        width: f32,
        value: impl FnOnce(&mut egui::Ui),
        change: Option<Action<'_>>,
    ) -> bool {
        let stacked = Self::stacked(ui);
        let change_width = change
            .as_ref()
            .map_or(0.0, |change| change.natural_width(ui));
        let width = width.min((ui.available_width() - change_width - design::L).max(design::XXL));
        if stacked {
            // An `S` gap above each stacked label keeps each label with its own value.
            ui.add_space(design::S);
            ui.label(Type::Small.rich(ui, label).color(colors().muted));
        }
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                if !stacked {
                    ui.allocate_ui_with_layout(
                        egui::vec2(
                            design::XXL * 3.0 + design::S,
                            design::Density::Scan.control(),
                        ),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            ui.set_min_width(design::XXL * 3.0 + design::S);
                            ui.label(Type::Small.rich(ui, label).color(colors().muted));
                        },
                    );
                }
                ui.allocate_ui_with_layout(
                    egui::vec2(width, design::Density::Scan.control()),
                    egui::Layout::left_to_right(egui::Align::Center).with_main_wrap(true),
                    |ui| {
                        ui.set_width(width);
                        value(ui);
                    },
                );
                let Some(change) = change else {
                    return false;
                };
                ui.add_space((design::L - ui.spacing().item_spacing.x).max(0.0));
                change.show(ui).clicked()
            })
            .inner
        })
        .inner
    }

    /// A change result on this note. It is newer than the page's repair result, so it takes
    /// the one place for results.
    pub(crate) fn has_change_result(&self, campaign: &Campaign, path: &str) -> bool {
        self.undo
            .as_ref()
            .is_some_and(|change| change.shows_on(campaign, path))
            || self.kept.as_ref().is_some_and(|(kept, _)| kept == path)
            || self
                .name_undo
                .as_ref()
                .is_some_and(|change| change.shows_on(campaign, path))
    }

    pub fn clear_notice(&mut self) {
        self.undo = None;
        self.kept = None;
    }

    /// This reader's messages for the open note, for its pane's footer (round 26): the error
    /// of the last action, then every result with its Undo. Each keeps its owner and its end
    /// event: a change, a kept form and an error belong to the open note and end when another
    /// note opens (`prepare`), a later change replaces them, and Undo ends them; a rename,
    /// a name change and a separate note end at their Undo or the next of their kind; the
    /// page's result is the page's (`page`). Draw them with `draw_footer`, before the
    /// body, and take its action with `footer_act`.
    pub(crate) fn footer_lines(
        &mut self,
        item: &Item,
        index: &ContentIndex,
        campaign: Option<&Campaign>,
        page: Option<(&PageResult, u64)>,
    ) -> Vec<FooterLine> {
        self.prepare(item);
        let id = item.entry.id;
        let editing = campaign.is_some_and(|campaign| self.editing(id, campaign));
        let mut lines = self.gone_log_notices(campaign.map(|campaign| campaign.id));
        // A draft whose note is gone, and its Undo, whatever note is open, also in an edit form.
        lines.extend(self.lost_lines(index));
        // An open `Add to Log` field waits while an edit form of its entry is open, here or in
        // the other reader. It ends when the form closes (Save, Discard) or the reader shows
        // another entry.
        if let Some(campaign) = campaign
            && takes_log(item, index)
            && self.log_fields.contains_key(&(campaign.id, id))
            && (self.editing(id, campaign) || self.edited_elsewhere.contains(&id))
        {
            lines.push(FooterLine::new(
                FooterRank::Reason,
                Tone::Info,
                LOG_WAITS,
                FooterKind::Plain,
            ));
        }
        let has_unread = self
            .group_fault
            .as_ref()
            .filter(|fault| !editing && fault.id == id)
            .is_some_and(|fault| fault.unread.is_some());
        // A refused Part of write shows as the unread warning below the file line instead.
        if let Some(error) = &self.error
            && !(self.group_error && has_unread)
        {
            // A file that Splat cannot read, no editor can open either.
            let editor = campaign
                .filter(|_| self.group_error && !self.group_error_unread_file)
                .map(|campaign| campaign.path.join(&item.entry.path));
            lines.push(FooterLine::new(
                FooterRank::Reason,
                Tone::Error,
                error,
                FooterKind::Error { editor },
            ));
        }
        let Some(campaign) = campaign else {
            return lines;
        };
        if self
            .identity_undo
            .as_ref()
            .is_some_and(|(owner, _)| *owner != campaign.id)
        {
            self.identity_undo = None;
        }
        let page_order = page.map_or(0, |(_, order)| order);
        let page = page.map(|(page, _)| page);
        if editing {
            lines.extend(page.map(|page| page.line()));
            return self.newest_first(lines, page_order);
        }
        let undo = |words: &str, label: &'static str, act| {
            FooterLine::new(
                FooterRank::Undo,
                CHANGED,
                words,
                FooterKind::Undo { label, act },
            )
        };
        if let Some(change) = self
            .undo
            .as_ref()
            .filter(|change| change.shows_on(campaign, &item.entry.path))
        {
            lines.push(undo(
                &change.word,
                change.kind.undo_label(),
                FooterAct::Change,
            ));
        }
        lines.extend(self.discarded_line(campaign, id));
        if item.entry.kind == "world" && !index.duplicate_world.contains_key(&item.entry.path) {
            if let Some(rename) = self
                .file_undo
                .as_ref()
                .filter(|rename| rename.shows_on(campaign, &item.entry.path))
            {
                lines.push(undo(
                    &rename.message(),
                    "Undo file rename",
                    FooterAct::FileRename,
                ));
            }
            if let Some(change) = self
                .name_undo
                .as_ref()
                .filter(|change| change.shows_on(campaign, &item.entry.path))
            {
                lines.push(undo(&change.word, "Undo name change", FooterAct::Name));
            }
        }
        if let Some((_, change)) = &self.identity_undo {
            lines.push(undo(
                change.message(),
                change.identity_undo_label(),
                FooterAct::Identity,
            ));
        }
        lines.extend(page.map(|page| page.line()));
        // The form closed because the note already held every change. Nothing to undo.
        if edit::editable(item, index)
            && self
                .kept
                .as_ref()
                .is_some_and(|(path, _)| *path == item.entry.path)
        {
            lines.push(FooterLine::new(
                FooterRank::Notice,
                Tone::Info,
                edit::ALREADY_SAVED,
                FooterKind::Plain,
            ));
        }
        self.newest_first(lines, page_order)
    }

    /// A kept result was made now: it goes first among the results with a live Undo.
    pub(super) fn stamp(&mut self, act: FooterAct) {
        self.orders.insert(act, next_order());
    }

    /// Each line with the order its action was made at; then the footer's order: results with
    /// a live Undo newest first, then reasons, then notices.
    fn newest_first(&self, lines: Vec<FooterLine>, page: u64) -> Vec<FooterLine> {
        let mut lines: Vec<_> = lines
            .into_iter()
            .map(|line| {
                let order = match &line.kind {
                    FooterKind::Undo { act, .. } | FooterKind::Discarded { act, .. } => match act {
                        FooterAct::Page => page,
                        act => self.orders.get(act).copied().unwrap_or(0),
                    },
                    _ if line.rank == FooterRank::Notice => page,
                    _ => 0,
                };
                line.at(order)
            })
            .collect();
        lines.sort_by_key(|line| (line.rank, std::cmp::Reverse(line.order)));
        lines
    }

    /// Take a footer line's action on the open note. `FooterAct::Page` is the page's.
    pub(crate) fn footer_act(
        &mut self,
        ctx: &egui::Context,
        act: FooterAct,
        item: &Item,
        campaign: &Campaign,
    ) -> Option<Nav> {
        let go = match act {
            FooterAct::Change => self.undo_change(campaign, item.entry.id),
            FooterAct::FileRename => match self.file_undo.as_ref()?.undo(campaign) {
                Ok(()) => {
                    self.file_undo = None;
                    self.error = None;
                    Some(Go::Reload(item.entry.id))
                }
                Err(error) => {
                    self.error = Some(error);
                    None
                }
            },
            FooterAct::Name => match self.name_undo.as_ref()?.undo(campaign) {
                Ok(()) => {
                    self.name_undo = None;
                    self.error = None;
                    Some(Go::Reload(item.entry.id))
                }
                Err(error) => {
                    self.error = Some(error);
                    None
                }
            },
            FooterAct::Identity => {
                let (_, undo) = self.identity_undo.as_ref()?;
                match undo.undo(campaign) {
                    Ok(()) => {
                        let id = note_id(&undo.path);
                        self.identity_undo = None;
                        self.error = None;
                        Some(Go::Reload(id))
                    }
                    Err(error) => {
                        self.error = Some(error);
                        None
                    }
                }
            }
            FooterAct::Discarded => {
                if let Some((id, draft)) = self.discarded.take() {
                    self.edits.insert(id, draft);
                }
                None
            }
            // The page takes its own result's Undo, and the pane the trackers'.
            FooterAct::Page | FooterAct::Trackers => None,
            FooterAct::GoneLog(id) => {
                self.end_gone_log(id);
                None
            }
            FooterAct::LostCopy(id) => {
                self.copy_lost(ctx, id);
                None
            }
            FooterAct::LostDiscard(id) => {
                self.discard_lost(id);
                None
            }
            // The World page takes a tick's Undo.
            FooterAct::Untick(..) => None,
        };
        go.and_then(|go| self.settle(go))
    }

    fn undo_change(&mut self, campaign: &Campaign, id: Uuid) -> Option<Go> {
        match self.undo.as_ref()?.undo(campaign) {
            Ok(_) => {
                self.undo = None;
                // A finished Undo ends the reason an earlier action left.
                self.error = None;
                Some(Go::Reload(id))
            }
            Err(error) => {
                self.error = Some(error);
                None
            }
        }
    }

    fn relationships(
        &mut self,
        ui: &mut egui::Ui,
        item: &Item,
        index: &ContentIndex,
        campaign: &Campaign,
    ) -> Option<Go> {
        let mut go = None;
        let value_width = self.relationship_value_width(ui, item, index);
        let stacked = Self::stacked(ui);
        for (key, label) in [("creature", "Stat block"), ("in", "In")] {
            if key == "creature" && !fights(&item.entry) {
                continue;
            }
            let target = if key == "in" {
                index.place_of(item.entry.id)
            } else {
                index.world_target(&item.entry, "creature")
            };
            if key != "creature" && reader_field(&item.entry, key).is_none_or(Value::is_null) {
                continue;
            }
            ui.push_id(key, |ui| {
                let changed = Self::relationship_row(
                    ui,
                    label,
                    value_width,
                    |ui| {
                        if let Some(at) = target {
                            if design::wrapped_link(ui, &index.items[at].entry.name).clicked() {
                                go = Some(Go::Item(at, None));
                            }
                        } else if key == "in" && !index.place_choices(item.entry.id).is_empty() {
                            let choices = index.place_choices(item.entry.id);
                            let name = reader_field(&item.entry, key)
                                .map(|value| link_name(value, label))
                                .unwrap_or_default();
                            if warning_link(ui, &place_choices_words(&name, choices.len()))
                                .clicked()
                            {
                                self.place_choices_open = !self.place_choices_open;
                            }
                        } else if let Some(value) = reader_field(&item.entry, key) {
                            missing_link(ui, &link_name(value, label), stacked);
                        } else {
                            ui.label(Type::Small.rich(ui, "not set").color(colors().muted));
                        }
                    },
                    None,
                );
                if changed {
                    self.picker = Some(Picker {
                        key,
                        focus: true,
                        ..Default::default()
                    });
                }
            });
            if key == "in" && self.place_choices_open {
                let (followed, chosen) = self.place_chooser(ui, item, index, Some(value_width));
                go = go.or(followed);
                if let Some(at) = chosen {
                    let path = index.items[at].entry.path.trim_end_matches(".md");
                    return self.change(campaign, item, index, "in", format!("[[{path}]]").into());
                }
            }
        }
        let groups = index.groups_of(&item.entry);
        // An unread list keeps its row: `not read`, with Change blocked by the reason.
        let unread = self.unread_groups(item.entry.id);
        if !groups.is_empty() || unread.is_some() {
            ui.push_id("part_of", |ui| {
                let changed = Self::relationship_row(
                    ui,
                    "Part of",
                    value_width,
                    |ui| {
                        if unread.is_some() {
                            ui.label(
                                Type::Small
                                    .rich(ui, "not read")
                                    .color(colors().result.warning),
                            );
                            return;
                        }
                        for (name, target) in groups {
                            match target {
                                Some(at) => {
                                    if design::wrapped_link(ui, &index.items[*at].entry.name)
                                        .clicked()
                                    {
                                        go = Some(Go::Item(*at, None));
                                    }
                                }
                                None => missing_link(ui, name, stacked),
                            }
                        }
                    },
                    unread.as_deref(),
                );
                if changed {
                    self.picker = Some(Picker {
                        key: "part_of",
                        focus: true,
                        ..Default::default()
                    });
                }
            });
        }
        if let Some(key) = self.picker.as_ref().map(|picker| picker.key)
            && let Some(value) = self.pick(ui, item, index, campaign, None)
        {
            return self.change(campaign, item, index, key, value);
        }
        go
    }

    /// The open picker. Returns the value the DM chose, or `Value::Null` to clear it.
    fn pick(
        &mut self,
        ui: &mut egui::Ui,
        item: &Item,
        index: &ContentIndex,
        campaign: &Campaign,
        draft_groups: Option<&[Value]>,
    ) -> Option<Value> {
        let picker = self.picker.as_mut()?;
        let key = picker.key;
        let label = match key {
            "in" => "Find a location",
            "part_of" => "Find a group",
            _ => "Find a stat block",
        };
        let groups: Vec<Value> = draft_groups.map_or_else(
            || part_of(&item.entry).into_iter().cloned().collect(),
            <[Value]>::to_vec,
        );
        let joined: Vec<usize> = groups
            .iter()
            .filter_map(|value| index.world_value_target(value))
            .collect();
        let width = ui.available_width();
        let response = design::search_field(ui, label, &mut picker.search, label, width);
        if std::mem::take(&mut picker.focus) {
            response.request_focus();
        }
        let matches = if key == "creature" {
            index.creatures_matching(&picker.search, 5)
        } else {
            // Anything can hold a place or a member. Locations come first for a place.
            let wanted = picker.search.to_lowercase();
            let mut found: Vec<(bool, usize)> = index
                .items
                .iter()
                .enumerate()
                .filter(|(at, i)| {
                    i.is_private()
                        && i.entry.kind == "world"
                        && i.entry.id != item.entry.id
                        && !(key == "part_of" && joined.contains(at))
                        && i.entry.name.to_lowercase().contains(&wanted)
                })
                .map(|(at, i)| (key == "in" && world_type(&i.entry) != "location", at))
                .collect();
            found.sort_by_key(|&(later, _)| later);
            found.into_iter().map(|(_, at)| at).take(5).collect()
        };
        let mut chosen = None;
        let mut cancel = false;
        for at in matches {
            let entry = &index.items[at].entry;
            if design::link(ui, &index.shown_name(at)).clicked() {
                chosen = if key == "part_of" {
                    let mut groups = groups.clone();
                    groups.push(index.link_to(at).into());
                    Some(Value::Sequence(groups))
                } else {
                    serde_yaml_ng::to_value(EntryRef::id(entry.id)).ok()
                };
            }
        }
        if key == "part_of" {
            for (at, group) in groups.iter().enumerate() {
                let name = format!("Remove {}", link_name(group, "group"));
                if Action::quiet(&name).line_start().show(ui).clicked() {
                    let mut groups = groups.clone();
                    groups.remove(at);
                    chosen = Some(Value::Sequence(groups));
                }
            }
        }
        if key == "in" {
            design::quiet_section(ui, "New location…", |ui| {
                design::text_field(ui, "Location name", &mut picker.new_location, None, None);
                let reason = picker
                    .new_location
                    .trim()
                    .is_empty()
                    .then_some("Type a name first.");
                ui.horizontal_wrapped(|ui| {
                    if Action::secondary("Create location here")
                        .blocked(reason)
                        .show(ui)
                        .clicked()
                    {
                        match create_entry(campaign, &picker.new_location, "location") {
                            Ok(id) => {
                                let mut target = EntryRef::id(id);
                                target.link = Some(picker.new_location.trim().to_owned());
                                chosen = serde_yaml_ng::to_value(target).ok();
                            }
                            Err(error) => self.error = Some(error),
                        }
                    }
                    design::blocked_reason(ui, reason);
                });
            });
        }
        ui.horizontal_wrapped(|ui| {
            if key != "part_of" && Action::quiet("Clear value").line_start().show(ui).clicked() {
                chosen = Some(Value::Null);
            }
            cancel = starts_line(ui, Action::quiet("Cancel"), key == "part_of")
                .show(ui)
                .clicked();
        });
        if chosen.is_some()
            || cancel
            || ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
        {
            self.picker = None;
        }
        chosen
    }

    fn entry_controls(
        &mut self,
        ui: &mut egui::Ui,
        item: &Item,
        index: &ContentIndex,
        campaign: Option<&Campaign>,
    ) -> Option<Go> {
        let campaign = campaign.filter(|_| item.is_private())?;
        let mut go = None;
        if self.compact {
            let mut targets = vec![
                ("Stat block", index.world_target(&item.entry, "creature")),
                ("In", index.place_of(item.entry.id)),
            ];
            targets.extend(
                index
                    .groups_of(&item.entry)
                    .iter()
                    .map(|(_, group)| ("Part of", *group)),
            );
            let targets: Vec<_> = targets
                .into_iter()
                .filter_map(|(label, target)| target.map(|at| (label, at)))
                .collect();
            // With no target, the line draws no row, so it adds no gap.
            if !targets.is_empty() {
                ui.horizontal_wrapped(|ui| {
                    for (n, (label, at)) in targets.into_iter().enumerate() {
                        if n > 0 {
                            ui.label(Type::Small.rich(ui, "·").color(colors().muted));
                        }
                        ui.label(Type::Small.rich(ui, label).color(colors().muted));
                        if design::link(ui, &index.items[at].entry.name).clicked() {
                            go = Some(Go::Item(at, None));
                        }
                    }
                });
            }
            // A link fault reads as every compact value does (FD6): the muted label, then the
            // value in the warning tone, `⚠ <name> · Not found`.
            if broken_links(item, index).contains(&LinkProblem::StatBlockNotFound) {
                let name = reader_field(&item.entry, "creature")
                    .map(|v| link_name(v, "Stat block"))
                    .unwrap_or_else(|| "Stat block".into());
                compact_missing(ui, "Stat block", &name);
            }
            let choices = index.place_choices(item.entry.id);
            if !choices.is_empty() {
                ui.horizontal(|ui| {
                    ui.label(Type::Small.rich(ui, "In").color(colors().muted));
                    let name = world_field(&item.entry, "in")
                        .map(|value| link_name(value, "Place"))
                        .unwrap_or_default();
                    if warning_link(ui, &place_choices_words(&name, choices.len())).clicked() {
                        self.place_choices_open = !self.place_choices_open;
                    }
                });
                if self.place_choices_open {
                    let (followed, chosen) = self.place_chooser(ui, item, index, None);
                    go = go.or(followed);
                    if let Some(at) = chosen {
                        let path = index.items[at].entry.path.trim_end_matches(".md");
                        return self.change(
                            campaign,
                            item,
                            index,
                            "in",
                            format!("[[{path}]]").into(),
                        );
                    }
                }
            }
            if index.place_of(item.entry.id).is_none()
                && index.place_choices(item.entry.id).is_empty()
                && let Some(place) = reader_field(&item.entry, "in").and_then(Value::as_str)
            {
                compact_missing(ui, "In", place.trim_matches(['[', ']']));
            }
            if go.is_some() {
                return go;
            }
        }
        if !self.compact
            && let Some(go) = self.relationships(ui, item, index, campaign)
        {
            return Some(go);
        }
        // Status and its Change show in both readers, World's and Lookup's: one act, one write
        // and one Undo wherever the DM reads the entry.
        let value_width = self.relationship_value_width(ui, item, index);
        if !self.status_editing
            && (!status(&item.entry).trim().is_empty() || status_unread(&item.entry))
        {
            let value = status(&item.entry);
            let unread = status_unread(&item.entry);
            let changed = Self::relationship_row(
                ui,
                "Status",
                value_width,
                |ui| {
                    // Round 26 FE5: a set value is in `Body` in the text colour, as a set
                    // link is; only `not read` and `not set` stay `Small`.
                    let (words, role, color) = if unread {
                        ("not read", Type::Small, colors().result.warning)
                    } else if value.is_empty() {
                        ("not set", Type::Small, colors().muted)
                    } else {
                        (value, Type::Body, colors().text)
                    };
                    ui.label(role.rich(ui, words).color(color));
                },
                None,
            );
            if changed {
                // The field starts from the value the reader shows now, not a copy from when
                // the entry opened: the other reader or Obsidian may have changed it since.
                status(&item.entry).clone_into(&mut self.status);
                self.status_editing = true;
                self.status_select = true;
            }
        }
        if self.status_editing {
            let response = design::text_field(ui, "Status", &mut self.status, None, None);
            // Change puts the keyboard in the field with its words selected, so the DM types
            // the new value at once. A pass that still holds Change's click takes the keyboard
            // from a field that was not clicked, so the request waits for a pass with no click.
            if self.status_select {
                design::focus_and_select(ui, response.id, &self.status);
                self.status_select = ui.input(|input| input.pointer.any_click());
            }
            // While the field has the keyboard, Escape is the field's, not Lookup's.
            if response.has_focus() || response.lost_focus() {
                self.form_focused = true;
            }
            let wanted = self.status.trim().to_owned();
            if wanted != status(&item.entry) {
                // A one-line field lets go of the keyboard on Enter, so Enter saves as it lets go.
                let mut save = (response.has_focus() || response.lost_focus())
                    && ui.input(|input| input.key_pressed(egui::Key::Enter));
                ui.horizontal(|ui| {
                    design::status(
                        ui,
                        design::Status {
                            icon: design::icons::DRAFT,
                            word: "Unsaved",
                            color: colors().document.unsaved,
                        },
                    );
                    save |= Action::secondary("Save status")
                        .density(Density::Read)
                        .show(ui)
                        .clicked();
                });
                if save {
                    let result = self.change(campaign, item, index, "status", wanted.into());
                    if result.is_some() {
                        self.status_editing = false;
                        self.status_drafts.remove(&item.entry.id);
                    }
                    return result;
                }
            }
        }
        if !fights(&item.entry) {
            return None;
        }
        if let Some((index, initiative)) = self.in_run.get(&item.entry.id) {
            let word = format!(
                "In the run · initiative {}",
                initiative.map_or_else(|| "unknown".to_owned(), |n| n.to_string())
            );
            if design::status_link(
                ui,
                design::Status {
                    icon: design::icons::RUN,
                    word: &word,
                    color: colors().accent,
                },
            )
            .clicked()
            {
                self.select_run = Some(*index);
            }
            return None;
        }
        let sides = self.sides.as_ref()?;
        let Some(adding) = &mut self.adding else {
            if Action::secondary("Add to fight")
                .icon(design::icons::ADD)
                .density(Density::Read)
                .show(ui)
                .clicked()
            {
                self.adding = Some(Adding {
                    stat_block: index.world_target(&item.entry, "creature"),
                    save: true,
                    side: if sides.is_empty() || sides.iter().any(|side| side == "enemies") {
                        "enemies".to_owned()
                    } else {
                        sides[0].clone()
                    },
                    ..Default::default()
                });
            }
            return None;
        };
        match add_form(ui, item, index, sides, adding)? {
            FormEnd::Cancel => self.adding = None,
            FormEnd::Add => {
                let stat_block = &index.items[adding.stat_block?].entry;
                let total = match adding.initiative.trim() {
                    "" => None,
                    text => match text.parse::<i64>() {
                        Ok(total) => Some(total),
                        Err(_) => {
                            self.error = Some("Initiative must be a whole number.".to_owned());
                            return None;
                        }
                    },
                };
                let saved = adding.save && index.world_target(&item.entry, "creature").is_none();
                if saved {
                    match set_world_field(
                        campaign,
                        &item.entry,
                        "creature",
                        stored_link(stat_block).into(),
                    ) {
                        Ok(mut change) => {
                            change.word = change_words("Stat block", None, Some(&stat_block.name));
                            self.undo = Some(change);
                            // A field of its own: `adding` still borrows the form.
                            self.orders.insert(FooterAct::Change, next_order());
                            self.kept = None;
                        }
                        Err(error) => self.error = Some(error),
                    }
                }
                let group = npc_group(&item.entry, stat_block, &adding.side);
                self.adding = None;
                return Some(Go::Join(Join {
                    group,
                    total,
                    saved,
                }));
            }
        }
        None
    }
}

/// Choose the stat block, side and initiative. Returns how the form ended, if it did.
fn add_form(
    ui: &mut egui::Ui,
    item: &Item,
    index: &ContentIndex,
    sides: &[String],
    adding: &mut Adding,
) -> Option<FormEnd> {
    ui.label(Type::Strong.rich(ui, format!("Add {} to the fight", item.entry.name)));
    match adding.stat_block.map(|at| &index.items[at].entry) {
        Some(stat_block) => {
            ui.horizontal_wrapped(|ui| {
                ui.label(Type::Small.rich(ui, "Stat block").color(colors().muted));
                ui.label(Type::Body.rich(ui, &stat_block.name).color(colors().text));
                if Action::quiet("Change")
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    adding.search = stat_block.name.clone();
                    adding.stat_block = None;
                }
            });
        }
        None => {
            let width = ui.available_width();
            design::search_field(
                ui,
                "Stat block",
                &mut adding.search,
                "Stat block: Commoner, Guard…",
                width,
            );
            for position in index.creatures_matching(&adding.search, 5) {
                if design::link(ui, &index.items[position].entry.name).clicked() {
                    adding.stat_block = Some(position);
                }
            }
        }
    }
    if creature(&item.entry).is_none() {
        ui.checkbox(&mut adding.save, "Save this as their stat block");
    }
    if sides.len() > 1 {
        design::small_field(ui, "Side", None, None, |ui| {
            let combo = egui::ComboBox::from_id_salt("world_add_side")
                .selected_text(design::reader::human_label(&adding.side))
                .icon(design::combo_icon)
                .show_ui(ui, |ui| {
                    for side in sides {
                        ui.selectable_value(
                            &mut adding.side,
                            side.clone(),
                            design::reader::human_label(side),
                        );
                    }
                });
            design::named(ui, &combo.response, "Side");
        });
    }
    design::small_field(
        ui,
        "Initiative",
        Some("Leave empty to roll it."),
        None,
        |ui| {
            design::text_input(
                ui,
                "Initiative",
                &mut adding.initiative,
                "",
                design::L * 5.0,
            );
        },
    );
    let mut end = None;
    ui.horizontal(|ui| {
        if Action::secondary("Add to fight")
            .density(Density::Read)
            .blocked_if(adding.stat_block.is_none(), "Choose a stat block first.")
            .show(ui)
            .clicked()
        {
            end = Some(FormEnd::Add);
        }
        if Action::quiet("Cancel")
            .density(Density::Read)
            .show(ui)
            .clicked()
        {
            end = Some(FormEnd::Cancel);
        }
    });
    end
}

#[cfg(test)]
mod tests {
    #[test]
    fn iter9_lead_duplicate_with_one_path_names_no_other_file() {
        // Rook.md (world), Rook item.md (same id, an item) and Rook copy.md (world) leave
        // Rook copy.md with one duplicate path; its first action must not read a second one.
        let one = vec!["Rook copy.md".to_string()];
        assert_eq!(duplicate_consequence(&one, 0), None);
        let two = vec!["Rook.md".to_string(), "Rook copy.md".to_string()];
        assert_eq!(
            duplicate_consequence(&two, 0).as_deref(),
            Some("Existing links keep Rook copy.md.")
        );
        assert_eq!(
            duplicate_consequence(&two, 1).as_deref(),
            Some("Existing links will then use Rook.md.")
        );
    }

    use super::*;

    /// Round 24 FD5: `Rename the file to <name>.md` and its Undo never replace or lose a file.
    /// Bad inputs: a name that cannot be a file name, a file already at the target, a note
    /// changed on disk since the DM saw it, the note moved away, and Undo after either side
    /// changed.
    #[test]
    fn iter24_d_fd5_file_rename_is_guarded_and_undone() {
        assert_eq!(
            renamed_path("Notes/Broken.md", "Repaired watchkeeper").as_deref(),
            Some("Notes/Repaired watchkeeper.md")
        );
        assert_eq!(
            renamed_path("Broken.md", "Rook").as_deref(),
            Some("Rook.md")
        );
        for name in ["", "  ", "A/B", "Who?", ".hidden", "[[Rook]]", "Broken"] {
            assert_eq!(renamed_path("Notes/Broken.md", name), None, "{name:?}");
        }
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Rename checks".into(),
            path: root.into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        std::fs::create_dir_all(root.join("Notes")).unwrap();
        let text = "---\nsplat_schema: 2\nsplat_kind: world\nsplat_name: Rook\n---\n";
        let from = root.join("Notes/Broken.md");
        let to = root.join("Notes/Rook.md");
        std::fs::write(&from, text).unwrap();
        // A file already at the target keeps its bytes, and the note stays where it was.
        std::fs::write(&to, "Another note.").unwrap();
        let refused = FileRename::apply(&campaign, "Notes/Broken.md", "Notes/Rook.md", text);
        assert!(refused.is_err(), "fd5-target-exists");
        assert_eq!(std::fs::read_to_string(&to).unwrap(), "Another note.");
        assert_eq!(std::fs::read_to_string(&from).unwrap(), text);
        std::fs::remove_file(&to).unwrap();
        // Review repair 2: a file that arrives at the target after the check, just before the
        // move, keeps its bytes, and so does the note.
        let raced = move_note(root, "Notes/Broken.md", "Notes/Rook.md", text, || {
            std::fs::write(&to, "Arrived during the move.").unwrap()
        });
        assert!(raced.is_err(), "review-2-race-refused");
        assert_eq!(
            std::fs::read_to_string(&to).unwrap(),
            "Arrived during the move.",
            "review-2-race-target-bytes"
        );
        assert_eq!(
            std::fs::read_to_string(&from).unwrap(),
            text,
            "review-2-race-source-bytes"
        );
        std::fs::remove_file(&to).unwrap();
        // Review re-check 3: a file system that does not offer RENAME_NOREPLACE (EINVAL). A hard
        // link and the removal of the old name move the note instead.
        let einval = |_: &Path, _: &Path| Err(std::io::Error::from_raw_os_error(libc::EINVAL));
        let link_moves = Moves {
            rename: einval,
            link: |source, target| std::fs::hard_link(source, target),
            remove: |path| std::fs::remove_file(path),
        };
        move_note_with(
            &link_moves,
            root,
            "Notes/Broken.md",
            "Notes/Rook.md",
            text,
            || {},
        )
        .expect("re-check-3-fallback");
        assert!(!from.exists(), "re-check-3-fallback-moved");
        assert_eq!(
            std::fs::read_to_string(&to).unwrap(),
            text,
            "re-check-3-fallback-bytes"
        );
        move_note_with(
            &link_moves,
            root,
            "Notes/Rook.md",
            "Notes/Broken.md",
            text,
            || {},
        )
        .expect("re-check-3-fallback-back");
        // The fallback never replaces a file that arrives during the move.
        let raced = move_note_with(
            &link_moves,
            root,
            "Notes/Broken.md",
            "Notes/Rook.md",
            text,
            || std::fs::write(&to, "Arrived during the move.").unwrap(),
        );
        assert!(raced.is_err(), "re-check-3-fallback-race");
        assert_eq!(
            std::fs::read_to_string(&to).unwrap(),
            "Arrived during the move."
        );
        assert_eq!(std::fs::read_to_string(&from).unwrap(), text);
        std::fs::remove_file(&to).unwrap();
        // Neither move is offered (hard links refused with EPERM, as on FAT): a plain refusal,
        // and both names keep what they held.
        let no_moves = Moves {
            rename: einval,
            link: |_, _| Err(std::io::Error::from_raw_os_error(libc::EPERM)),
            remove: |path| std::fs::remove_file(path),
        };
        assert_eq!(
            move_note_with(
                &no_moves,
                root,
                "Notes/Broken.md",
                "Notes/Rook.md",
                text,
                || {}
            ),
            Err(NO_SAFE_MOVE.to_owned()),
            "re-check-3-refused"
        );
        assert_eq!(
            std::fs::read_to_string(&from).unwrap(),
            text,
            "re-check-3-source"
        );
        assert!(!to.exists(), "re-check-3-target");
        // Re-check 2, repair 1: the link works, then the old name cannot be removed (a folder
        // made read only). The new name goes again, so nothing moved, and the words say so.
        let keep_old = Moves {
            rename: einval,
            link: |source, target| std::fs::hard_link(source, target),
            remove: |path| {
                if path.ends_with("Broken.md") {
                    Err(std::io::Error::from(std::io::ErrorKind::PermissionDenied))
                } else {
                    std::fs::remove_file(path)
                }
            },
        };
        let refused = move_note_with(
            &keep_old,
            root,
            "Notes/Broken.md",
            "Notes/Rook.md",
            text,
            || {},
        );
        assert_eq!(
            refused,
            Err("Splat has no permission to save Notes/Broken.md. Nothing was renamed.".to_owned()),
            "re-check-2-1-new-name-removed"
        );
        assert!(
            std::fs::read_to_string(&from).unwrap() == text && !to.exists(),
            "re-check-2-1-one-name"
        );
        // Neither name can be removed: both stay, and the words name both and the way out.
        let keep_both = Moves {
            rename: einval,
            link: |source, target| std::fs::hard_link(source, target),
            remove: |_| Err(std::io::Error::from(std::io::ErrorKind::PermissionDenied)),
        };
        let refused = move_note_with(
            &keep_both,
            root,
            "Notes/Broken.md",
            "Notes/Rook.md",
            text,
            || {},
        );
        assert_eq!(
            refused,
            Err(two_names("Notes/Broken.md", "Notes/Rook.md")),
            "re-check-2-1-two-names"
        );
        assert!(
            refused
                .unwrap_err()
                .contains("Delete Notes/Rook.md in your file manager"),
            "re-check-2-1-way-out"
        );
        assert!(
            std::fs::read_to_string(&from).unwrap() == text
                && std::fs::read_to_string(&to).unwrap() == text,
            "re-check-2-1-both-names"
        );
        std::fs::remove_file(&to).unwrap();
        // The note changed on disk after the DM saw it.
        let refused = FileRename::apply(&campaign, "Notes/Broken.md", "Notes/Rook.md", "Old text");
        assert!(refused.is_err() && !to.exists(), "fd5-changed");
        // The note moved away: nothing to rename.
        assert!(
            FileRename::apply(&campaign, "Notes/Gone.md", "Notes/Rook.md", text).is_err(),
            "fd5-gone"
        );
        let rename =
            FileRename::apply(&campaign, "Notes/Broken.md", "Notes/Rook.md", text).unwrap();
        assert!(!from.exists(), "fd5-moved");
        assert_eq!(std::fs::read_to_string(&to).unwrap(), text, "fd5-bytes");
        assert_eq!(rename.message(), "The file is now Rook.md.");
        // Review repair 3: the Undo belongs to its campaign. Another campaign with a note at the
        // same relative path neither shows it nor moves anything.
        let other_dir = tempfile::tempdir().unwrap();
        let other = Campaign {
            id: Uuid::new_v4(),
            path: other_dir.path().into(),
            ..campaign.clone()
        };
        std::fs::create_dir_all(other_dir.path().join("Notes")).unwrap();
        std::fs::write(other_dir.path().join("Notes/Rook.md"), text).unwrap();
        assert!(
            !rename.shows_on(&other, "Notes/Rook.md")
                && rename.shows_on(&campaign, "Notes/Rook.md"),
            "review-3-owner"
        );
        assert!(rename.undo(&other).is_err(), "review-3-undo-refused");
        assert!(
            other_dir.path().join("Notes/Rook.md").exists()
                && !other_dir.path().join("Notes/Broken.md").exists(),
            "review-3-other-bytes"
        );
        // Undo refuses while a new file holds the old path, and keeps both.
        std::fs::write(&from, "A new note.").unwrap();
        assert!(rename.undo(&campaign).is_err(), "fd5-undo-target-exists");
        assert_eq!(std::fs::read_to_string(&from).unwrap(), "A new note.");
        assert_eq!(std::fs::read_to_string(&to).unwrap(), text);
        std::fs::remove_file(&from).unwrap();
        // Undo refuses when the renamed file changed since.
        std::fs::write(&to, format!("{text}\nAn outside edit.\n")).unwrap();
        assert!(rename.undo(&campaign).is_err(), "fd5-undo-changed");
        assert!(!from.exists());
        std::fs::write(&to, text).unwrap();
        rename.undo(&campaign).unwrap();
        assert_eq!(std::fs::read_to_string(&from).unwrap(), text, "fd5-undo");
        assert!(!to.exists(), "fd5-undo-moved");
    }

    #[test]
    fn iter8_optional_links_warn_only_when_set_and_unresolved() {
        for (value, expected) in [
            ("null", false),
            ("''", false),
            ("'  '", false),
            ("'[[Nowhere]]'", true),
        ] {
            for (key, warning) in [
                ("in", LinkProblem::PlaceNotFound),
                ("creature", LinkProblem::StatBlockNotFound),
            ] {
                let dir = tempfile::tempdir().unwrap();
                let campaign = sample_campaign(dir.path());
                let id = create_entry(&campaign, "Rook", "npc").unwrap();
                let path = dir.path().join("NPCs/Rook.md");
                let field = if key == "creature" {
                    "splat_stat_block"
                } else {
                    "splat_in"
                };
                let file = fs::read_to_string(&path).unwrap().replacen(
                    "---\n",
                    &format!("---\n{field}: {value}\n"),
                    1,
                );
                fs::write(path, file).unwrap();
                let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
                let item = &index.items[index.position(id).unwrap()];
                assert_eq!(
                    broken_links(item, &index).contains(&warning),
                    expected,
                    "FD-1-{key}-{value}"
                );
            }
        }
    }

    #[test]
    fn iter12_d_fd1_a_place_with_two_matches_needs_fixing() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = sample_campaign(dir.path());
        let id = create_entry(&campaign, "Rook", "npc").unwrap();
        for folder in ["Locations", "Other"] {
            fs::create_dir_all(dir.path().join(folder)).unwrap();
            fs::write(
                dir.path().join(format!("{folder}/Watch House.md")),
                "# Watch House\n",
            )
            .unwrap();
        }
        let path = dir.path().join("NPCs/Rook.md");
        let file = fs::read_to_string(&path).unwrap();
        for (place, expected) in [
            ("[[Watch House]]", vec![LinkProblem::PlaceMatches(2)]),
            // `Always use this one` writes the chosen file's path. The warning goes.
            ("[[Locations/Watch House]]", vec![]),
            // Neighbour: a missing place still says not found, not matches.
            ("[[Nowhere]]", vec![LinkProblem::PlaceNotFound]),
        ] {
            fs::write(
                &path,
                file.replacen("---\n", &format!("---\nsplat_in: '{place}'\n"), 1),
            )
            .unwrap();
            let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
            let item = &index.items[index.position(id).unwrap()];
            assert!(item.entry.flags.is_empty(), "fixture {place} reads");
            assert_eq!(broken_links(item, &index), expected, "FD1-{place}");
        }
        assert_eq!(
            LinkProblem::PlaceMatches(2).to_string(),
            "Place has 2 matches",
            "FD1-words"
        );
    }

    #[test]
    fn iter13_d_sd2_mixed_link_problems_need_fixing_and_missing_ones_are_broken() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = sample_campaign(dir.path());
        let id = create_entry(&campaign, "Rook", "npc").unwrap();
        for folder in ["Locations", "Other"] {
            fs::create_dir_all(dir.path().join(folder)).unwrap();
            fs::write(
                dir.path().join(format!("{folder}/Watch House.md")),
                "# Watch House\n",
            )
            .unwrap();
        }
        // Round 23 FG2: a place whose `In` leads back to Rook makes a loop.
        let loop_two = create_entry(&campaign, "Loop Two", "location").unwrap();
        let loop_path = {
            let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
            dir.path()
                .join(&index.items[index.position(loop_two).unwrap()].entry.path)
        };
        let loop_file = fs::read_to_string(&loop_path).unwrap();
        fs::write(
            &loop_path,
            loop_file.replacen("---\n", "---\nsplat_in: '[[Rook]]'\n", 1),
        )
        .unwrap();
        let path = dir.path().join("NPCs/Rook.md");
        let file = fs::read_to_string(&path).unwrap();
        for (fields, expected, words) in [
            // Round 23 FG2: a note in a place loop names the loop on its row.
            (
                "splat_in: '[[Loop Two]]'\n",
                vec![LinkProblem::PlaceLoop],
                "Place loop",
            ),
            // A loop and a missing group: the loop is not a broken link.
            (
                "splat_in: '[[Loop Two]]'\nsplat_part_of: ['[[Nowhere]]']\n",
                vec![LinkProblem::PlaceLoop, LinkProblem::GroupNotFound],
                "2 links need fixing",
            ),
            // Lookalike: a place that does not lead back is no loop.
            ("splat_in: '[[Locations/Watch House]]'\n", vec![], ""),
            // A place with two matches and a missing group: one waits for a choice.
            (
                "splat_in: '[[Watch House]]'\nsplat_part_of: ['[[Nowhere]]']\n",
                vec![LinkProblem::PlaceMatches(2), LinkProblem::GroupNotFound],
                "2 links need fixing",
            ),
            // Two missing targets are broken links.
            (
                "splat_in: '[[Nowhere]]'\nsplat_part_of: ['[[Elsewhere]]']\n",
                vec![LinkProblem::PlaceNotFound, LinkProblem::GroupNotFound],
                "2 broken links",
            ),
            // Neighbour: a single place with two matches names itself.
            (
                "splat_in: '[[Watch House]]'\n",
                vec![LinkProblem::PlaceMatches(2)],
                "Place has 2 matches",
            ),
            // Neighbour: a single missing group names itself.
            (
                "splat_part_of: ['[[Nowhere]]']\n",
                vec![LinkProblem::GroupNotFound],
                "Group not found",
            ),
        ] {
            fs::write(&path, file.replacen("---\n", &format!("---\n{fields}"), 1)).unwrap();
            let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
            let item = &index.items[index.position(id).unwrap()];
            assert!(item.entry.flags.is_empty(), "fixture {fields} reads");
            let problems = broken_links(item, &index);
            assert_eq!(problems, expected, "SD2-problems-{fields}");
            if !words.is_empty() {
                assert_eq!(link_problem_words(&problems), words, "SD2-words-{fields}");
            }
            // The other end of the loop names the loop too, and only while Rook closes it.
            let other = &index.items[index.position(loop_two).unwrap()];
            assert_eq!(
                broken_links(other, &index).contains(&LinkProblem::PlaceLoop),
                fields.contains("Loop Two"),
                "FG2-other-end-{fields}"
            );
        }
    }

    #[test]
    fn iter11_d_fd1_each_missing_group_is_a_broken_link() {
        for (groups, expected) in [
            ("['[[Nowhere]]']", 1),
            ("['[[Nowhere]]', '[[Elsewhere|the else]]']", 2),
            ("['[[Arena Guild]]', '[[Nowhere]]']", 1),
            ("['[[Factions/Arena Guild|the arena]]']", 0),
            ("['[[Arena Guild]]']", 0),
            ("[]", 0),
            ("['']", 0),
            ("['  ']", 0),
            // A key with no value reads as an empty list.
            ("", 0),
            ("# No groups yet.", 0),
        ] {
            let dir = tempfile::tempdir().unwrap();
            let campaign = sample_campaign(dir.path());
            let id = create_entry(&campaign, "Rook", "npc").unwrap();
            create_entry(&campaign, "Arena Guild", "faction").unwrap();
            let path = dir.path().join("NPCs/Rook.md");
            let file = fs::read_to_string(&path).unwrap().replacen(
                "---\n",
                &format!("---\nsplat_part_of: {groups}\n"),
                1,
            );
            fs::write(path, file).unwrap();
            let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
            let item = &index.items[index.position(id).unwrap()];
            assert!(item.entry.flags.is_empty(), "fixture {groups} reads");
            let problems = broken_links(item, &index);
            assert_eq!(
                problems
                    .iter()
                    .filter(|problem| **problem == LinkProblem::GroupNotFound)
                    .count(),
                expected,
                "FD1-{groups}"
            );
            // Neighbour: Part of adds no In or Stat block warning.
            assert_eq!(problems.len(), expected, "FD1-only-groups-{groups}");
        }
    }

    #[test]
    fn iter8_d2_compact_reader_warns_only_for_nonempty_unresolved_links() {
        let mut actual = Vec::new();
        let mut expected = Vec::new();
        for key in ["splat_in", "splat_stat_block"] {
            for (value, warning) in [("''", false), ("'  '", false), ("'[[Nowhere]]'", true)] {
                let dir = tempfile::tempdir().unwrap();
                let campaign = sample_campaign(dir.path());
                let id = create_entry(&campaign, "Rook", "npc").unwrap();
                let path = dir.path().join("NPCs/Rook.md");
                let file = fs::read_to_string(&path).unwrap().replacen(
                    "---\n",
                    &format!("---\n{key}: {value}\n"),
                    1,
                );
                fs::write(path, file).unwrap();
                let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
                let item = &index.items[index.position(id).unwrap()];
                let mut reader = Reader {
                    compact: true,
                    sides: Some(Vec::new()),
                    ..Default::default()
                };
                let ctx = egui::Context::default();
                crate::design::theme::install(&ctx);
                ctx.enable_accesskit();
                let output = ctx.run(Default::default(), |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        reader.entry_controls(ui, item, &index, Some(&campaign));
                    });
                });
                let nodes = output.platform_output.accesskit_update.unwrap().nodes;
                let labels: Vec<_> = nodes
                    .iter()
                    .filter_map(|(_, node)| node.label().or_else(|| node.value()))
                    .collect();
                assert!(
                    labels.contains(&"Add to fight"),
                    "D2-compact-fixture: compact reader controls must render"
                );
                actual.push((
                    key,
                    value,
                    labels.iter().any(|label| label.contains("Not found")),
                ));
                expected.push((key, value, warning));
            }
        }
        assert_eq!(
            actual, expected,
            "D2-compact: empty optional links have no warning; unresolved links keep it"
        );
    }

    #[test]
    fn i19_h1_world_notice_names_the_change_and_undo() {
        let ctx = egui::Context::default();
        crate::design::theme::install(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let line = FooterLine::new(
                    FooterRank::Undo,
                    CHANGED,
                    "Place set to North Gate. It was Watch House.",
                    FooterKind::Undo {
                        label: ChangeKind::Place.undo_label(),
                        act: FooterAct::Change,
                    },
                );
                draw_footer(ui, "test_footer", &[line]);
            });
        });
        let nodes = output.platform_output.accesskit_update.unwrap().nodes;
        let labels: Vec<_> = nodes.iter().filter_map(|(_, node)| node.label()).collect();
        assert_eq!(
            labels
                .iter()
                .filter(|s| s.starts_with("Undo"))
                .copied()
                .collect::<Vec<_>>(),
            ["Undo place change"]
        );
    }

    #[test]
    fn integrity_d27_focus_checks_include_run_json_files() {
        let dir = tempfile::tempdir().unwrap();
        let runs = dir.path().join("Encounter Runs");
        fs::create_dir_all(&runs).unwrap();
        let before = fingerprint(dir.path());
        let path = runs.join("fight.json");
        fs::write(&path, "run").unwrap();
        let saved = fingerprint(dir.path());
        assert_ne!(saved, before);
        fs::write(&path, "DM changed the run").unwrap();
        assert_ne!(fingerprint(dir.path()), saved);
    }

    fn note(dir: &Path, path: &str, text: &str) {
        let path = dir.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }

    #[test]
    fn renaming_links_keeps_aliases_headings_code_and_other_targets() {
        let text = "[[Cinder Wolf]] [[Cinder Wolf#Bite|fang]] [[Other/Cinder Wolf]] `[[Cinder Wolf]]`\n```md\n[[Cinder Wolf]]\n```\n";
        let (after, count) = rename_note_links(text, "Cinder Wolf", "Ash Wolf");
        assert_eq!(count, 2);
        assert_eq!(
            after,
            text.replacen("[[Cinder Wolf]]", "[[Ash Wolf]]", 1)
                .replace("[[Cinder Wolf#Bite|fang]]", "[[Ash Wolf#Bite|fang]]")
        );
    }

    #[test]
    fn world_identity_repair_writes_only_its_file_and_undo_keeps_later_edits() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Test".into(),
            path: dir.path().into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        let before = splat::content::formats::patch_model(
            &crate::library::note_text("Rook", "world", None),
            &[(
                "splat_world",
                Some(serde_yaml_ng::from_str("{type: npc}").unwrap()),
            )],
        )
        .unwrap();
        fs::write(dir.path().join("Rook.md"), &before).unwrap();
        fs::write(dir.path().join("Rook copy.md"), &before).unwrap();
        let (undo, id) = super::new_identity(&campaign, "Rook copy.md").unwrap();
        assert_eq!(
            fs::read_to_string(dir.path().join("Rook.md")).unwrap(),
            before
        );
        assert_eq!(undo.message(), "Rook copy.md is now a separate note.");
        assert_eq!(undo.identity_undo_label(), "Undo separate note");
        assert_eq!(Entry::parse("copy.md", &undo.after).unwrap().id, id);
        let repaired = Entry::parse("copy.md", &undo.after).unwrap();
        assert_eq!(repaired.name, "Rook copy");
        assert!(repaired.body.contains("# Rook copy\n"));
        undo.undo(&campaign).unwrap();
        assert_eq!(
            fs::read_to_string(dir.path().join("Rook copy.md")).unwrap(),
            before
        );
        let (undo, _) = super::new_identity(&campaign, "Rook copy.md").unwrap();
        fs::write(
            dir.path().join("Rook copy.md"),
            format!("{}\nExternal edit.", undo.after),
        )
        .unwrap();
        assert!(undo.undo(&campaign).is_err());
    }

    #[test]
    fn a_note_is_found_by_its_role_and_not_its_folder() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let sessions = root.join("Sessions");
        note(root, "Setting.md", "# Setting\n");
        note(root, "Powers/Verori.md", "# Verori\n");
        note(root, "Sessions/Beside the sessions.md", "World\n");
        note(
            root,
            "Sessions/Session 1.md",
            "---\nsession_id: x\n---\nWe met.\n",
        );
        note(
            root,
            "Sessions/Timeline.md",
            "---\ntimeline_version: 2\n---\n",
        );
        note(root, "Sessions/Templates/Lazy.md", "Template\n");
        note(root, "Sessions/Recordings/a/a.transcript.md", "raw\n");
        note(
            root,
            "Sessions/Recordings/a/a.marked.md",
            "---\nmarked_model: x\n---\nbest\n",
        );
        note(root, "Sessions/Recordings/a/a.review.md", "no\n");
        note(root, "Sessions/Recordings/b/b.transcript.md", "only\n");
        note(root, "Trackers.md", "---\ntrackers_version: 1\n---\n");
        note(root, "Entry.md", "---\nsplat_schema: 2\n---\n");
        note(root, ".obsidian/plugins/a/README.md", "no\n");
        note(root, ".trash/Old.md", "no\n");
        fs::write(root.join("Map.PNG"), b"x").unwrap();
        let scan = scan(root, &sessions);
        let found: Vec<_> = scan
            .notes
            .iter()
            .map(|n| (n.path.as_str(), n.role))
            .collect();
        assert_eq!(
            found,
            [
                ("Powers/Verori.md", Role::World),
                ("Sessions/Beside the sessions.md", Role::World),
                ("Sessions/Recordings/a/a.marked.md", Role::Transcript),
                ("Sessions/Recordings/b/b.transcript.md", Role::Transcript),
                ("Sessions/Session 1.md", Role::Session),
                ("Setting.md", Role::World),
            ]
        );
        assert_eq!(scan.notes[2].body, "best\n");
        assert!(scan.images.contains_key("map.png"));
    }

    #[test]
    fn a_session_folder_at_the_root_hides_no_world_note() {
        let dir = tempfile::tempdir().unwrap();
        note(dir.path(), "Setting.md", "# Setting\n");
        note(dir.path(), "Timeline.md", "---\ntimeline_version: 2\n---\n");
        note(
            dir.path(),
            "Trackers.md",
            "---\ntrackers_version: 1\nchanges: []\n---\n",
        );
        let scan = scan(dir.path(), dir.path());
        assert_eq!(scan.notes.len(), 1);
        assert_eq!(scan.notes[0].path, "Setting.md");
        // The scan and the ledger's reader use one rule: a ledger with a quoted key is one.
        let quoted = "---\n\"trackers_version\": 1\nchanges: []\n---\n";
        assert!(crate::trackers::ledger::Ledger::parse(quoted).is_ok());
        note(dir.path(), "Trackers.md", quoted);
        assert_eq!(super::scan(dir.path(), dir.path()).notes.len(), 1);
        // A note of the DM's named Trackers.md that is no ledger stays a world note.
        note(
            dir.path(),
            "Trackers.md",
            "---\ntags: [race]\n---\n\n# Trackers\n",
        );
        let scan = super::scan(dir.path(), dir.path());
        assert_eq!(scan.notes.len(), 2);
    }

    #[test]
    fn links_become_reader_links_outside_code() {
        let images = HashMap::from([("map.png".to_owned(), PathBuf::from("/c/Map.png"))]);
        let body = "See [[Verori]], [[Gate Mechanics#Reliability]] and [[Kaun|the East]].\n\
                    ![[Map.png]]\n![[Gone.png]]\n`[[code]]`\n```\n[[fence]]\n```\n";
        let text = with_links(body, &images);
        assert!(text.contains("[Verori](<splat-note:Verori>)"));
        assert!(text.contains("[Reliability](<splat-note:Gate Mechanics#Reliability>)"));
        assert!(text.contains("[the East](<splat-note:Kaun>)"));
        assert!(text.contains("[image](<splat-file:/c/Map.png>)"));
        assert!(text.contains("(Image not found: Gone.png)"));
        assert!(text.contains("`[[code]]`"));
        assert!(text.contains("\n[[fence]]\n"));
        assert_eq!(
            links_in(body),
            ["verori", "gate mechanics", "kaun", "map.png", "gone.png"]
        );
    }

    #[test]
    fn headings_skip_code_and_keep_their_words() {
        assert_eq!(
            headings("# One\ntext\n## Two ##\n```\n# Not\n```\n#nope\n### Three\n"),
            ["One", "Two", "Three"]
        );
    }

    #[test]
    fn a_new_entry_parses_and_never_replaces_a_note() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Sample".into(),
            path: dir.path().into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        let id = create_entry(&campaign, "Ashori: Deepcut", "npc").unwrap();
        let path = "NPCs/Ashori Deepcut.md";
        let text = fs::read_to_string(dir.path().join(path)).unwrap();
        let entry = Entry::parse(path, &text).unwrap();
        assert_eq!(entry.id, id);
        assert_eq!(entry.name, "Ashori: Deepcut");
        assert_eq!(world_type(&entry), "npc");
        assert_eq!(status(&entry), "alive");
        assert!(create_entry(&campaign, "Ashori: Deepcut", "npc").is_err());
        assert!(create_entry(&campaign, " :: ", "npc").is_err());
        set_world_field(&campaign, &entry, "status", "dead".into()).unwrap();
        let text = fs::read_to_string(dir.path().join(path)).unwrap();
        let entry = Entry::parse(path, &text).unwrap();
        assert_eq!(status(&entry), "dead");
        assert_eq!(world_type(&entry), "npc", "the rest of the entry stays");
        assert!(text.ends_with("# Ashori: Deepcut\n"), "the body stays");
    }

    #[test]
    fn a_generated_npc_saves_its_attributes_and_never_replaces_a_note() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Sample".into(),
            path: dir.path().into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        let npc = crate::npc_generator::Generated {
            name: "Tally".into(),
            attributes: ["Ink".into(), "Bored".into(), "A clean writ".into()],
        };
        let id = create_generated(&campaign, &npc).unwrap();
        let path = "NPCs/Tally.md";
        let text = fs::read_to_string(dir.path().join(path)).unwrap();
        let entry = Entry::parse(path, &text).unwrap();
        assert_eq!(entry.id, id);
        assert_eq!(world_type(&entry), "npc");
        assert_eq!(status(&entry), "alive");
        assert!(text.ends_with(
            "# Tally\n\n- **Look:** Ink\n- **Manner:** Bored\n- **Want:** A clean writ\n"
        ));
        let again = create_generated(&campaign, &npc).unwrap_err();
        assert!(again.contains("already exists"), "{again}");
        assert_eq!(fs::read_to_string(dir.path().join(path)).unwrap(), text);
    }

    /// AD2: the card's `Will save` path is the path Save NPC writes, for a campaign with no
    /// NPCs, NPCs in another folder, and NPCs at the campaign's root.
    #[test]
    fn iter15_d_ad2_the_card_promises_the_path_the_save_writes() {
        let npc_at = |path: &str, name: &str| {
            format!(
                "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: world\nsplat_name: {name}\nsplat_type: npc\n---\n\n# {name}\n",
                splat::character::stable_id(path)
            )
        };
        for (existing, expected) in [
            (vec![], "NPCs/Wren Hallam.md"),
            (
                vec!["People/Rook.md", "People/Tally.md", "NPCs/Blot.md"],
                "People/Wren Hallam.md",
            ),
            (vec!["Rook.md"], "Wren Hallam.md"),
        ] {
            let dir = tempfile::tempdir().unwrap();
            let campaign = sample_campaign(dir.path());
            for path in &existing {
                let file = dir.path().join(path);
                fs::create_dir_all(file.parent().unwrap()).unwrap();
                let name = Path::new(path).file_stem().unwrap().to_string_lossy();
                fs::write(file, npc_at(path, &name)).unwrap();
            }
            let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
            let promised = new_entry_path(Some(&index), "Wren Hallam", "npc");
            assert_eq!(promised, expected, "AD2-promise: {existing:?}");
            let npc = crate::npc_generator::Generated {
                name: "Wren Hallam".into(),
                attributes: ["Ink".into(), "Bored".into(), "A writ".into()],
            };
            create_generated(&campaign, &npc).unwrap();
            assert!(
                dir.path().join(&promised).is_file(),
                "AD2-write: the save wrote the promised {promised} for {existing:?}"
            );
        }
    }
    fn sample_campaign(root: &Path) -> Campaign {
        Campaign {
            id: Uuid::new_v4(),
            name: "Sample".into(),
            path: root.into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        }
    }

    #[test]
    fn plain_note_creation_uses_a_stable_path_identity_and_never_replaces_a_file() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = sample_campaign(dir.path());
        let id = create_entry(&campaign, "Watch notes", "").unwrap();
        assert_eq!(id, note_id("Notes/Watch notes.md"));
        let path = dir.path().join("Notes/Watch notes.md");
        assert_eq!(fs::read_to_string(&path).unwrap(), "# Watch notes\n");
        assert!(create_entry(&campaign, "Watch notes", "").is_err());
        assert_eq!(
            scan(dir.path(), &dir.path().join("Sessions")).notes.len(),
            1
        );
    }

    #[test]
    fn iter3_d_relationship_changes_share_one_column() {
        for width in [300.0, 600.0, 1000.0] {
            let ctx = egui::Context::default();
            design::theme::install(&ctx);
            let mut output = None;
            for _ in 0..2 {
                output = Some(ctx.run(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(width, 640.0),
                        )),
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            // The value column is the widest value, as in the reader.
                            let width = ["Goblin Warrior", "North Gate", "not read"]
                                .iter()
                                .map(|value| design::number_width(ui, value, &Type::Small.font(ui)))
                                .fold(0.0_f32, f32::max)
                                .min(Reader::relationship_width(ui));
                            for (label, value) in [
                                ("Stat block", "Goblin Warrior"),
                                ("In", "North Gate"),
                                ("Status", "not read"),
                            ] {
                                Reader::relationship_row(
                                    ui,
                                    label,
                                    width,
                                    |ui| {
                                        ui.label(Type::Small.rich(ui, value));
                                    },
                                    None,
                                );
                            }
                        });
                    },
                ));
            }
            let output = output.unwrap();
            let widest_value = output
                .shapes
                .iter()
                .filter_map(|shape| match &shape.shape {
                    egui::Shape::Text(text)
                        if ["Goblin Warrior", "North Gate", "not read"]
                            .contains(&text.galley.job.text.as_str()) =>
                    {
                        Some(text.pos.x + text.galley.size().x)
                    }
                    _ => None,
                })
                .fold(0.0_f32, f32::max);
            let changes: Vec<_> = output
                .shapes
                .iter()
                .filter_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) if text.galley.job.text == "Change" => Some(text.pos.x),
                    _ => None,
                })
                .collect();
            assert_eq!(changes.len(), 3);
            assert!(
                changes.iter().all(|x| (x - changes[0]).abs() < 1.0),
                "{width}: {changes:?}"
            );
            // Change follows the widest value after an L gap (plus item spacing), not the cap.
            assert!(
                changes[0] <= widest_value + design::L + design::S + 1.0,
                "{width}: Change at {} after a widest value ending at {widest_value}",
                changes[0]
            );
        }
    }

    #[test]
    fn undo_restores_exact_bytes_and_refuses_a_later_disk_edit() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = sample_campaign(dir.path());
        create_entry(&campaign, "Rook", "npc").unwrap();
        let path = "NPCs/Rook.md";
        let before = fs::read_to_string(dir.path().join(path)).unwrap();
        let after = fields::patch(&before, "in", "[[Ashfall Keep]]".into()).unwrap();
        write_note(dir.path(), path, Some(&before), &after).unwrap();
        let change = Change {
            campaign: campaign.id,
            path: path.into(),
            before: before.clone(),
            after: after.clone(),
            word: "Place changed.".into(),
            kind: ChangeKind::Place,
        };
        // Lead ruling, round 24 review: a change belongs to its campaign. Another campaign with a
        // note at the same path neither shows it nor has it undone, and keeps its bytes.
        let other_dir = tempfile::tempdir().unwrap();
        let other = sample_campaign(other_dir.path());
        fs::create_dir_all(other_dir.path().join("NPCs")).unwrap();
        fs::write(other_dir.path().join(path), &after).unwrap();
        assert!(
            !change.shows_on(&other, path) && change.shows_on(&campaign, path),
            "owner-shows-on"
        );
        assert!(change.undo(&other).is_err(), "owner-undo-refused");
        assert_eq!(
            fs::read_to_string(other_dir.path().join(path)).unwrap(),
            after,
            "owner-other-bytes"
        );
        change.undo(&campaign).unwrap();
        assert_eq!(fs::read_to_string(dir.path().join(path)).unwrap(), before);
        let external = format!("{after}\nWords from Obsidian.\n");
        fs::write(dir.path().join(path), &external).unwrap();
        assert!(
            change
                .undo(&campaign)
                .unwrap_err()
                .contains("changed on disk")
        );
        assert_eq!(fs::read_to_string(dir.path().join(path)).unwrap(), external);
    }

    #[test]
    fn a_change_in_play_writes_the_field_and_a_log_line_and_one_undo_removes_both() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = sample_campaign(dir.path());
        let hob = create_entry(&campaign, "Hob", "npc").unwrap();
        let arena = create_entry(&campaign, "The Arena", "location").unwrap();
        let cart = create_entry(&campaign, "Harvest Cart", "team").unwrap();
        let session = format!(
            "---\nsession_id: {}\nsession_title: ''\nsession_date: 2026-01-01\nsession_number: 1\n---\n# Session 1\n",
            Uuid::new_v4()
        );
        fs::create_dir_all(dir.path().join("Sessions")).unwrap();
        fs::write(dir.path().join("Sessions/Session 1.md"), session).unwrap();
        let path = dir.path().join("NPCs/Hob.md");
        let original = fs::read_to_string(&path).unwrap();
        let mut reader = Reader {
            played: Some("[[Session 1]]".into()),
            day: Some(2),
            ..Default::default()
        };
        let mut change = |key: &str, value: Value| {
            let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
            let item = &index.items[index.position(hob).unwrap()];
            assert!(reader.change(&campaign, item, &index, key, value).is_some());
            reader.undo.clone().unwrap()
        };
        let status = change("status", "dead".into());
        let place = change("in", serde_yaml_ng::to_value(EntryRef::id(arena)).unwrap());
        let group = change("part_of", Value::Sequence(vec!["[[Harvest Cart]]".into()]));
        let text = fs::read_to_string(&path).unwrap();
        assert!(
            text.ends_with(
                "## Log\n\n- [[Session 1]], Day 2: Status alive → dead.\n- [[Session 1]], Day 2: In not set → [[The Arena]].\n- [[Session 1]], Day 2: Part of not set → [[Harvest Cart]].\n"
            ),
            "{text}"
        );
        // Round 26 FE3: a result is a sentence; only the Log keeps the arrow form.
        assert_eq!(status.word, "Status set to dead. It was alive.");
        assert_eq!(place.word, "Place set to The Arena.");
        assert_eq!(group.word, "Part of set to Harvest Cart.");
        assert_eq!(group.kind, ChangeKind::PartOf);

        // The session, the place and the group each show the line. The entry holds it.
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let changes = index.world_changes(&dir.path().join("Sessions/Session 1.md"));
        assert_eq!(changes.len(), 3);
        assert!(
            changes
                .iter()
                .all(|(source, _)| index.items[*source].entry.id == hob)
        );
        for (id, words) in [
            (arena, "In not set → The Arena."),
            (cart, "Part of not set → Harvest Cart."),
        ] {
            let about = index.lines_about(id);
            assert!(
                about.iter().any(|mention| {
                    index.items[mention.source].lines[mention.line.unwrap()]
                        .words()
                        .ends_with(words)
                }),
                "{words}"
            );
        }
        assert_eq!(index.members(cart).len(), 1);

        // F6: `Add to Log` writes the DM's words as one line with the stamp a change has at
        // that moment, and the session lists it. Its Undo takes back that line alone.
        let before_log = fs::read_to_string(&path).unwrap();
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let item = &index.items[index.position(hob).unwrap()];
        assert!(
            reader
                .add_log_line(&campaign, item, "Gave the gate key to Iselle.")
                .is_some()
        );
        let logged = fs::read_to_string(&path).unwrap();
        assert_eq!(
            logged,
            format!("{before_log}- [[Session 1]], Day 2: Gave the gate key to Iselle.\n")
        );
        let line = reader.undo.clone().unwrap();
        assert_eq!(line.word, "Added to Hob’s Log.");
        assert_eq!(line.kind.undo_label(), "Undo Log line");
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        assert_eq!(
            index
                .world_changes(&dir.path().join("Sessions/Session 1.md"))
                .len(),
            4
        );
        line.undo(&campaign).unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), before_log);

        // Undo takes back the field and its Log line together.
        group.undo(&campaign).unwrap();
        let text = fs::read_to_string(&path).unwrap();
        assert!(!text.contains("splat_part_of") && !text.contains("Harvest Cart"));
        assert!(text.ends_with("In not set → [[The Arena]].\n"));

        // Choosing the value that is already there writes nothing.
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let item = &index.items[index.position(hob).unwrap()];
        let same_place = serde_yaml_ng::to_value(EntryRef::id(arena)).unwrap();
        assert!(
            reader
                .change(&campaign, item, &index, "in", same_place)
                .is_none()
        );
        assert!(
            reader
                .change(&campaign, item, &index, "status", "dead".into())
                .is_none()
        );
        assert_eq!(fs::read_to_string(&path).unwrap(), text);

        // With no session, a change writes no Log line.
        fs::write(&path, &original).unwrap();
        reader.played = None;
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let item = &index.items[index.position(hob).unwrap()];
        reader.change(&campaign, item, &index, "status", "dead".into());
        assert!(!fs::read_to_string(&path).unwrap().contains("## Log"));
        // With no session, `Add to Log` still writes the line the DM asked for, with the day
        // alone; with no day either, the words alone.
        for (day, line) in [(Some(2), "- Day 2: Rode north."), (None, "- Rode north.")] {
            reader.day = day;
            let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
            let item = &index.items[index.position(hob).unwrap()];
            let before = fs::read_to_string(&path).unwrap();
            reader.add_log_line(&campaign, item, "Rode north.");
            let after = fs::read_to_string(&path).unwrap();
            assert!(after.ends_with(&format!("## Log\n\n{line}\n")), "{after}");
            fs::write(&path, &before).unwrap();
        }
    }

    /// Section 4.1 of the session-cycle design for `Add to Log`'s write, and its Undo.
    #[test]
    fn a_log_line_follows_the_disk_and_a_failed_write_keeps_the_words() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let campaign = sample_campaign(dir.path());
        let hob = create_entry(&campaign, "Hob", "npc").unwrap();
        let oren = create_entry(&campaign, "Oren", "npc").unwrap();
        let path = dir.path().join("NPCs/Hob.md");
        let reader = || Reader {
            played: Some("[[Session 1]]".into()),
            ..Default::default()
        };
        let mut lookup = reader();
        let key = (campaign.id, hob);
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let item = &index.items[index.position(hob).unwrap()];
        lookup.open_log(campaign.id, hob, "Hob", "the ropes");
        lookup.log_fields.get_mut(&key).unwrap().field.words = "Told who burned the ropes.".into();

        // An edit in Obsidian a moment before, while the field holds typed words: the line
        // goes after it, in the entry's new Log heading.
        let edited = format!("{}Seen at the ford.\n", fs::read_to_string(&path).unwrap());
        fs::write(&path, &edited).unwrap();
        assert!(
            lookup
                .add_log_line(&campaign, item, "Told who burned the ropes.")
                .is_some()
        );
        let a = fs::read_to_string(&path).unwrap();
        assert_eq!(
            a,
            format!("{edited}\n## Log\n\n- [[Session 1]]: Told who burned the ropes.\n")
        );
        assert!(
            !lookup.log_fields.contains_key(&key),
            "Enter closes the field"
        );

        // Act A in Lookup, an unrelated act B in the World page's reader, then Undo of A: A's
        // line goes, B's stays.
        let mut world = reader();
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let other = &index.items[index.position(oren).unwrap()];
        let oren_path = dir.path().join("NPCs/Oren.md");
        let oren_before = fs::read_to_string(&oren_path).unwrap();
        world
            .add_log_line(&campaign, other, "Paid the toll.")
            .unwrap();
        let oren_after = fs::read_to_string(&oren_path).unwrap();
        assert_ne!(oren_before, oren_after);
        assert!(lookup.undo_change(&campaign, hob).is_some());
        assert_eq!(fs::read_to_string(&path).unwrap(), edited);
        assert_eq!(fs::read_to_string(&oren_path).unwrap(), oren_after);

        // A failed write names the file, writes nothing, and keeps the field with its words.
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let item = &index.items[index.position(hob).unwrap()];
        lookup.open_log(campaign.id, hob, "Hob", "");
        lookup.log_fields.get_mut(&key).unwrap().field.words = "Kept.".into();
        let folder = dir.path().join("NPCs");
        fs::set_permissions(&folder, fs::Permissions::from_mode(0o555)).unwrap();
        let failed = lookup.add_log_line(&campaign, item, "Kept.");
        fs::set_permissions(&folder, fs::Permissions::from_mode(0o755)).unwrap();
        assert!(failed.is_none());
        assert!(
            lookup
                .error
                .as_deref()
                .is_some_and(|error| error.contains("NPCs/Hob.md"))
        );
        assert_eq!(fs::read_to_string(&path).unwrap(), edited);
        assert_eq!(lookup.log_fields[&key].field.words, "Kept.");

        // A note that cannot be read, and a note renamed or deleted under the open field: the
        // act names the file and writes nothing.
        fs::write(&path, [0xff, 0xfe, b'\n']).unwrap();
        assert!(lookup.add_log_line(&campaign, item, "Kept.").is_none());
        assert!(
            lookup
                .error
                .as_deref()
                .is_some_and(|error| error.contains("NPCs/Hob.md"))
        );
        assert_eq!(fs::read(&path).unwrap(), [0xff, 0xfe, b'\n']);
        fs::remove_file(&path).unwrap();
        assert!(lookup.add_log_line(&campaign, item, "Kept.").is_none());
        assert!(
            lookup
                .error
                .as_deref()
                .is_some_and(|error| error.contains("NPCs/Hob.md"))
        );
        assert!(!path.exists());
        assert_eq!(lookup.log_fields[&key].field.words, "Kept.");

        // A done scene's words fill only an empty field: words the DM typed stay, and nothing
        // selects them. Clearing Lookup's search keeps them too.
        lookup.open_log(campaign.id, hob, "Hob", "the ropes");
        assert_eq!(lookup.log_fields[&key].field.words, "Kept.");
        lookup.clear();
        assert_eq!(lookup.log_fields[&key].field.words, "Kept.");
        // The app gives the reader the session being played again on each frame.
        lookup.played = Some("[[Session 1]]".into());
        // Each field keeps its own request to select its words.
        lookup.open_log(campaign.id, oren, "Oren", "paid the toll");
        assert!(!lookup.log_fields[&key].select);
        assert!(lookup.log_fields[&(campaign.id, oren)].select);

        // Typed Log words are unsaved work: the campaign switch's guard and the close warning
        // count them, by the entry's name. An empty field holds nothing to lose.
        assert!(lookup.has_unsaved_edits());
        assert!(
            lookup
                .unsaved_edits()
                .iter()
                .any(|(id, name)| *id == hob && name == "Hob")
        );
        let mut empty = reader();
        empty.open_log(campaign.id, oren, "Oren", "");
        assert!(!empty.has_unsaved_edits() && empty.unsaved_edits().is_empty());

        // A copied note in another campaign has the same ID. Its reader shows no field with
        // this campaign's words, so Enter there cannot write them into the copy.
        let other_dir = tempfile::tempdir().unwrap();
        let other = sample_campaign(other_dir.path());
        fs::create_dir_all(other_dir.path().join("NPCs")).unwrap();
        fs::write(other_dir.path().join("NPCs/Hob.md"), &edited).unwrap();
        let index = ContentIndex::load(&[], Some(&other)).unwrap();
        let copy = &index.items[index.position(hob).unwrap()];
        let ctx = egui::Context::default();
        crate::design::theme::install(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                lookup.read(ui, copy, &index, Some(&other));
            });
        });
        let nodes = output.platform_output.accesskit_update.unwrap().nodes;
        assert!(
            nodes
                .iter()
                .any(|(_, node)| node.label() == Some("Edit Hob")),
            "the copy's reader drew its header"
        );
        assert!(
            !nodes.iter().any(|(_, node)| {
                node.label() == Some("Log line for Hob")
                    || node.value().is_some_and(|value| value.contains("Kept."))
            }),
            "another campaign's reader shows this campaign's typed words"
        );
        assert!(!lookup.log_fields.contains_key(&(other.id, hob)));

        // Every draft that the guards count has a field in view. A renamed played session
        // does not hide typed words: the stamp is the session at Enter, not the field's key.
        fs::write(&path, &edited).unwrap();
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let item = &index.items[index.position(hob).unwrap()];
        lookup.played = Some("[[Renamed Session]]".into());
        assert!(lookup.has_unsaved_edits());
        let output = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                lookup.read(ui, item, &index, Some(&campaign));
            });
        });
        let nodes = output.platform_output.accesskit_update.unwrap().nodes;
        assert!(
            nodes
                .iter()
                .any(|(_, node)| node.label() == Some("Log line for Hob")),
            "the guarded words have a field after the session is renamed"
        );

        // An entry that is gone: its draft waits for no guard, and the pane's footer says the
        // words, so nothing is lost without a word.
        let hob_text = fs::read_to_string(&path).unwrap();
        fs::remove_file(&path).unwrap();
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        assert!(index.position(hob).is_none());
        lookup.forget_gone_log_lines(&index);
        assert!(lookup.unsaved_edits().iter().all(|(id, _)| *id != hob));
        let item = &index.items[index.position(oren).unwrap()];
        let notices: Vec<_> = lookup
            .footer_lines(item, &index, Some(&campaign), None)
            .into_iter()
            .map(|line| line.words)
            .collect();
        assert!(
            notices.contains(
                &"\u{201c}Hob\u{201d} was deleted. The Log line was not added: \u{201c}Kept.\u{201d}"
                    .to_owned()
            ),
            "{notices:?}"
        );
        // X28 of app round 2: the notice stays until the DM copies or dismisses it. Lookup's
        // Clear, and another entry shown after it, keep it; its choice ends it.
        lookup.clear();
        let notices = |lookup: &mut Reader| {
            lookup
                .footer_lines(item, &index, Some(&campaign), None)
                .iter()
                .filter(|line| matches!(line.kind, FooterKind::GoneLog(_)))
                .count()
        };
        assert_eq!(notices(&mut lookup), 1);
        assert_eq!(notices(&mut lookup), 1);
        // The entry comes back with its ID, the DM types again, and it goes again: both lost
        // drafts keep their notice (Astra's review of PR 287).
        fs::write(&path, &hob_text).unwrap();
        let again = ContentIndex::load(&[], Some(&campaign)).unwrap();
        lookup.forget_gone_log_lines(&again);
        lookup.open_log(campaign.id, hob, "Hob", "");
        // The words keep every byte the DM typed, for Copy text (review round 1 of PR 287).
        let again = "  \"Again.\"\nsecond line\n  ";
        lookup.log_fields.get_mut(&key).unwrap().field.words = again.into();
        fs::remove_file(&path).unwrap();
        lookup.forget_gone_log_lines(&ContentIndex::load(&[], Some(&campaign)).unwrap());
        assert_eq!(notices(&mut lookup), 2);
        let gone: Vec<_> = lookup
            .footer_lines(item, &index, Some(&campaign), None)
            .into_iter()
            .filter_map(|line| match line.kind {
                FooterKind::GoneLog(gone) => Some(gone),
                _ => None,
            })
            .collect();
        assert!(gone.iter().any(|gone| gone.words == again), "{gone:?}");
        // A notice belongs to its campaign: another campaign's footer does not show it.
        let other = Campaign {
            id: Uuid::new_v4(),
            ..campaign.clone()
        };
        assert!(
            lookup
                .footer_lines(item, &index, Some(&other), None)
                .iter()
                .all(|line| !matches!(line.kind, FooterKind::GoneLog(_)))
        );
        let first = gone.iter().find(|gone| gone.words == "Kept.").unwrap().id;
        lookup.footer_act(&egui::Context::default(), FooterAct::GoneLog(first), item, &campaign);
        assert_eq!(notices(&mut lookup), 1);
    }

    #[test]
    fn title_line_actions_keep_their_words_beside_or_under_the_title() {
        // Edit, Start fight and Add to Log with their words (round 27 FC4, FJ3, FM5, FF5).
        let widths = [60.0, 100.0, 108.0];
        let title = 150.0;
        let all = 60.0 + 100.0 + 108.0 + 2.0 * design::XS;
        let fits = title + design::L - design::S + all;
        // All of them fit after the title: they share its line.
        assert_eq!(title_line_rows(fits, fits, title, &widths), None);
        // One pixel short: all of them move together under the title, none goes away, in one
        // row while the line holds them.
        assert_eq!(
            title_line_rows(fits - 1.0, fits - 1.0, title, &widths),
            Some(vec![0, 0, 0])
        );
        // A line too narrow for all three takes as many as fit, in order, on more rows.
        let two = 60.0 + design::XS + 100.0;
        assert_eq!(
            title_line_rows(two, two, title, &widths),
            Some(vec![0, 0, 1])
        );
        assert_eq!(
            title_line_rows(two - 1.0, two - 1.0, title, &widths),
            Some(vec![0, 1, 2])
        );
        // A long title alone sends them under it, with room to spare under the title.
        assert_eq!(
            title_line_rows(600.0, 600.0, 600.0, &widths),
            Some(vec![0, 0, 0])
        );
        // No actions: nothing to place.
        assert_eq!(title_line_rows(10.0, 10.0, 600.0, &[]), None);
    }

    #[test]
    fn groups_save_as_a_list_and_an_empty_list_leaves_no_property() {
        let before = "---\nsplat_schema: 2\nsplat_id: 00000000-0000-0000-0000-000000000001\nsplat_kind: world\nsplat_name: Zarel\nsplat_type: npc\n---\n\n# Zarel\n\nKeep this.\n";
        let groups = Value::Sequence(vec!["[[Arena Guild]]".into(), "[[Verori]]".into()]);
        let joined = fields::patch(before, "part_of", groups).unwrap();
        let entry = Entry::parse("Zarel.md", &joined).unwrap();
        let names: Vec<_> = part_of(&entry)
            .into_iter()
            .map(|group| link_name(group, "Group"))
            .collect();
        assert_eq!(names, ["Arena Guild", "Verori"]);
        assert!(joined.ends_with("# Zarel\n\nKeep this.\n"));
        let left = fields::patch(&joined, "part_of", Value::Sequence(Vec::new())).unwrap();
        assert_eq!(left, before);
    }

    #[test]
    fn dates_count_back_from_day_one_and_say_a_span_once() {
        assert_eq!(ago("About 2 years ago"), Some(2 * 12 * 30 * 24 * 60));
        assert_eq!(ago("a year ago"), ago("1 year ago"));
        assert_eq!(ago("long ago"), None);
        assert_eq!(ago("2 years"), None);
        let event = |dates: &str| {
            Entry::parse(
                "Event.md",
                &format!("---\nsplat_schema: 2\nsplat_id: 00000000-0000-0000-0000-000000000001\nsplat_kind: world\nsplat_name: Event\nsplat_type: event\n{dates}\n---\n"),
            )
            .unwrap()
        };
        assert_eq!(
            date_words(&event(
                "splat_when: 70 years ago\nsplat_until: 45 years ago"
            ))
            .as_deref(),
            Some("70 to 45 years ago")
        );
        assert_eq!(
            date_words(&event("splat_when: 2 years ago\nsplat_until: 6 months ago")).as_deref(),
            Some("2 years ago to 6 months ago")
        );
        assert_eq!(date_words(&event("splat_status: done")), None);
        // Most entries of a type decide its folder. Of two equal folders, the first by name.
        let entries = [
            ("Places/A.md", "location"),
            ("Places/B.md", "location"),
            ("Old/C.md", "location"),
            ("Zed/D.md", "team"),
            ("Alpha/E.md", "team"),
            ("F.md", "lore"),
        ];
        let folder = |kind| common_folder(entries.iter().copied(), kind);
        assert_eq!(folder("location").as_deref(), Some("Places"));
        assert_eq!(folder("team").as_deref(), Some("Alpha"));
        assert_eq!(folder("lore").as_deref(), Some(""));
        assert_eq!(folder("npc"), None);
        assert_eq!(type_group("lore"), "Lore");
        assert_eq!(type_group("company"), "Companies");
    }

    #[test]
    fn obsidian_paths_encode_query_characters_spaces_and_unicode() {
        assert_eq!(
            obsidian_url(Path::new("/vault/A & B/#é?.md")),
            "obsidian://open?path=/vault/A%20%26%20B/%23%C3%A9%3F.md"
        );
    }

    #[test]
    fn making_an_npc_links_its_creature_and_keeps_that_creatures_file() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = sample_campaign(dir.path());
        let mut entry = note_entry("Creatures/Varra.md", "Varra", "A captain.".into());
        entry.kind = "creature".into();
        let id = make_npc_note(&campaign, &entry).unwrap();
        let text = fs::read_to_string(dir.path().join("NPCs/Varra.md")).unwrap();
        let npc = Entry::parse("NPCs/Varra.md", &text).unwrap();
        assert_eq!(npc.id, id);
        assert_eq!(creature(&npc), Some(Reference::Id(entry.id)));
        assert!(make_npc_note(&campaign, &entry).is_err());
        assert_eq!(
            fs::read_to_string(dir.path().join("NPCs/Varra.md")).unwrap(),
            text
        );
    }
}
