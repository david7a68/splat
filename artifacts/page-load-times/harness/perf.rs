//! Page load timing tour for the real app, in a real window. Off unless SPLAT_PERF_LOG is set.
//!
//! `cold`: start on the page that ui-state.json names, and log until the content is ready.
//! `visit`: start on a base page, wait for all background loads to end, then go to the
//! target page and back for a number of rounds. Round 0 is the first visit.
//! Each update adds a line to the log. The log is written to disk only at exit.
use super::*;
use eframe::glow::HasContext;
use std::{
    cell::RefCell,
    fmt::Write as _,
    sync::OnceLock,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

const IDLE_FRAMES: u32 = 12;
const LEAVE_FRAMES: u32 = 6;
const WARMUP_FRAMES: u32 = 30;
const WARMUP_SECONDS: f64 = 1.5;
const TIMEOUT_SECONDS: f64 = 180.0;

struct Marks {
    main: Instant,
    epoch_ns: u128,
}
static MARKS: OnceLock<Marks> = OnceLock::new();
static CREATOR_US: OnceLock<u128> = OnceLock::new();
static APP_US: OnceLock<u128> = OnceLock::new();
static RENDERER: OnceLock<String> = OnceLock::new();

pub fn enabled() -> bool {
    std::env::var_os("SPLAT_PERF_LOG").is_some()
}

pub fn mark_main() {
    let _ = MARKS.set(Marks {
        main: Instant::now(),
        epoch_ns: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos(),
    });
}

fn since_main() -> u128 {
    MARKS.get().map_or(0, |m| m.main.elapsed().as_micros())
}

pub fn mark_creator(cc: &eframe::CreationContext<'_>) {
    let _ = CREATOR_US.set(since_main());
    if let Some(gl) = &cc.gl {
        let _ = RENDERER.set(unsafe { gl.get_parameter_string(eframe::glow::RENDERER) });
    }
}

pub fn mark_app() {
    let _ = APP_US.set(since_main());
}

#[derive(Clone, Copy, PartialEq)]
enum Phase {
    Warmup,
    Cold,
    Visit,
    Leave,
}

impl Phase {
    fn name(self) -> &'static str {
        match self {
            Self::Warmup => "warmup",
            Self::Cold => "cold",
            Self::Visit => "visit",
            Self::Leave => "leave",
        }
    }
}

struct Tour {
    log: String,
    target: String,
    base: Page,
    rounds: u32,
    round: u32,
    phase: Phase,
    quiet: u32,
    frame: u64,
    update_begin: Instant,
    begin_us: u128,
    prev_cpu_us: i64,
    switch_due: bool,
    leave_due: bool,
    done: bool,
}

thread_local! {
    static TOUR: RefCell<Option<Tour>> = const { RefCell::new(None) };
}

fn var(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.is_empty())
}

fn new_tour() -> Tour {
    let mode = var("SPLAT_PERF_MODE").unwrap_or_else(|| "cold".to_owned());
    let base = var("SPLAT_PERF_BASE")
        .as_deref()
        .and_then(Page::from_name)
        .unwrap_or(Page::Settings);
    let mut log = String::new();
    let marks = MARKS.get().expect("mark_main ran");
    let _ = writeln!(
        log,
        "{{\"main_epoch_ns\":{},\"creator_us\":{},\"app_us\":{},\"renderer\":{:?}}}",
        marks.epoch_ns,
        CREATOR_US.get().copied().unwrap_or(0),
        APP_US.get().copied().unwrap_or(0),
        RENDERER.get().map_or("unknown", String::as_str),
    );
    Tour {
        log,
        target: var("SPLAT_PERF_TARGET").unwrap_or_default(),
        base,
        rounds: var("SPLAT_PERF_ROUNDS").and_then(|v| v.parse().ok()).unwrap_or(10),
        round: 0,
        phase: if mode == "visit" { Phase::Warmup } else { Phase::Cold },
        quiet: 0,
        frame: 0,
        update_begin: Instant::now(),
        begin_us: 0,
        prev_cpu_us: -1,
        switch_due: false,
        leave_due: false,
        done: false,
    }
}

