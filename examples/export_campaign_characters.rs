//! Publish cached party records without contacting D&D Beyond. New definitions go to the campaign
//! package. List the folders of the packages that sheets refer to, so validation can find them.
use splat::{character::Cache, content::Entry};
fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() < 3 {
        return Err("Usage: export_campaign_characters <local-campaign-uuid> <ddb-campaign-id> <absolute-campaign-folder> [package-folder...]".into());
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
    let (mut entries, mut errors) = splat::content::read_directory(root)?;
    let local = entries.len();
    for package in &args[3..] {
        let (more, more_errors) = splat::content::read_directory(std::path::Path::new(package))?;
        entries.extend(more);
        errors.extend(more_errors);
    }
    let report = splat::content::validate_with_builtin(&entries)?;
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
        "Validated {local} local content entries against the shared catalogue; {gaps} conversion gaps."
    );
    Ok(())
}
