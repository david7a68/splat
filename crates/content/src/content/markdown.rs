//! Explicit body data only. Ordinary Markdown and code examples are never data.
use super::{field, local_key};
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use serde_yaml_ng::{Mapping, Value};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
struct Node {
    value: Option<Value>,
    children: BTreeMap<String, Node>,
}
impl Node {
    fn ensure_record(&mut self, path: &[&str]) -> Result<(), String> {
        if self.value.is_some() {
            return Err("Table row overlaps a block or frontmatter value".into());
        }
        if let Some((key, tail)) = path.split_first() {
            self.children
                .entry((*key).into())
                .or_default()
                .ensure_record(tail)?;
        }
        Ok(())
    }
    fn insert(&mut self, path: &[&str], value: Value) -> Result<(), String> {
        if self.value.is_some() || (path.is_empty() && !self.children.is_empty()) {
            return Err("Duplicate or overlapping Markdown data field".into());
        }
        if let Some((key, tail)) = path.split_first() {
            self.children
                .entry((*key).into())
                .or_default()
                .insert(tail, value)
        } else {
            self.value = Some(value);
            Ok(())
        }
    }
    fn finish(self, path: &str, arrays: &BTreeSet<String>) -> Result<Value, String> {
        if let Some(value) = self.value {
            return Ok(value);
        }
        if arrays.contains(path) {
            let len = self.children.len();
            let mut children = self.children;
            let mut result = Vec::new();
            for i in 0..len {
                let key = i.to_string();
                let node = children
                    .remove(&key)
                    .ok_or("Row numbers must be contiguous from zero")?;
                result.push(node.finish(&format!("{path}/{key}"), arrays)?);
            }
            Ok(Value::Sequence(result))
        } else {
            let mut result = Mapping::new();
            for (key, child) in self.children {
                let subpath = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{path}/{key}")
                };
                result.insert(Value::String(key), child.finish(&subpath, arrays)?);
            }
            Ok(Value::Mapping(result))
        }
    }
}

#[derive(Clone)]
struct Marker {
    mode: String,
    path: String,
    mapping: Option<super::table_mapping::Table>,
}

fn marker(raw: &str) -> Result<Option<Marker>, String> {
    let raw = raw.trim();
    if !raw.starts_with("<!-- splat:")
        || raw.starts_with("<!-- splat:inventory")
        || raw.starts_with(super::ROLLS_MARKER)
    {
        return Ok(None);
    }
    if let Some(config) = raw
        .strip_prefix("<!-- splat:table\n")
        .and_then(|s| s.strip_suffix("-->"))
    {
        let mapping: super::table_mapping::Table = serde_yaml_ng::from_value(yaml(config)?)
            .map_err(|e| format!("Invalid table mapping: {e}"))?;
        mapping.validate()?;
        // Share the root and identity restrictions with existing data markers.
        marker(&format!("<!-- splat:rows path={} -->", mapping.path))?;
        return Ok(Some(Marker {
            mode: "table".into(),
            path: mapping.path.clone(),
            mapping: Some(mapping),
        }));
    }
    let (mode, path) = raw
        .strip_prefix("<!-- splat:")
        .and_then(|s| s.strip_suffix(" -->"))
        .and_then(|s| s.split_once(" path="))
        .ok_or_else(|| {
            format!(
                "Invalid Splat data marker: {}",
                raw.chars().take(100).collect::<String>()
            )
        })?;
    if !["values", "map", "rows", "abilities", "yaml"].contains(&mode) {
        return Err("Unknown Splat data marker".into());
    }
    let parts: Vec<_> = path.split('/').collect();
    if !parts[0].starts_with("splat_")
        || parts.iter().any(|p| {
            !local_key(p)
                && !p.strip_prefix('@').is_some_and(local_key)
                && !(p.parse::<usize>().is_ok_and(|i| i.to_string() == *p))
        })
    {
        return Err("Invalid Markdown data path".into());
    }
    if [
        "splat_schema",
        "splat_id",
        "splat_key",
        "splat_kind",
        "splat_name",
        "splat_origin",
        "splat_sources",
    ]
    .contains(&parts[0])
    {
        return Err("Identity and provenance belong in frontmatter".into());
    }
    Ok(Some(Marker {
        mode: mode.into(),
        path: path.into(),
        mapping: None,
    }))
}

