use defmt::info;

use super::OS_TCB_REF;
use crate::util::SyncUnsafeCell;

pub(crate) struct TimerQueue {
    head: SyncUnsafeCell<Option<OS_TCB_REF>>,
    pub(crate) set_time: SyncUnsafeCell<u64>,
}

impl TimerQueue {
    pub const fn new() -> Self {
        Self {
            head: SyncUnsafeCell::new(None),
            set_time: SyncUnsafeCell::new(u64::MAX),
        }
    }
    /// Insert a task into the timer queue.(sorted by `expires_at`,the header is the nearest task)
    /// return the next expiration time.
    pub(crate) unsafe fn update(&self, p: OS_TCB_REF) -> u64 {
        let p_expires_at = &p.expires_at;
        // by noah：this indicate that the time queue is not updated or the time queue is null
        if *p_expires_at.get_unmut() == u64::MAX {
            return u64::MAX;
        }
        let head = self.head.get_unmut();
        // range from head to find one larger than p_expires_at and insert p.
        let mut cur = head;
        let mut prev: &Option<OS_TCB_REF> = &None;
        while let Some(cur_ref) = cur {
            let cur_expires_at = &cur_ref.expires_at;
            if cur_expires_at > p_expires_at {
                break;
            }
            prev = cur;
            cur = cur_ref.OSTimerNext.get_unmut();
        }
        // insert p
        p.OSTimerNext.set(*cur);
        p.OSTimerPrev.set(*prev);

        if let Some(cur_ref) = cur {
            cur_ref.OSTimerPrev.set(Some(p));
        }
        if let Some(prev_ref) = prev {
            prev_ref.OSTimerNext.set(Some(p));
        } else {
            self.head.set(Some(p));
        }
        let mut cur = head;
        // by noah:*TEST* to konw whether the update is successful
        while let Some(cur_ref) = cur{
            let cur_expires_at = &cur_ref.expires_at;
            info!("---the cur_expires_at is {}---", cur_expires_at.get_unmut());
            cur = cur_ref.OSTimerNext.get_unmut();
        }
        return *head.as_ref().unwrap().expires_at.get_unmut();
    }

    pub(crate) unsafe fn next_expiration(&self) -> u64 {
        let head = self.head.get_unmut();
        if let Some(head_ref) = head {
            *head_ref.expires_at.get_unmut()
        } else {
            u64::MAX
        }
    }
    pub(crate) unsafe fn dequeue_expired(&self, now: u64, on_task: impl Fn(OS_TCB_REF)) {
        info!("dequeue expired");
        let head = self.head.get_unmut();
        let mut cur = head;
        while let Some(cur_ref) = cur {
            let cur_expires_at = &cur_ref.expires_at;
            info!("the cur_expires_at is {}", cur_expires_at.get_unmut());
            if *cur_expires_at.get_unmut() > now {
                break;
            }
            on_task(*cur_ref);
            // by noah: clear the expire time
            cur_ref.expires_at.set(u64::MAX);
            let next = cur_ref.OSTimerNext.get_unmut();
            if let Some(next_ref) = next {
                next_ref.OSTimerPrev.set(*cur_ref.OSTimerPrev.get_unmut());
            }
            if let Some(prev_ref) = cur_ref.OSTimerPrev.get_unmut() {
                prev_ref.OSTimerNext.set(*next);
            } else {
                self.head.set(*next);
            }
            cur = next;
        }
    }
}
