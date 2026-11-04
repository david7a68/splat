//! Shared preview and baseline-checked writes for the dialog and headless command.
use crate::{
    file_batch::{self, Change},
    library::{note_path, write_note},
};
use splat::content::{self, Entry, formats};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct File {
    pub root: PathBuf,
    pub path: String,
    pub before: String,
    pub after: String,
}

pub fn publish(file: &File) -> Result<Change, String> {
    let path = note_path(&file.root, &file.path)?;
    if [&path, &file.root]
        .iter()
        .any(|p| fs::metadata(p).is_ok_and(|m| m.permissions().readonly()))
    {
        return Err("This file is read only.".into());
    }
    let change = Change::text(path, Some(&file.before), Some(&file.after));
    if file.path.ends_with(crate::timeline::FILE) || file.path.ends_with(".json") {
        file_batch::apply(std::slice::from_ref(&change))?;
    } else {
        write_note(&file.root, &file.path, Some(&file.before), &file.after)?;
    }
    Ok(change)
}

pub fn preview(root: &Path, path: &str, before: String, entries: &[Entry]) -> Result<File, String> {
    let after = if path.ends_with(".json") {
        content::legacy::convert_run(&before)?
    } else if Path::new(path)
        .file_name()
        .is_some_and(|n| n == crate::timeline::FILE)
    {
        crate::timeline::formats::convert(&before, &Default::default())?
    } else {
        formats::convert_resolved(path, &before, entries)?
    };
    Ok(File {
        root: root.into(),
        path: path.into(),
        before,
        after,
    })
}

fn special_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    fn walk(path: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
        let mut children = fs::read_dir(path)
            .map_err(|e| format!("{}: {e}", path.display()))?
            .map(|e| e.map(|e| e.path()).map_err(|e| e.to_string()))
            .collect::<Result<Vec<_>, _>>()?;
        children.sort();
        for child in children {
            let meta = fs::symlink_metadata(&child).map_err(|e| e.to_string())?;
            if meta.file_type().is_symlink()
                || child
                    .file_name()
                    .is_some_and(|n| n.to_string_lossy().starts_with('.'))
            {
                continue;
            }
            if meta.is_dir() {
                walk(&child, files)?;
            } else if child
                .file_name()
                .is_some_and(|n| n == crate::timeline::FILE)
                || child.extension().is_some_and(|n| n == "json")
            {
                files.push(child);
            }
        }
        Ok(())
    }
    let mut files = Vec::new();
    walk(root, &mut files)?;
    Ok(files)
}

pub fn timelines(root: &Path) -> Result<Vec<PathBuf>, String> {
    Ok(special_files(root)?
        .into_iter()
        .filter(|p| p.file_name().is_some_and(|n| n == crate::timeline::FILE))
        .collect())
}
pub fn runs(root: &Path) -> Result<Vec<PathBuf>, String> {
    let (entries, _) = content::read_directory(root)?;
    let mut directories = vec![root.join("Encounter Runs")];
    for entry in entries.iter().filter(|e| e.kind == "campaign") {
        let folder = entry
            .fields
            .get("splat_run_storage")
            .and_then(|v| v.get("directory"))
            .and_then(serde_yaml_ng::Value::as_str)
            .unwrap_or("Encounter Runs");
        directories.push(root.join(&entry.path).parent().unwrap().join(folder));
    }
    Ok(special_files(root)?
        .into_iter()
        .filter(|p| {
            p.extension().is_some_and(|n| n == "json")
                && (directories.iter().any(|directory| p.starts_with(directory))
                    || fs::read_to_string(p).map_or(true, |text| {
                        serde_json::from_str::<serde_json::Value>(&text).map_or(true, |v| {
                            v.get("state").is_some() && v.get("version").is_some()
                        })
                    }))
        })
        .collect())
}
fn check_run_entries(value: &serde_json::Value, problems: &mut Vec<String>) {
    if value
        .get("fields")
        .and_then(|v| v.get("splat_schema"))
        .is_some()
    {
        match serde_json::from_value::<Entry>(value.clone()) {
            Err(error) => problems.push(error.to_string()),
            Ok(entry) => problems.extend(
                entry
                    .flags
                    .iter()
                    .map(|f| format!("{} (entry {}, line {})", f.message, entry.path, f.line)),
            ),
        }
    } else {
        match value {
            serde_json::Value::Object(map) => {
                for child in map.values() {
                    check_run_entries(child, problems);
                }
            }
            serde_json::Value::Array(values) => {
                for child in values {
                    check_run_entries(child, problems);
                }
            }
            _ => {}
        }
    }
}
pub fn check(root: &Path) -> Result<(Vec<String>, usize), String> {
    let (entries, errors) = content::read_directory(root)?;
    let mut problems: Vec<_> = errors
        .into_iter()
        .map(|e| {
            format!(
                "{}:{}: {}",
                e.path,
                diagnostic_line(
                    &std::fs::read_to_string(root.join(&e.path)).unwrap_or_default(),
                    &e.message
                ),
                e.message
            )
        })
        .collect();
    for entry in &entries {
        problems.extend(
            entry
                .flags
                .iter()
                .map(|f| format!("{}:{}: {}", entry.path, f.line, f.message)),
        );
    }
    let mut count = entries.len();
    for path in timelines(root)? {
        let name = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .display()
            .to_string();
        match fs::read_to_string(&path) {
            Err(error) => problems.push(format!("{name}:1: {error}")),
            Ok(text) => match crate::timeline::parse(&text) {
                Err(error) => problems.push(format!("{name}:2: {error}")),
                Ok(_) => {
                    count += 1;
                    problems.extend(
                        crate::timeline::formats::parse(&text)
                            .1
                            .into_iter()
                            .map(|f| format!("{name}:{}: {}", f.line, f.message)),
                    );
                }
            },
        }
    }
    for path in runs(root)? {
        let name = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .display()
            .to_string();
        let mut run_problems = Vec::new();
        let result = fs::read_to_string(&path)
            .map_err(|e| e.to_string())
            .and_then(|text| {
                let value: serde_json::Value =
                    serde_json::from_str(&text).map_err(|e| e.to_string())?;
                check_run_entries(&value, &mut run_problems);
                if run_problems.is_empty() {
                    crate::encounter_run::check_saved_text(&text)?;
                }
                Ok(())
            });
        if let Err(error) = result {
            run_problems.push(error);
        }
        if run_problems.is_empty() {
            count += 1;
        }
        let text = fs::read_to_string(&path).unwrap_or_default();
        problems.extend(
            run_problems
                .into_iter()
                .map(|error| format!("{name}:{}: {error}", diagnostic_line(&text, &error))),
        );
    }
    problems.sort();
    Ok((problems, count))
}

