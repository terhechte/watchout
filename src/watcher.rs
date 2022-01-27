use druid::{ExtEventSink, Target};

use crossbeam_channel::{unbounded, Sender};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};

use std::path::PathBuf;

/// Watch paths. Each `path` will be watched recursively
pub fn watch(paths: Vec<PathBuf>, sink: ExtEventSink) -> notify::Result<()> {
    std::thread::spawn(move || {
        let (tx, rx) = unbounded();
        let mut watcher: RecommendedWatcher = Watcher::new(tx).unwrap();

        for path in paths {
            watcher.watch(&path, RecursiveMode::Recursive).unwrap();
        }

        let mut last_sender: Option<Sender<bool>> = None;
        loop {
            match rx.recv() {
                Ok(event) => match event {
                    Ok(ev) => {
                        if ev.kind.is_create() || ev.kind.is_modify() {
                            last_sender.map(|s| s.send(true));
                            last_sender = Some(debounce(ev.paths, sink.clone()));
                        }
                    }
                    Err(e) => {
                        tracing::error!("Watch error {:?}", &e);
                    }
                },
                Err(e) => {
                    tracing::error!("watch error: {:?}", e);
                    ()
                }
            }
        }
    });

    Ok(())
}

// A simple debouncer
fn debounce(paths: Vec<PathBuf>, sink: ExtEventSink) -> Sender<bool> {
    let (s, receiver) = unbounded();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(250));
        match receiver.try_recv() {
            Ok(true) => {
                tracing::debug!("Debouncing ok..");
                return;
            }
            _ => (),
        }
        tracing::debug!("Executing..");
        sink.submit_command(crate::SELECTOR_EVENT, paths, Target::Auto)
            .expect("Expect submit command to work");
    });
    s
}
