use crate::file_system::copy::copy;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

pub type ProcessId = u64;

#[derive(Debug, Clone)]
pub enum ProcessKind {
    Copy,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProcessStatus {
    Running,
    Finished,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct Process {
    pub id: ProcessId,
    pub kind: ProcessKind,

    pub status: ProcessStatus,

    pub progress: Option<f64>,
    pub bytes_done: Option<u64>,
    pub bytes_total: Option<u64>,

    pub speed: Option<f64>,
    pub eta: Option<Duration>,
    pub message: Option<String>,

    pub started_at: Instant,
    pub duration: Option<Duration>,

    /// Flag cooperativo de cancelación: el worker lo revisa entre
    /// operaciones y detiene la copia.
    pub cancel_flag: Arc<AtomicBool>,
}

pub enum ProcessEvent {
    Progress {
        id: ProcessId,
        progress: Option<f64>,
        bytes_done: Option<u64>,
        bytes_total: Option<u64>,
        speed: Option<f64>,
        eta: Option<Duration>,
        message: Option<String>,
    },
    Finished {
        id: ProcessId,
    },
    Failed {
        id: ProcessId,
        error: String,
    },
}

pub struct ProcessManager {
    pub processes: HashMap<ProcessId, Process>,
    pub next_id: ProcessId,
    pub task: HashMap<ProcessId, tokio::task::JoinHandle<()>>,
    pub tx: tokio::sync::mpsc::Sender<ProcessEvent>,
    pub rx: tokio::sync::mpsc::Receiver<ProcessEvent>,
}

impl ProcessManager {
    pub fn new() -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        Self {
            processes: HashMap::new(),
            task: HashMap::new(),
            next_id: 0,
            tx,
            rx,
        }
    }

    pub fn handle_event(&mut self, event: ProcessEvent) {
        match event {
            ProcessEvent::Progress {
                id,
                progress,
                bytes_done,
                bytes_total,
                speed,
                eta,
                message,
            } => {
                if let Some(process) = self.processes.get_mut(&id) {
                    process.progress = progress;
                    process.bytes_done = bytes_done;
                    process.bytes_total = bytes_total;
                    process.speed = speed;
                    process.eta = eta;
                    process.message = message;
                }
            }
            ProcessEvent::Failed { id, error } => {
                if let Some(process) = self.processes.get_mut(&id)
                    && process.status != ProcessStatus::Cancelled
                {
                    process.status = ProcessStatus::Failed;
                    process.progress = None;
                    process.bytes_done = None;
                    process.bytes_total = None;
                    process.speed = None;
                    process.eta = None;
                    process.message = Some(error);
                    process.duration = Some(process.started_at.elapsed())
                }

                self.task.remove(&id);
            }
            ProcessEvent::Finished { id } => {
                if let Some(process) = self.processes.get_mut(&id)
                    && process.status != ProcessStatus::Cancelled
                {
                    process.status = ProcessStatus::Finished;
                    process.progress = None;
                    process.bytes_done = None;
                    process.bytes_total = None;
                    process.speed = None;
                    process.eta = None;
                    process.message = None;
                    process.duration = Some(process.started_at.elapsed());
                }
                self.task.remove(&id);
            }
        }
    }

    pub fn cancel(&mut self, id: ProcessId) {
        if let Some(process) = self.processes.get_mut(&id)
            && process.status == ProcessStatus::Running
        {
            process.status = ProcessStatus::Cancelled;
            process.cancel_flag.store(true, Ordering::Relaxed);
            process.duration = Some(process.started_at.elapsed());
        }
    }

    pub fn spawn_copy(&mut self, src: Vec<PathBuf>, destination: PathBuf) -> ProcessId {
        let id = self.next_id;
        self.next_id += 1;

        let cancel_flag = Arc::new(AtomicBool::new(false));

        let process = Process {
            id,
            kind: ProcessKind::Copy,
            status: ProcessStatus::Running,

            progress: None,
            bytes_done: None,
            bytes_total: None,
            speed: None,
            eta: None,
            message: None,
            started_at: std::time::Instant::now(),
            duration: None,
            cancel_flag: cancel_flag.clone(),
        };

        let tx = self.tx.clone();

        let task = tokio::spawn(async move {
            let event = match copy(src, destination, tx.clone(), &cancel_flag, id).await {
                Ok(()) => ProcessEvent::Finished { id },
                Err(err) => ProcessEvent::Failed {
                    id,
                    error: err.to_string(),
                },
            };
            let _ = tx.send(event).await;
        });

        self.processes.insert(id, process);
        self.task.insert(id, task);

        id
    }
}
