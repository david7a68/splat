//! Dispatch before the instance lock, keyring, or window is opened.
use crate::folder_check;
use std::{io::Write, path::Path};

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
    if args.len() != 2 || args[0] != "check" {
        return Err(usage());
    }
    let root = Path::new(&args[1])
        .canonicalize()
        .map_err(|e| e.to_string())?;
    let (problems, count) = folder_check::check(&root)?;
    for problem in &problems {
        writeln!(out, "{problem}").map_err(|e| e.to_string())?;
    }
    writeln!(
        out,
        "{count} files loaded; {} flags or unreadable files.",
        problems.len()
    )
    .map_err(|e| e.to_string())?;
    Ok(i32::from(!problems.is_empty()))
}
fn usage() -> String {
    "Usage: splat check <folder>".into()
}
