//! Readable text from provider HTML. No HTML is executed or loaded.
pub fn description_text(input: &str) -> String {
    let mut out = String::new();
    let mut rest = input;
    while let Some(start) = rest.find('<') {
        out.push_str(&rest[..start]);
        let Some(end) = rest[start..].find('>') else {
            out.push_str(&rest[start..]);
            rest = "";
            break;
        };
        let tag = rest[start + 1..start + end]
            .split_whitespace()
            .next()
            .unwrap_or("")
            .trim_end_matches('/')
            .to_ascii_lowercase();
        match tag.as_str() {
            "p" | "/p" | "div" | "/div" | "h1" | "/h1" | "h2" | "/h2" | "h3" | "/h3" | "h4"
            | "/h4" => out.push_str("\n\n"),
            "br" | "tr" | "/tr" | "/li" => out.push('\n'),
            "li" => {
                if !out.ends_with('\n') {
                    out.push('\n');
                }
                out.push_str("• ");
            }
            "/td" | "/th" => out.push_str(" | "),
            _ => (),
        }
        rest = &rest[start + end + 1..];
    }
    out.push_str(rest);
    for (entity, value) in [
        ("&nbsp;", " "),
        ("&amp;", "&"),
        ("&quot;", "\""),
        ("&#39;", "'"),
        ("&rsquo;", "’"),
        ("&lsquo;", "‘"),
        ("&rdquo;", "”"),
        ("&ldquo;", "“"),
        ("&mdash;", "—"),
        ("&ndash;", "–"),
        ("&lt;", "<"),
        ("&gt;", ">"),
    ] {
        out = out.replace(entity, value);
    }
    let mut lines = Vec::new();
    for line in out.lines() {
        let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
        let line = line
            .trim_end_matches(" | ")
            .trim_end_matches(" |")
            .to_owned();
        if !line.is_empty()
            || lines
                .last()
                .is_some_and(|previous: &String| !previous.is_empty())
        {
            lines.push(line);
        }
    }
    lines.join("\n").trim().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn descriptions_keep_paragraphs_lists_and_inline_words() {
        assert_eq!(
            description_text(
                "<p>You <strong>can</strong> cast.</p><p>Choose &ldquo;one&rdquo; &ndash; then:</p><ul><li>Fire</li><li>Ice</li></ul>"
            ),
            "You can cast.\n\nChoose “one” – then:\n\n• Fire\n• Ice"
        );
        assert_eq!(description_text("<p>30<br/>feet</p>"), "30\nfeet");
    }
}
