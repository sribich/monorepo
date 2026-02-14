//! The [`SerialExecutor`] is a single-threaded task worker which processes
//! incoming tasks serially.
//!
//! High-priority or realtime tasks should not be submitted to the executor
//! as it will be blocked during task execution. The intent behind this
//! executor is to run infrequent, but intensive jobs on a separate thread
//! as to not block http threads.
use std::{future::Future, pin::Pin, sync::Arc};

use async_trait::async_trait;
use railgun::error::ResultExt;
use tokio::{
    runtime::Handle,
    sync::{mpsc, oneshot},
    task::{JoinHandle, spawn_blocking},
};

type FnType = Box<
    dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>
        + Send
        + Sync
        + 'static,
>;

pub trait Task<T = ()>: Send + Sync + 'static {
    fn execute(&self) -> impl Future<Output = T> + Send + Sync;
}

#[derive(Clone)]
pub struct SerialExecutor {
    handle: Arc<JoinHandle<()>>,
    // sender: mpsc::Sender<(oneshot::Sender<u32>, Arc<dyn Task>)>,
    sender: mpsc::Sender<FnType>,
}

impl SerialExecutor {
    pub fn new() -> Arc<Self> {
        let (sender, receiver) = mpsc::channel(256);
        let thread = SerialExecutorThread::new(receiver);

        let handle = Arc::new(spawn_blocking(move || {
            thread.run();
        }));

        Arc::new(Self { handle, sender })
    }

    pub async fn send<T: Send + Sync + 'static>(&self, task: impl Task<T>) -> T
    where
        T: core::fmt::Debug,
    {
        let (tx, mut rx) = oneshot::channel::<T>();

        println!("{:#?}", rx);
        self.sender
            .send(Box::new(move || {
                Box::pin(async move {
                    tx.send(task.execute().await);
                })
            }))
            .await
            .expect("receiver closed");

        /// TODO: expect
        rx.await.expect("TODO")
    }
}

pub struct SerialExecutorThread {
    receiver: mpsc::Receiver<FnType>,
}

impl SerialExecutorThread {
    pub fn new(receiver: mpsc::Receiver<FnType>) -> Self {
        Self { receiver }
    }

    pub fn run(mut self) {
        let handle = Handle::current();

        handle.block_on(async {
            while let Some(f) = self.receiver.recv().await {
                (f)().await;

                // message.execute(tx).await;
            }
        })
    }
}
