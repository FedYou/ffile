use notify::{RecommendedWatcher, Watcher};
use std::sync::{Mutex, MutexGuard, OnceLock};
use tokio::sync::Mutex as AsyncMutex;

static WATCHER: OnceLock<Mutex<RecommendedWatcher>> = OnceLock::new();
static WATCHER_CHANNEL: OnceLock<WatcherChannel> = OnceLock::new();

pub use notify::EventKind;

pub struct WatcherChannel {
    tx: tokio::sync::mpsc::Sender<notify::Event>,
    rx: AsyncMutex<tokio::sync::mpsc::Receiver<notify::Event>>,
}

pub fn init_watcher() {
    let (watcher_tx, watcher_rx) = tokio::sync::mpsc::channel::<notify::Event>(100);

    let (tx, rx) = std::sync::mpsc::channel();

    let watcher = loop {
        match RecommendedWatcher::new(tx.clone(), notify::Config::default()) {
            Ok(w) => break w,
            _ => {}
        }
    };

    let watcher_tx_clone = watcher_tx.clone();
    std::thread::spawn(move || {
        while let Ok(res) = rx.recv() {
            if let Ok(event) = res {
                let _ = watcher_tx_clone.blocking_send(event);
            }
        }
    });

    let _ = WATCHER_CHANNEL.set(WatcherChannel {
        tx: watcher_tx,
        rx: AsyncMutex::new(watcher_rx),
    });

    let _ = WATCHER.set(Mutex::new(watcher));
}

pub fn get_watcher() -> MutexGuard<'static, RecommendedWatcher> {
    WATCHER
        .get()
        .expect("Watcher not initialized")
        .lock()
        .expect("Watcher mutex poisoned")
}

pub fn get_tx() -> &'static tokio::sync::mpsc::Sender<notify::Event> {
    &WATCHER_CHANNEL
        .get()
        .expect("Watcher Channel not initialized")
        .tx
}

pub fn get_rx() -> &'static AsyncMutex<tokio::sync::mpsc::Receiver<notify::Event>> {
    &WATCHER_CHANNEL
        .get()
        .expect("Watcher Channel not initialized")
        .rx
}
