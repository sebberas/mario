use std::future::*;
use std::sync::mpsc::{self, Receiver, Sender};

use futures::executor::*;
use futures::future::*;
use futures::task::*;

/// Handles multithreading of the game logic.
pub struct TaskManager {
    pool: ThreadPool,
    sender: Sender<RemoteHandle<()>>,
    receiver: Receiver<RemoteHandle<()>>,
}

impl TaskManager {
    pub fn new() -> Self {
        let pool = ThreadPool::new().unwrap();
        let (sender, receiver) = mpsc::channel();

        TaskManager {
            pool,
            sender,
            receiver,
        }
    }

    /// Updates the task manager. This function runs once pr. frame.
    ///
    /// All tasks queued since the previous call to `update` are guaranteed to
    /// be completed.
    pub async fn update(&mut self) {
        join_all(self.receiver.try_iter()).then(|_| async {}).await
    }

    /// Spawns a future that should complete before the next call to `update`.
    pub fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        let Self { pool, sender, .. } = &self;
        let handle = pool.spawn_with_handle(future).unwrap();
        sender.send(handle).unwrap();
    }
}
