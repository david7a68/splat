use crate::content::Entry;
use include_dir::{Dir, include_dir};
static MECHANICS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/builtin/dnd2024-mechanics");
pub fn entries() -> Result<Vec<Entry>, String> {
    fn walk(dir: &Dir<'_>, entries: &mut Vec<Entry>) -> Result<(), String> {
        for file in dir.files() {
            if file.path().extension().is_some_and(|e| e == "md") {
                entries.push(Entry::parse(
                    &file.path().to_string_lossy(),
                    file.contents_utf8().ok_or("Content must be UTF-8")?,
                )?);
            }
        }
        for d in dir.dirs() {
            walk(d, entries)?;
        }
        Ok(())
    }
    let mut entries = Vec::new();
    walk(&MECHANICS, &mut entries)?;
    entries.sort_by(|a, b| a.key.cmp(&b.key));
    Ok(entries)
}