fn yaml(raw: &str) -> Result<Value, String> {
    serde_yaml_ng::from_str(raw).map_err(|e| format!("Invalid Markdown data YAML: {e}"))
}

fn insert(root: &mut Node, path: &str, value: Value) -> Result<(), String> {
    root.insert(&path.split('/').collect::<Vec<_>>(), value)
        .map_err(|e| format!("{path}: {e}"))
}

fn add_table(
    root: &mut Node,
    marker: &Marker,
    headers: &[String],
    rows: &[Vec<String>],
    arrays: &mut BTreeSet<String>,
) -> Result<(), String> {
    if headers.iter().collect::<BTreeSet<_>>().len() != headers.len() {
        return Err("Duplicate data column".into());
    }
    if marker.mode == "abilities" {
        if marker.path != "splat_stats" || headers != ["Ability", "Score", "Save override"] {
            return Err(
                "Abilities table requires Ability, Score and Save override columns at splat_stats"
                    .into(),
            );
        }
        for row in rows {
            let key = yaml(&row[0])?
                .as_str()
                .ok_or("Ability must be a string")?
                .to_lowercase();
            if ![
                "strength",
                "dexterity",
                "constitution",
                "intelligence",
                "wisdom",
                "charisma",
            ]
            .contains(&key.as_str())
            {
                return Err("Unknown ability".into());
            }
            let score = yaml(&row[1])?;
            if score.as_u64().is_none() {
                return Err("Ability score must be a nonnegative integer".into());
            }
            insert(root, &format!("splat_stats/{key}/value"), score)?;
            if !row[2].is_empty() {
                let save = yaml(&row[2])?;
                if save.as_i64().is_none() {
                    return Err("Save override must be an integer".into());
                }
                insert(root, &format!("splat_stats/save_{key}/value"), save)?;
            }
        }
    } else if marker.mode == "values" {
        if headers != ["Field", "Value"] {
            return Err("Values table requires Field and Value columns".into());
        }
        for row in rows {
            let key = yaml(&row[0])?
                .as_str()
                .ok_or("Field must be a string")?
                .to_owned();
            if !local_key(&key) {
                return Err("Invalid field name".into());
            }
            if row[1].is_empty() {
                return Err("Data value cannot be blank; use null explicitly".into());
            }
            insert(root, &format!("{}/{}", marker.path, key), yaml(&row[1])?)?;
        }
    } else {
        let first = if marker.mode == "rows" { "Row" } else { "Key" };
        if headers.first().map(String::as_str) != Some(first)
            || headers.len() < 2
            || headers[1..].iter().any(|h| !local_key(h))
        {
            return Err(format!(
                "Data table requires {first} then schema field columns"
            ));
        }
        if marker.mode == "rows" && !arrays.insert(marker.path.clone()) {
            return Err("Duplicate rows table".into());
        }
        let mut seen = BTreeSet::new();
        for row in rows {
            let key = if marker.mode == "rows" {
                yaml(&row[0])?
                    .as_u64()
                    .ok_or("Row must be a nonnegative integer")?
                    .to_string()
            } else {
                let key = yaml(&row[0])?
                    .as_str()
                    .ok_or("Key must be a string")?
                    .to_owned();
                if !local_key(&key) {
                    return Err("Invalid row key".into());
                }
                key
            };
            if !seen.insert(key.clone()) {
                return Err("Duplicate data row".into());
            }
            let mut count = 0;
            for (column, raw) in headers[1..].iter().zip(&row[1..]) {
                if !raw.is_empty() {
                    insert(root, &format!("{}/{key}/{column}", marker.path), yaml(raw)?)?;
                    count += 1;
                }
            }
            if count == 0 {
                return Err("Empty data row".into());
            }
        }
        if marker.mode == "rows" && !(0..rows.len()).all(|i| seen.contains(&i.to_string())) {
            return Err("Row numbers must be contiguous from zero".into());
        }
    }
    if rows.is_empty() {
        return Err("Empty data table; use a YAML block for empty values".into());
    }
    Ok(())
}

