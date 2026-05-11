use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use crate::Ui;

/// call spawn with an async block
/// it gets assigned a unique ID
/// gets sent to the tokio runtime to run in the background
/// when it finishes it stores its callback in a HashMap under that ID and notifies the main thread
/// main thread looks it up and runs it

pub struct EventQueue {
    // the winit proxy wrapped in a closure
    // set up once at the start
    shared_sender: Arc<Mutex<Option<Arc<dyn Fn(u64) + Send + Sync>>>>,

    // sync callbacks
    // mostly for internal use
    pub(crate) callbacks: HashMap<u64, Box<dyn FnOnce(&mut Ui)>>,

    // async callbacks waiting to run on main thread
    // Arc<Mutex> because both async and main thread access it
    pub(crate) async_callbacks: Arc<Mutex<HashMap<u64, Box<dyn FnOnce(&mut Ui) + Send>>>>,

    // next callback id
    next_id: u64,

    // futures arrived before spawner was set
    pending_futures: Vec<Pin<Box<dyn Future<Output = ()> + Send>>>,

    // tokio runtime spawn function
    // set up once at the start
    spawner: Option<Arc<dyn Fn(Pin<Box<dyn Future<Output = ()> + Send>>) + Send + Sync>>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            shared_sender: Arc::new(Mutex::new(None)),
            callbacks: HashMap::new(),
            async_callbacks: Arc::new(Mutex::new(HashMap::new())),
            next_id: 0,
            pending_futures: Vec::new(),
            spawner: None,
        }
    }

    /// stores the winit proxy closure used to notify the main thread when an async task completes
    pub fn set_sender(&mut self, sender: Arc<dyn Fn(u64) + Send + Sync>) {
        *self.shared_sender.lock().unwrap() = Some(sender);
    }

    /// stores the tokio spawn function and immediately spawns any futures that arrived before setup
    pub fn set_spawner(
        &mut self,
        spawner: Arc<dyn Fn(Pin<Box<dyn Future<Output = ()> + Send>>) + Send + Sync>,
    ) {
        self.spawner = Some(spawner.clone());
        for fut in self.pending_futures.drain(..) {
            spawner(fut);
        }
    }

    /// wraps a future so that when it completes its callback is stored and winit is notified,
    /// then sends it to the tokio runtime to run in the background
    pub fn spawn<F, C>(&mut self, future: F)
    where
        F: std::future::Future<Output = C> + Send + 'static,
        C: FnOnce(&mut Ui) + Send + 'static,
    {
        let id = self.next_id;
        self.next_id += 1;

        let async_callbacks = self.async_callbacks.clone();
        let shared_sender = self.shared_sender.clone();

        let fut = Box::pin(async move {
            let callback = future.await;
            async_callbacks
                .lock()
                .unwrap()
                .insert(id, Box::new(callback));
            if let Some(sender) = shared_sender.lock().unwrap().as_ref() {
                sender(id);
            }
        });

        if let Some(spawner) = &self.spawner {
            spawner(fut);
        } else {
            self.pending_futures.push(fut);
        }
    }

    /// convenience wrapper around spawn for simple time delayed callbacks
    pub fn timer<C>(&mut self, duration: f32, callback: C)
    where
        C: FnOnce(&mut Ui) + Send + 'static,
    {
        self.spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs_f32(duration)).await;
            callback
        });
    }
}