fn go(app: &mut Splat, target: &str) {
    match target {
        "Lookup" => {
            if !app.lookup.open {
                app.lookup.toggle();
            }
        }
        "Run" => {
            if !app.encounters.has_run() {
                let path = var("SPLAT_PERF_RUN").expect("SPLAT_PERF_RUN for the Run target");
                let id = app.campaigns.active_campaign().expect("active campaign").id;
                app.encounters.restore_run(std::path::Path::new(&path), id, &[]);
            }
            app.page = Page::Run;
        }
        "Campaign:Rules" => {
            app.campaign_tab = CampaignTab::RunSettings;
            app.page = Page::Campaign;
        }
        "Campaign:Link" => {
            app.campaign_tab = CampaignTab::Link;
            app.page = Page::Campaign;
        }
        name => app.page = Page::from_name(name).expect("known page"),
    }
}

fn leave(app: &mut Splat, target: &str, base: Page) {
    if target == "Lookup" {
        if app.lookup.open {
            app.lookup.toggle();
        }
    } else {
        app.page = base;
    }
}

fn busy_flags(app: &Splat) -> String {
    let mut flags = String::new();
    for (busy, letter) in [
        (app.settings.busy(), 's'),
        (app.library.busy(), 'L'),
        (app.campaign_content.busy(), 'c'),
        (app.encounters.busy(), 'e'),
        (app.party.busy(), 'p'),
        (app.lookup.busy(), 'k'),
    ] {
        if busy {
            flags.push(letter);
        }
    }
    flags
}

pub fn begin(app: &mut Splat, ctx: &egui::Context, frame: &eframe::Frame) {
    if !enabled() {
        return;
    }
    let pass = ctx.current_pass_index();
    TOUR.with_borrow_mut(|slot| {
        let tour = slot.get_or_insert_with(new_tour);
        if pass == 0 {
            if tour.switch_due {
                tour.switch_due = false;
                tour.phase = Phase::Visit;
                tour.quiet = 0;
                let target = tour.target.clone();
                go(app, &target);
            } else if tour.leave_due {
                tour.leave_due = false;
                tour.phase = Phase::Leave;
                tour.quiet = 0;
                let target = tour.target.clone();
                leave(app, &target, tour.base);
            }
        }
        tour.prev_cpu_us = frame
            .info()
            .cpu_usage
            .map_or(-1, |seconds| (seconds as f64 * 1e6) as i64);
        tour.begin_us = since_main();
        tour.update_begin = Instant::now();
    });
}

pub fn end(app: &mut Splat, ctx: &egui::Context) {
    if !enabled() {
        return;
    }
    let update_us = TOUR.with_borrow(|slot| slot.as_ref().map(|t| t.update_begin.elapsed().as_micros()));
    let flags = busy_flags(app);
    let pass = ctx.current_pass_index();
    let finish = TOUR.with_borrow_mut(|slot| {
        let tour = slot.as_mut().expect("begin ran");
        let _ = writeln!(
            tour.log,
            "{{\"f\":{},\"t\":{},\"u\":{},\"c\":{},\"ph\":\"{}\",\"r\":{},\"b\":\"{}\",\"pass\":{},\"page\":\"{}\"}}",
            tour.frame,
            tour.begin_us,
            update_us.unwrap_or(0),
            tour.prev_cpu_us,
            tour.phase.name(),
            tour.round,
            flags,
            pass,
            app.page.name(),
        );
        tour.frame += 1;
        tour.quiet = if flags.is_empty() { tour.quiet + 1 } else { 0 };
        let elapsed = since_main() as f64 / 1e6;
        match tour.phase {
            Phase::Warmup => {
                if tour.quiet >= WARMUP_FRAMES && elapsed >= WARMUP_SECONDS {
                    tour.switch_due = true;
                }
            }
            Phase::Cold => {
                if tour.quiet >= IDLE_FRAMES {
                    tour.done = true;
                }
            }
            Phase::Visit => {
                if tour.quiet >= IDLE_FRAMES {
                    tour.leave_due = true;
                }
            }
            Phase::Leave => {
                if tour.quiet >= LEAVE_FRAMES {
                    tour.round += 1;
                    if tour.round >= tour.rounds {
                        tour.done = true;
                    } else {
                        tour.switch_due = true;
                    }
                }
            }
        }
        if elapsed >= TIMEOUT_SECONDS {
            let _ = writeln!(tour.log, "{{\"timeout\":true,\"flags\":\"{flags}\"}}");
            tour.done = true;
        }
        tour.done.then(|| std::mem::take(&mut tour.log))
    });
    if let Some(log) = finish {
        std::fs::write(var("SPLAT_PERF_LOG").expect("log path"), log).expect("write log");
        std::process::exit(0);
    }
    ctx.request_repaint();
}
