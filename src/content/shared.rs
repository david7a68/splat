//! Resolve authored/imported notes against the same built-in catalogue as the app.
use super::*;

pub fn validate_with_builtin(entries: &[Entry]) -> Result<Report, String> {
    let definitions = definitions::entries()?;
    let builtin = || super::builtin::entries().iter().chain(definitions);
    let ids: BTreeMap<_, _> = builtin().map(|e| (e.id, e)).collect();
    let keys: BTreeMap<_, _> = builtin()
        .filter_map(|e| e.key.as_ref().map(|k| (k, e)))
        .collect();
    let mut combined = entries.to_vec();
    let mut seen: BTreeSet<_> = entries.iter().map(|e| e.id).collect();
    // A campaign cannot replace a built-in identity by shadowing it.
    if entries
        .iter()
        .any(|e| ids.contains_key(&e.id) || e.key.as_ref().is_some_and(|k| keys.contains_key(k)))
    {
        return Err("Local content duplicates a shared built-in identity".into());
    }
    let mut cursor = 0;
    while cursor < combined.len() {
        for reference in required_references(&combined[cursor])? {
            let dependency = match reference {
                Reference::Id(id) => ids.get(&id),
                Reference::Key(key) => keys.get(&key),
            };
            if let Some(dependency) = dependency
                && seen.insert(dependency.id)
            {
                combined.push((*dependency).clone());
            }
        }
        cursor += 1;
    }
    Ok(validate(&combined))
}
