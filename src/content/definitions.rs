//! Whole-SRD catalogue. Structural validity and execution readiness are separate.
use super::{Diagnostic, Entry, Report, builtin, field, resource_key, validate};
use include_dir::{Dir, include_dir};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

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
    pub engine_gaps: Vec<String>,
    pub conversion_gaps: Vec<String>,
    pub execution_profiles: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_span: Option<LineSpan>,
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
    let meta: Conversion = serde_yaml_ng::from_value(value.clone()).map_err(|e| e.to_string())?;
    let clauses: Vec<Clause> = serde_yaml_ng::from_value(
        field(&entry.fields, "splat_clauses")
            .ok_or("Conversion requires source clauses")?
            .clone(),
    )
    .map_err(|e| e.to_string())?;
    if meta.pages.is_empty() || meta.pages.iter().any(|p| !(1..=364).contains(p)) {
        return Err("Conversion has invalid SRD pages".into());
    }
    if meta.source_sha256.len() != 64 || !meta.source_sha256.bytes().all(|b| b.is_ascii_hexdigit())
    {
        return Err("Conversion requires SHA256 provenance".into());
    }
    if meta.source_path.is_empty() || meta.category.is_empty() || clauses.is_empty() {
        return Err("Conversion requires source, category and clauses".into());
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
        .map(|v| serde_yaml_ng::from_value(v.clone()))
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

pub fn entries() -> Result<Vec<Entry>, String> {
    fn walk(dir: &Dir<'_>, result: &mut Vec<Entry>) -> Result<(), String> {
        for file in dir.files() {
            if file.path().extension().is_some_and(|e| e == "md") {
                result.push(Entry::parse(
                    &format!("definitions/{}", file.path().display()),
                    file.contents_utf8().ok_or("Definition must be UTF-8")?,
                )?);
            }
        }
        for child in dir.dirs() {
            walk(child, result)?;
        }
        Ok(())
    }
    let mut result = Vec::new();
    walk(&DEFINITIONS, &mut result)?;
    result.sort_by(|a, b| a.key.cmp(&b.key));
    Ok(result)
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
        if registry.format != 1 {
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
        for entry in &definitions {
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
                for capability in &contract.capabilities {
                    report
                        .engine_capabilities
                        .entry(capability.clone())
                        .or_default()
                        .push(key.into());
                }
            }
            match meta.status {
                Status::Partial => report.partial_definitions += 1,
                Status::Specified => report.specified_contract_definitions += 1,
                Status::Informational => report.informational_definitions += 1,
            }
            for gap in meta.engine_gaps {
                report.engine_gaps.entry(gap).or_default().push(key.into());
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
        all.extend(definitions);
        let checked = validate(&all);
        report.structure.entries = checked.entries;
        report.structure.errors.extend(checked.errors);
        report.structure.warnings.extend(checked.warnings);
        report.structure.limitations = vec![
            "Catalogue definitions and proof profiles are alternate views of the same identities. Do not load both as distinct definitions.".into(),
            "Rule specifications are converted, but their registered adapters are not implemented. Informational entries have no rule behavior. Source-specific choices and geometry facts must be resolved for affected runs.".into(),
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
        assert!(report.conversion_gaps.is_empty());
        assert!(report.engine_capabilities.contains_key("spellcasting"));
    }

    #[test]
    fn cannot_hide_unconverted_clauses_or_invent_execution_status() {
        let mut entry = entries()
            .unwrap()
            .into_iter()
            .find(|e| e.key.as_deref() == Some("dnd2024:spells/fireball"))
            .unwrap();
        let original = entry.clone();
        entry
            .fields
            .get_mut(Value::String("splat_clauses".into()))
            .unwrap()
            .as_sequence_mut()
            .unwrap()[0]
            .as_mapping_mut()
            .unwrap()
            .insert(
                Value::String("status".into()),
                Value::String("needs_semantic_conversion".into()),
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
    fn editing_source_clause_requires_a_new_fingerprint() {
        let mut entry = entries().unwrap().remove(0);
        let clauses = entry
            .fields
            .get_mut(Value::String("splat_clauses".into()))
            .unwrap()
            .as_sequence_mut()
            .unwrap();
        clauses[0].as_mapping_mut().unwrap().insert(
            Value::String("text".into()),
            Value::String("The target also dies.".into()),
        );
        assert!(conversion(&entry).unwrap_err().contains("fingerprint"));
    }

    #[test]
    fn specified_contracts_cannot_be_treated_as_no_ops() {
        for entry in entries().unwrap() {
            let metadata = conversion(&entry).unwrap().unwrap();
            if metadata.status == Status::Specified {
                let error = crate::execution::model::compile(&entry).unwrap_err();
                assert!(error.contains("not executable"));
                assert!(error.contains(&metadata.engine_gaps[0]));
            }
        }
    }
}
