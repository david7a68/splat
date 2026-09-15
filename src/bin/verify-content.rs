//! Validate embedded SRD or local authored notes without writing to either.
use splat::content::{Diagnostic, builtin, read_directory, validate};
use std::path::Path;

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        println!(
            "Usage: verify-content [--executable | --definitions [--require-converted] | DIRECTORY ...]\nNo arguments: verify embedded SRD completeness.\n--definitions: verify the full catalogue and report conversion/engine gaps.\n--require-converted: fail if semantic conversion remains incomplete.\n--executable: additionally compile the converted mechanics.\nDirectories: check marked notes together for structure and references. Read-only; no simulation."
        );
        return;
    }
    if args == ["--definitions"] || args == ["--definitions", "--require-converted"] {
        let report = splat::content::definitions::verify();
        println!(
            "{}",
            serde_json::to_string_pretty(&report).expect("Report is serializable")
        );
        if !report.structure.errors.is_empty()
            || (args.len() == 2 && report.partial_definitions > 0)
        {
            std::process::exit(1);
        }
        return;
    }
    let report = if args == ["--executable"] {
        let mut package = builtin::load();
        match splat::execution::builtin::entries() {
            Ok(entries) => {
                let count = entries.len();
                let mut errors = Vec::new();
                for entry in &entries {
                    if let Err(message) = splat::execution::model::compile(entry) {
                        errors.push(Diagnostic {
                            path: entry.path.clone(),
                            message,
                        });
                    }
                }
                package.entries.extend(entries);
                let mut report = validate(&package.entries);
                report.errors.extend(package.report.errors);
                let executable = count - errors.len();
                report.errors.extend(errors);
                report.executable_entries = executable;
                report.limitations=vec!["Executable means supported by the bounded proof interpreter. Full creature lifecycle, ongoing effects, reactions, equipment transitions, and campaign selection are not yet implemented. Run execution-proof and cargo test for exercised outcomes.".into()];
                report
            }
            Err(message) => {
                package.report.errors.push(Diagnostic {
                    path: "mechanics".into(),
                    message,
                });
                package.report
            }
        }
    } else if args.is_empty() {
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