fn add_mapped_table(
    root: &mut Node,
    table: &super::table_mapping::Table,
    headers: &[String],
    rows: &[Vec<String>],
    links: &[Vec<Option<String>>],
    arrays: &mut BTreeSet<String>,
    aliases: &mut BTreeMap<String, Vec<String>>,
) -> Result<(), String> {
    if headers.iter().collect::<BTreeSet<_>>().len() != headers.len()
        || headers.iter().collect::<BTreeSet<_>>() != table.columns.keys().collect()
    {
        return Err("Table columns differ from their mapping".into());
    }
    if !arrays.insert(table.path.clone())
        || aliases
            .insert(table.path.clone(), table.order.clone())
            .is_some()
    {
        return Err("Duplicate table target".into());
    }
    let identity = headers.iter().position(|h| h == &table.key).unwrap();
    let mut seen = BTreeSet::new();
    for (row, links) in rows.iter().zip(links) {
        let key_value =
            table.columns[&table.key].decode(&row[identity], links[identity].as_deref())?;
        let key = if table.columns[&table.key].kind == super::table_mapping::Kind::Reference {
            key_value
                .as_mapping()
                .and_then(|m| m.values().next())
                .and_then(Value::as_str)
                .ok_or("Invalid reference key")?
                .rsplit('/')
                .next()
                .unwrap()
                .to_owned()
        } else {
            key_value.as_str().ok_or("Invalid row key")?.to_owned()
        };
        if !local_key(&key) || !table.order.contains(&key) || !seen.insert(key.clone()) {
            return Err("Unknown or duplicate stable row key".into());
        }
        root.ensure_record(
            &format!("{}/@{key}", table.path)
                .split('/')
                .collect::<Vec<_>>(),
        )?;
        for ((header, raw), link) in headers.iter().zip(row).zip(links) {
            let column = &table.columns[header];
            if raw.is_empty() {
                if !column.optional {
                    return Err(format!("{key}/{header}: required cell is blank"));
                }
                continue;
            }
            if link.is_some()
                && ![
                    super::table_mapping::Kind::Reference,
                    super::table_mapping::Kind::Section,
                ]
                .contains(&column.kind)
            {
                return Err("Only reference and section columns accept links".into());
            }
            if column.kind == super::table_mapping::Kind::Section
                && link.as_deref() != Some(&format!("section:{key}"))
            {
                return Err("Section link must name its stable row key".into());
            }
            let value = column
                .decode(raw, link.as_deref())
                .map_err(|e| format!("{key}/{header}: {e}"))?;
            if let Some(path) = &column.path {
                if path.is_empty() {
                    for (field, value) in value
                        .as_mapping()
                        .ok_or("Compound column must be a mapping")?
                    {
                        insert(
                            root,
                            &format!(
                                "{}/@{key}/{}",
                                table.path,
                                field.as_str().ok_or("Invalid compound field")?
                            ),
                            value.clone(),
                        )?;
                    }
                } else {
                    insert(root, &format!("{}/@{key}/{path}", table.path), value)?;
                }
            }
        }
    }
    if seen.len() != table.order.len() {
        return Err("Table order names missing rows".into());
    }
    Ok(())
}

fn resolve_aliases(
    node: &mut Node,
    path: &str,
    aliases: &BTreeMap<String, Vec<String>>,
) -> Result<(), String> {
    let order = aliases.get(path);
    for (key, child) in &mut node.children {
        if key.starts_with('@') && order.is_none() {
            return Err("Stable row address has no mapped table".into());
        }
        let subpath = if path.is_empty() {
            key.clone()
        } else {
            format!("{path}/{key}")
        };
        resolve_aliases(child, &subpath, aliases)?;
    }
    if let Some(order) = order {
        if node.value.is_some() {
            return Err("Table overlaps a block assignment".into());
        }
        let mut children = BTreeMap::new();
        for (name, child) in std::mem::take(&mut node.children) {
            let key = name
                .strip_prefix('@')
                .ok_or("Mapped tables require stable @row addresses")?;
            let index = order
                .iter()
                .position(|k| k == key)
                .ok_or("Block names unknown table row")?;
            children.insert(index.to_string(), child);
        }
        node.children = children;
    }
    Ok(())
}

