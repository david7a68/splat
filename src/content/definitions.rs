//! Whole-SRD catalogue. Structural validity and execution readiness are separate.
use super::{Diagnostic, Entry, Report, builtin, field, resource_key, validate};
use include_dir::{Dir, File, include_dir};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::Path;
use std::sync::OnceLock;

static DEFINITIONS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/builtin/dnd2024-definitions");

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
            markdown_files(&DEFINITIONS, &mut files);
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
        .get_or_init(parse_entries)
        .as_deref()
        .map_err(Clone::clone)
}

fn parse_entries() -> Result<Vec<Entry>, String> {
    let mut files = Vec::new();
    markdown_files(&DEFINITIONS, &mut files);
    let mut result: Vec<_> = files
        .par_iter()
        .map(|file| {
            Entry::parse(
                &format!("definitions/{}", file.path().display()),
                file.contents_utf8().ok_or("Definition must be UTF-8")?,
            )
            .map_err(|e| format!("{}: {e}", file.path().display()))
        })
        .collect::<Result<_, _>>()?;
    result.sort_by(|a, b| a.key.cmp(&b.key));
    attach_reports(
        &mut result,
        DEFINITIONS
            .get_file("coverage.json")
            .and_then(|f| f.contents_utf8())
            .ok_or("Missing coverage")?,
        DEFINITIONS
            .get_file("contracts.json")
            .and_then(|f| f.contents_utf8())
            .ok_or("Missing contracts")?,
    )?;
    Ok(result)
}

