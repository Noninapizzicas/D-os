//! Gestión de trabajos de crawl en memoria, con streaming por WebSocket.

use std::collections::HashMap;
use std::sync::Mutex;

use serde::Serialize;
use tokio::sync::broadcast;

use crate::dto::{JobResult, JobState, JobStatus};

/// Evento emitido durante la ejecución de un trabajo (se serializa a JSON y
/// se envía por el WebSocket).
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "event", rename_all = "lowercase")]
pub enum StreamEvent {
    /// El trabajo ha comenzado.
    Started { id: String },
    /// Se ha procesado una página.
    Page {
        url: String,
        ok: bool,
        completed: usize,
    },
    /// El trabajo ha terminado con éxito.
    Done { completed: usize },
    /// El trabajo ha fallado.
    Failed { error: String },
}

/// Un trabajo de crawl vivo.
pub struct Job {
    id: String,
    status: Mutex<JobStatus>,
    result: Mutex<Option<JobResult>>,
    tx: broadcast::Sender<StreamEvent>,
}

impl Job {
    fn new(id: String) -> Self {
        let (tx, _) = broadcast::channel(256);
        Self {
            status: Mutex::new(JobStatus {
                id: id.clone(),
                state: JobState::Queued,
                completed: 0,
                error: None,
            }),
            result: Mutex::new(None),
            tx,
            id,
        }
    }

    /// Identificador del trabajo.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Devuelve una copia del estado actual.
    pub fn status(&self) -> JobStatus {
        self.status.lock().unwrap().clone()
    }

    /// Devuelve el resultado, si ya está disponible.
    pub fn result(&self) -> Option<JobResult> {
        self.result.lock().unwrap().clone()
    }

    /// Suscribe un receptor al flujo de eventos del trabajo.
    pub fn subscribe(&self) -> broadcast::Receiver<StreamEvent> {
        self.tx.subscribe()
    }

    /// Emite un evento a los suscriptores (ignora si no hay ninguno).
    pub fn emit(&self, event: StreamEvent) {
        let _ = self.tx.send(event);
    }

    /// Marca el trabajo como en ejecución.
    pub fn mark_running(&self) {
        let mut s = self.status.lock().unwrap();
        s.state = JobState::Running;
    }

    /// Actualiza el contador de páginas completadas.
    pub fn set_completed(&self, completed: usize) {
        self.status.lock().unwrap().completed = completed;
    }

    /// Marca el trabajo como terminado con éxito y guarda el resultado.
    pub fn finish(&self, result: JobResult) {
        {
            let mut s = self.status.lock().unwrap();
            s.state = JobState::Done;
            s.completed = result.pages.len();
        }
        self.emit(StreamEvent::Done {
            completed: result.pages.len(),
        });
        *self.result.lock().unwrap() = Some(result);
    }

    /// Marca el trabajo como fallido.
    pub fn fail(&self, error: impl Into<String>) {
        let error = error.into();
        {
            let mut s = self.status.lock().unwrap();
            s.state = JobState::Failed;
            s.error = Some(error.clone());
        }
        self.emit(StreamEvent::Failed { error });
    }
}

/// Registro en memoria de todos los trabajos.
#[derive(Default)]
pub struct JobManager {
    jobs: Mutex<HashMap<String, std::sync::Arc<Job>>>,
}

impl JobManager {
    /// Crea un gestor vacío.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registra un trabajo nuevo con el `id` dado y lo devuelve.
    pub fn create(&self, id: String) -> std::sync::Arc<Job> {
        let job = std::sync::Arc::new(Job::new(id.clone()));
        self.jobs.lock().unwrap().insert(id, job.clone());
        job
    }

    /// Recupera un trabajo por su identificador.
    pub fn get(&self, id: &str) -> Option<std::sync::Arc<Job>> {
        self.jobs.lock().unwrap().get(id).cloned()
    }
}
