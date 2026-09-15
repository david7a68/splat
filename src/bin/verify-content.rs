//! Validate embedded SRD or local authored notes without writing to either.
use splat::content::{Diagnostic, builtin, read_directory, validate};
use std::path::Path;

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        println!(
            "Usage: verify-content [DIRECTORY ...]\nNo arguments: verify embedded SRD completeness.\nDirectories: check marked notes together for structure and references. Read-only; no simulation."
        );
        return;
    }
    let report = if args.is_empty() {
        builtin::load().report
    } else {
        let mut entries = Vec::new();
        let mut errors = Vec::new();
        for path in args {
            match read_directory(Path::new(&path)) {
                Ok((mut found, mut diagnostics)) => {
                    entries.append(&mut found);
                    errors.append(&mut diagnostics);
                }
                Err(message) => errors.push(Diagnostic { path, message }),
            }
        }
        let mut report = validate(&entries);
        report.errors.extend(errors);
        if report.entries == 0 {
            report.errors.push(Diagnostic {
                path: "input".into(),
                message: "No content notes found".into(),
            });
        }
        report
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("Report is serializable")
    );
    if !report.errors.is_empty() {
        std::process::exit(1);
    }
}
