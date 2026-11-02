//! Validate embedded SRD or local authored notes without writing to either.
use splat::content::{Diagnostic, builtin, read_directory, validate};
use std::path::Path;

fn main() {
    let mut args: Vec<_> = std::env::args().skip(1).collect();
    let with_builtin = args.first().is_some_and(|arg| arg == "--with-builtin");
    if with_builtin {
        args.remove(0);
    }
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        println!(
            "Usage: verify-content [--executable | --definitions [--require-converted] | --check-characters CACHE_DIRECTORY | [--with-builtin] DIRECTORY ...]\nNo arguments: verify embedded SRD completeness.\n--definitions: verify the full catalogue and report conversion/engine gaps.\n--require-converted: fail if semantic conversion remains incomplete.\n--executable: additionally compile the converted mechanics.\n--check-characters: compare cached D&D Beyond characters with the SRD rules; fail on a difference or a choice the rules do not allow. An unset choice is reported but is not a failure.\nDirectories: check marked notes together for structure and references. --with-builtin resolves shared SRD dependencies. Read-only; no simulation."
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
    if args.first().is_some_and(|arg| arg == "--check-characters") {
        let [_, directory] = args.as_slice() else {
            eprintln!("--check-characters takes one directory");
            std::process::exit(2);
        };
        check_characters(Path::new(directory));
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
        let mut report = if with_builtin {
            match splat::content::validate_with_builtin(&entries) {
                Ok(report) => report,
                Err(message) => {
                    errors.push(Diagnostic {
                        path: "builtin".into(),
                        message,
                    });
                    validate(&entries)
                }
            }
        } else {
            validate(&entries)
        };
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

/// Read-only: prints one review per cached character. Character records are `.json` files, so a
/// `.json` file that cannot be read counts as a failure. A run that checks nothing also fails.
fn check_characters(directory: &Path) {
    use splat::character::{EntitySnapshot, check};
    let mut failures = 0;
    let mut reviewed = 0;
    let mut stack = vec![directory.to_owned()];
    while let Some(path) = stack.pop() {
        if path.is_dir() {
            let children = std::fs::read_dir(&path).expect("Cache directory is readable");
            stack.extend(children.flatten().map(|child| child.path()));
            continue;
        }
        if path.extension().is_none_or(|extension| extension != "json") {
            continue;
        }
        let result = std::fs::read(&path)
            .map_err(|e| e.to_string())
            .and_then(|bytes| {
                serde_json::from_slice::<EntitySnapshot>(&bytes).map_err(|e| e.to_string())
            })
            .and_then(|record| {
                let entry = record.entry()?;
                check::review(&record, &entry)
            });
        let file = path.display().to_string();
        let line = match result {
            Ok(review) => {
                reviewed += 1;
                failures += review.count(check::FindingKind::Differs)
                    + review.count(check::FindingKind::InvalidChoice);
                serde_json::json!({"file": file, "review": review})
            }
            Err(message) => {
                failures += 1;
                serde_json::json!({"file": file, "error": message})
            }
        };
        println!("{line}");
    }
    if reviewed == 0 {
        eprintln!("No character records found in {}", directory.display());
        std::process::exit(1);
    }
    if failures > 0 {
        std::process::exit(1);
    }
}
