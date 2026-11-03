//! Give the visual suite its own temporary files at the original visible paths.
//! Only the suite is re-executed: other tests stay in the parent test process.
use std::{env, fs, path::Path, process::Command};

const CHILD: &str = "SPLAT_VISUAL_NAMESPACE_CHILD";
const MOUNT: &str = r#"
set -eu
mount --make-rprivate /
mount --bind "$1" "$2"
shift 2
exec "$@"
"#;

pub(super) fn is_child() -> bool {
    env::var_os(CHILD).is_some()
}

/// Return true if a child ran the suite. Unsupported hosts keep the original lock.
pub(super) fn run() -> bool {
    if is_child() || env::var("SPLAT_VISUAL_ISOLATE").is_ok_and(|value| value == "0") {
        return false;
    }
    let prepare = || -> std::io::Result<_> {
        let temp = fs::canonicalize(env::temp_dir())?;
        let cwd = env::current_dir()?;
        let executable = env::current_exe()?;
        // Binding over temp would hide these paths from the child. Keep the
        // original locked route for unusual builds/captures inside temp.
        let output = env::var_os("SPLAT_VISUAL_DIR").map(std::path::PathBuf::from);
        if cwd.starts_with(&temp)
            || executable.starts_with(&temp)
            || output.is_some_and(|path| cwd.join(path).starts_with(&temp))
        {
            return Err(std::io::Error::other("build or captures are inside temp"));
        }
        // Two commands in one worktree may write the same capture label. Keep
        // those runs serial, without holding up any other worktree.
        fs::create_dir_all(cwd.join("target"))?;
        let worktree_lock = fs::File::create(cwd.join("target/splat-visual.lock"))?;
        let started = std::time::Instant::now();
        worktree_lock.lock()?;
        if env::var_os("SPLAT_VISUAL_TIMINGS").is_some() {
            println!("Timing worktree lock wait = {:?}", started.elapsed());
        }
        // Stay on the original filesystem. Directory enumeration and fsync
        // behavior differ between tmpfs and disk; both matter to these fixtures.
        // Only synthetic fixtures live here, never builds, captures or videos.
        let backing = tempfile::Builder::new()
            .prefix("splat-visual-namespace-")
            .tempdir_in(&temp)?;
        Ok((backing, temp, executable, worktree_lock))
    };
    let (backing, temp, executable, _worktree_lock) = match prepare() {
        Ok(paths) => paths,
        Err(error) => {
            eprintln!("Visual namespace unavailable; using the shared world lock: {error}");
            return false;
        }
    };
    // Probe setup separately: failure to create a namespace can fall back, but
    // a failing scenario in an established namespace must never be retried away.
    let probe = namespace(backing.path(), &temp).arg("true").output();
    match probe {
        Ok(output) if output.status.success() => {}
        result => {
            let reason = match result {
                Ok(output) => String::from_utf8_lossy(&output.stderr).trim().to_owned(),
                Err(error) => error.to_string(),
            };
            eprintln!("Visual namespace unavailable; using the shared world lock: {reason}");
            return false;
        }
    }
    println!(
        "Visual namespace: {} at {}",
        backing.path().display(),
        temp.display()
    );
    let status = namespace(backing.path(), &temp)
        .arg(executable)
        .args(["--exact", "visual_tests::visual_suite", "--nocapture"])
        .env(CHILD, "1")
        .status()
        .expect("start isolated visual suite");
    assert!(status.success(), "isolated visual suite failed: {status}");
    true
}

fn namespace(backing: &Path, temp: &Path) -> Command {
    let mut command = Command::new("unshare");
    command
        .args([
            "--user",
            "--map-root-user",
            "--mount",
            "sh",
            "-c",
            MOUNT,
            "visual-namespace",
        ])
        .arg(backing)
        .arg(temp);
    command
}

#[test]
fn namespace_keeps_the_same_path_private_without_removing_host_files() {
    let temp = fs::canonicalize(env::temp_dir()).unwrap();
    let backing = tempfile::tempdir_in(&temp).unwrap();
    let backing = fs::canonicalize(backing.path()).unwrap();
    let marker = tempfile::NamedTempFile::new_in(&temp).unwrap();
    let name = marker.path().file_name().unwrap();
    fs::write(marker.path(), "host").unwrap();
    let result = namespace(&backing, &temp)
        .args(["sh", "-c", "printf private > \"$1\"", "namespace-probe"])
        .arg(marker.path())
        .output();
    match result {
        Ok(output) if output.status.success() => {
            assert_eq!(fs::read_to_string(backing.join(name)).unwrap(), "private");
            assert_eq!(fs::read_to_string(marker.path()).unwrap(), "host");
        }
        result => eprintln!("Namespace unavailable on this host: {result:?}"),
    }
}
