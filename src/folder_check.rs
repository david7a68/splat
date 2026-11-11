//! `splat check`: report every note, timeline and saved run in a folder that Splat cannot
//! read, and every flag on the ones it can.
use splat::content::{self, Entry};
use std::{
    fs,
    path::{Path, PathBuf},
};

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

fn timelines(root: &Path) -> Result<Vec<PathBuf>, String> {
    Ok(special_files(root)?
        .into_iter()
        .filter(|p| p.file_name().is_some_and(|n| n == crate::timeline::FILE))
        .collect())
}
fn runs(root: &Path) -> Result<Vec<PathBuf>, String> {
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
/// Collect a run file's errors, and the flags its saved entry copies hold. A saved flag has only
/// its sentence, which names its line in the entry's note, so it is given with its entry only.
fn check_run_entries(
    value: &serde_json::Value,
    problems: &mut Vec<String>,
    flags: &mut Vec<String>,
) {
    if value
        .get("fields")
        .and_then(|v| v.get("splat_schema"))
        .is_some()
    {
        match serde_json::from_value::<Entry>(value.clone()) {
            Err(error) => problems.push(error.to_string()),
            Ok(entry) => flags.extend(
                entry
                    .flags
                    .iter()
                    .map(|f| format!("{} (entry {})", f.message, entry.path)),
            ),
        }
    } else {
        match value {
            serde_json::Value::Object(map) => {
                for child in map.values() {
                    check_run_entries(child, problems, flags);
                }
            }
            serde_json::Value::Array(values) => {
                for child in values {
                    check_run_entries(child, problems, flags);
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
        .map(|e| format!("{}:{}: {}", e.path, diagnostic_line(&e.message), e.message))
        .collect();
    for entry in &entries {
        problems.extend(
            entry
                .flags
                .iter()
                .map(|f| format!("{}: {}", entry.path, f.message)),
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
                            .map(|f| format!("{name}: {}", f.message)),
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
        let mut entry_flags = Vec::new();
        let result = fs::read_to_string(&path)
            .map_err(|e| e.to_string())
            .and_then(|text| {
                let value: serde_json::Value =
                    serde_json::from_str(&text).map_err(|e| e.to_string())?;
                check_run_entries(&value, &mut run_problems, &mut entry_flags);
                if run_problems.is_empty() && entry_flags.is_empty() {
                    crate::encounter_run::check_saved_text(&text)?;
                }
                Ok(())
            });
        if let Err(error) = result {
            run_problems.push(error);
        }
        if run_problems.is_empty() && entry_flags.is_empty() {
            count += 1;
        }
        problems.extend(
            run_problems
                .into_iter()
                .map(|error| format!("{name}:{}: {error}", diagnostic_line(&error))),
        );
        problems.extend(
            entry_flags
                .into_iter()
                .map(|flag| format!("{name}: {flag}")),
        );
    }
    problems.sort();
    Ok((problems, count))
}

/// The line a message names, such as "line 7", or else the first line.
fn diagnostic_line(message: &str) -> usize {
    message
        .split("line ")
        .nth(1)
        .and_then(|s| s.split(|c: char| !c.is_ascii_digit()).next())
        .and_then(|s| s.parse().ok())
        .unwrap_or(1)
}
