//! Publish cached party records without contacting D&D Beyond.
use splat::{character::Cache, content::Entry};
fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 3 {
        return Err("Usage: export_campaign_characters <local-campaign-uuid> <ddb-campaign-id> <absolute-campaign-folder>".into());
    }
    let local = args[0]
        .parse()
        .map_err(|_| "Invalid local campaign UUID.")?;
    let root = std::path::Path::new(&args[2]);
    let records = Cache::user()?.load_all(local, &args[1])?;
    if records.is_empty() {
        return Err("No cached party members found.".into());
    }
    for record in records {
        let path = record.save_campaign_copy(root)?;
        let text = std::fs::read_to_string(&path).map_err(|_| "Could not verify exported copy.")?;
        let entry = Entry::parse(path.to_str().ok_or("Invalid campaign path.")?, &text)?;
        if entry.id != record.definition.splat_id {
            return Err("Exported identity does not match.".into());
        }
        println!("{}: {}", record.definition.splat_name, path.display());
    }
    let (entries, errors) = splat::content::read_directory(&root.join("Characters/D&D Beyond"))?;
    let report = splat::content::validate(&entries);
    if !errors.is_empty() || !report.errors.is_empty() {
        return Err(format!(
            "Published content validation failed: {errors:?} {:?}",
            report.errors
        ));
    }
    let gaps: usize = entries
        .iter()
        .filter_map(|entry| entry.fields.get("splat_import"))
        .filter_map(|v| v.get("gaps"))
        .filter_map(serde_yaml_ng::Value::as_sequence)
        .map(Vec::len)
        .sum();
    println!(
        "Validated {} content entries; {gaps} conversion gaps.",
        entries.len()
    );
    Ok(())
}
