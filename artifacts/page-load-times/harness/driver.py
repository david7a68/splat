"""Run the page tour in a nested headless KWin with the real GPU, and summarise the logs.

Usage: driver.py <scratch folder> <debug|release> <cold repeats> <visit processes> <visit rounds>
Data is a copy. The child has no network and no D-Bus, so it cannot sign in to D&D Beyond.
"""
import json
import os
import pathlib
import shutil
import statistics
import subprocess
import sys
import time

P = pathlib.Path(sys.argv[1]).resolve()
PROFILE, COLD, VISITS, ROUNDS = sys.argv[2], int(sys.argv[3]), int(sys.argv[4]), int(sys.argv[5])
BIN = P / "target" / PROFILE / "splat"
HOME = pathlib.Path.home()
SOCKET = "splat-perf"
LOGS = P / "logs" / PROFILE
LOGS.mkdir(parents=True, exist_ok=True)
APP_ENV = dict(
    line.split("=", 1) for line in (P / "app-env.txt").read_text().splitlines() if "=" in line
)


def prepare_real():
    root = P / "data" / "real"
    if root.exists():
        shutil.rmtree(root)
    (root / "config" / "splat").mkdir(parents=True)
    (root / "data" / "splat").mkdir(parents=True)
    settings = json.loads((HOME / ".config/splat/settings.json").read_text())
    for index, campaign in enumerate(settings["campaigns"]):
        copy = root / "campaigns" / str(index)
        shutil.copytree(campaign["path"], copy)
        campaign["path"] = str(copy)
    for index, module in enumerate(settings.get("modules", [])):
        copy = root / "modules" / str(index)
        shutil.copytree(module["path"], copy)
        module["path"] = str(copy)
    (root / "config/splat/settings.json").write_text(json.dumps(settings))
    shutil.copytree(HOME / ".local/share/splat/characters", root / "data/splat/characters")
    return root, settings["active_campaign_id"], None


def prepare_fixture():
    world = P / "tmp" / "fixture"
    root = P / "data" / "fixture"
    if root.exists():
        shutil.rmtree(root)
    (root / "config" / "splat").mkdir(parents=True)
    (root / "data" / "splat").mkdir(parents=True)
    shutil.copy(world / "world/settings/settings.json", root / "config/splat/settings.json")
    shutil.copytree(world / "character-cache", root / "data/splat/characters")
    settings = json.loads((root / "config/splat/settings.json").read_text())
    active = settings["active_campaign_id"]
    needle = "Siege of Emberfall"
    run = None
    for path in (world / "world").rglob("*.json"):
        if path.parent.name == "Encounter Runs" and needle in path.read_text():
            run = path
    assert run, "no large run in the fixture"
    return root, active, run


def busy_cores(seconds=1.0):
    def sample():
        fields = [int(v) for v in open("/proc/stat").readline().split()[1:]]
        return sum(fields) - fields[3] - fields[4], sum(fields)
    used, total = sample()
    time.sleep(seconds)
    used2, total2 = sample()
    return (used2 - used) / (total2 - total) * os.cpu_count()


WAITED = {"seconds": 0.0, "before_run_cores": []}


def wait_quiet():
    """Other work on this machine, such as another thread's visual suite, spoils a sample."""
    started = time.time()
    while (cores := busy_cores()) > 2.5:
        if time.time() - started > 3000:
            raise RuntimeError(f"the machine stayed busy: {cores:.1f} cores")
        time.sleep(4)
    WAITED["seconds"] += time.time() - started
    WAITED["before_run_cores"].append(round(cores, 2))


