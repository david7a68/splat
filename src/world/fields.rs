//! Small, lossless edits to one key in `splat_world`. Unsupported YAML stays untouched.
use serde_yaml_ng::Value;

pub(super) fn patch(text: &str, key: &str, value: Value) -> Result<String, String> {
    let entry = splat::content::Entry::parse("World.md", text)?;

    let flat = splat::content::formats::WORLD_FIELDS
        .iter()
        .find(|(old, _)| *old == key)
        .ok_or("Unknown World field")?
        .1;
    // An empty list leaves no property behind.
    let stored = (!value.as_sequence().is_some_and(Vec::is_empty))
        .then(|| splat::content::formats::flat_reference(&value));
    let output = splat::content::patch_fields(text, &[(flat, stored)])?;
    let checked = splat::content::Entry::parse("World.md", &output)?;
    let mut expected = (*entry.fields).clone();
    let world = expected
        .get_mut("splat_world")
        .and_then(Value::as_mapping_mut)
        .ok_or("Missing World properties")?;
    match checked.fields["splat_world"].get(key) {
        Some(value) => world.insert(key.into(), value.clone()),
        None => world.remove(key),
    };
    if *checked.fields != expected {
        return Err("Edit changed another World value".into());
    }
    Ok(output)
}
