//! Schema 2 building blocks, shared with later format phases. `Document::section`
//! finds a heading outside code; `Section::table` matches its header; `Table::row`
//! matches the first cell without case or emphasis. Cells carry original byte
//! ranges and line numbers. `replace_cell` changes value bytes, then checks the
//! complete table model. `replace_value` checks a caller's complete parsed model.
//! These functions return text only: callers retain their baseline-checked writer.
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use serde::{Deserialize, Serialize};
use serde_yaml_ng::Value;
use std::ops::Range;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Flag {
    pub line: usize,
    pub message: String,
}

pub fn label(value: &str) -> String {
    bare_label(value).to_lowercase()
}

fn bare_label(value: &str) -> &str {
    value
        .trim()
        .trim_start_matches("- ")
        .trim()
        .trim_matches('*')
        .trim()
}

/// `label(a) == label(b)` without allocating for the usual ASCII labels.
pub fn same_label(a: &str, b: &str) -> bool {
    same_bare(bare_label(a), bare_label(b))
}

fn same_bare(a: &str, b: &str) -> bool {
    if a.is_ascii() && b.is_ascii() {
        a.eq_ignore_ascii_case(b)
    } else {
        a.to_lowercase() == b.to_lowercase()
    }
}

pub struct Document<'a> {
    text: &'a str,
}

pub struct Section<'a> {
    text: &'a str,
    range: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub range: Range<usize>,
    pub line: usize,
    pub value: String,
}

#[derive(Clone, Debug)]
pub struct Table {
    pub header: Vec<Cell>,
    pub rows: Vec<Vec<Cell>>,
}

impl<'a> Document<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    pub fn table(&self, headers: &[&str]) -> Option<Table> {
        Section {
            text: self.text,
            range: 0..self.text.len(),
        }
        .table(headers)
    }

    pub fn section(&self, name: &str) -> Option<Section<'a>> {
        self.section_at_level(name, None)
    }

    pub fn section_at_level(
        &self,
        name: &str,
        required: Option<pulldown_cmark::HeadingLevel>,
    ) -> Option<Section<'a>> {
        let mut heading = None;
        let mut words = String::new();
        let mut found = None;
        for (event, range) in Parser::new_ext(self.text, Options::ENABLE_TABLES).into_offset_iter()
        {
            match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    if let Some((depth, start)) = found
                        && level <= depth
                    {
                        return Some(Section {
                            text: self.text,
                            range: start..range.start,
                        });
                    }
                    heading = Some((level, range.end));
                    words.clear();
                }
                Event::Text(word) | Event::Code(word) if heading.is_some() => words.push_str(&word),
                Event::End(TagEnd::Heading(_)) => {
                    if same_label(&words, name)
                        && found.is_none()
                        && required
                            .is_none_or(|level| heading.is_some_and(|(actual, _)| actual == level))
                    {
                        found = heading;
                    }
                    heading = None;
                }
                _ => {}
            }
        }
        found.map(|(_, start)| Section {
            text: self.text,
            range: start..self.text.len(),
        })
    }
}

impl Section<'_> {
    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }

    pub fn table(&self, headers: &[&str]) -> Option<Table> {
        for (event, range) in
            Parser::new_ext(&self.text[self.range.clone()], Options::ENABLE_TABLES)
                .into_offset_iter()
        {
            if !matches!(event, Event::Start(Tag::Table(_))) {
                continue;
            }
            let start = self.range.start + range.start;
            let mut offset = start;
            let mut lines = self.text[start..self.range.start + range.end].split_inclusive('\n');
            let first = lines.next()?;
            let mut line = self.text[..offset].bytes().filter(|b| *b == b'\n').count() + 1;
            let header = cells(offset, line, first);
            if header.len() != headers.len()
                || !header
                    .iter()
                    .zip(headers)
                    .all(|(a, b)| same_label(&a.value, b))
            {
                continue;
            }
            offset += first.len();
            let separator = lines.next()?;
            offset += separator.len();
            line += 2;
            let rows = lines
                .map(|text| {
                    let row = cells(offset, line, text);
                    offset += text.len();
                    line += 1;
                    row
                })
                .collect();
            return Some(Table { header, rows });
        }
        None
    }
}

/// `offset` and `number` place `line` in the document: its byte offset and line number.
fn cells(offset: usize, number: usize, line: &str) -> Vec<Cell> {
    let line = line.trim_end_matches(['\r', '\n']);
    let mut boundaries = Vec::new();
    let mut escaped = false;
    let mut wiki = false;
    let mut code = false;
    for (at, ch) in line.char_indices() {
        if ch == '|' && !escaped && !wiki && !code {
            boundaries.push(at);
        }
        if !escaped {
            if line[at..].starts_with("[[") {
                wiki = true;
            }
            if line[at..].starts_with("]]") {
                wiki = false;
            }
            if ch == '`' {
                code = !code;
            }
        }
        escaped = ch == '\\' && !escaped;
    }
    let mut spans = Vec::new();
    let mut start = 0;
    for end in boundaries {
        if start != 0 || !line[..end].trim().is_empty() {
            spans.push(start..end);
        }
        start = end + 1;
    }
    if !line[start..].trim().is_empty() {
        spans.push(start..line.len());
    }
    spans
        .into_iter()
        .map(|span| {
            let raw = &line[span.clone()];
            let begin = span.start + raw.len() - raw.trim_start().len();
            let end = begin + raw.trim().len();
            Cell {
                range: offset + begin..offset + end,
                line: number,
                value: raw.trim().replace("\\|", "|"),
            }
        })
        .collect()
}

