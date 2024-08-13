use crate::util::SyncUnsafeCell;

use super::OS_TCB_REF;

pub(crate) struct TimerQueue {
    head: SyncUnsafeCell<Option<OS_TCB_REF>>,
}

impl TimerQueue {
    pub const fn new() -> Self {
        Self {
            head: SyncUnsafeCell::new(None),
        }
    }
    /// Insert a task into the timer queue.(sorted by `expires_at`,the header is the nearest task)
    pub(crate) unsafe fn update(&self, p:OS_TCB_REF){
        let head = self.head.get_unmut();
        let p_expires_at = &p.expires_at;
        // range from head to find one smaller than p_expires_at and insert p.
        let mut cur = head; 
        while let Some(cur_ref) = cur {
            let cur_expires_at = &cur_ref.expires_at;
            if cur_expires_at > p_expires_at {
                break;
            }
            cur = cur_ref.OSTimerNext.get_unmut();
        }
    }
}