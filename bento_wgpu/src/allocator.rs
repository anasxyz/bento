// allocator.rs
//
// Free-list slot allocator.
// Hands out stable u32 indices into GPU instance buffers.
// When a node is removed its slot is returned here and reused.
// This ensures the GPU buffer stays compact without shifting other slots.

pub(crate) struct SlotAllocator {
    next: u32,
    free: Vec<u32>,
}

impl SlotAllocator {
    pub fn new() -> Self {
        Self { next: 0, free: Vec::new() }
    }

    pub fn alloc(&mut self) -> u32 {
        if let Some(slot) = self.free.pop() {
            slot
        } else {
            let s = self.next;
            self.next += 1;
            s
        }
    }

    pub fn free(&mut self, slot: u32) {
        if slot != u32::MAX {
            self.free.push(slot);
        }
    }

    pub fn free_many(&mut self, slots: &[u32]) {
        for &s in slots {
            self.free(s);
        }
    }

    /// Highest slot index ever issued. Used to size GPU buffers.
    pub fn capacity(&self) -> u32 {
        self.next
    }
}
