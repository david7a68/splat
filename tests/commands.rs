use std::{
    fs,
    process::{Command, Output},
};
fn run(root: &std::path::Path, command: &str, flags: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_splat"))
        .arg(command)
        .arg(root)
        .args(flags)
        .env_remove("DISPLAY")
        .env_remove("WAYLAND_DISPLAY")
        .output()
        .unwrap()
}
fn old(name: &str) -> String {
    format!(
        "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: world\nsplat_name: {name}\nsplat_world: {{type: npc}}\n---\n\n# {name}\n",
        uuid::Uuid::new_v4()
    )
}
#[test]
fn convert_and_check_are_headless_and_dry_run_writes_nothing() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("Rook.md");
    let before = old("Rook");
    fs::write(&path, &before).unwrap();
    let check = run(dir.path(), "check", &[]);
    assert_eq!(check.status.code(), Some(1));
    let output = String::from_utf8(check.stdout).unwrap();
    assert!(output.contains("Rook.md:2: Old layout"), "{output}");
    let dry = run(dir.path(), "convert", &["--dry-run"]);
    assert!(dry.status.success());
    assert!(
        String::from_utf8(dry.stdout)
            .unwrap()
            .contains("1 files would convert; 0 skipped")
    );
    assert_eq!(fs::read_to_string(&path).unwrap(), before);
    assert!(run(dir.path(), "convert", &[]).status.success());
    assert!(
        fs::read_to_string(&path)
            .unwrap()
            .contains("splat_schema: 2")
    );
    assert!(run(dir.path(), "check", &[]).status.success());
    fs::write(dir.path().join("Broken.md"), "---\nsplat_schema: [\n---\n").unwrap();
    assert_eq!(run(dir.path(), "check", &[]).status.code(), Some(1));
    let convert = run(dir.path(), "convert", &[]);
    assert_eq!(convert.status.code(), Some(1));
    assert!(
        String::from_utf8(convert.stdout)
            .unwrap()
            .contains("Broken.md: skipped:")
    );
}
#[test]
fn generated_packages_can_be_explicitly_converted() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("Package.md"), format!("---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: package\nsplat_name: Book\nsplat_namespace: book\nsplat_generated: true\n---\n", uuid::Uuid::new_v4())).unwrap();
    let path = dir.path().join("Rook.md");
    let before = old("Rook");
    fs::write(&path, &before).unwrap();
    let skipped = run(dir.path(), "convert", &[]);
    assert_eq!(skipped.status.code(), Some(1));
    assert!(
        String::from_utf8(skipped.stdout)
            .unwrap()
            .contains("Generated package")
    );
    assert_eq!(fs::read_to_string(&path).unwrap(), before);
    assert!(
        run(dir.path(), "convert", &["--include-generated"])
            .status
            .success()
    );
    assert!(run(dir.path(), "check", &[]).status.success());
}

#[test]
fn old_timeline_is_not_loaded_and_converts_without_changing_events() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("Timeline.md");
    let old = "---\ntimeline_version: 1\ntimeline:\n- id: t-road\n  kind: travel\n  title: Old bridge\n  duration: 10 minutes\n---\n\n# Timeline\n\nKeep this prose.\n";
    fs::write(&path, old).unwrap();
    let check = run(dir.path(), "check", &[]);
    assert_eq!(check.status.code(), Some(1));
    assert!(
        String::from_utf8(check.stdout)
            .unwrap()
            .contains("Timeline.md:2: Old layout")
    );
    assert!(run(dir.path(), "convert", &["--dry-run"]).status.success());
    assert_eq!(fs::read_to_string(&path).unwrap(), old);
    assert!(run(dir.path(), "convert", &[]).status.success());
    let converted = fs::read_to_string(&path).unwrap();
    assert!(converted.contains("timeline_version: 2"));
    assert!(
        converted.contains("Old bridge")
            && converted.contains("10 minutes")
            && converted.contains("Keep this prose.")
    );
    assert!(run(dir.path(), "check", &[]).status.success());
}

#[test]
fn malformed_run_is_reported_and_skipped() {
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
    assert_eq!(run(dir.path(), "convert", &[]).status.code(), Some(1));
}
