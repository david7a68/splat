use std::{
    fs,
    process::{Command, Output},
};
fn run(root: &std::path::Path, command: &str, flags: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_splat"))
        .arg(command)
        .arg(root)
        .args(flags)
        .env("XDG_CONFIG_HOME", root.join(".config"))
        .env_remove("DISPLAY")
        .env_remove("WAYLAND_DISPLAY")
        .output()
        .unwrap()
}
fn note(schema: u64, name: &str) -> String {
    format!(
        "---\nsplat_schema: {schema}\nsplat_id: {}\nsplat_kind: world\nsplat_name: {name}\nsplat_world: {{type: npc}}\n---\n\n# {name}\n",
        uuid::Uuid::new_v4()
    )
}
#[test]
fn check_is_headless_and_reports_unreadable_notes_without_writing() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("Wren.md"), note(2, "Wren")).unwrap();
    let check = run(dir.path(), "check", &[]);
    assert!(check.status.success());
    let output = String::from_utf8(check.stdout).unwrap();
    assert!(
        output.contains("1 files loaded; 0 flags or unreadable files."),
        "{output}"
    );

    let path = dir.path().join("Rook.md");
    let before = note(1, "Rook");
    fs::write(&path, &before).unwrap();
    fs::write(dir.path().join("Broken.md"), "---\nsplat_schema: [\n---\n").unwrap();
    let check = run(dir.path(), "check", &[]);
    assert_eq!(check.status.code(), Some(1));
    let output = String::from_utf8(check.stdout).unwrap();
    assert!(
        output.contains("Rook.md:1: Unsupported content schema; expected 2"),
        "{output}"
    );
    assert!(output.contains("Broken.md:"), "{output}");
    assert!(
        output.contains("1 files loaded; 2 flags or unreadable files."),
        "{output}"
    );
    assert_eq!(fs::read_to_string(&path).unwrap(), before);
}

#[test]
fn only_check_is_a_command() {
    let dir = tempfile::tempdir().unwrap();
    for (command, flags) in [
        ("convert", &[][..]),
        ("convert", &["--dry-run"][..]),
        ("check", &["--verify"][..]),
    ] {
        let result = run(dir.path(), command, flags);
        assert_eq!(result.status.code(), Some(2), "{command} {flags:?}");
        assert!(
            String::from_utf8(result.stdout)
                .unwrap()
                .contains("Usage: splat check <folder>")
        );
    }
}

#[test]
fn version_one_timeline_is_unreadable_and_kept() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("Timeline.md");
    let text = "---\ntimeline_version: 1\ntimeline:\n- id: t-road\n  kind: travel\n  title: Bridge\n---\n\n# Timeline\n\nKeep this prose.\n";
    fs::write(&path, text).unwrap();
    let check = run(dir.path(), "check", &[]);
    assert_eq!(check.status.code(), Some(1));
    let output = String::from_utf8(check.stdout).unwrap();
    assert!(
        output.contains("Timeline.md:2: Unsupported timeline version (expected 2)."),
        "{output}"
    );
    assert_eq!(fs::read_to_string(&path).unwrap(), text);
}

#[test]
fn malformed_run_is_reported() {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir(dir.path().join("Encounter Runs")).unwrap();
    fs::write(
        dir.path().join("Encounter Runs/Broken.json"),
        "{\n \"state\": [",
    )
    .unwrap();
    let check = run(dir.path(), "check", &[]);
    assert_eq!(check.status.code(), Some(1));
    assert!(
        String::from_utf8(check.stdout)
            .unwrap()
            .contains("Broken.json:2:")
    );
}
