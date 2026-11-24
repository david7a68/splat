//! Whole-SRD catalogue. Structural validity and execution readiness are separate.
use super::{Entry, builtin, field, resource_key};
use include_dir::{Dir, File};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use splat_assets::{CONTRACTS, COVERAGE};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::Path;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Partial,
    Specified,
    Informational,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LineSpan {
    first: usize,
    count: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Conversion {
    pub status: Status,
    pub category: String,
    pub source_sha256: String,
    pub source_path: String,
    pub pages: Vec<u16>,
    /// The contracts this definition requires. `adapters` says which of them are still open.
    pub engine_gaps: Vec<String>,
    pub conversion_gaps: Vec<String>,
    pub execution_profiles: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_span: Option<LineSpan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_spans: Option<Vec<LineSpan>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub research_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specification_sha256: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Clause {
    key: String,
    text: String,
    sha256: String,
    status: ClauseStatus,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum ClauseStatus {
    NeedsSemanticConversion,
    ContractSpecified,
    Informational,
}

fn hash(text: &str) -> String {
    format!("{:x}", Sha256::digest(text.as_bytes()))
}

/// Validate authored status declarations; neither status grants execution.
pub fn conversion(entry: &Entry) -> Result<Option<Conversion>, String> {
    let Some(value) = field(&entry.fields, "splat_conversion") else {
        if field(&entry.fields, "splat_clauses").is_some() {
            return Err("splat_clauses requires splat_conversion".into());
        }
        return Ok(None);
    };
    let meta = Conversion::deserialize(value).map_err(|e| e.to_string())?;
    let clauses: Vec<Clause> = field(&entry.fields, "splat_clauses")
        .map(Vec::<Clause>::deserialize)
        .transpose()
        .map_err(|e| e.to_string())?
        .unwrap_or_default();
    if meta.pages.is_empty() || meta.pages.iter().any(|p| !(1..=364).contains(p)) {
        return Err("Conversion has invalid SRD pages".into());
    }
    if meta.source_sha256.len() != 64 || !meta.source_sha256.bytes().all(|b| b.is_ascii_hexdigit())
    {
        return Err("Conversion requires SHA256 provenance".into());
    }
    if meta.source_path.is_empty() || meta.category.is_empty() {
        return Err("Conversion requires source and category".into());
    }
    if meta.line_span.is_some() && meta.line_spans.is_some() {
        return Err("Conversion must use line_span or line_spans, not both".into());
    }
    if let Some(spans) = &meta.line_spans
        && (spans.is_empty()
            || spans.iter().any(|s| s.count == 0)
            || spans.windows(2).any(|pair| {
                pair[0]
                    .first
                    .checked_add(pair[0].count)
                    .is_none_or(|end| end > pair[1].first)
            }))
    {
        return Err("Invalid disjoint source line spans".into());
    }
    let mut keys = BTreeSet::new();
    for clause in &clauses {
        if !super::local_key(&clause.key) || !keys.insert(&clause.key) {
            return Err("Duplicate or invalid source clause key".into());
        }
        if clause.text.is_empty() || hash(&clause.text) != clause.sha256 {
            return Err("Source clause fingerprint mismatch".into());
        }
        if clause.status == ClauseStatus::NeedsSemanticConversion
            && !meta.conversion_gaps.contains(&clause.key)
        {
            return Err("Unconverted clause missing from conversion gaps".into());
        }
    }
    match meta.status {
        Status::Partial if meta.conversion_gaps.is_empty() => {
            return Err("Partial conversion requires a conversion gap".into());
        }
        Status::Specified
            if !meta.conversion_gaps.is_empty()
                || clauses
                    .iter()
                    .any(|c| c.status != ClauseStatus::ContractSpecified)
                || meta.engine_gaps.is_empty() =>
        {
            return Err(
                "Specified contract must have no conversion gaps and declare engine gaps".into(),
            );
        }
        Status::Informational
            if !meta.conversion_gaps.is_empty()
                || !meta.engine_gaps.is_empty()
                || clauses
                    .iter()
                    .any(|c| c.status != ClauseStatus::Informational) =>
        {
            return Err("Informational source cannot contain rule gaps".into());
        }
        _ => {}
    }
    let declarations = super::contracts::declarations(entry)?;
    if meta.status == Status::Specified
        && (declarations.is_empty()
            || meta.research_path.is_none()
            || meta.specification_sha256.is_none())
    {
        return Err("Researched rules require a typed contract interface".into());
    }
    if meta.status == Status::Informational
        && (!declarations.is_empty() || field(&entry.fields, "splat_actions").is_some())
    {
        return Err("Informational source cannot declare actions or implementations".into());
    }
    if let Some(expected) = &meta.specification_sha256 {
        let actual = super::contracts::fingerprint(&super::contracts::specification(entry)?)?;
        if &actual != expected {
            return Err("Typed specification fingerprint mismatch".into());
        }
    }
    let required: Vec<String> = field(&entry.fields, "splat_requires_mechanics")
        .map(Vec::<String>::deserialize)
        .transpose()
        .map_err(|e| e.to_string())?
        .unwrap_or_default();
    if meta.engine_gaps.iter().any(|g| !resource_key(g))
        || meta.engine_gaps.iter().collect::<BTreeSet<_>>()
            != required.iter().collect::<BTreeSet<_>>()
    {
        return Err("Engine gaps must match required mechanics".into());
    }
    if declarations
        .iter()
        .any(|contract| !required.contains(&contract.id))
    {
        return Err("Contract implementation missing from required mechanics".into());
    }
    if meta.execution_profiles.iter().any(|k| !resource_key(k)) {
        return Err("Invalid execution profile key".into());
    }
    Ok(Some(meta))
}

/// Original Markdown for read-only library display.
pub fn source_file(path: &str) -> Option<&'static str> {
    static FILES: OnceLock<HashMap<&'static Path, &'static File<'static>>> = OnceLock::new();
    FILES
        .get_or_init(|| {
            let mut files = Vec::new();
            markdown_files(&splat_assets::DEFINITIONS, &mut files);
            files.into_iter().map(|file| (file.path(), file)).collect()
        })
        .get(Path::new(path.strip_prefix("definitions/")?))?
        .contents_utf8()
}

fn markdown_files<'a>(dir: &Dir<'a>, files: &mut Vec<&'a File<'a>>) {
    files.extend(
        dir.files()
            .filter(|file| file.path().extension().is_some_and(|e| e == "md")),
    );
    for child in dir.dirs() {
        markdown_files(child, files);
    }
}

/// Every definition, parsed once. They are embedded, so they cannot change while the app runs.
pub fn entries() -> Result<&'static [Entry], String> {
    static ENTRIES: OnceLock<Result<Vec<Entry>, String>> = OnceLock::new();
    ENTRIES
        .get_or_init(|| {
            // Read both at once without making a Rayon worker wait on a
            // OnceLock whose initializer also needs the Rayon pool.
            std::thread::scope(|scope| {
                scope.spawn(builtin::entries);
                parse_entries()
            })
        })
        .as_deref()
        .map_err(Clone::clone)
}

