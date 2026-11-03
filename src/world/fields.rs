//! Small, lossless edits to one key in `splat_world`. Unsupported YAML stays untouched.
use serde_yaml_ng::Value;

pub(super) fn patch(text: &str, key: &str, value: Value) -> Result<String, String> {
    let fail = "Use a plain splat_world mapping in Obsidian before changing this value.";
    let newline = if text.contains("\r\n") { "\r\n" } else { "\n" };
    let mut offset = 0;
    let mut lines = Vec::new();
    for line in text.split_inclusive('\n') {
        lines.push((offset, line.trim_end_matches(['\r', '\n'])));
        offset += line.len();
    }
    let start = lines
        .iter()
        .position(|(_, line)| line.starts_with("splat_world:"))
        .ok_or(fail)?;
    let (at, line) = lines[start];
    let rest = &line["splat_world:".len()..];
    let serialized = serde_yaml_ng::to_string(&value).map_err(|e| e.to_string())?;
    // Reference values use flow syntax, so a single replacement cannot move other keys.
    let encoded = if let Some(id) = value.get("id").and_then(Value::as_str) {
        format!("{{id: {id}}}")
    } else {
        serialized.trim_end().to_owned()
    };
    if rest.trim_start().starts_with('{') {
        let begin = at + line.find('{').ok_or(fail)?;
        let end = at + line.rfind('}').ok_or(fail)?;
        let inner = &text[begin + 1..end];
        let mut quote = None;
        let mut depth = 0usize;
        let mut boundaries = vec![0];
        for (i, ch) in inner.char_indices() {
            if matches!(ch, '\'' | '"') {
                if quote == Some(ch) {
                    quote = None;
                } else if quote.is_none() {
                    quote = Some(ch);
                }
            }
            if quote.is_some() {
                continue;
            }
            match ch {
                '{' | '[' => depth += 1,
                '}' | ']' => depth = depth.saturating_sub(1),
                ',' if depth == 0 => boundaries.push(i + 1),
                _ => {}
            }
        }
        boundaries.push(inner.len() + 1);
        for span in boundaries.windows(2) {
            let a = span[0];
            let b = span[1] - 1;
            let member = &inner[a..b];
            if let Some(v) = member.trim_start().strip_prefix(&format!("{key}:")) {
                let value_start =
                    begin + 1 + a + member.len() - v.len() + v.len() - v.trim_start().len();
                let value_end = begin + 1 + b - (v.len() - v.trim_end().len());
                let mut output = text.to_owned();
                output.replace_range(value_start..value_end, &encoded);
                return validate(output, key, value);
            }
        }
        let separator = if inner.trim().is_empty() { "" } else { ", " };
        let mut output = text.to_owned();
        output.insert_str(end, &format!("{separator}{key}: {encoded}"));
        return validate(output, key, value);
    }
    if !rest.trim().is_empty() && !rest.trim_start().starts_with('#') {
        return Err(fail.into());
    }
    let stop = (start + 1..lines.len())
        .find(|&i| {
            let line = lines[i].1;
            !line.is_empty() && !line.starts_with([' ', '\t', '#'])
        })
        .unwrap_or(lines.len());
    let indent = (start + 1..stop)
        .find_map(|i| {
            let line = lines[i].1;
            (!line.trim().is_empty() && !line.trim_start().starts_with('#'))
                .then(|| line.len() - line.trim_start().len())
        })
        .unwrap_or(2);
    let wanted = format!("{}{key}:", " ".repeat(indent));
    let mut output = text.to_owned();
    if let Some(i) = (start + 1..stop).find(|&i| lines[i].1.starts_with(&wanted)) {
        let begin = lines[i].0;
        let mut end = begin + lines[i].1.len() + newline.len();
        for &(at, line) in &lines[i + 1..stop] {
            if line.trim().is_empty() || line.trim_start().starts_with('#') {
                break;
            }
            if line.len() - line.trim_start().len() <= indent {
                break;
            }
            end = at + line.len() + newline.len();
        }
        // Keep an inline comment on the edited key, too.
        let comment = lines[i]
            .1
            .split_once(" #")
            .map_or("", |(_, comment)| comment);
        let comment = if comment.is_empty() {
            String::new()
        } else {
            format!(" #{comment}")
        };
        output.replace_range(
            begin..end,
            &format!("{}{key}: {encoded}{comment}{newline}", " ".repeat(indent)),
        );
    } else {
        let at = lines.get(stop).map_or(text.len(), |line| line.0);
        output.insert_str(
            at,
            &format!("{}{key}: {encoded}{newline}", " ".repeat(indent)),
        );
    }
    validate(output, key, value)
}

fn validate(output: String, key: &str, value: Value) -> Result<String, String> {
    let parsed = splat::content::Entry::parse("note.md", &output)?;
    if parsed.fields.get("splat_world").and_then(|v| v.get(key)) != Some(&value) {
        return Err("This YAML needs an edit in Obsidian first.".into());
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn base(world: &str) -> String {
        format!(
            "\u{feff}---\nsplat_schema: 1\nsplat_id: 00000000-0000-0000-0000-000000000001\nsplat_kind: world\nsplat_name: Rook\n{world}\ncustom: 'keep me' # comment\n---\n\nBody  \n"
        )
    }
    #[test]
    fn edits_only_the_requested_key_in_block_and_flow_maps() {
        for world in [
            "splat_world:\n  type: npc # keep\n  in: {id: old}\n  aliases: [Rook, 'the watch']",
            "splat_world: {type: npc, in: {id: old}, aliases: [Rook, 'the watch']}",
        ] {
            let text = base(world);
            let value = serde_yaml_ng::from_str("{id: new}").unwrap();
            assert_eq!(
                patch(&text, "in", value).unwrap(),
                text.replace("{id: old}", "{id: new}")
            );
        }
    }
    #[test]
    fn adding_a_key_keeps_comments_body_and_crlf() {
        let text = base("splat_world:\n  type: npc\n  # keep this comment").replace('\n', "\r\n");
        let edited = patch(&text, "in", Value::Null).unwrap();
        assert_eq!(edited, text.replace("custom:", "  in: null\r\ncustom:"));
    }
}