impl Table {
    pub fn row(&self, name: &str) -> Option<&[Cell]> {
        self.rows
            .iter()
            .find(|row| {
                row.first()
                    .is_some_and(|cell| same_label(&cell.value, name))
            })
            .map(Vec::as_slice)
    }

    /// A bad or duplicate row stays in the document. The first row alone is read.
    pub fn read<T>(
        &self,
        name: &str,
        column: usize,
        flags: &mut Vec<Flag>,
        parse: impl Fn(&str) -> Result<T, String>,
    ) -> Option<T> {
        self.read_any(&[name], column, flags, parse)
    }

    /// `read` for a row with other names: any of `names` matches, and messages use the first.
    pub fn read_any<T>(
        &self,
        names: &[&str],
        column: usize,
        flags: &mut Vec<Flag>,
        parse: impl Fn(&str) -> Result<T, String>,
    ) -> Option<T> {
        let name = names[0];
        let bare: Vec<_> = names.iter().map(|name| bare_label(name)).collect();
        let mut result = None;
        let mut seen = false;
        for row in &self.rows {
            if row.first().is_none_or(|cell| {
                let cell = bare_label(&cell.value);
                !bare.iter().any(|name| same_bare(cell, name))
            }) {
                continue;
            }
            let first = &row[0];
            let error = if seen {
                Some(format!("Duplicate {name} row"))
            } else if row.len() != self.header.len() {
                Some(format!("{name}: missing or extra cells"))
            } else if let Some(cell) = row.get(column) {
                match parse(&cell.value) {
                    Ok(value) => {
                        result = Some(value);
                        None
                    }
                    Err(error) => Some(error),
                }
            } else {
                Some(format!("{name}: missing value cell"))
            };
            seen = true;
            if let Some(message) = error {
                flags.push(Flag {
                    line: first.line,
                    message,
                });
            }
        }
        result
    }
}

pub fn replace_cell(
    text: &str,
    section: &str,
    headers: &[&str],
    name: &str,
    column: usize,
    value: &str,
) -> Result<String, String> {
    replace_cell_at_level(text, section, headers, name, column, value, None)
}

pub fn replace_cell_at_level(
    text: &str,
    section: &str,
    headers: &[&str],
    name: &str,
    column: usize,
    value: &str,
    level: Option<pulldown_cmark::HeadingLevel>,
) -> Result<String, String> {
    let table = Document::new(text)
        .section_at_level(section, level)
        .and_then(|s| s.table(headers))
        .ok_or("Table not found")?;
    let row = table
        .rows
        .iter()
        .position(|row| row.first().is_some_and(|c| same_label(&c.value, name)))
        .ok_or("Row not found")?;
    let cell = table.rows[row].get(column).ok_or("Cell not found")?;
    let model = |table: &Table| {
        table
            .rows
            .iter()
            .map(|row| row.iter().map(|c| c.value.clone()).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    };
    let mut expected = model(&table);
    expected[row][column] = value.into();
    let mut after = text.to_owned();
    after.replace_range(cell.range.clone(), value);
    let checked = Document::new(&after)
        .section_at_level(section, level)
        .and_then(|s| s.table(headers))
        .ok_or("Edit changed the table layout")?;
    if model(&checked) != expected || value.contains(['\n', '\r']) {
        return Err("Edit changed another cell".into());
    }
    Ok(after)
}

pub fn replace_value(
    text: &str,
    range: Range<usize>,
    value: &str,
    path: &[&str],
    expected: Value,
    parse: impl Fn(&str) -> Result<Value, String>,
) -> Result<String, String> {
    let mut model = parse(text)?;
    let mut target = &mut model;
    for part in path {
        target = target.get_mut(*part).ok_or("Value not found")?;
    }
    *target = expected;
    let mut after = text.to_owned();
    after.replace_range(range, value);
    if parse(&after)? != model {
        return Err("Edit changed another value".into());
    }
    Ok(after)
}

/// Original line spans outside fenced and indented code blocks.
pub fn content_lines(text: &str) -> Vec<Cell> {
    let code: Vec<_> = Parser::new_ext(text, Options::ENABLE_TABLES)
        .into_offset_iter()
        .filter_map(|(event, range)| {
            matches!(event, Event::Start(Tag::CodeBlock(_))).then_some(range)
        })
        .collect();
    let mut offset = 0;
    text.split_inclusive('\n')
        .enumerate()
        .filter_map(|(line, raw)| {
            let start = offset;
            offset += raw.len();
            (!code.iter().any(|range| range.contains(&start))).then(|| Cell {
                range: start..offset,
                line: line + 1,
                value: raw.trim_end_matches(['\n', '\r']).into(),
            })
        })
        .collect()
}