def run_once(root, mode, target, page, active, run_path, rounds, label, evict=False):
    wait_quiet()
    state = {"page": page, "open_run": None}
    if mode == "cold" and target == "Run":
        state["open_run"] = {"campaign": active, "path": str(run_path), "synced": []}
    (root / "config/splat/ui-state.json").write_text(json.dumps(state))
    log = LOGS / f"{label}.jsonl"
    log.unlink(missing_ok=True)
    env = {
        "HOME": str(HOME),
        "PATH": os.environ["PATH"],
        "XDG_RUNTIME_DIR": os.environ["XDG_RUNTIME_DIR"],
        "WAYLAND_DISPLAY": SOCKET,
        "XDG_CONFIG_HOME": str(root / "config"),
        "XDG_DATA_HOME": str(root / "data"),
        "DBUS_SESSION_BUS_ADDRESS": "unix:path=/nonexistent",
        "SPLAT_PERF_LOG": str(log),
        "SPLAT_PERF_MODE": mode,
        "SPLAT_PERF_TARGET": target,
        "SPLAT_PERF_BASE": "Encounters" if target == "Settings" else "Settings",
        "SPLAT_PERF_ROUNDS": str(rounds),
        "SPLAT_PERF_RUN": str(run_path or ""),
        **APP_ENV,
    }
    if evict:
        files = [BIN] + [p for p in root.rglob("*") if p.is_file()]
        for path in files:
            fd = os.open(path, os.O_RDONLY)
            os.fsync(fd)
            os.posix_fadvise(fd, 0, 0, os.POSIX_FADV_DONTNEED)
            os.close(fd)
    spawned = time.time_ns()
    done = subprocess.run(
        ["unshare", "-Unc", str(BIN)],
        env=env,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.PIPE,
        timeout=240,
    )
    if done.returncode != 0 or not log.exists():
        raise RuntimeError(f"{label}: exit {done.returncode}: {done.stderr.decode()[-1500:]}")
    lines = [json.loads(line) for line in log.read_text().splitlines()]
    assert "llvmpipe" not in lines[0]["renderer"], f"{label}: software rendering"
    return spawned, lines[0], [l for l in lines[1:] if "f" in l], any("timeout" in l for l in lines)


def frames_of(frames):
    """Group updates into real frames. A second egui pass belongs to the same frame."""
    grouped = []
    for frame in frames:
        if frame["pass"] == 0:
            grouped.append(dict(frame))
        elif grouped:
            grouped[-1]["u"] += frame["u"]
            grouped[-1]["b"] = frame["b"]
            grouped[-1]["passes"] = grouped[-1].get("passes", 1) + 1
    return grouped


def ms(us):
    return us / 1000


def visit_metrics(frames):
    """`frames` is one page visit, plus the frame after it that gives the last frame's cost."""
    start = frames[0]["t"]
    ready = next(i for i, frame in enumerate(frames) if not frame["b"])
    costs = [frames[i + 1]["c"] for i in range(len(frames) - 1) if frames[i + 1]["c"] >= 0]
    idle = costs[-8:]
    return {
        "first_frame_update_ms": ms(frames[0]["u"]),
        "first_frame_total_ms": ms(frames[1]["c"]),
        "first_frame_shown_ms": ms(frames[1]["t"] - start),
        "ready_ms": ms(frames[ready + 1]["t"] - start),
        "longest_frame_ms": ms(max(costs[:ready + 2])),
        "idle_frame_ms": ms(statistics.median(idle)),
        "frames_to_ready": ready + 1,
        "extra_passes": sum(frame.get("passes", 1) - 1 for frame in frames[:ready + 1]),
    }


# The background load that each page's own content waits for.
OWN_LOAD = {"Encounters": "e", "Party": "p", "Library": "L", "Campaign": "c", "Settings": "s", "Run": "e"}


def cold_metrics(spawned, head, frames, target):
    grouped = frames_of(frames)
    own = OWN_LOAD[target]
    own_ready = next(i for i, f in enumerate(grouped) if own not in f["b"])
    offset = ms((head["main_epoch_ns"] - spawned) / 1000)
    return {
        "exec_to_main_ms": offset,
        "window_ready_ms": offset + ms(head["creator_us"]),
        "first_update_ms": offset + ms(grouped[0]["t"]),
        "first_frame_shown_ms": offset + ms(grouped[1]["t"]),
        "own_ready_ms": offset + ms(grouped[own_ready + 1]["t"]),
        "all_loads_done_ms": offset + ms(grouped[next(i for i, f in enumerate(grouped) if not f["b"]) + 1]["t"]),
        "longest_frame_ms": ms(max(f["c"] for f in grouped[1:])),
        "first_frame_update_ms": ms(grouped[0]["u"]),
        "first_frame_total_ms": ms(grouped[1]["c"]),
        "idle_frame_ms": ms(statistics.median([f["c"] for f in grouped[-8:]])),
        "busy": "".join(sorted({c for f in grouped for c in f["b"]})),
    }