fn parse_entries() -> Result<Vec<Entry>, String> {
    let mut files = Vec::new();
    markdown_files(&splat_assets::DEFINITIONS, &mut files);
    // The reports are large JSON files. Read them while the notes are parsed.
    let (result, reports) = rayon::join(
        || {
            files
                .par_iter()
                .map(|file| {
                    Entry::parse(
                        &format!("definitions/{}", file.path().display()),
                        file.contents_utf8().ok_or("Definition must be UTF-8")?,
                    )
                    .map_err(|e| format!("{}: {e}", file.path().display()))
                })
                .collect::<Result<Vec<_>, String>>()
        },
        || {
            let (coverage, registry) = rayon::join(
                || serde_json::from_str::<serde_json::Value>(COVERAGE),
                || read_registry(CONTRACTS),
            );
            Ok::<_, String>((
                coverage.map_err(|e| e.to_string())?,
                registry.map_err(|e| e.to_string())?,
            ))
        },
    );
    let mut result = result?;
    result.sort_by(|a, b| a.key.cmp(&b.key));
    let (coverage, registry) = reports?;
    attach_reports(&mut result, coverage, registry)?;
    Ok(result)
}

/// The registry holds thousands of nested contract shapes, and reading them is slow.
/// Read the file once, then read each contract on its own thread.
#[doc(hidden)]
pub fn read_registry(text: &str) -> Result<super::contracts::Registry, serde_json::Error> {
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Wire<'a> {
        format: u32,
        #[serde(borrow)]
        contracts: Vec<&'a serde_json::value::RawValue>,
    }
    let wire: Wire = serde_json::from_str(text)?;
    Ok(super::contracts::Registry {
        format: wire.format,
        contracts: wire
            .contracts
            .into_par_iter()
            .map(|contract| serde_json::from_str(contract.get()))
            .collect::<Result<_, _>>()?,
    })
}

