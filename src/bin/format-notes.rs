//! Build-time bridge for Python generators. Uses the same checked converter as Splat.
use std::io::{self, BufRead, Write};
fn main() {
    for line in io::stdin().lock().lines() {
        let result = (|| -> Result<String, String> {
            let (path, text): (String, String) =
                serde_json::from_str(&line.map_err(|e| e.to_string())?)
                    .map_err(|e| e.to_string())?;
            if path == "__entry_model__" {
                let entry = splat::content::legacy::parse(&path, &text)?;
                let mut fields = (*entry.fields).clone();
                fields.remove("splat_schema");
                serde_json::to_string(&serde_json::json!({
                    "fields": fields, "id": entry.id, "key": entry.key,
                    "name": entry.name, "kind": entry.kind,
                    "inventory": entry.inventory, "inventory_containers": entry.inventory_containers
                }))
                .map_err(|e| e.to_string())
            } else if path == "__model__" {
                let entry = splat::content::legacy::parse(&path, &text)?;
                serde_yaml_ng::to_string(&*entry.fields).map_err(|e| e.to_string())
            } else {
                splat::content::formats::convert(&path, &text)
            }
        })();
        println!("{}", serde_json::to_string(&result).unwrap());
        io::stdout().flush().unwrap();
    }
}
