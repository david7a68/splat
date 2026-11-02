//! Embedded, read-only SRD source package and completeness checks.
use super::{Diagnostic, Entry, Report, validate};
use include_dir::{Dir, include_dir};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::OnceLock,
};

static SRD: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/builtin/srd-5.2.1");
pub const SOURCE_URL: &str =
    "https://media.dndbeyond.com/compendium-images/srd/5.2/SRD_CC_v5.2.1.pdf";
const PDF_HASH: &str = "8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87";
const TEXT_HASH: &str = "4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562";
const COUNTS: [(&str, usize); 6] = [
    ("spell", 339),
    ("magic_item", 258),
    ("creature", 336),
    ("glossary", 155),
    ("page", 364),
    ("package", 1),
];
pub const ATTRIBUTION: &str = "This work includes material from the System Reference Document 5.2.1 (“SRD 5.2.1”) by Wizards of the Coast LLC, available at https://www.dndbeyond.com/srd. The SRD 5.2.1 is licensed under the Creative Commons Attribution 4.0 International License, available at https://creativecommons.org/licenses/by/4.0/legalcode.";

#[derive(Deserialize)]
struct Inventory {
    format: u32,
    source_url: String,
    source_sha256: String,
    source_text_sha256: String,
    pages: usize,
    counts: BTreeMap<String, usize>,
    files: Vec<Record>,
}

#[derive(Deserialize)]
struct Record {
    path: String,
    sha256: String,
    category: String,
    pages: Vec<usize>,
}

pub struct Package {
    pub entries: Vec<Entry>,
    pub report: Report,
}

/// The source bytes are embedded, so loading needs no network or working directory.
pub fn source_file(path: &str) -> Option<&'static [u8]> {
    SRD.get_file(path).map(|file| file.contents())
}

pub fn load() -> Package {
    fn files<'a>(dir: &'a Dir<'a>, out: &mut BTreeMap<String, &'a [u8]>) {
        for file in dir.files() {
            out.insert(file.path().to_string_lossy().into_owned(), file.contents());
        }
        for subdir in dir.dirs() {
            files(subdir, out);
        }
    }
    let mut contents = BTreeMap::new();
    files(&SRD, &mut contents);
    verify(&contents)
}

/// The SRD entries, verified once. They are embedded, so they cannot change while the app runs.
pub fn entries() -> &'static [Entry] {
    static ENTRIES: OnceLock<Vec<Entry>> = OnceLock::new();
    ENTRIES.get_or_init(|| load().entries)
}

fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn verify(files: &BTreeMap<String, &[u8]>) -> Package {
    let mut entries = Vec::new();
    let mut errors = Vec::new();
    let mut error = |path: &str, message: &str| {
        errors.push(Diagnostic {
            path: path.into(),
            message: message.into(),
        })
    };
    let inventory = files
        .get("inventory.json")
        .ok_or_else(|| "Missing inventory.json".to_string())
        .and_then(|bytes| serde_json::from_slice::<Inventory>(bytes).map_err(|e| e.to_string()));
    let inventory = match inventory {
        Ok(value) => value,
        Err(message) => {
            error("inventory.json", &message);
            return Package {
                entries,
                report: Report {
                    errors,
                    ..validate(&[])
                },
            };
        }
    };
    let counts = COUNTS[..4]
        .iter()
        .map(|(k, v)| (k.to_string(), *v))
        .collect();
    if inventory.format != 1
        || inventory.pages != 364
        || inventory.source_url != SOURCE_URL
        || inventory.source_sha256 != PDF_HASH
        || inventory.source_text_sha256 != TEXT_HASH
        || inventory.counts != counts
    {
        error(
            "inventory.json",
            "Source identity, page count, or category counts differ from pinned SRD 5.2.1",
        );
    }
    for (path, expected) in [("source.pdf", PDF_HASH), ("source.txt", TEXT_HASH)] {
        if files.get(path).is_none_or(|bytes| hash(bytes) != expected) {
            error(path, "Missing or changed pinned source");
        }
    }
    for path in ["LICENSE.txt", "Package.md"] {
        if files
            .get(path)
            .and_then(|bytes| std::str::from_utf8(bytes).ok())
            .is_none_or(|text| !text.contains(ATTRIBUTION))
        {
            error(path, "Missing required SRD attribution");
        }
    }
    let mut expected_files: BTreeSet<String> =
        ["source.pdf", "source.txt", "LICENSE.txt", "inventory.json"]
            .into_iter()
            .map(str::to_owned)
            .collect();
    let mut actual_counts = BTreeMap::<String, usize>::new();
    let mut page_numbers = BTreeSet::new();
    for record in &inventory.files {
        if !expected_files.insert(record.path.clone()) {
            error(&record.path, "Duplicate inventory path");
        }
        *actual_counts.entry(record.category.clone()).or_default() += 1;
        if record.pages.is_empty()
            || record.pages.iter().any(|p| !(1..=364).contains(p))
            || record.pages.windows(2).any(|p| p[0] >= p[1])
        {
            error(&record.path, "Invalid source page range");
        }
        if record.category == "page" {
            if record.pages.len() != 1 {
                error(&record.path, "Page note must cover exactly one page");
            }
            page_numbers.extend(record.pages.iter().copied());
        }
        let Some(bytes) = files.get(&record.path) else {
            error(&record.path, "Missing indexed note");
            continue;
        };
        if hash(bytes) != record.sha256 {
            error(&record.path, "Note hash mismatch");
        }
        let parsed = std::str::from_utf8(bytes)
            .map_err(|e| e.to_string())
            .and_then(|text| Entry::parse(&record.path, text));
        match parsed {
            Ok(entry) => {
                if entry.category() != record.category {
                    error(&record.path, "Category differs from inventory");
                }
                let pages = entry
                    .reference_metadata()
                    .and_then(|m| super::field(m, "pages"))
                    .and_then(|v| serde_yaml_ng::from_value::<Vec<usize>>(v.clone()).ok());
                if pages.as_ref() != Some(&record.pages) {
                    error(&record.path, "Pages differ from inventory");
                }
                if !entry
                    .key
                    .as_deref()
                    .is_some_and(|k| k.starts_with("dnd2024:"))
                {
                    error(&record.path, "Entry outside built-in namespace");
                }
                entries.push(entry);
            }
            Err(message) => error(&record.path, &message),
        }
    }
    if expected_files != files.keys().cloned().collect() {
        error("inventory.json", "File membership differs from inventory");
    }
    if actual_counts != COUNTS.iter().map(|(k, v)| (k.to_string(), *v)).collect()
        || page_numbers != (1..=364).collect()
    {
        error("inventory.json", "Incomplete entry or page coverage");
    }
    // Independently compare every facsimile page with the full extracted source.
    if let Some(text) = files
        .get("source.txt")
        .and_then(|bytes| std::str::from_utf8(bytes).ok())
    {
        let pages: Vec<_> = text.split('\u{c}').collect();
        if pages.len() != 365 || !pages[364].trim().is_empty() {
            error("source.txt", "Invalid page breaks");
        }
        for (index, page) in pages.iter().take(364).enumerate() {
            let path = format!("pages/{:03}.md", index + 1);
            let expected = page
                .trim_end()
                .lines()
                .map(str::trim_end)
                .collect::<Vec<_>>()
                .join("\n");
            let actual = entries
                .iter()
                .find(|e| e.path == path)
                .and_then(|e| e.body.split_once("```text\n"))
                .and_then(|(_, s)| s.strip_suffix("\n```\n"));
            if actual != Some(expected.as_str()) {
                error(&path, "Page text differs from full source extraction");
            }
        }
    }
    let mut report = validate(&entries);
    report.errors.extend(errors);
    report.limitations.push("Complete original PDF and all page text are included. Named entries are source excerpts, not typed spells, items, or creatures. PDF extraction can change table layout; consult the original PDF for typography.".into());
    Package { entries, report }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_embedded_source_loads_without_network() {
        let package = load();
        assert!(
            package.report.errors.is_empty(),
            "{:?}",
            package.report.errors
        );
        assert_eq!(package.report.entries, 1453);
        assert_eq!(package.report.reference_only, 1452);
        assert_eq!(package.report.executable_entries, 0);
        for (name, category, text) in [
            ("Hydra", "creature", "Reactive Heads."),
            ("Gray Ooze", "creature", "Pseudopod."),
            ("Animated Object", "creature", "Actions"),
            ("Avatar of Death", "creature", "Actions"),
            ("Acid Splash", "spell", "Cantrip Upgrade"),
            ("Zone of Truth", "spell", "deliberate lie"),
            (
                "Amulet of Proof against Detection and Location",
                "magic_item",
                "Divination",
            ),
            ("Efficient Quiver", "magic_item", "compartment"),
            ("Tome of Leadership and Influence", "magic_item", "Charisma"),
        ] {
            let entry = package
                .entries
                .iter()
                .find(|e| e.name.eq_ignore_ascii_case(name) && e.category() == category)
                .unwrap_or_else(|| panic!("Missing {name}"));
            assert!(entry.body.contains(text), "Incomplete {name}");
        }
    }

    #[test]
    fn altered_or_missing_content_fails_verification() {
        let mut files = BTreeMap::new();
        fn collect(dir: &Dir<'static>, files: &mut BTreeMap<String, &'static [u8]>) {
            for file in dir.files() {
                files.insert(file.path().to_string_lossy().into_owned(), file.contents());
            }
            for subdir in dir.dirs() {
                collect(subdir, files);
            }
        }
        collect(&SRD, &mut files);
        files.insert("source.pdf".into(), b"changed");
        files.insert("pages/001.md".into(), b"changed");
        files.remove("spell/acid_arrow.md");
        let report = verify(&files).report;
        for path in ["source.pdf", "pages/001.md", "spell/acid_arrow.md"] {
            assert!(
                report.errors.iter().any(|e| e.path == path),
                "Missing diagnostic for {path}"
            );
        }
    }
}
