//! Dispatch before the instance lock, settings, keyring, or window is opened.
use crate::{content_workspace::generated_package, conversion};
use splat::content;
use std::{fs, io::Write, path::Path};

pub fn run(args: &[String], out: &mut impl Write) -> i32 {
    match execute(args, out) {
        Ok(code) => code,
        Err(error) => {
            let _ = writeln!(out, "{error}");
            2
        }
    }
}
fn execute(args: &[String], out: &mut impl Write) -> Result<i32, String> {
    let Some(command) = args.first().map(String::as_str) else {
        return Err(usage());
    };
    if args.len() < 2 || !["convert", "check"].contains(&command) {
        return Err(usage());
    }
    if args[2..].iter().any(|arg| {
        command != "convert" || !["--include-generated", "--dry-run"].contains(&arg.as_str())
    }) {
        return Err(usage());
    }
    let root = Path::new(&args[1])
        .canonicalize()
        .map_err(|e| e.to_string())?;
    if command == "check" {
        let (problems, count) = conversion::check(&root)?;
        for problem in &problems {
            writeln!(out, "{problem}").map_err(|e| e.to_string())?;
        }
        writeln!(
            out,
            "{count} files loaded; {} flags or unreadable files.",
            problems.len()
        )
        .map_err(|e| e.to_string())?;
        return Ok(i32::from(!problems.is_empty()));
    }
    let dry = args.iter().any(|a| a == "--dry-run");
    let include = args.iter().any(|a| a == "--include-generated");
    let (entries, errors) = content::legacy::read_directory(&root)?;
    let mut skipped = errors.len();
    let mut converted = 0;
    for error in errors {
        writeln!(out, "{}: skipped: {}", error.path, error.message).map_err(|e| e.to_string())?;
    }
    let mut paths: Vec<_> = entries.iter().map(|e| root.join(&e.path)).collect();
    paths.extend(conversion::timelines(&root)?);
    paths.extend(conversion::runs(&root)?);
    paths.sort();
    paths.dedup();
    for path in paths {
        let relative = path
            .strip_prefix(&root)
            .map_err(|e| e.to_string())?
            .to_string_lossy()
            .to_string();
        let result = (|| -> Result<bool, String> {
            let before = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            let file = conversion::preview(&root, &relative, before, &entries)?;
            if file.before == file.after {
                return Ok(false);
            }
            if !include
                && path
                    .ancestors()
                    .skip(1)
                    .take_while(|p| p.starts_with(&root))
                    .any(generated_package)
            {
                return Err("Generated package. Use --include-generated; its converter will overwrite these files on its next run.".into());
            }
            if fs::metadata(&path)
                .map_err(|e| e.to_string())?
                .permissions()
                .readonly()
            {
                return Err("This file is read only.".into());
            }
            if !dry {
                conversion::publish(&file)?;
            }
            Ok(true)
        })();
        match result {
            Ok(true) => {
                converted += 1;
                writeln!(
                    out,
                    "{relative}: {}",
                    if dry { "would convert" } else { "converted" }
                )
            }
            Ok(false) => writeln!(out, "{relative}: already uses schema 2"),
            Err(error) => {
                skipped += 1;
                writeln!(out, "{relative}: skipped: {error}")
            }
        }
        .map_err(|e| e.to_string())?;
    }
    writeln!(
        out,
        "{converted} files {}; {skipped} skipped.",
        if dry { "would convert" } else { "converted" }
    )
    .map_err(|e| e.to_string())?;
    Ok(i32::from(skipped != 0))
}
fn usage() -> String {
    "Usage: splat convert <folder> [--include-generated] [--dry-run]\n       splat check <folder>"
        .into()
}
