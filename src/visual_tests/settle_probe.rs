//! Exercise timeout exits without a renderer or a three-minute wait.
use super::{
    driver::{Driver, check_settle_deadline},
    fixture::{Seed, World},
    isolation, slots,
};
use std::{
    env, fs,
    os::unix::process::CommandExt,
    path::{Path, PathBuf},
    process::{Child, Command},
    sync::mpsc,
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

const TEST: &str =
    "visual_tests::settle_probe::expired_settle_exits_child_and_parent_and_releases_slot";
const ROLE: &str = "SPLAT_SETTLE_PROBE_ROLE";
const ROOT: &str = "SPLAT_SETTLE_PROBE_ROOT";
// The first frame must still wait for the remaining quiet frames. One nanosecond
// expires that real wait deterministically, without a slow or stuck app worker.
const DEADLINE: Duration = Duration::from_nanos(1);
const EXIT_BOUND: Duration = Duration::from_secs(5);

fn timestamp() -> Duration {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
}

fn record_time(path: &Path) {
    fs::write(path, timestamp().as_nanos().to_string()).unwrap();
}

fn read_time(path: &Path) -> Duration {
    Duration::from_nanos(fs::read_to_string(path).unwrap().parse().unwrap())
}

fn command(role: &str, root: &Path) -> Command {
    let mut command = Command::new(env::current_exe().unwrap());
    command
        .args(["--exact", TEST, "--nocapture"])
        .env(ROLE, role)
        .env(ROOT, root)
        .env_remove("SPLAT_VISUAL_NAMESPACE_CHILD");
    command
}

// On a broken exit path, reap both subprocesses rather than leaving a hung test.
struct ProbeProcess(Child);

impl Drop for ProbeProcess {
    fn drop(&mut self) {
        // The parent starts a new process group; its child inherits that group.
        unsafe { libc::kill(-(self.0.id() as i32), libc::SIGKILL) };
        self.0.wait().unwrap();
    }
}

struct BlockingCleanup(mpsc::Sender<()>);

impl Drop for BlockingCleanup {
    fn drop(&mut self) {
        let _ = self.0.send(());
        loop {
            thread::park();
        }
    }
}

fn blocked_child(root: &Path) {
    assert!(isolation::is_child());
    thread::scope(|scope| {
        let (entered, cleanup_entered) = mpsc::channel();
        // An earlier ordered join is already stuck in sibling cleanup.
        let sibling = scope.spawn(|| drop(BlockingCleanup(entered)));
        cleanup_entered.recv_timeout(EXIT_BOUND).unwrap();
        scope.spawn(move || {
            let world = World::build(
                &root.join("world"),
                &root.join("cache"),
                "blocked-settle-probe",
                Seed::Empty,
            )
            .unwrap();
            let mut driver = Driver::new(&world, "blocked-settle-probe", root, None, false);
            driver.settle_timeout = DEADLINE;
            // The timed-out scenario would also hang while unwinding.
            let (entered, _receive) = mpsc::channel();
            let _cleanup = BlockingCleanup(entered);
            record_time(&root.join("expired"));
            driver.settle();
            panic!("the expired settle check returned");
        });
        sibling.join().unwrap();
    });
}

fn suite_parent(root: &Path) {
    assert!(!isolation::is_child());
    let _slot = slots::acquire_in(root, 1).unwrap();
    assert!(slots::try_acquire(root, 1).unwrap().is_none());
    let observed = root.join("child-exited");
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        // run_child has received the child's unsuccessful status at this point.
        record_time(&observed);
        original_hook(info);
    }));
    isolation::run_child(command("child", root));
    panic!("the failed suite child was accepted");
}

#[test]
fn expired_settle_exits_child_and_parent_and_releases_slot() {
    if let Ok(role) = env::var(ROLE) {
        let root = PathBuf::from(env::var_os(ROOT).unwrap());
        match role.as_str() {
            "parent" => suite_parent(&root),
            "child" => blocked_child(&root),
            _ => panic!("unknown probe role"),
        }
        return;
    }
    let output = env::current_dir().unwrap().join("artifacts/screenshots");
    fs::create_dir_all(&output).unwrap();
    let root = tempfile::Builder::new()
        .prefix("settle-probe-")
        .tempdir_in(output)
        .unwrap();
    let log = fs::File::create(root.path().join("probe.log")).unwrap();
    let mut process = ProbeProcess(
        command("parent", root.path())
            .process_group(0)
            .stdout(log.try_clone().unwrap())
            .stderr(log)
            .spawn()
            .unwrap(),
    );
    let started = Instant::now();
    let status = loop {
        if let Some(status) = process.0.try_wait().unwrap() {
            break status;
        }
        assert!(
            started.elapsed() < EXIT_BOUND,
            "H-exit-bound: suite parent hung past {EXIT_BOUND:?}; log:\n{}",
            fs::read_to_string(root.path().join("probe.log")).unwrap(),
        );
        thread::sleep(Duration::from_millis(5));
    };
    let parent_exited = timestamp();
    let log = fs::read_to_string(root.path().join("probe.log")).unwrap();
    println!("{log}");
    assert_eq!(status.code(), Some(101), "H-parent-status");
    assert!(
        log.contains("blocked-settle-probe: the app did not settle within 1ns"),
        "H-diagnostic",
    );
    assert!(
        log.contains("isolated visual suite failed: exit status: 101"),
        "H-child-status",
    );
    let expired = read_time(&root.path().join("expired"));
    let child_delay = read_time(&root.path().join("child-exited")) - expired;
    let parent_delay = parent_exited - expired;
    assert!(child_delay < EXIT_BOUND, "H-child-exit-bound");
    assert!(parent_delay < EXIT_BOUND, "H-parent-exit-bound");
    assert!(
        slots::try_acquire(root.path(), 1).unwrap().is_some(),
        "H-slot-released",
    );
    println!(
        "Expired settle probe: child observed exited after {child_delay:?}; parent exited after {parent_delay:?}; slot released",
    );
}

#[test]
fn expired_settle_outside_child_still_panics() {
    assert!(!isolation::is_child());
    check_settle_deadline("not-expired", Instant::now(), Duration::from_secs(180));
    let failure = std::panic::catch_unwind(|| {
        check_settle_deadline("parent-settle", Instant::now(), Duration::ZERO);
    })
    .expect_err("H-parent-panic: the ordinary timeout must remain a panic");
    assert_eq!(
        failure.downcast_ref::<String>().unwrap(),
        "parent-settle: the app did not settle within 0ns",
    );
}