fn resolved_path(path: &str, aliases: &BTreeMap<String, Vec<String>>) -> Result<String, String> {
    let mut logical = String::new();
    let mut parts = Vec::new();
    for part in path.split('/') {
        let physical = if let Some(key) = part.strip_prefix('@') {
            aliases
                .get(&logical)
                .and_then(|order| order.iter().position(|k| k == key))
                .ok_or("Unknown stable row address")?
                .to_string()
        } else {
            part.to_owned()
        };
        parts.push(physical);
        if !logical.is_empty() {
            logical.push('/');
        }
        logical.push_str(part);
    }
    Ok(parts.join("/"))
}

fn check_table_width(raw: &str) -> Result<(), String> {
    let widths: Vec<_> = raw
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let line = line.trim();
            let mut separators = 0;
            let mut escaped = false;
            for c in line.chars() {
                if c == '|' && !escaped {
                    separators += 1;
                }
                escaped = c == '\\' && !escaped;
            }
            separators + 1 - usize::from(line.starts_with('|')) - usize::from(line.ends_with('|'))
        })
        .collect();
    if widths
        .first()
        .is_some_and(|width| widths.iter().any(|n| n != width))
    {
        return Err("Data table row has missing or extra cells".into());
    }
    Ok(())
}

fn check_fence(raw: &str) -> Result<(), String> {
    let lines: Vec<_> = raw.trim_end().lines().collect();
    let first = lines.first().ok_or("Missing YAML fence")?.trim_start();
    let delimiter = first.chars().next().ok_or("Missing YAML fence")?;
    let width = first.chars().take_while(|c| *c == delimiter).count();
    let last = lines
        .last()
        .ok_or("Missing closing YAML fence")?
        .trim()
        .trim_start_matches('>')
        .trim();
    if lines.len() < 2 || last.len() < width || !last.chars().all(|c| c == delimiter) {
        return Err("Unclosed marked YAML fence".into());
    }
    Ok(())
}

