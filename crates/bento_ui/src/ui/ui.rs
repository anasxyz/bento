use super::EventQueue;

pub struct Ui {
    pub events: EventQueue,
}

impl Ui {
    /// looks up a completed async callback by id and runs it with &mut Ui on the main thread
    pub fn fire_callback(&mut self, id: u64) {
        if let Some(callback) = self.events.callbacks.remove(&id) {
            callback(self);
        } else {
            let callback = self.events.async_callbacks.lock().unwrap().remove(&id);
            if let Some(callback) = callback {
                callback(self);
            }
        }
    }
}