def split_visits(frames):
    grouped = frames_of(frames)
    rounds = {}
    for index, frame in enumerate(grouped):
        if frame["ph"] == "visit":
            rounds.setdefault(frame["r"], []).append(index)
    return {
        round_: grouped[indexes[0] : indexes[-1] + 2]
        for round_, indexes in rounds.items()
    }


def summary(rows):
    out = {"n": len(rows)}
    for key in rows[0]:
        values = [row[key] for row in rows]
        if isinstance(values[0], str):
            out[key] = sorted(set(values))
        else:
            values = sorted(values)
            out[key] = {
                "median": round(statistics.median(values), 2),
                "min": round(values[0], 2),
                "max": round(values[-1], 2),
            }
    return out


def main():
    kwin_log = open(P / "kwin.log", "w")
    # The inherited environment. Without it, the app falls back to software rendering here.
    kwin = subprocess.Popen(
        ["kwin_wayland", "--virtual", "--socket", SOCKET, "--width", "1920", "--height", "1080",
         "--no-lockscreen", "--no-global-shortcuts", "--no-kactivities"],
        env=dict(os.environ),
        stdout=kwin_log,
        stderr=kwin_log,
    )
    socket = pathlib.Path(os.environ["XDG_RUNTIME_DIR"]) / SOCKET
    for _ in range(100):
        if socket.exists():
            break
        time.sleep(0.1)
    assert socket.exists(), "KWin did not open its socket"
    time.sleep(3)
    results = {"profile": PROFILE, "datasets": {}}
    try:
        for dataset, prepare in [("real", prepare_real), ("fixture", prepare_fixture)]:
            root, active, run_path = prepare()
            cold_pages = [("Encounters", "Encounters"), ("Party", "Party"), ("Library", "Library"),
                          ("Campaign", "Campaign"), ("Settings", "Settings")]
            visit_targets = ["Encounters", "Party", "Library", "Campaign", "Campaign:Rules",
                             "Campaign:Link", "Settings", "Lookup"]
            if run_path:
                cold_pages.append(("Run", "Encounters"))
                visit_targets.append("Run")
            data = {"cold": {}, "first_visit": {}, "revisit": {}, "disk_cold": {}}
            for target, page in cold_pages:
                rows = []
                for repeat in range(COLD + 1):
                    label = f"{dataset}-cold-{target}-{repeat}".replace(":", "_")
                    spawned, head, frames, timeout = run_once(root, "cold", target, page, active, run_path, 1, label)
                    assert not timeout, label
                    results["renderer"] = head["renderer"]
                    if repeat:  # The first run warms the disk cache.
                        rows.append(cold_metrics(spawned, head, frames, target))
                data["cold"][target] = summary(rows)
                print(dataset, "cold", target, data["cold"][target]["own_ready_ms"]["median"], flush=True)
            if dataset == "real":
                rows = []
                for repeat in range(max(3, COLD // 2)):
                    label = f"{dataset}-diskcold-{repeat}"
                    spawned, head, frames, timeout = run_once(root, "cold", "Library", "Library", active, run_path, 1, label, evict=True)
                    assert not timeout, label
                    rows.append(cold_metrics(spawned, head, frames, "Library"))
                data["disk_cold"]["Library"] = summary(rows)
            for target in visit_targets:
                first, later = [], []
                for repeat in range(VISITS):
                    label = f"{dataset}-visit-{target}-{repeat}".replace(":", "_")
                    _, _, frames, timeout = run_once(root, "visit", target, "Encounters" if target == "Settings" else "Settings", active, run_path, ROUNDS, label)
                    assert not timeout, label
                    visits = split_visits(frames)
                    first.append(visit_metrics(visits[0]))
                    later.extend(visit_metrics(visits[r]) for r in sorted(visits) if r > 0)
                data["first_visit"][target] = summary(first)
                data["revisit"][target] = summary(later)
                print(dataset, "visit", target, data["first_visit"][target]["ready_ms"]["median"],
                      data["revisit"][target]["ready_ms"]["median"], flush=True)
            results["datasets"][dataset] = data
            results["waited_seconds"] = round(WAITED["seconds"])
            results["before_run_cores_max"] = max(WAITED["before_run_cores"])
            results["before_run_cores_median"] = statistics.median(WAITED["before_run_cores"])
            (P / f"results-{PROFILE}.json").write_text(json.dumps(results, indent=1))
    finally:
        kwin.terminate()
        kwin.wait(timeout=10)
    print("done", flush=True)


main()
