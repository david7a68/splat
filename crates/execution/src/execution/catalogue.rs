//! Verify the bundled catalogue against the execution profiles.
use crate::content::definitions::{Status, conversion, entries, read_registry};
use crate::content::{Diagnostic, Report, adapters, builtin, validate};
use serde::Serialize;
use sha2::{Digest, Sha256};
use splat_assets::{CONTRACTS, COVERAGE};
use std::collections::{BTreeMap, BTreeSet};

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
        let manifest: serde_json::Value =
            serde_json::from_str(COVERAGE).map_err(|e| e.to_string())?;
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
        let registry = read_registry(CONTRACTS).map_err(|e| e.to_string())?;
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
        let profiles = super::builtin::entries()?;
        let profile_keys: BTreeSet<_> = profiles.iter().filter_map(|e| e.key.as_deref()).collect();
        for profile in &profiles {
            super::model::compile(profile)?;
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
            let raw = splat_assets::DEFINITIONS
                .get_file(path)
                .and_then(|f| f.contents_utf8())
                .ok_or("Inventory file missing")?;
            if entry.path != format!("definitions/{path}")
                || record["sha256"] != format!("{:x}", Sha256::digest(raw.as_bytes()))
            {
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
            if super::model::compile(entry).is_ok() {
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
                if !adapters::is_implemented(&contract.id) {
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
                if adapters::is_implemented(&gap) {
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
        if let Some(unknown) = adapters::implemented()
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
    fn cannot_invent_execution_status() {
        use serde_yaml_ng::Value;

        let mut entry = entries()
            .unwrap()
            .iter()
            .find(|e| e.key.as_deref() == Some("dnd2024:spells/fireball"))
            .unwrap()
            .clone();
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
        assert!(super::super::model::compile(&entry).is_err());
    }

    #[test]
    fn specified_contracts_cannot_be_treated_as_no_ops() {
        for entry in entries().unwrap() {
            let metadata = conversion(entry).unwrap().unwrap();
            if metadata.status == Status::Specified {
                let error = super::super::model::compile(entry).unwrap_err();
                assert!(error.contains("not executable"));
                assert!(error.contains(&metadata.engine_gaps[0]));
            }
        }
    }
}