/// Reports are generated package data, not editable note properties.
/// Attach only after checking identity and the mechanical fingerprint.
fn attach_reports(
    entries: &mut [Entry],
    coverage: serde_json::Value,
    registry: super::contracts::Registry,
) -> Result<(), String> {
    if registry.format != 2 {
        return Err("Unsupported contract registry".into());
    }
    let records = coverage["entries"]
        .as_array()
        .ok_or("Missing coverage entries")?;
    let interfaces: BTreeMap<_, _> = registry
        .contracts
        .iter()
        .map(|c| (c.definition.as_str(), c))
        .collect();
    let mut records_by_key = BTreeMap::new();
    for record in records {
        let key = record["key"].as_str().ok_or("Missing coverage key")?;
        if records_by_key.insert(key, record).is_some() {
            return Err("Duplicate coverage key".into());
        }
    }
    if interfaces.len() != registry.contracts.len() {
        return Err("Duplicate contract definition".into());
    }
    entries.par_iter_mut().try_for_each(|entry| {
        let record = entry
            .key
            .as_deref()
            .and_then(|key| records_by_key.get(key))
            .ok_or("Definition missing coverage record")?;
        if record["id"] != entry.id.to_string() {
            return Err("Report identity mismatch".into());
        }
        let mut metadata = record.as_object().ok_or("Invalid coverage record")?.clone();
        for key in ["key", "id", "path", "source", "sha256"] {
            metadata.remove(key);
        }
        // In the same order in every build, whatever order the report file and serde_json's
        // `preserve_order` feature (which GPUI turns on) give its keys.
        metadata.sort_keys();
        if field(&entry.fields, "splat_conversion").is_some()
            || field(&entry.fields, "splat_contracts").is_some()
        {
            return Err("Note duplicates generated report metadata".into());
        }
        entry.fields.insert(
            serde_yaml_ng::Value::String("splat_conversion".into()),
            serde_yaml_ng::to_value(metadata).map_err(|e| e.to_string())?,
        );
        if let Some(contract) = entry.key.as_deref().and_then(|key| interfaces.get(key)) {
            entry.fields.insert(
                serde_yaml_ng::Value::String("splat_contracts".into()),
                serde_yaml_ng::to_value(vec![contract.interface()]).map_err(|e| e.to_string())?,
            );
            contract.validate(entry)?;
        }
        conversion(entry)?;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml_ng::Value;

    #[test]
    fn cannot_hide_unconverted_clauses() {
        let mut entry = entries()
            .unwrap()
            .iter()
            .find(|e| e.key.as_deref() == Some("dnd2024:spells/fireball"))
            .unwrap()
            .clone();
        entry.fields.insert(
            Value::String("splat_clauses".into()),
            serde_yaml_ng::to_value(vec![Clause {
                key: "source_rules".into(),
                text: "A rule".into(),
                sha256: hash("A rule"),
                status: ClauseStatus::NeedsSemanticConversion,
            }])
            .unwrap(),
        );
        let meta = entry
            .fields
            .get_mut(Value::String("splat_conversion".into()))
            .unwrap()
            .as_mapping_mut()
            .unwrap();
        meta.insert(
            Value::String("conversion_gaps".into()),
            Value::Sequence(vec![]),
        );
        assert!(
            conversion(&entry)
                .unwrap_err()
                .contains("Unconverted clause")
        );
    }

    #[test]
    fn editing_body_mechanics_requires_a_new_fingerprint() {
        let mut entry = entries()
            .unwrap()
            .iter()
            .find(|e| e.name == "Fireball")
            .unwrap()
            .clone();
        entry
            .fields
            .get_mut(Value::String("splat_spell".into()))
            .unwrap()
            .as_mapping_mut()
            .unwrap()
            .insert(Value::String("level".into()), Value::Number(9.into()));
        assert!(conversion(&entry).unwrap_err().contains("fingerprint"));
    }
}
