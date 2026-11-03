//! Stage all files first. A later failure rolls back only bytes this batch wrote.
use std::{fs, io::Write, path::PathBuf};

pub struct Change {
    pub path: PathBuf,
    pub before: Option<Vec<u8>>,
    pub after: Option<Vec<u8>>,
}

impl Change {
    pub fn text(path: PathBuf, before: Option<&str>, after: Option<&str>) -> Self {
        Self {
            path,
            before: before.map(|s| s.as_bytes().to_vec()),
            after: after.map(|s| s.as_bytes().to_vec()),
        }
    }
    fn matches(&self, expected: Option<&[u8]>) -> Result<(), String> {
        let current = match fs::read(&self.path) {
            Ok(bytes) => Some(bytes),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(error) => return Err(format!("{}: {error}", self.path.display())),
        };
        if current.as_deref() != expected {
            return Err(format!(
                "{} changed on disk. No outside edit was replaced.",
                self.path.display()
            ));
        }
        Ok(())
    }
}

pub fn apply(changes: &[Change]) -> Result<(), String> {
    apply_inner(changes, |_| {})
}

fn apply_inner(changes: &[Change], before_commit: impl Fn(usize)) -> Result<(), String> {
    let mut paths = std::collections::BTreeSet::new();
    let mut staged = Vec::new();
    for change in changes {
        if !paths.insert(&change.path) {
            return Err(format!(
                "{} occurs twice in this save.",
                change.path.display()
            ));
        }
        change.matches(change.before.as_deref())?;
        let parent = change.path.parent().ok_or("Missing parent folder.")?;
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        let file = if let Some(bytes) = &change.after {
            let mut temp = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
            temp.write_all(bytes)
                .and_then(|()| temp.as_file().sync_all())
                .map_err(|e| e.to_string())?;
            Some(temp)
        } else {
            None
        };
        staged.push(file);
    }
    for change in changes {
        change.matches(change.before.as_deref())?;
    }
    for (index, (change, staged)) in changes.iter().zip(staged).enumerate() {
        before_commit(index);
        let result = change.matches(change.before.as_deref()).and_then(|()| {
            if let Some(staged) = staged {
                if change.before.is_some() {
                    staged.persist(&change.path)
                } else {
                    staged.persist_noclobber(&change.path)
                }
                .map_err(|e| e.to_string())?;
            } else {
                fs::remove_file(&change.path).map_err(|e| e.to_string())?;
            }
            Ok(())
        });
        if let Err(error) = result {
            let mut errors = vec![error];
            for written in changes[..index].iter().rev() {
                if let Err(error) = rollback(written) {
                    errors.push(format!("Rollback needs repair: {error}"));
                }
            }
            return Err(errors.join(" "));
        }
    }
    for parent in changes
        .iter()
        .filter_map(|change| change.path.parent())
        .collect::<std::collections::BTreeSet<_>>()
    {
        crate::sessions::sync_folder(parent)?;
    }
    Ok(())
}

fn rollback(change: &Change) -> Result<(), String> {
    change.matches(change.after.as_deref())?;
    if let Some(bytes) = &change.before {
        let mut temp =
            tempfile::NamedTempFile::new_in(change.path.parent().ok_or("Missing parent folder")?)
                .map_err(|e| e.to_string())?;
        temp.write_all(bytes)
            .and_then(|()| temp.as_file().sync_all())
            .map_err(|e| e.to_string())?;
        if change.after.is_some() {
            temp.persist(&change.path)
        } else {
            temp.persist_noclobber(&change.path)
        }
        .map_err(|e| e.to_string())?;
    } else {
        fs::remove_file(&change.path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integrity_d26_a_second_file_conflict_rolls_back_the_first() {
        let dir = tempfile::tempdir().unwrap();
        let first = dir.path().join("one.md");
        let second = dir.path().join("two.md");
        fs::write(&first, "one").unwrap();
        fs::write(&second, "two").unwrap();
        let changes = [
            Change::text(first.clone(), Some("one"), Some("new one")),
            Change::text(second.clone(), Some("two"), Some("new two")),
        ];
        let error = apply_inner(&changes, |index| {
            if index == 1 {
                fs::write(&second, "DM's edit").unwrap();
            }
        })
        .unwrap_err();
        assert!(error.contains("two.md"));
        assert_eq!(fs::read_to_string(first).unwrap(), "one");
        assert_eq!(fs::read_to_string(second).unwrap(), "DM's edit");
    }
}
