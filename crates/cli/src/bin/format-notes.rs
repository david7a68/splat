//! Build-time bridge for Python generators. Uses the same checked note writer and reader as Splat.
//! Each input line is `{"write": {path, fields, body}}` or `{"read": {path, text}}`.
use std::io::{self, BufRead, Write};

#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
enum Request {
    Write {
        path: String,
        fields: serde_yaml_ng::Mapping,
        body: String,
    },
    Read {
        path: String,
        text: String,
    },
}

fn answer(line: &str) -> Result<String, String> {
    match serde_json::from_str(line).map_err(|e| e.to_string())? {
        Request::Write { path, fields, body } => {
            splat::content::formats::write(&path, &fields, &body)
        }
        Request::Read { path, text } => {
            let entry = splat::content::Entry::parse(&path, &text)?;
            let mut fields = (*entry.fields).clone();
            fields.remove("splat_schema");
            serde_json::to_string(&serde_json::json!({
                "fields": fields, "id": entry.id, "key": entry.key,
                "name": entry.name, "kind": entry.kind,
                "inventory": entry.inventory, "inventory_containers": entry.inventory_containers
            }))
            .map_err(|e| e.to_string())
        }
    }
}

fn main() {
    for line in io::stdin().lock().lines() {
        let result = line
            .map_err(|e| e.to_string())
            .and_then(|line| answer(&line));
        println!("{}", serde_json::to_string(&result).unwrap());
        io::stdout().flush().unwrap();
    }
}
