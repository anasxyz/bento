use std::{
    cell::RefCell,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex, OnceLock},
};

#[cfg(not(target_arch = "wasm32"))]
type BoxFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
#[cfg(target_arch = "wasm32")]
type BoxFuture = Pin<Box<dyn Future<Output = ()> + 'static>>;

#[cfg(not(target_arch = "wasm32"))]
type PendingCallback = Box<dyn FnOnce() + Send + 'static>;
#[cfg(target_arch = "wasm32")]
type PendingCallback = Box<dyn FnOnce() + 'static>;

#[cfg(not(target_arch = "wasm32"))]
type WakerFn = Arc<dyn Fn() + Send + Sync + 'static>;
#[cfg(target_arch = "wasm32")]
type WakerFn = Arc<dyn Fn() + 'static>;

#[cfg(not(target_arch = "wasm32"))]
static PENDING: Mutex<Vec<PendingCallback>> = Mutex::new(Vec::new());
#[cfg(not(target_arch = "wasm32"))]
static WAKER: OnceLock<WakerFn> = OnceLock::new();

#[cfg(target_arch = "wasm32")]
thread_local! {
    static PENDING: RefCell<Vec<PendingCallback>> = RefCell::new(Vec::new());
    static WAKER: RefCell<Option<WakerFn>> = RefCell::new(None);
}

thread_local! {
    static SPAWNER: RefCell<Option<Box<dyn Fn(BoxFuture)>>> = RefCell::new(None);
    static PENDING_FUTURES: RefCell<Vec<BoxFuture>> = RefCell::new(Vec::new());
}

pub fn set_spawner(f: impl Fn(BoxFuture) + 'static) {
    SPAWNER.with(|s| *s.borrow_mut() = Some(Box::new(f)));
    let pending = PENDING_FUTURES.with(|p| p.borrow_mut().drain(..).collect::<Vec<_>>());
    SPAWNER.with(|s| {
        if let Some(spawner) = s.borrow().as_ref() {
            for fut in pending {
                spawner(fut);
            }
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn drain_callbacks() -> Vec<PendingCallback> {
    PENDING.lock().unwrap().drain(..).collect()
}
#[cfg(target_arch = "wasm32")]
pub fn drain_callbacks() -> Vec<PendingCallback> {
    PENDING.with(|p| p.borrow_mut().drain(..).collect())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn set_waker(f: WakerFn) {
    WAKER.set(f).ok();
}
#[cfg(target_arch = "wasm32")]
pub fn set_waker(f: WakerFn) {
    WAKER.with(|w| *w.borrow_mut() = Some(f));
}

fn do_spawn(future: BoxFuture) {
    SPAWNER.with(|s| {
        if let Some(spawner) = s.borrow().as_ref() {
            spawner(future);
        } else {
            PENDING_FUTURES.with(|p| p.borrow_mut().push(future));
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn spawn<F, C>(future: F)
where
    F: Future<Output = C> + Send + 'static,
    C: FnOnce() + Send + 'static,
{
    do_spawn(Box::pin(async move {
        let callback = future.await;
        PENDING.lock().unwrap().push(Box::new(callback));
        if let Some(waker) = WAKER.get() {
            waker();
        }
    }));
}

#[cfg(target_arch = "wasm32")]
pub fn spawn<F, C>(future: F)
where
    F: Future<Output = C> + 'static,
    C: FnOnce() + 'static,
{
    do_spawn(Box::pin(async move {
        let callback = future.await;
        PENDING.with(|p| p.borrow_mut().push(Box::new(callback)));
        WAKER.with(|w| {
            if let Some(waker) = w.borrow().as_ref() {
                waker();
            }
        });
    }));
}

#[cfg(not(target_arch = "wasm32"))]
pub fn timer<C>(duration: f32, callback: C)
where
    C: FnOnce() + Send + 'static,
{
    spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs_f32(duration)).await;
        move || callback()
    });
}

#[cfg(target_arch = "wasm32")]
pub fn timer<C>(duration: f32, callback: C)
where
    C: FnOnce() + 'static,
{
    spawn(async move {
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
        move || callback()
    });
}
