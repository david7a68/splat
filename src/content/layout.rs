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
    value
        .trim()
        .trim_start_matches("- ")
        .trim()
        .trim_matches('*')
        .trim()
        .to_lowercase()
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

    pub fn section(&self, name: &str) -> Option<Section<'a>> {
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
                    if label(&words) == label(name) && found.is_none() {
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
            let header = cells(self.text, offset, first);
            if header.len() != headers.len()
                || !header
                    .iter()
                    .zip(headers)
                    .all(|(a, b)| label(&a.value) == label(b))
            {
                continue;
            }
            offset += first.len();
            let separator = lines.next()?;
            offset += separator.len();
            let rows = lines
                .map(|line| {
                    let row = cells(self.text, offset, line);
                    offset += line.len();
                    row
                })
                .collect();
            return Some(Table { header, rows });
        }
        None
    }
}

fn cells(text: &str, offset: usize, line: &str) -> Vec<Cell> {
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
                line: text[..offset].bytes().filter(|b| *b == b'\n').count() + 1,
                value: raw.trim().into(),
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
                    .is_some_and(|cell| label(&cell.value) == label(name))
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
        let mut result = None;
        let mut seen = false;
        for row in &self.rows {
            if row
                .first()
                .is_none_or(|cell| label(&cell.value) != label(name))
            {
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
    let table = Document::new(text)
        .section(section)
        .and_then(|s| s.table(headers))
        .ok_or("Table not found")?;
    let row = table
        .rows
        .iter()
        .position(|row| row.first().is_some_and(|c| label(&c.value) == label(name)))
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
        .section(section)
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