/// Reports are generated package data, not editable note properties.
/// Attach only after checking identity and the mechanical fingerprint.
fn attach_reports(entries: &mut [Entry], coverage: &str, contracts: &str) -> Result<(), String> {
    let coverage: serde_json::Value = serde_json::from_str(coverage).map_err(|e| e.to_string())?;
    let registry: super::contracts::Registry =
        serde_json::from_str(contracts).map_err(|e| e.to_string())?;
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

#[derive(Serialize)]
pub struct CatalogueReport {
    pub structure: Report,
    pub definitions: usize,
    pub specified_contract_definitions: usize,
    pub partial_definitions: usize,
    pub informational_definitions: usize,
    pub engine_capabilities: BTreeMap<String, Vec<String>>,
    pub adjudications: Vec<serde_json::Value>,
    pub executable_full_definitions: usize,
    pub executable_profiles: usize,
    /// Contracts the catalogue requires, whether or not they are implemented.
    pub required_engine_contracts: usize,
    /// Required contracts on the engine's implemented list.
    pub implemented_engine_contracts: Vec<String>,
    /// Required contracts with no implementation yet.
    pub engine_gaps: BTreeMap<String, Vec<String>>,
    pub conversion_gaps: BTreeMap<String, Vec<String>>,
}

pub fn verify() -> CatalogueReport {
    let source = builtin::load();
    let mut report = CatalogueReport {
        structure: source.report,
        definitions: 0,
        specified_contract_definitions: 0,
        partial_definitions: 0,
        informational_definitions: 0,
        engine_capabilities: BTreeMap::new(),
        adjudications: Vec::new(),
        executable_full_definitions: 0,
        executable_profiles: 0,
        required_engine_contracts: 0,
        implemented_engine_contracts: Vec::new(),
        engine_gaps: BTreeMap::new(),
        conversion_gaps: BTreeMap::new(),
    };
    let result = (|| -> Result<(), String> {
        let definitions = entries()?;
        report.definitions = definitions.len();
        let manifest: serde_json::Value = serde_json::from_str(
            DEFINITIONS
                .get_file("coverage.json")
                .and_then(|f| f.contents_utf8())
                .ok_or("Missing definition coverage inventory")?,
        )
        .map_err(|e| e.to_string())?;
        report.adjudications = manifest["adjudications"]
            .as_array()
            .ok_or("Missing source adjudication inventory")?
            .clone();
        let records = manifest["entries"]
            .as_array()
            .ok_or("Missing inventory entries")?;
        if records.len() != definitions.len() {
            return Err("Definition inventory count mismatch".into());
        }
        let registry: super::contracts::Registry = serde_json::from_str(
            DEFINITIONS
                .get_file("contracts.json")
                .and_then(|f| f.contents_utf8())
                .ok_or("Missing contract registry")?,
        )
        .map_err(|e| e.to_string())?;
        if registry.format != 2 {
            return Err("Unsupported contract registry".into());
        }
        let mut contract_definitions = BTreeMap::new();
        let mut contract_ids = BTreeSet::new();
        for contract in &registry.contracts {
            if !contract_ids.insert(&contract.id)
                || contract_definitions
                    .insert(contract.definition.as_str(), contract)
                    .is_some()
            {
                return Err("Duplicate contract registry identity".into());
            }
        }
        let profiles = crate::execution::builtin::entries()?;
        let profile_keys: BTreeSet<_> = profiles.iter().filter_map(|e| e.key.as_deref()).collect();
        for profile in &profiles {
            crate::execution::model::compile(profile)?;
            report.executable_profiles += 1;
        }
        let mut seen = BTreeSet::new();
        let mut source_hashes = BTreeMap::new();
        for entry in definitions {
            let key = entry
                .key
                .as_deref()
                .ok_or("Catalogue entries require keys")?;
            let record = records
                .iter()
                .find(|r| r["key"] == key)
                .ok_or("Definition absent from inventory")?;
            if !seen.insert(key) || record["id"] != entry.id.to_string() {
                return Err(format!("Definition identity mismatch: {key}"));
            }
            let path = record["path"].as_str().ok_or("Inventory path missing")?;
            let raw = DEFINITIONS
                .get_file(path)
                .and_then(|f| f.contents_utf8())
                .ok_or("Inventory file missing")?;
            if entry.path != format!("definitions/{path}") || record["sha256"] != hash(raw) {
                return Err(format!("Definition file fingerprint mismatch: {key}"));
            }
            let meta = conversion(entry)?.ok_or("Missing conversion metadata")?;
            let source_path = meta
                .source_path
                .strip_prefix("builtin/srd-5.2.1/")
                .ok_or("Definition provenance must name the bundled SRD")?;
            let source_bytes =
                builtin::source_file(source_path).ok_or("Definition source file missing")?;
            let source_hash = source_hashes
                .entry(source_path.to_owned())
                .or_insert_with(|| format!("{:x}", Sha256::digest(source_bytes)));
            if *source_hash != meta.source_sha256 {
                return Err(format!("Source fingerprint mismatch: {key}"));
            }
            if let Some(profile) = profiles.iter().find(|p| p.key == entry.key)
                && profile.id != entry.id
            {
                return Err(format!("Profile and catalogue disagree on identity: {key}"));
            }
            let encoded = serde_json::to_value(&meta).map_err(|e| e.to_string())?;
            for (field, value) in encoded.as_object().ok_or("Invalid conversion metadata")? {
                if record[field] != *value {
                    return Err(format!("Inventory metadata mismatch: {key}/{field}"));
                }
            }
            if meta
                .execution_profiles
                .iter()
                .any(|p| !profile_keys.contains(p.as_str()))
            {
                return Err(format!("Missing execution profile for {key}"));
            }
            if crate::execution::model::compile(entry).is_ok() {
                return Err(format!("Catalogue definition unexpectedly executed: {key}"));
            }
            if meta.status == Status::Specified {
                let contract = contract_definitions
                    .remove(key)
                    .ok_or("Specified definition missing contract registry entry")?;
                contract.validate(entry)?;
                if contract.source_sha256 != meta.source_sha256
                    || Some(&contract.research_path) != meta.research_path.as_ref()
                {
                    return Err(format!("Contract provenance mismatch: {key}"));
                }
                if !super::adapters::is_implemented(&contract.id) {
                    for capability in &contract.capabilities {
                        report
                            .engine_capabilities
                            .entry(capability.clone())
                            .or_default()
                            .push(key.into());
                    }
                }
            }
            match meta.status {
                Status::Partial => report.partial_definitions += 1,
                Status::Specified => report.specified_contract_definitions += 1,
                Status::Informational => report.informational_definitions += 1,
            }
            for gap in meta.engine_gaps {
                report.required_engine_contracts += 1;
                if super::adapters::is_implemented(&gap) {
                    report.implemented_engine_contracts.push(gap);
                } else {
                    report.engine_gaps.entry(gap).or_default().push(key.into());
                }
            }
            if !meta.conversion_gaps.is_empty() {
                report
                    .conversion_gaps
                    .insert(key.into(), meta.conversion_gaps);
            }
        }
        for case in &report.adjudications {
            let key = case["key"]
                .as_str()
                .ok_or("Adjudication requires a definition key")?;
            if !seen.contains(key) {
                return Err(format!("Adjudication references unknown definition: {key}"));
            }
        }
        if !contract_definitions.is_empty() {
            return Err("Registry contains unknown definitions".into());
        }
        if let Some(unknown) = super::adapters::implemented()
            .iter()
            .find(|id| !contract_ids.contains(id))
        {
            return Err(format!("Implemented adapter has no contract: {unknown}"));
        }
        for reference in &source.entries {
            if ["creature", "spell", "magic_item", "glossary"].contains(&reference.category()) {
                let key = reference
                    .key
                    .as_deref()
                    .ok_or("Source reference missing key")?;
                if !records.iter().any(|r| r["source"] == key) {
                    return Err(format!("Source definition missing from catalogue: {key}"));
                }
            }
        }
        for page in 5..=364 {
            if !records.iter().any(|r| {
                r["pages"]
                    .as_array()
                    .is_some_and(|p| p.contains(&serde_json::json!(page)))
            }) {
                return Err(format!("SRD page {page} absent from catalogue coverage"));
            }
        }
        let mut all = source.entries;
        all.extend_from_slice(definitions);
        let checked = validate(&all);
        report.structure.entries = checked.entries;
        report.structure.errors.extend(checked.errors);
        report.structure.warnings.extend(checked.warnings);
        report.structure.limitations = vec![
            "Catalogue definitions and proof profiles are alternate views of the same identities. Do not load both as distinct definitions.".into(),
            "Rule specifications are converted. Implemented means engine code handles the contract in full; every other required contract is an open engine gap. It does not make a definition executable by the action interpreter. Informational entries have no rule behavior. Source-specific choices and geometry facts must be resolved for affected runs.".into(),
        ];
        Ok(())
    })();
    if let Err(message) = result {
        report.structure.errors.push(Diagnostic {
            path: "definitions".into(),
            message,
        });
    }
    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml_ng::Value;

    #[test]
    fn complete_catalogue_is_valid_but_does_not_claim_execution() {
        let report = verify();
        assert!(
            report.structure.errors.is_empty(),
            "{:?}",
            report.structure.errors
        );
        assert!(
            report.structure.warnings.is_empty(),
            "{:?}",
            report.structure.warnings
        );
        assert_eq!(report.definitions, 2065);
        assert_eq!(report.specified_contract_definitions, 1970);
        assert_eq!(report.partial_definitions, 0);
        assert_eq!(report.informational_definitions, 95);
        assert_eq!(report.executable_full_definitions, 0);
        assert_eq!(report.executable_profiles, 32);
        assert!(report.engine_gaps.contains_key("dnd2024:spell_polymorph"));
        assert_eq!(report.required_engine_contracts, 1970);
        assert_eq!(report.implemented_engine_contracts.len(), 35);
        assert_eq!(report.engine_gaps.len(), 1970 - 35);
        for implemented in &report.implemented_engine_contracts {
            assert!(!report.engine_gaps.contains_key(implemented));
        }
        // These need more than character building, so they stay open.
        for open in [
            "dnd2024:class_features_wizard_level_3_wizard_subclass",
            "dnd2024:classes_barbarian",
            "dnd2024:class_features_barbarian_level_4_ability_score_improvement",
            "dnd2024:class_features_barbarian_level_19_epic_boon",
        ] {
            assert!(report.engine_gaps.contains_key(open), "{open}");
        }
        assert!(report.conversion_gaps.is_empty());
        assert!(report.engine_capabilities.contains_key("spellcasting"));
    }

    #[test]
    fn cannot_hide_unconverted_clauses_or_invent_execution_status() {
        let mut entry = entries()
            .unwrap()
            .iter()
            .find(|e| e.key.as_deref() == Some("dnd2024:spells/fireball"))
            .unwrap()
            .clone();
        let original = entry.clone();
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
        entry = original;
        entry
            .fields
            .get_mut(Value::String("splat_conversion".into()))
            .unwrap()
            .as_mapping_mut()
            .unwrap()
            .insert(
                Value::String("status".into()),
                Value::String("executable".into()),
            );
        assert!(conversion(&entry).is_err());
        assert!(crate::execution::model::compile(&entry).is_err());
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

    #[test]
    fn specified_contracts_cannot_be_treated_as_no_ops() {
        for entry in entries().unwrap() {
            let metadata = conversion(entry).unwrap().unwrap();
            if metadata.status == Status::Specified {
                let error = crate::execution::model::compile(entry).unwrap_err();
                assert!(error.contains("not executable"));
                assert!(error.contains(&metadata.engine_gaps[0]));
            }
        }
    }
}
