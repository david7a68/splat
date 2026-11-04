//! Timeline version 2. A run patches its Result line; DM edits patch owned lines.
use super::*;
use splat::content::{
    layout::{Flag, replace_value},
    list_formats::machine,
};
use std::ops::Range;

#[derive(Clone)]
struct Line {
    range: Range<usize>,
    text: String,
    number: usize,
}
struct Row {
    event: Event,
    head: Line,
    lines: Vec<Line>,
    missing_id: bool,
}
fn lines(text: &str) -> Vec<Line> {
    splat::content::layout::content_lines(text)
        .into_iter()
        .map(|line| Line {
            range: line.range,
            text: line.value,
            number: line.line,
        })
        .collect()
}
fn stable_id(line: &Line) -> String {
    use sha2::{Digest, Sha256};
    format!(
        "t-{:x}",
        Sha256::digest(format!("{}:{}", line.number, line.text).as_bytes())
    )[..12]
        .into()
}
fn event(kind: Kind, id: String, title: String) -> Event {
    Event {
        id,
        kind,
        title,
        duration: String::new(),
        place: String::new(),
        notes: String::new(),
        result: String::new(),
        session: None,
        run: None,
        marker: String::new(),
        restores: false,
        continued_in: None,
        played_order: 0,
        extra: Default::default(),
    }
}
fn kind(raw: &str) -> Option<Kind> {
    match raw.trim().to_lowercase().as_str() {
        "fight" | "encounter" => Some(Kind::Encounter),
        "short rest" => Some(Kind::ShortRest),
        "long rest" => Some(Kind::LongRest),
        "travel" => Some(Kind::Travel),
        "major event" | "event" => Some(Kind::Event),
        _ => None,
    }
}
fn meta(event: &mut Event, text: &str) {
    for key in ["session", "run", "continued_in"] {
        if key == "continued_in" && machine(text, key).as_deref() == Some("none") {
            event.continued_in = None;
        }
        if let Some(id) = machine(text, key).and_then(|raw| Uuid::parse_str(&raw).ok()) {
            match key {
                "session" => event.session = Some(id),
                "run" => event.run = Some(id),
                _ => event.continued_in = Some(id),
            }
        }
    }
    if let Some(order) = machine(text, "order").and_then(|raw| raw.parse().ok()) {
        event.played_order = order;
    }
    if let Some(encoded) = machine(text, "data") {
        use base64::Engine;
        if let Ok(bytes) = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(encoded)
            && let Ok(extra) = serde_yaml_ng::from_slice::<serde_yaml_ng::Mapping>(&bytes)
        {
            for (key, value) in extra {
                event.extra.entry(key).or_insert(value);
            }
        }
    }
    if let Some(encoded) = machine(text, "marker") {
        use base64::Engine;
        if let Ok(bytes) = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(encoded)
            && let Ok(marker) = String::from_utf8(bytes)
        {
            event.marker = marker;
        }
    }
}
fn rows(text: &str) -> (Vec<Row>, Vec<Flag>) {
    let mut rows: Vec<Row> = Vec::new();
    let mut flags = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    let mut active = false;
    let mut child: Option<String> = None;
    for line in lines(text) {
        let raw = &line.text;
        let parsed = if let Some(day) = raw.strip_prefix("## Day ") {
            if day
                .split(':')
                .next()
                .is_some_and(|n| n.trim().parse::<u32>().is_ok())
            {
                let title = day
                    .split_once(':')
                    .map_or("", |(_, title)| title.trim())
                    .to_owned();
                Some((Kind::Day, title, None))
            } else {
                None
            }
        } else if let Some(item) = raw.strip_prefix("- ") {
            let (item, id) = item
                .rsplit_once(" ^")
                .map_or((item, None), |(item, id)| (item, Some(id.to_owned())));
            let (label, title) = item.split_once(':').unwrap_or((item, ""));
            if let Some(kind) = kind(label) {
                Some((kind, title.trim().trim_matches('*').to_owned(), id))
            } else {
                flags.push(Flag {
                    line: line.number,
                    message: "Timeline item not read. Start it with Fight, Travel, Short rest, Long rest or Major event.".into(),
                });
                None
            }
        } else {
            None
        };
        if let Some((kind, title, id)) = parsed {
            let missing_id = id.is_none();
            let id = id.unwrap_or_else(|| stable_id(&line));
            rows.push(Row {
                event: event(kind, id, title),
                head: line.clone(),
                lines: Vec::new(),
                missing_id,
            });
            active = true;
            child = None;
            continue;
        }
        if raw.starts_with('#') || (!raw.is_empty() && !raw.starts_with([' ', '\t', '%'])) {
            active = false;
        }
        if !active {
            continue;
        }
        let Some(row) = rows.last_mut() else {
            continue;
        };
        if raw.trim_start().starts_with("%% splat ") {
            if row.event.kind == Kind::Day
                && let Some(id) = machine(raw, "id")
            {
                row.event.id = id;
                row.missing_id = false;
            }
            meta(&mut row.event, raw);
            row.lines.push(line);
            continue;
        }
        if let Some(item) = raw.trim_start().strip_prefix("- ")
            && let Some((label, value)) = item.split_once(':')
        {
            let value = value.trim();
            let value = value.split("%% splat ").next().unwrap_or(value).trim();
            let target = match label.to_lowercase().as_str() {
                "place" => Some(&mut row.event.place),
                "duration" => Some(&mut row.event.duration),
                "result" => Some(&mut row.event.result),
                "notes" => Some(&mut row.event.notes),
                _ => None,
            };
            if let Some(target) = target {
                *target = value.into();
                child = Some(label.to_lowercase());
            } else if label.eq_ignore_ascii_case("Restores") {
                match value.to_lowercase().as_str() {
                    "yes" => row.event.restores = true,
                    "no" => row.event.restores = false,
                    _ => flags.push(Flag {
                        line: line.number,
                        message: "Restores: use yes or no".into(),
                    }),
                }
                child = None;
            } else if label.eq_ignore_ascii_case("Session")
                || label.eq_ignore_ascii_case("Continued in")
            {
                child = None;
            } else {
                row.event.extra.insert(
                    label.into(),
                    serde_yaml_ng::from_str(value).unwrap_or_else(|_| value.into()),
                );
                child = None;
            }
            meta(&mut row.event, raw);
            row.lines.push(line);
            continue;
        }
        if raw.starts_with("    ")
            && let Some(label) = &child
        {
            let target = match label.as_str() {
                "place" => &mut row.event.place,
                "duration" => &mut row.event.duration,
                "result" => &mut row.event.result,
                _ => &mut row.event.notes,
            };
            target.push('\n');
            target.push_str(raw.trim());
            row.lines.push(line);
        }
    }
    for row in &mut rows {
        if let Some(line) = row
            .lines
            .iter()
            .find(|l| l.text.trim_start().to_lowercase().starts_with("- result:"))
        {
            meta(&mut row.event, &line.text);
        }
    }
    let events: Vec<_> = rows.iter().map(|row| row.event.clone()).collect();
    for (row, expected) in rows.iter().zip(day_numbers(&events)) {
        if row.event.kind == Kind::Day {
            let typed = row
                .head
                .text
                .strip_prefix("## Day ")
                .and_then(|day| day.split(':').next())
                .and_then(|n| n.trim().parse::<u32>().ok());
            if let Some(typed) = typed.filter(|typed| *typed != expected) {
                flags.push(Flag {
                    line: row.head.number,
                    message: format!("\"Day {typed}\" is day {expected} in order. Splat renumbers it when it saves."),
                });
            }
        }
    }
    rows.retain(|row| {
        if !seen.insert(row.event.id.clone()) {
            flags.push(Flag {
                line: row.head.number,
                message: "Duplicate timeline id".into(),
            });
            false
        } else {
            true
        }
    });
    (rows, flags)
}
pub fn flagged_days(text: &str) -> Vec<String> {
    let (rows, flags) = rows(text);
    rows.into_iter()
        .filter(|row| {
            row.event.kind == Kind::Day && flags.iter().any(|flag| flag.line == row.head.number)
        })
        .map(|row| row.event.id)
        .collect()
}
pub fn parse(text: &str) -> (Vec<Event>, Vec<Flag>) {
    let (rows, flags) = rows(text);
    (rows.into_iter().map(|r| r.event).collect(), flags)
}
pub fn parse_with_sessions(
    text: &str,
    sessions: &BTreeMap<Uuid, String>,
) -> (Vec<Event>, Vec<Flag>) {
    let (mut rows, mut flags) = rows(text);
    for row in &mut rows {
        for line in &row.lines {
            let Some((label, value)) = line
                .text
                .trim_start()
                .strip_prefix("- ")
                .and_then(|s| s.split_once(':'))
            else {
                continue;
            };
            if !label.eq_ignore_ascii_case("Session") {
                continue;
            }
            let raw = value.trim().trim_start_matches("[[").trim_end_matches("]]");
            let target = raw.split('|').next().unwrap_or(raw).trim_end_matches(".md");
            let matches: Vec<_> = sessions
                .iter()
                .filter(|(_, name)| name.eq_ignore_ascii_case(target))
                .collect();
            if matches.len() == 1 {
                row.event.session = Some(*matches[0].0);
            } else {
                flags.push(Flag {
                    line: line.number,
                    message: "Session link not found. The saved session id is used.".into(),
                });
            }
        }
    }
    (rows.into_iter().map(|r| r.event).collect(), flags)
}
fn clean(text: &str) -> String {
    text.replace(['\r', '\n'], " ")
}
fn machine_line(event: &Event) -> String {
    use base64::Engine;
    let mut terms = Vec::new();
    if event.kind == Kind::Day {
        terms.push(format!("id={}", event.id));
    }
    for (key, id) in [
        ("session", event.session),
        ("run", event.run),
        ("continued_in", event.continued_in),
    ] {
        if let Some(id) = id {
            terms.push(format!("{key}={id}"));
        }
    }
    if event.played_order > 0 {
        terms.push(format!("order={}", event.played_order));
    }
    if !event.marker.is_empty() {
        terms.push(format!(
            "marker={}",
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(event.marker.as_bytes())
        ));
    }
    if terms.is_empty() {
        String::new()
    } else {
        format!(
            "{}%% splat {} %%\n",
            if event.kind == Kind::Day { "" } else { "  " },
            terms.join(" ")
        )
    }
}
fn head(event: &Event, day: u32) -> String {
    if event.kind == Kind::Day {
        format!("## {}\n", clean(&event.label(day)))
    } else {
        format!(
            "- {}{} ^{}\n",
            event.kind.word(),
            if event.title.is_empty() {
                String::new()
            } else {
                format!(": **{}**", clean(&event.title))
            },
            event.id
        )
    }
}
fn result(event: &Event) -> String {
    let mut terms = Vec::new();
    if event.played_order > 0 {
        terms.push(format!("order={}", event.played_order));
    }
    if let Some(id) = event.continued_in {
        terms.push(format!("continued_in={id}"));
    } else if event.run.is_some() {
        terms.push("continued_in=none".into());
    }
    if let Some(id) = event.run {
        terms.push(format!("run={id}"));
    }
    let suffix = if terms.is_empty() {
        String::new()
    } else {
        format!(" %% splat {} %%", terms.join(" "))
    };
    format!("  - Result: {}{suffix}\n", clean(&event.result))
}
fn event_text(event: &Event, day: u32, sessions: &BTreeMap<Uuid, String>) -> String {
    let mut out = head(event, day);
    for (label, value) in [
        ("Place", &event.place),
        ("Duration", &event.duration),
        ("Notes", &event.notes),
    ] {
        if !value.is_empty() {
            out.push_str(&format!("  - {label}: {}\n", value.replace('\n', "\n    ")));
        }
    }
    if !event.result.is_empty() {
        out.push_str(&result(event));
    }
    if event.restores {
        out.push_str("  - Restores: yes\n");
    }
    if let Some(name) = event.session.and_then(|id| sessions.get(&id)) {
        out.push_str(&format!("  - Session: [[{}]]\n", clean(name)));
    }
    for (key, value) in &event.extra {
        if let Some(key) = key.as_str() {
            let yaml = serde_yaml_ng::to_string(value).unwrap_or_default();
            let value = if yaml.trim_end().contains('\n') {
                serde_json::to_string(value).unwrap_or_default()
            } else {
                yaml.trim_end().to_owned()
            };
            out.push_str(&format!("  - {key}: {value}\n"));
        }
    }
    out.push_str(&machine_line(event));
    out
}
pub fn render(events: &[Event], sessions: &BTreeMap<Uuid, String>) -> String {
    let mut out="---\ntimeline_version: 2\n---\n\n# Timeline\n\nEdit this list in Obsidian or in Splat.\n\n".to_owned();
    for (event, day) in events.iter().zip(day_numbers(events)) {
        out.push_str(&event_text(event, day, sessions));
        out.push('\n');
    }
    out
}
pub fn is_v2(text: &str) -> bool {
    crate::sessions::split_note(text)
        .and_then(|(front, _)| {
            serde_yaml_ng::from_str::<serde_yaml_ng::Mapping>(
                front
                    .trim_start_matches('\u{feff}')
                    .trim()
                    .trim_start_matches("---")
                    .trim_end_matches("---"),
            )
            .ok()
        })
        .and_then(|m| {
            m.get("timeline_version")
                .and_then(serde_yaml_ng::Value::as_u64)
        })
        .is_some_and(|n| n == 2)
}
pub fn convert(text: &str, sessions: &BTreeMap<Uuid, String>) -> Result<String, String> {
    if is_v2(text) {
        return Ok(text.into());
    }
    let events = super::parse(text)?;
    let mut after = crate::content_workspace::patch_fields(
        text,
        &[("timeline_version", Some(2.into())), ("timeline", None)],
    )?;
    let (front, body) = crate::sessions::split_note(&after).ok_or("Missing timeline properties")?;
    let mut noise = String::new();
    for line in body.lines() {
        if line == "# Timeline"
            || line.starts_with("Splat writes this page from the list above.")
            || events
                .iter()
                .any(|event| line.ends_with(&format!(" ^{}", event.id)))
        {
            continue;
        }
        if !line.trim().is_empty() {
            noise.push_str(line);
            noise.push('\n');
        }
    }
    let generated = render(&events, sessions);
    let generated = crate::sessions::body_of(&generated);
    after = format!("{front}{generated}");
    if !noise.is_empty() {
        after.push('\n');
        after.push_str(&noise);
    }
    if parse(&after).0 != events {
        return Err("Conversion changed timeline values".into());
    }
    Ok(after)
}
pub fn patch(
    text: &str,
    events: &[Event],
    sessions: &BTreeMap<Uuid, String>,
    run_only: bool,
) -> Result<String, String> {
    let (old, _) = rows(text);
    let newline = if text.contains("\r\n") { "\r\n" } else { "\n" };
    let mut edits: Vec<(Range<usize>, String)> = Vec::new();
    for row in &old {
        let Some((index, event)) = events
            .iter()
            .enumerate()
            .find(|(_, e)| e.id == row.event.id)
        else {
            edits.push((row.head.range.clone(), String::new()));
            for line in &row.lines {
                edits.push((line.range.clone(), String::new()));
            }
            continue;
        };
        if *event == row.event && !row.missing_id {
            continue;
        }
        let only_result = {
            let mut e = row.event.clone();
            e.result = event.result.clone();
            e.played_order = event.played_order;
            e.continued_in = event.continued_in;
            e.run = event.run;
            e == *event
        };
        if (run_only || only_result) && !row.missing_id {
            let value = result(event).replace('\n', newline);
            if let Some(line) = row
                .lines
                .iter()
                .find(|l| l.text.trim_start().to_lowercase().starts_with("- result:"))
            {
                edits.push((line.range.clone(), value));
            } else {
                edits.push((row.head.range.end..row.head.range.end, value));
            }
            continue;
        }
        if row.event.kind != event.kind || row.event.title != event.title || row.missing_id {
            edits.push((
                row.head.range.clone(),
                head(event, day_numbers(events)[index]).replace('\n', newline),
            ));
        }
        let mut addition = String::new();
        for (label, value, previous) in [
            ("Place", &event.place, &row.event.place),
            ("Duration", &event.duration, &row.event.duration),
            ("Notes", &event.notes, &row.event.notes),
            ("Result", &event.result, &row.event.result),
        ] {
            if value == previous
                && !(label == "Result"
                    && (row.event.run != event.run
                        || row.event.continued_in != event.continued_in
                        || row.event.played_order != event.played_order))
            {
                continue;
            }
            let owned: Vec<_> = row
                .lines
                .iter()
                .enumerate()
                .filter(|(_, l)| {
                    l.text
                        .trim_start()
                        .to_lowercase()
                        .starts_with(&format!("- {}:", label.to_lowercase()))
                })
                .collect();
            let replacement = format!(
                "  - {label}: {}{newline}",
                value.replace('\n', &format!("{newline}    "))
            );
            if let Some((at, line)) = owned.first() {
                let mut end = line.range.end;
                for continuation in row
                    .lines
                    .iter()
                    .skip(at + 1)
                    .take_while(|l| l.text.starts_with("    "))
                {
                    end = continuation.range.end;
                }
                edits.push((line.range.start..end, replacement));
            } else {
                addition.push_str(&replacement);
            }
        }
        if row.event.restores != event.restores {
            let replacement = format!(
                "  - Restores: {}{newline}",
                if event.restores { "yes" } else { "no" }
            );
            if let Some(line) = row.lines.iter().find(|l| {
                l.text
                    .trim_start()
                    .to_lowercase()
                    .starts_with("- restores:")
            }) {
                edits.push((line.range.clone(), replacement));
            } else {
                addition.push_str(&replacement);
            }
        }
        if row.event.session != event.session
            || row.event.run != event.run
            || row.event.continued_in != event.continued_in
            || row.event.played_order != event.played_order
            || row.event.marker != event.marker
            || row.event.extra != event.extra
            || row.missing_id
        {
            let replacement = machine_line(event).replace('\n', newline);
            if let Some(line) = row
                .lines
                .iter()
                .find(|l| l.text.trim_start().starts_with("%% splat "))
            {
                edits.push((line.range.clone(), replacement));
            } else {
                addition.push_str(&replacement);
            }
        }
        if row.event.session != event.session {
            let replacement = event
                .session
                .and_then(|id| sessions.get(&id))
                .map(|name| format!("  - Session: [[{}]]{newline}", clean(name)))
                .unwrap_or_default();
            if let Some(line) = row
                .lines
                .iter()
                .find(|l| l.text.trim_start().to_lowercase().starts_with("- session:"))
            {
                edits.push((line.range.clone(), replacement));
            } else {
                addition.push_str(&replacement);
            }
        }
        if !addition.is_empty() {
            edits.push((row.head.range.end..row.head.range.end, addition));
        }
    }
    for (index, event) in events
        .iter()
        .enumerate()
        .filter(|(_, e)| !old.iter().any(|r| r.event.id == e.id))
    {
        let at = events[..index]
            .iter()
            .rev()
            .find_map(|previous| old.iter().find(|r| r.event.id == previous.id))
            .map(|r| r.lines.last().map_or(r.head.range.end, |l| l.range.end))
            .unwrap_or(text.len());
        edits.push((
            at..at,
            format!(
                "{newline}{}",
                event_text(event, day_numbers(events)[index], sessions).replace('\n', newline)
            ),
        ));
    }
    edits.sort_by_key(|(range, _)| (range.start, range.end));
    let mut after = text.to_owned();
    for (range, value) in edits.into_iter().rev() {
        after.replace_range(range, &value);
    }
    let (current, _) = rows(&after);
    if current.len() == events.len()
        && current
            .iter()
            .all(|r| events.iter().any(|e| e.id == r.event.id))
        && current
            .iter()
            .map(|r| &r.event.id)
            .ne(events.iter().map(|e| &e.id))
    {
        let chunks: BTreeMap<_, _> = current
            .iter()
            .map(|r| {
                let end = r.lines.last().map_or(r.head.range.end, |l| l.range.end);
                (
                    r.event.id.clone(),
                    after[r.head.range.start..end].to_owned(),
                )
            })
            .collect();
        for (row, event) in current.iter().zip(events).rev() {
            let end = row.lines.last().map_or(row.head.range.end, |l| l.range.end);
            after.replace_range(row.head.range.start..end, &chunks[&event.id]);
        }
    }
    // Normalize headings after moving chunks, using the app's words and day numbers.
    let (current, _) = rows(&after);
    for (row, day) in current.iter().zip(day_numbers(events)).rev() {
        let heading = head(&row.event, day).replace('\n', newline);
        if after[row.head.range.clone()] != heading {
            after.replace_range(row.head.range.clone(), &heading);
        }
    }
    let parse_model = |raw: &str| -> Result<serde_yaml_ng::Value, String> {
        let mut model = serde_yaml_ng::Mapping::new();
        model.insert(
            "timeline".into(),
            serde_yaml_ng::to_value(parse_with_sessions(raw, sessions).0)
                .map_err(|e| e.to_string())?,
        );
        Ok(model.into())
    };
    let after = replace_value(
        text,
        0..text.len(),
        &after,
        &["timeline"],
        serde_yaml_ng::to_value(events).map_err(|e| e.to_string())?,
        parse_model,
    )?;
    Ok(after)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn i20_x24_app_and_file_kind_words_read_in_any_case_with_or_without_colons() {
        for (label, kind) in [
            ("Fight", Kind::Encounter),
            ("Encounter", Kind::Encounter),
            ("Major event", Kind::Event),
            ("Event", Kind::Event),
            ("Travel", Kind::Travel),
            ("Short rest", Kind::ShortRest),
            ("Long rest", Kind::LongRest),
        ] {
            for label in [label.to_owned(), label.to_uppercase(), label.to_lowercase()] {
                for suffix in [": **At the bridge** ^t-one", " ^t-one", ""] {
                    let text = format!("---\ntimeline_version: 2\n---\n\n- {label}{suffix}\n");
                    let (events, flags) = parse(&text);
                    assert!(flags.is_empty(), "{text}: {flags:?}");
                    assert_eq!(events.len(), 1, "{text}");
                    assert_eq!(events[0].kind, kind);
                    let after = patch(&text, &events, &BTreeMap::new(), false).unwrap();
                    assert!(after.contains(&format!("- {}", kind.word())), "{after}");
                    assert_eq!(parse(&after).0, events);
                }
            }
        }
        for item in ["Dance: Around the fire", "Dance"] {
            let (_, flags) = parse(&format!("- {item}\n"));
            assert_eq!(flags.len(), 1);
            assert_eq!(
                flags[0].message,
                "Timeline item not read. Start it with Fight, Travel, Short rest, Long rest or Major event."
            );
        }
        let text = render(
            &[
                event(Kind::Encounter, "t-fight".into(), "Bridge".into()),
                event(Kind::Event, "t-event".into(), "Dawn".into()),
                event(Kind::LongRest, "t-rest".into(), String::new()),
            ],
            &BTreeMap::new(),
        );
        assert!(text.contains("- Fight: **Bridge** ^t-fight"));
        assert!(text.contains("- Major event: **Dawn** ^t-event"));
        assert!(text.contains("- Long rest ^t-rest"));
    }

    #[test]
    fn i20_x23_every_write_renumbers_days_after_add_remove_move_and_outside_edits() {
        let sessions = BTreeMap::new();
        let days: Vec<_> = ["Start", "Bridge", "Storm"]
            .iter()
            .enumerate()
            .map(|(i, title)| event(Kind::Day, format!("t-{i}"), (*title).into()))
            .collect();
        let text = render(&days, &sessions);
        for (events, headings) in [
            (
                vec![days[0].clone(), days[2].clone()],
                vec!["## Day 1: Start", "## Day 2: Storm"],
            ),
            (
                vec![days[2].clone(), days[0].clone(), days[1].clone()],
                vec!["## Day 1: Storm", "## Day 2: Start", "## Day 3: Bridge"],
            ),
            (
                vec![
                    event(Kind::Day, "t-new".into(), "New".into()),
                    days[0].clone(),
                    days[1].clone(),
                    days[2].clone(),
                ],
                vec![
                    "## Day 1: New",
                    "## Day 2: Start",
                    "## Day 3: Bridge",
                    "## Day 4: Storm",
                ],
            ),
        ] {
            let after = patch(&text, &events, &sessions, false).unwrap();
            assert_eq!(
                after
                    .lines()
                    .filter(|line| line.starts_with("## Day "))
                    .collect::<Vec<_>>(),
                headings
            );
            assert!(parse(&after).1.is_empty());
        }
        let outside = text.replace("## Day 1:", "## Day 5:");
        let (events, flags) = parse(&outside);
        assert_eq!(
            flags[0].message,
            "\"Day 5\" is day 1 in order. Splat renumbers it when it saves."
        );
        let after = patch(&outside, &events, &sessions, true).unwrap();
        assert!(after.contains("## Day 1: Start"));
        assert!(parse(&after).1.is_empty());
    }

    #[test]
    fn fmt_f3_session_link_wins_and_invalid_link_uses_fallback() {
        let first = Uuid::new_v4();
        let second = Uuid::new_v4();
        let sessions = BTreeMap::from([(first, "Session 1".into()), (second, "Session 2".into())]);
        let text = format!(
            "---\ntimeline_version: 2\n---\n\n- Travel: **Road** ^t-road\n  - Session: [[Session 2]]\n  %% splat session={first} %%\n\nDM prose.\n"
        );
        let (events, flags) = parse_with_sessions(&text, &sessions);
        assert!(flags.is_empty());
        assert_eq!(events[0].session, Some(second));
        let edited = patch(&text, &events, &sessions, false).unwrap();
        assert!(edited.contains(&format!("session={second}")));
        assert!(edited.contains("DM prose."));
        let (events, flags) =
            parse_with_sessions(&text.replace("Session 2]]", "Missing]]"), &sessions);
        assert_eq!(events[0].session, Some(first));
        assert_eq!(flags.len(), 1);
    }
    #[test]
    fn fmt_f3_dm_move_clears_continuation_and_keeps_next_run_save_working() {
        let dir = tempfile::tempdir().unwrap();
        let first = Uuid::new_v4();
        let second = Uuid::new_v4();
        let run = Uuid::new_v4();
        let sessions = BTreeMap::from([(first, "Session 1".into()), (second, "Session 2".into())]);
        let mut e = event(Kind::Encounter, "t-fight".into(), "Bridge".into());
        e.session = Some(first);
        e.continued_in = Some(second);
        e.run = Some(run);
        e.result = "Round 2".into();
        e.played_order = 2;
        let path = dir.path().join(FILE);
        std::fs::write(&path, format!("{}\nDM prose.\n", render(&[e], &sessions))).unwrap();
        let mut book = Book {
            session_links: sessions.clone(),
            ..Default::default()
        };
        book.open(Some(dir.path()));
        book.run_open = true;
        book.events[0].session = Some(second);
        book.events[0].continued_in = None;
        book.touch();
        book.record_run(
            &RunFact {
                id: run,
                copied_from: None,
                name: "Bridge".into(),
                round: 3,
                state: RunState::Running,
                steps: 3,
            },
            Some(second),
            true,
        );
        book.save();
        assert!(book.error.is_none(), "{:?}", book.error);
        book.reload();
        assert_eq!(book.events[0].session, Some(second));
        assert_eq!(book.events[0].continued_in, None);
        assert_eq!(book.events[0].result, "Round 3");
        assert!(std::fs::read_to_string(path).unwrap().contains("DM prose."));
    }
    #[test]
    fn i19_s2_legacy_conflict_compares_in_disk_schema_and_converts_on_overwrite() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(FILE);
        let old = "---\ntags: [DM]\ntimeline_version: 1\ntimeline:\n- id: t-road\n  kind: travel\n  weather: rain\n---\n\nDM prose.\n";
        std::fs::write(&path, old).unwrap();
        let mut book = Book::default();
        book.open(Some(dir.path()));
        book.touch();
        book.save();
        assert!(book.conflict);
        let (draft, disk) = book.conflict_comparison().unwrap();
        assert_eq!(disk, old);
        assert!(!is_v2(&draft));
        assert!(draft.contains("tags: [DM]"));
        assert_eq!(
            super::super::parse(&draft).unwrap()[0].extra["weather"],
            serde_yaml_ng::Value::from("rain")
        );
        book.overwrite();
        let saved = std::fs::read_to_string(path).unwrap();
        assert!(is_v2(&saved));
        assert_eq!(saved, convert(&draft, &BTreeMap::new()).unwrap());
    }
    #[test]
    fn i19_x14_custom_fields_are_visible_and_deleting_one_keeps_it_deleted() {
        let old = "---\ntimeline_version: 1\ntimeline:\n- id: t-road\n  kind: travel\n  weather: rain\n  loot: {gold: 20}\n---\n";
        let converted = convert(old, &BTreeMap::new()).unwrap();
        assert!(converted.contains("  - weather: rain\n"));
        assert!(!converted.contains("data="));
        assert_eq!(super::super::parse(old).unwrap(), parse(&converted).0);
        let edited = converted
            .lines()
            .filter(|line| !line.starts_with("  - loot:"))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(!parse(&edited).0[0].extra.contains_key("loot"));
    }

    #[test]
    fn i19_x15_day_number_out_of_order_is_flagged_on_its_line() {
        let text = "---\ntimeline_version: 2\n---\n\n## Day 1: Start\n%% splat id=t-one %%\n\n## Day 5: Storm\n%% splat id=t-two %%\n";
        let (events, flags) = parse(text);
        assert_eq!(events.len(), 2);
        assert!(flags.iter().any(|flag| flag.line == 8
            && flag.message == "\"Day 5\" is day 2 in order. Splat renumbers it when it saves."));
    }

    #[test]
    fn i19_s2_one_outside_paragraph_is_one_line_difference() {
        let dir = tempfile::tempdir().unwrap();
        let old = super::super::render(
            &[event(Kind::Travel, "t-road".into(), "Road".into())],
            &BTreeMap::new(),
        )
        .unwrap();
        let disk = format!("{old}DM's paragraph.\n");
        std::fs::write(dir.path().join(FILE), &disk).unwrap();
        let mut book = Book::default();
        book.open(Some(dir.path()));
        let (draft, disk) = book.conflict_comparison().unwrap();
        assert_eq!(crate::design::comparison_changes(&draft, &disk).0, 1);
    }

    #[test]
    fn fmt_f3_repeated_run_save_changes_only_result_line() {
        let dir = tempfile::tempdir().unwrap();
        let run = Uuid::new_v4();
        let mut event = event(Kind::Encounter, "t-fight".into(), "Bridge".into());
        event.run = Some(run);
        event.result = "Round 1".into();
        let text = format!("{}\nDM prose.\n", render(&[event], &BTreeMap::new()));
        let path = dir.path().join(FILE);
        std::fs::write(&path, &text).unwrap();
        let mut book = Book::default();
        book.open(Some(dir.path()));
        for round in [2, 3] {
            let before = std::fs::read_to_string(&path).unwrap();
            book.record_run(
                &RunFact {
                    id: run,
                    copied_from: None,
                    name: "Bridge".into(),
                    round,
                    state: RunState::Running,
                    steps: round as usize,
                },
                None,
                true,
            );
            book.save();
            assert!(book.error.is_none(), "{:?}", book.error);
            let after = std::fs::read_to_string(&path).unwrap();
            let noise = |raw: &str| {
                raw.lines()
                    .filter(|l| !l.starts_with("  - Result:"))
                    .map(str::to_owned)
                    .collect::<Vec<_>>()
            };
            assert_eq!(noise(&before), noise(&after));
            assert_ne!(before, after);
        }
    }
    #[test]
    fn fmt_f3_timeline_golden_noise_and_result_only_run_write() {
        let old = "---\n# Keep this comment\ntags: [road]\ntimeline_version: 1\ntimeline:\n- id: t-day\n  kind: day\n  title: Begins\n- id: t-fight\n  kind: encounter\n  title: Bridge\n  result: Round 1\n  weather: rain\n---\n\n# Timeline\n\nA paragraph from the DM.\n";
        let new = "---\n# Keep this comment\ntags: [road]\ntimeline_version: 2\n---\n\n# Timeline\n\n## Day 1: Begins\n%% splat id=t-day %%\n\n- Encounter: **Bridge** ^t-fight\n  - Result: Round 1\n  - weather: rain\n\nA paragraph from the DM.\n";
        assert_eq!(super::super::parse(old).unwrap(), parse(new).0);
        let converted = convert(old, &BTreeMap::new()).unwrap();
        assert!(converted.contains("# Keep this comment"));
        assert!(converted.contains("tags: [road]"));
        assert!(converted.contains("A paragraph from the DM."));
        assert_eq!(super::super::parse(old).unwrap(), parse(&converted).0);
        assert_eq!(converted, convert(&converted, &BTreeMap::new()).unwrap());
        let mut events = parse(new).0;
        events[1].result = "Party won".into();
        let edited = patch(new, &events, &BTreeMap::new(), true).unwrap();
        assert_eq!(
            edited,
            new.replace("Round 1", "Party won")
                .replace("- Encounter:", "- Fight:")
        );
    }
    #[test]
    fn fmt_f3_timeline_bad_items_duplicates_and_missing_ids_survive() {
        let text = "---\ntimeline_version: 2\n---\n\n# Timeline\n\n- Travel: **Road** ^t-road\n  - Loot: 30 gp\n- Dance: Not read\n- Event: **Duplicate** ^t-road\n- Event: **New**\n\nKeep this prose.\n";
        let (events, flags) = parse(text);
        assert_eq!(events.len(), 2);
        assert_eq!(flags.len(), 2);
        let edited = patch(text, &events, &BTreeMap::new(), false).unwrap();
        assert!(edited.contains("- Dance: Not read"));
        assert!(edited.contains("- Event: **Duplicate** ^t-road"));
        assert!(edited.contains("Keep this prose."));
        assert!(edited.contains("- Loot: 30 gp"));
        assert!(edited.contains(&format!("- Major event: **New** ^{}", events[1].id)));
    }
    #[test]
    fn fmt_f3_run_does_not_convert_legacy_timeline() {
        let dir = tempfile::tempdir().unwrap();
        let text = super::super::render(&[], &BTreeMap::new()).unwrap();
        std::fs::write(dir.path().join(FILE), text).unwrap();
        let mut book = Book::default();
        book.open(Some(dir.path()));
        book.record_run(
            &RunFact {
                id: Uuid::new_v4(),
                copied_from: None,
                name: "Bridge".into(),
                round: 1,
                state: RunState::Running,
                steps: 1,
            },
            None,
            true,
        );
        book.save();
        let saved = std::fs::read_to_string(dir.path().join(FILE)).unwrap();
        assert!(saved.contains("timeline_version: 1"));
        assert_eq!(super::super::parse(&saved).unwrap().len(), 1);
    }
}
