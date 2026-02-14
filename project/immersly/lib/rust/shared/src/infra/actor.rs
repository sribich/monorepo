use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use railgun::error::Error;
use railgun::error::Location;
use railgun::error::ResultExt as _;
use tokio::runtime::Handle;
use tokio::sync::mpsc;
use tokio::sync::oneshot::error::RecvError;
use tokio::sync::oneshot::{self};
use tokio::task::JoinHandle;
use tokio::task::spawn_blocking;

type FnType =
    Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + Sync + 'static>;

pub trait Task<T = ()>: Send + Sync + 'static {
    fn execute(&self) -> impl Future<Output = T> + Send; // + Sync;
}

pub struct Actor {
    handle: Arc<JoinHandle<()>>,
    tx: mpsc::Sender<FnType>,
}

#[derive(Error)]
pub enum Error {
    // SendError,
    #[error(display("Sender closed before receiving a response."))]
    NoResponse {
        error: RecvError,
        location: Location,
    },
}

/// TODO: Handle draining & stop on shutdown
impl Default for Actor {
    fn default() -> Self {
        Self::new()
    }
}

impl Actor {
    pub fn new() -> Actor {
        let (tx, rx) = mpsc::channel(256);
        let handler = ActorHandler::new(rx);

        // TODO: The spawn_blocking call here should be handled by a separate
        //       "Executor" which can handle serial/parallel and optional
        //       blocking tasks.
        let handle = Arc::new(spawn_blocking(move || {
            handler.run();
        }));

        Self { handle, tx }
    }

    pub async fn send<T: Send + Sync + 'static>(&self, task: impl Task<T>) -> Result<T, Error> {
        let (tx, rx) = oneshot::channel::<T>();

        self.tx
            .send(Box::new(move || {
                Box::pin(async move {
                    tx.send(task.execute().await)
                        .map_err(|e| "SEND ERROR")
                        .unwrap();
                })
            }))
            .await
            .expect("receiver closed");

        rx.await.context(NoResponseContext {})
    }
}

pub struct ActorHandler {
    rx: mpsc::Receiver<FnType>,
}

impl ActorHandler {
    pub fn new(rx: mpsc::Receiver<FnType>) -> Self {
        Self { rx }
    }

    pub fn run(mut self) {
        let handle = Handle::current();

        handle.block_on(async {
            while let Some(f) = self.rx.recv().await {
                f().await;
            }
        });
    }
}