fn diagnostic_line(text: &str, message: &str) -> usize {
    if message.contains(content::OLD_LAYOUT) {
        return text
            .lines()
            .position(|line| {
                line.trim_start().starts_with("splat_schema:")
                    || line.contains("\"splat_schema\":")
                    || line.starts_with("timeline_version:")
            })
            .map_or(1, |n| n + 1);
    }
    message
        .split("line ")
        .nth(1)
        .and_then(|s| s.split(|c: char| !c.is_ascii_digit()).next())
        .and_then(|s| s.parse().ok())
        .unwrap_or(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dialog_and_command_publisher_refuse_an_outside_edit() {
        let dir = tempfile::tempdir().unwrap();
        let text = "---\nsplat_schema: 1\nsplat_id: 00000000-0000-0000-0000-000000000001\nsplat_kind: world\nsplat_name: Rook\nsplat_world: {type: npc}\n---\n";
        fs::write(dir.path().join("Rook.md"), text).unwrap();
        let file = preview(dir.path(), "Rook.md", text.into(), &[]).unwrap();
        fs::write(dir.path().join("Rook.md"), "Outside edit").unwrap();
        assert!(publish(&file).is_err());
        assert_eq!(
            fs::read_to_string(dir.path().join("Rook.md")).unwrap(),
            "Outside edit"
        );
    }

    #[test]
    fn saved_entry_copies_reject_old_layout_until_explicit_conversion() {
        let old = include_str!("../tests/fixtures/conversion/creature.old.md");
        let entry = content::legacy::parse("Wolf.md", old).unwrap();
        let copy = serde_json::to_value(&entry).unwrap();
        assert!(
            serde_json::from_value::<Entry>(copy.clone())
                .unwrap_err()
                .to_string()
                .contains(content::OLD_LAYOUT)
        );
        let run = serde_json::json!({"version": 1, "state": {"definition": copy, "hp": 17}, "keep": "Run notes"});
        let converted = content::legacy::convert_run(&run.to_string()).unwrap();
        let after: serde_json::Value = serde_json::from_str(&converted).unwrap();
        let entry_after: Entry =
            serde_json::from_value(after["state"]["definition"].clone()).unwrap();
        let mut expected = (*entry.fields).clone();
        expected.insert("splat_schema".into(), 2.into());
        assert_eq!(*entry_after.fields, expected);
        assert_eq!(after["state"]["hp"], 17);
        assert_eq!(after["keep"], "Run notes");
        assert_eq!(content::legacy::convert_run(&converted).unwrap(), converted);
    }
}