pub fn read(body: &str, fields: &mut Mapping) -> Result<(), String> {
    let mut root = Node::default();
    for (key, value) in std::mem::take(fields) {
        root.children.insert(
            key.as_str().ok_or("Field name must be a string")?.into(),
            Node {
                value: Some(value),
                ..Node::default()
            },
        );
    }
    let mut arrays = BTreeSet::new();
    let mut aliases = BTreeMap::new();
    let mut links = Vec::new();
    let mut cell_links = Vec::new();
    let mut cell_link: Option<String> = None;
    let mut pending: Option<Marker> = None;
    let mut table: Option<Marker> = None;
    let mut code: Option<Marker> = None;
    let mut code_text = String::new();
    let mut headers = Vec::new();
    let mut rows = Vec::new();
    let mut cells = Vec::new();
    let mut cell = String::new();
    let mut in_cell = false;
    let mut marked_blocks = BTreeSet::new();
    let mut html_block = false;
    for (event, range) in Parser::new_ext(body, Options::ENABLE_TABLES).into_offset_iter() {
        if code.is_some() {
            match event {
                Event::Text(text) => code_text.push_str(&text),
                Event::End(TagEnd::CodeBlock) => {
                    let active = code.take().unwrap();
                    insert(&mut root, &active.path, yaml(&code_text)?)?;
                }
                _ => (),
            }
            continue;
        }
        if table.is_some() {
            match event {
                Event::Start(Tag::TableHead | Tag::TableRow) => {
                    cells.clear();
                    cell_links.clear();
                }
                Event::Start(Tag::TableCell) => {
                    cell.clear();
                    cell_link = None;
                    in_cell = true;
                }
                Event::Start(Tag::Link { title, .. })
                    if in_cell && table.as_ref().is_some_and(|m| m.mapping.is_some()) =>
                {
                    if cell_link.is_some() || !cell.is_empty() || title.is_empty() {
                        return Err(
                            "Reference cell requires one link with an identity title".into()
                        );
                    }
                    cell_link = Some(title.to_string());
                }
                Event::Text(text) | Event::Code(text) if in_cell => cell.push_str(&text),
                Event::End(TagEnd::TableCell) => {
                    cells.push(cell.trim().to_owned());
                    cell_links.push(cell_link.clone());
                    in_cell = false;
                }
                Event::End(TagEnd::TableHead) => headers = cells.clone(),
                Event::End(TagEnd::TableRow) => {
                    rows.push(cells.clone());
                    links.push(cell_links.clone());
                }
                Event::End(TagEnd::Table) => {
                    let active = table.take().unwrap();
                    if let Some(mapping) = &active.mapping {
                        add_mapped_table(
                            &mut root,
                            mapping,
                            &headers,
                            &rows,
                            &links,
                            &mut arrays,
                            &mut aliases,
                        )?;
                    } else {
                        add_table(&mut root, &active, &headers, &rows, &mut arrays)?;
                    }
                }
                Event::Start(_)
                | Event::Html(_)
                | Event::InlineHtml(_)
                | Event::SoftBreak
                | Event::HardBreak
                    if in_cell =>
                {
                    return Err("Data cells must be plain text or inline code".into());
                }
                _ => (),
            }
            continue;
        }
        if html_block {
            if matches!(event, Event::End(TagEnd::HtmlBlock)) {
                html_block = false;
            }
            continue;
        }
        if pending.is_none() && matches!(event, Event::Start(Tag::HtmlBlock)) {
            html_block = true;
            if let Some(active) = marker(&body[range])? {
                if !marked_blocks.insert((active.mode.clone(), active.path.clone())) {
                    return Err("Duplicate Markdown data block".into());
                }
                pending = Some(active);
            }
            continue;
        }
        if let Some(active) = pending.take() {
            match event {
                Event::End(TagEnd::HtmlBlock) => pending = Some(active),
                Event::Start(Tag::Table(_)) if active.mode != "yaml" => {
                    check_table_width(&body[range])?;
                    table = Some(active);
                    headers.clear();
                    rows.clear();
                    links.clear();
                }
                Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref info)))
                    if active.mode == "yaml" && info.as_ref() == "yaml" =>
                {
                    check_fence(&body[range])?;
                    code = Some(active);
                    code_text.clear();
                }
                _ => {
                    return Err(
                        "Splat marker must immediately precede its table or YAML fence".into(),
                    );
                }
            }
            continue;
        }
        if let Event::Html(raw) | Event::InlineHtml(raw) = event
            && let Some(active) = marker(&raw)?
        {
            if !marked_blocks.insert((active.mode.clone(), active.path.clone())) {
                return Err("Duplicate Markdown data block".into());
            }
            pending = Some(active);
        }
    }
    if pending.is_some() || code.is_some() || table.is_some() {
        return Err("Incomplete marked Markdown data".into());
    }
    resolve_aliases(&mut root, "", &aliases)?;
    let arrays = arrays
        .iter()
        .map(|path| resolved_path(path, &aliases))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let Value::Mapping(resolved) = root.finish("", &arrays)? else {
        return Err("Invalid data root".into());
    };
    *fields = resolved;
    // A normal note does not need a package report to declare its adapter.
    if field(fields, "splat_requires_mechanics").is_none()
        && let Some(id) = field(fields, "splat_rule")
            .and_then(Value::as_mapping)
            .and_then(|r| field(r, "implementation"))
            .and_then(Value::as_str)
            .map(str::to_owned)
    {
        fields.insert(
            Value::String("splat_requires_mechanics".into()),
            Value::Sequence(vec![Value::String(id)]),
        );
    }
    Ok(())
}

/// Sheet text as Markdown paragraphs. Each line reads as written and cannot change the note's
/// structure: see `literal_line`.
pub fn body_text(text: &str) -> String {
    text.lines()
        .map(literal_line)
        .collect::<Vec<_>>()
        .join("\n")
}
/// Sheet text on one line, inside a generated heading, list item or sentence.
pub fn inline_text(text: &str) -> String {
    literal_line(&text.split_whitespace().collect::<Vec<_>>().join(" "))
}
/// A line that cannot start a block. Every block that could hide or end the generated part, or
/// add a section, starts with an ASCII punctuation mark or with indentation: a heading, fence,
/// HTML block, quote, list, table or heading underline. An escaped first mark is shown as written.
/// An HTML comment cannot open inside the line.
fn literal_line(line: &str) -> String {
    let line = line.trim_start().replace("<!--", "&lt;!--");
    match line.chars().next() {
        Some(first) if first.is_ascii_punctuation() => format!("\\{line}"),
        _ => line,
    }
}
