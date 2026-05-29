use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use super::ui::Ui;

#[cfg(not(target_arch = "wasm32"))]
type BoxFuture = Pin<Box<dyn Future<Output = ()> + Send>>;
#[cfg(not(target_arch = "wasm32"))]
type Spawner = Arc<dyn Fn(BoxFuture) + Send + Sync>;
#[cfg(not(target_arch = "wasm32"))]
type AsyncCallback = Box<dyn FnOnce(&mut Ui) + Send>;

#[cfg(target_arch = "wasm32")]
type BoxFuture = Pin<Box<dyn Future<Output = ()>>>;
#[cfg(target_arch = "wasm32")]
type Spawner = Arc<dyn Fn(BoxFuture)>;
#[cfg(target_arch = "wasm32")]
type AsyncCallback = Box<dyn FnOnce(&mut Ui)>;

pub struct AsyncEventQueue {
    shared_sender: Arc<Mutex<Option<Arc<dyn Fn(u64) + Send + Sync>>>>,
    pub(crate) callbacks: HashMap<u64, Box<dyn FnOnce(&mut Ui)>>,
    pub(crate) async_callbacks: Arc<Mutex<HashMap<u64, AsyncCallback>>>,
    next_id: u64,
    pending_futures: Vec<BoxFuture>,
    spawner: Option<Spawner>,
}

#[derive(Clone)]
pub struct TimerHandle {
    cancelled: Arc<Mutex<bool>>,
}

impl TimerHandle {
    pub fn cancel(&self) {
        *self.cancelled.lock().unwrap() = true;
    }
}

impl AsyncEventQueue {
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

    pub fn set_sender(&mut self, sender: Arc<dyn Fn(u64) + Send + Sync>) {
        *self.shared_sender.lock().unwrap() = Some(sender);
    }

    pub fn set_spawner(&mut self, spawner: Spawner) {
        self.spawner = Some(spawner.clone());
        for fut in self.pending_futures.drain(..) {
            spawner(fut);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
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
            async_callbacks.lock().unwrap().insert(id, Box::new(callback));
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

    #[cfg(target_arch = "wasm32")]
    pub fn spawn<F, C>(&mut self, future: F)
    where
        F: std::future::Future<Output = C> + 'static,
        C: FnOnce(&mut Ui) + 'static,
    {
        let id = self.next_id;
        self.next_id += 1;
        let async_callbacks = self.async_callbacks.clone();
        let shared_sender = self.shared_sender.clone();
        let fut = Box::pin(async move {
            let callback = future.await;
            async_callbacks.lock().unwrap().insert(id, Box::new(callback));
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

    #[cfg(not(target_arch = "wasm32"))]
    pub fn timer<C>(&mut self, duration: f32, callback: C) -> TimerHandle
    where
        C: FnOnce(&mut Ui) + Send + 'static,
    {
        let cancelled = Arc::new(Mutex::new(false));
        let cancelled_clone = cancelled.clone();
        self.spawn(async move {
            tokio::time::sleep(web_time::Duration::from_secs_f32(duration)).await;
            move |ui: &mut Ui| {
                if !*cancelled_clone.lock().unwrap() {
                    callback(ui);
                }
            }
        });
        TimerHandle { cancelled }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn timer<C>(&mut self, duration: f32, callback: C) -> TimerHandle
    where
        C: FnOnce(&mut Ui) + 'static,
    {
        let cancelled = Arc::new(Mutex::new(false));
        let cancelled_clone = cancelled.clone();
        self.spawn(async move {
            let promise = js_sys::Promise::new(&mut |resolve, _| {
                web_sys::window()
                    .unwrap()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        &resolve,
                        (duration * 1000.0) as i32,
                    )
                    .unwrap();
            });
            wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
            move |ui: &mut Ui| {
                if !*cancelled_clone.lock().unwrap() {
                    callback(ui);
                }
            }
        });
        TimerHandle { cancelled }
    }
}

impl Ui {
    pub fn fire_callback(&mut self, id: u64) {
        if let Some(callback) = self.asyncs.callbacks.remove(&id) {
            callback(self);
        } else {
            let callback = self.asyncs.async_callbacks.lock().unwrap().remove(&id);
            if let Some(callback) = callback {
                callback(self);
            }
        }
    }
}
