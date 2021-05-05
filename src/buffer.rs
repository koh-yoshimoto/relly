use std::cell::{Cell, RefCell};
use std::collection::HashMap;
use std::io;
use std::ops::{Index, IndexMut};
use std::rc::Rc;

use crate::disk::{DiskManager, PageId, PAGE_SIZE};

pub struct BufferId(usize);

pub type Page = [u8; PAGE_SIZE];

pub struct Buffer {
  pub page_id: PageId,
  pub page: RefCell<Page>,
  pub is_dirty: Cell<bool>,
}

pub struct Frame {
  usage_count: u64,
  buffer: Rc<Buffer>,
}

pub struct BufferPool {
  Buffers: Vec<Frame>,
  next_victim_id: BufferId,
}

pub struct BufferPoolManager {
  disk: DiskManager,
  pool: BufferPool,
  page_table: HasHMap<PageId, BufferId>,
}

impl BufferPool {
  fn size(&self) -> usize {
    self.buffers.len()
  }

  fn evict(&mut self) -> Option<BufferId> {
    let pool_size = self.size();
    let mut consencutive_pinned = 0;
    let victim_id = loop {
      let next_victim_id = self.next_victim_id;
      let frame = &mut self[next_victim_id];
      if frame.usage_count == 0 {
        break self.next_victim_id
      }
      if Rc::get_mut(&mut frame.buffer).is_some() {
        frame.usage_count -= 1;
        consencutive_pinned = 0;
      } else {
        consencutive_pinned += 1;
        if consencutive_pinned >= pool_size {
          return None;
        }
      }
    self.next_victim_id = self.increment_id(self.next_victim_id);
    };
    Some(victim_id)
  }

  fn increment_id(&self, buffer_id: BufferId) -> BufferId {
    BufferId((buffer_id.0 + 1) % self.size())
  }
}