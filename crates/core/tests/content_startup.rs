use splat_core::content::{builtin, definitions};
use std::{
    sync::{Arc, Barrier, mpsc},
    thread,
    time::Duration,
};

#[test]
fn concurrent_builtin_and_definition_loads_finish_with_one_worker() {
    // A separate test binary keeps both lazy catalogues cold.
    rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build_global()
        .unwrap();
    let start = Arc::new(Barrier::new(2));
    let (finished, results) = mpsc::channel();
    for load in [builtin::entries, || definitions::entries().unwrap()] {
        let start = start.clone();
        let finished = finished.clone();
        thread::spawn(move || {
            start.wait();
            assert!(!load().is_empty());
            finished.send(()).unwrap();
        });
    }
    for _ in 0..2 {
        results
            .recv_timeout(Duration::from_secs(30))
            .expect("content loaders must not block the worker they need");
    }
}
