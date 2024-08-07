//! Raw task storage and pool.

#[cfg_attr(feature = "cortex_m", path = "state_atomics_arm.rs")]
pub mod state;
pub mod waker;
use alloc::string::String;
use core::future::Future;
use core::mem;
use core::ops::{Deref, DerefMut};
use core::pin::Pin;
use core::ptr::NonNull;
use core::task::{Context, Poll};

use lazy_static::lazy_static;
// use run_queue_atomics::RunQueue;
use state::State;

pub use self::waker::task_from_waker;
use crate::arena::ARENA;
use crate::cfg::*;
use crate::heap::stack_allocator::{dealloc_stack, OS_STK_REF};
// use spawner::SpawnToken;
use crate::port::*;
use crate::ucosii::*;
use crate::util::{SyncUnsafeCell, UninitCell};

/*
****************************************************************************************************************************************
*                                                             global variables
****************************************************************************************************************************************
*/
// create a global executor
lazy_static! {
/// the global executor will be initialized at os init
    pub(crate) static ref GlobalSyncExecutor: Option<SyncExecutor> = Some(SyncExecutor::new(Pender(0 as *mut ())));
}
/*
****************************************************************************************************************************************
*                                                             type define
****************************************************************************************************************************************
*/

/// the TCB of the task. It contains the task's info
#[allow(unused)]
// we put it in executor crate to use "pub(crate)" to make it can be used in the other mod in order to reduce coupling
pub struct OS_TCB {
    // it maybe None
    OSTCBStkPtr: Option<OS_STK_REF>, /* Pointer to current top of stack                         */
    // Task specific extension. If the OS_TASK_CREATE_EXT_EN feature is not active, it will be None
    #[cfg(feature = "OS_TASK_CREATE_EXT_EN")]
    OSTCBExtInfo: OS_TCB_EXT,

    OSTCBNext: SyncUnsafeCell<Option<OS_TCB_REF>>, /* Pointer to next     TCB in the TCB list                 */
    OSTCBPrev: SyncUnsafeCell<Option<OS_TCB_REF>>, /* Pointer to previous TCB in the TCB list                 */

    // the poll fn that will be called by the executor. In the func, a waker will be create.
    OS_POLL_FN: SyncUnsafeCell<Option<unsafe fn(OS_TCB_REF)>>,

    #[cfg(feature = "OS_EVENT_EN")]
    OSTCBEventPtr: Option<OS_EVENT_REF>, /* Pointer to event control block                */

    #[cfg(any(all(feature = "OS_Q_EN", feature = "OS_MAX_QS"), feature = "OS_MBOX_EN"))]
    OSTCBMsg: PTR, /* Message received from OSMboxPost() or OSQPost()         */

    OSTCBDly: INT32U, /* Nbr ticks to delay task or, timeout waiting for event   */
    OSTCBStat: State, /* Task      status                                        */
    // no need
    // OSTCBStatPend: INT8U, /* Task PEND status                                        */
    OSTCBPrio: INT8U, /* Task priority (0 == highest)                            */

    OSTCBX: INT8U,    /* Bit position in group  corresponding to task priority   */
    OSTCBY: INT8U,    /* Index into ready table corresponding to task priority   */
    OSTCBBitX: INT8U, /* Bit mask to access bit position in ready table          */
    OSTCBBitY: INT8U, /* Bit mask to access bit position in ready group          */

    #[cfg(feature = "OS_TASK_DEL_EN")]
    OSTCBDelReq: INT8U, /* Indicates whether a task needs to delete itself         */

    #[cfg(feature = "OS_TASK_PROFILE_EN")]
    OSTCBCtxSwCtr: INT32U, /* Number of time the task was switched in                 */
    OSTCBCyclesTot: INT32U,           /* Total number of clock cycles the task has been running  */
    OSTCBCyclesStart: INT32U,         /* Snapshot of cycle counter at start of task resumption   */
    OSTCBStkBase: Option<OS_STK_REF>, /* Pointer to the beginning of the task stack              */
    OSTCBStkUsed: INT32U,             /* Number of bytes used from the stack                     */

    #[cfg(feature = "OS_TASK_REG_TBL_SIZE")]
    OSTCBRegTbl: [INT32U; OS_TASK_REG_TBL_SIZE],

    #[cfg(feature = "OS_TASK_NAME_EN")]
    OSTCBTaskName: String,
    // #[cfg(feature="OS_TASK_CREATE_EXT_EN")]
    // OS_TLS OSTCBTLSTbl[OS_TLS_TBL_SIZE];

    // OS_EVENT **OSTCBEventMultiPtr; /* Pointer to multiple  event control blocks               */
    // OS_EVENT *OSTCBEventMultiRdy;  /* Pointer to the first event control block readied        */
    // OS_FLAG_NODE *OSTCBFlagNode; /* Pointer to event flag node                              */

    // OS_FLAGS OSTCBFlagsRdy; /* Event flags that made task ready to run                 */
}

#[cfg(feature = "OS_TASK_CREATE_EXT_EN")]
#[allow(unused)]
pub(crate) struct OS_TCB_EXT {
    OSTCBExtPtr: PTR,                   /* Pointer to user definable data for TCB extension        */
    OSTCBStkBottom: Option<OS_STK_REF>, /* Pointer to bottom of stack                              */
    OSTCBStkSize: INT32U,               /* Size of task stack (in number of stack elements)        */
    OSTCBOpt: INT16U,                   /* Task options as passed by OSTaskCreateExt()             */
    OSTCBId: INT16U,                    /* Task ID (0..65535)                                      */
}

/// the storage of the task. It contains the task's TCB and the future
#[allow(unused)]
pub struct OS_TASK_STORAGE<F: Future + 'static> {
    task_tcb: OS_TCB,
    // this part is invisible to other crate
    // by noah: maybe we need to use raw ptr
    future: UninitCell<F>,
}

/// the ref of the TCB. In other crate only it can be used to access the TCB
#[derive(Clone, Copy)]
#[allow(unused)]
pub struct OS_TCB_REF {
    /// the pointer to the TCB
    pub ptr: Option<NonNull<OS_TCB>>,
}

// /// Raw storage that can hold up to N tasks of the same type.
// ///
// /// This is essentially a `[OS_TASK_STORAGE<F>; N]`.
// #[allow(unused)]
// pub struct TaskPool<F: Future + 'static, const N: usize> {
//     pool: [OS_TASK_STORAGE<F>; N],
// }

// /// by noah：this structure is used to define TaskPool in the global scope with static life time
// pub struct TaskPoolRef {
//     // type-erased `&'static mut TaskPool<F, N>`
//     // Needed because statics can't have generics.
//     ptr: Mutex<RefCell<*mut ()>>,
// }

/// An uninitialized [`OS_TASK_STORAGE`].
#[allow(unused)]
pub struct AvailableTask<F: Future + 'static> {
    task: &'static OS_TASK_STORAGE<F>,
}

/*
****************************************************************************************************************************************
*                                                             implement of structure
****************************************************************************************************************************************
*/

impl OS_TCB {
    // can only be called if the task owns the stack
    fn restore_context_from_stk(&mut self) {
        extern "Rust" {
            fn restore_arch_stk_user(stk: *mut OS_STK);
        }
        if self.OSTCBStkPtr.is_none() {
            return;
        }
        let stk = self.OSTCBStkPtr.as_mut().unwrap().STK_REF.as_ptr();
        unsafe { restore_arch_stk_user(stk) };
    }
}

impl OS_TCB_EXT {
    fn init(&mut self, pext: *mut (), opt: INT16U, id: INT16U) {
        self.OSTCBExtPtr = pext;
        // info about stack is no need to be init here
        // self.OSTCBStkBottom=None;
        // self.OSTCBStkSize=0;
        self.OSTCBOpt = opt;
        self.OSTCBId = id;
    }
}

impl<F: Future + 'static> OS_TASK_STORAGE<F> {
    // const NEW: Self = Self::new();
    /// create a new OS_TASK_STORAGE
    // Take a lazy approach, which means the TCB will be init when call the init func of TCB
    // this func will be used to init the global array
    const fn new() -> Self {
        Self {
            task_tcb: OS_TCB {
                OSTCBStkPtr: None,
                #[cfg(feature = "OS_TASK_CREATE_EXT_EN")]
                OSTCBExtInfo: OS_TCB_EXT {
                    OSTCBExtPtr: 0 as PTR,
                    OSTCBStkBottom: None,
                    OSTCBStkSize: 0,
                    OSTCBOpt: 0,
                    OSTCBId: 0,
                },
                OSTCBNext: SyncUnsafeCell::new(None),
                OSTCBPrev: SyncUnsafeCell::new(None),
                OS_POLL_FN: SyncUnsafeCell::new(None),
                #[cfg(feature = "OS_EVENT_EN")]
                OSTCBEventPtr: None,
                #[cfg(any(all(feature = "OS_Q_EN", feature = "OS_MAX_QS"), feature = "OS_MBOX_EN"))]
                OSTCBMsg: 0 as PTR,
                OSTCBDly: 0,
                OSTCBStat: State::new(),
                // no need
                // OSTCBStatPend: 0,
                OSTCBPrio: 0,
                OSTCBX: 0,
                OSTCBY: 0,
                OSTCBBitX: 0,
                OSTCBBitY: 0,
                #[cfg(feature = "OS_TASK_DEL_EN")]
                OSTCBDelReq: 0,
                #[cfg(feature = "OS_TASK_PROFILE_EN")]
                OSTCBCtxSwCtr: 0,
                OSTCBCyclesTot: 0,
                OSTCBCyclesStart: 0,
                OSTCBStkBase: None,
                OSTCBStkUsed: 0,
                #[cfg(feature = "OS_TASK_REG_TBL_SIZE")]
                OSTCBRegTbl: [0; OS_TASK_REG_TBL_SIZE],
                #[cfg(feature = "OS_TASK_NAME_EN")]
                OSTCBTaskName: String::new(),
            },
            future: UninitCell::uninit(),
        }
    }

    /// init the storage of the task, just like the spawn in Embassy
    //  this func will be called by OS_TASK_CTREATE
    //  just like OSTCBInit in uC/OS, but we don't need the stack ptr
    pub fn init(
        prio: INT8U,
        id: INT16U,
        pext: *mut (),
        opt: INT16U,
        _name: String,
        future_func: impl FnOnce() -> F,
    ) -> OS_ERR_STATE {
        // by noah: claim a TaskStorage
        let task_ref = OS_TASK_STORAGE::<F>::claim();

        let this: &mut OS_TASK_STORAGE<F>;
        unsafe {
            this = &mut *(task_ref.as_ptr() as *mut OS_TASK_STORAGE<F>);
            this.task_tcb.OS_POLL_FN.set(Some(OS_TASK_STORAGE::<F>::poll));
            this.future.write_in_place(future_func);
        }
        // set the prio also need to set it in the bitmap
        this.task_tcb.OSTCBPrio = prio;
        this.task_tcb.OSTCBY = prio >> 3;
        this.task_tcb.OSTCBX = prio & 0x07;
        this.task_tcb.OSTCBBitY = 1 << this.task_tcb.OSTCBY;
        this.task_tcb.OSTCBBitX = 1 << this.task_tcb.OSTCBX;
        // set the stat
        if !this.task_tcb.OSTCBStat.spawn() {
            panic!("task with prio {} spawn failed", prio);
        }
        // init ext info
        #[cfg(feature = "OS_TASK_CREATE_EXT_EN")]
        this.task_tcb.OSTCBExtInfo.init(pext, opt, id);
        // add the task to ready queue
        // the operation about the bitmap will be done in the RunQueue
        // need a cs
        critical_section::with(|_cs| {
            unsafe { GlobalSyncExecutor.as_ref().unwrap().enqueue(task_ref) };
        });
        #[cfg(feature = "OS_EVENT_EN")]
        {
            this.task_tcb.OSTCBEventPtr = None;
            #[cfg(feature = "OS_EVENT_MULTI_EN")]
            {
                // this.task_tcb.OSTCBEventMultiPtr
                // this.task_tcb.OSTCBEventMultiPtr
            }
        }
        // #[cfg(all(feature="OS_FLAG_EN",feature="OS_MAX_FLAGS",feature="OS_TASK_DEL_EN"))]
        // this.task_tcb.OSTCBFlagNode=None;
        #[cfg(any(feature = "OS_MBOX_EN", all(feature = "OS_Q_EN", feature = "OS_MAX_QS")))]
        {
            this.task_tcb.OSTCBMsg = 0 as PTR;
        }

        #[cfg(feature = "OS_TASK_NAME_EN")]
        {
            this.task_tcb.OSTCBTaskName = name;
        }

        if OS_TASK_REG_TBL_SIZE > 0 {
            for i in 0..OS_TASK_REG_TBL_SIZE {
                this.task_tcb.OSTCBRegTbl[i] = 0;
            }
        }

        #[cfg(feature = "OS_CPU_HOOKS_EN")]
        {
            // Call user defined hook
            OSTCBInitHook(ptcb);
            OSTaskCreateHook(ptcb);
        }
        return OS_ERR_STATE::OS_ERR_NONE;
        // we don't need to add the TaskRef into OSTCBPrioTbl because we did this in func enqueue
    }

    /// the poll fun called by the executor
    unsafe fn poll(p: OS_TCB_REF) {
        let this = &*(p.as_ptr() as *const OS_TASK_STORAGE<F>);

        let future = Pin::new_unchecked(this.future.as_mut());
        let waker = waker::from_task(p);
        let mut cx = Context::from_waker(&waker);
        match future.poll(&mut cx) {
            Poll::Ready(_) => {
                this.future.drop_in_place();
                this.task_tcb.OSTCBStat.despawn();
            }
            Poll::Pending => {}
        }

        // the compiler is emitting a virtual call for waker drop, but we know
        // it's a noop for our waker.
        mem::forget(waker);
    }

    /// this func will be called to create a new task(TCB)
    // refer to the get of TaskPoolRef in embassy
    fn claim() -> OS_TCB_REF {
        // by noah: for we can create task after OSTaskCreate, so we need a cs
        critical_section::with(|cs| {
            let task_storage = ARENA.alloc::<OS_TASK_STORAGE<F>>(cs);
            // create a new task which is not init
            task_storage.write(OS_TASK_STORAGE::new());
            // by noah：no panic will occurred here because if the Arena is not enough, the program will panic when alloc
            OS_TCB_REF {
                ptr: Some(NonNull::new(task_storage as *mut _ as _).unwrap()),
            }
        })
    }
}

unsafe impl Sync for OS_TCB_REF {}
unsafe impl Send for OS_TCB_REF {}

impl Default for OS_TCB_REF {
    // this func will not be called
    fn default() -> Self {
        // by noah:dangling is used to create a dangling pointer, which is just like the null pointer in C
        OS_TCB_REF { ptr: None }
    }
}

// impl deref for OS_TCB_REF
impl Deref for OS_TCB_REF {
    type Target = OS_TCB;
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.unwrap().as_ref() }
    }
}

// impl deref for mut OS_TCB_REF
impl DerefMut for OS_TCB_REF {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.unwrap().as_mut() }
    }
}

impl OS_TCB_REF {
    /// Safety: The pointer must have been obtained with `Task::as_ptr`
    pub(crate) unsafe fn from_ptr(ptr: *const OS_TCB) -> Self {
        Self {
            ptr: Some(NonNull::new_unchecked(ptr as *mut OS_TCB)),
        }
    }

    pub(crate) fn header(self) -> &'static OS_TCB {
        unsafe { self.ptr.unwrap().as_ref() }
    }

    /// The returned pointer is valid for the entire OS_TASK_STORAGE.
    pub(crate) fn as_ptr(self) -> *const OS_TCB {
        self.ptr.unwrap().as_ptr()
    }
}

impl<F: Future + 'static> AvailableTask<F> {}

/*
****************************************************************************************************************************************
*                                                             type define
****************************************************************************************************************************************
*/

/// Wake a task by `TaskRef`.
///
/// You can obtain a `TaskRef` from a `Waker` using [`task_from_waker`].
pub fn wake_task(task: OS_TCB_REF) {
    let header = task.header();
    if header.OSTCBStat.run_enqueue() {
        // We have just marked the task as scheduled, so enqueue it.
        unsafe {
            let executor = GlobalSyncExecutor.as_ref().unwrap_unchecked();
            executor.enqueue(task);
        }
    }
}

#[derive(Clone, Copy)]
#[allow(unused)]
pub(crate) struct Pender(*mut ());

unsafe impl Send for Pender {}
unsafe impl Sync for Pender {}

impl Pender {
    #[allow(unused)]
    pub(crate) fn pend(self) {
        extern "Rust" {
            fn __pender(context: *mut ());
        }
        unsafe { __pender(self.0) };
    }
}

/// The executor for the uC/OS-II RTOS.
pub(crate) struct SyncExecutor {
    // run_queue: RunQueue,
    // the prio tbl stores a relation between the prio and the task_ref
    os_prio_tbl: SyncUnsafeCell<[OS_TCB_REF; OS_LOWEST_PRIO + 1]>,
    // indicate the current running task
    OSPrioCur: SyncUnsafeCell<OS_PRIO>,
    _pender: Pender,
    // by liam: add a bitmap to record the status of the task
    #[cfg(feature = "OS_PRIO_LESS_THAN_64")]
    OSRdyGrp: SyncUnsafeCell<u8>,
    #[cfg(feature = "OS_PRIO_LESS_THAN_64")]
    OSRdyTbl: SyncUnsafeCell<[u8; OS_RDY_TBL_SIZE]>,
    #[cfg(feature = "OS_PRIO_LESS_THAN_256")]
    OSRdyGrp: u16,
    #[cfg(feature = "OS_PRIO_LESS_THAN_256")]
    OSRdyTbl: [u16; OS_RDY_TBL_SIZE],
}
impl SyncExecutor {
    /// The global executor for the uC/OS-II RTOS.
    pub(crate) fn new(pender: Pender) -> Self {
        Self {
            os_prio_tbl: SyncUnsafeCell::new([OS_TCB_REF::default(); OS_LOWEST_PRIO + 1]),
            OSPrioCur: SyncUnsafeCell::new(OS_LOWEST_PRIO),
            _pender: pender,
            OSRdyGrp: SyncUnsafeCell::new(0),
            OSRdyTbl: SyncUnsafeCell::new([0; OS_RDY_TBL_SIZE]),
        }
    }

    /// Enqueue a task in the task queue
    ///
    /// # Safety
    /// - `task` must be a valid pointer to a spawned task.
    /// - `task` must be set up to run in this executor.
    /// - `task` must NOT be already enqueued (in this executor or another one).
    #[inline(always)]
    unsafe fn enqueue(&self, task: OS_TCB_REF) {
        //according to the priority of the task, we place the task in the right place of os_prio_tbl
        // also we will set the corresponding bit in the OSRdyTbl and OSRdyGrp
        let prio = task.OSTCBPrio as usize;
        let tmp = self.OSRdyGrp.get_mut();
        *tmp = *tmp | task.OSTCBBitY;
        let tmp = self.OSRdyTbl.get_mut();
        tmp[task.OSTCBY as usize] |= task.OSTCBBitX;
        // set the task in the right place of os_prio_tbl
        let tmp = self.os_prio_tbl.get_mut();
        tmp[prio] = task;
    }
    // check by liam: we don't need to spawn, the init of global executor will be set in the os init
    // pub(super) unsafe fn spawn(&'static self, task: OS_TCB_REF) {
    //     SyncExecutor.set(Some(self));

    //     self.enqueue(task);
    // }

    /// since when it was called, there is no task running, we need poll all the task that is ready in bitmap
    pub(crate) unsafe fn poll(&'static self) {
        // find the highest priority task in the ready queue
        let task = critical_section::with(|_| self.find_highrdy_set_cur());
        if task.is_none() {
            return;
        }
        let mut task = task.unwrap();
        if task.OSTCBStat.run_dequeue() {
            // If task is not running, ignore it. This can happen in the following scenario:
            //   - Task gets dequeued, poll starts
            //   - While task is being polled, it gets woken. It gets placed in the queue.
            //   - Task poll finishes, returning done=true
            //   - RUNNING bit is cleared, but the task is already in the queue.

            if task.OSTCBStkPtr.is_none() {
                task.OS_POLL_FN.get().unwrap_unchecked()(task);
            } else {
                // if the task has stack, it's a thread, we need to resume it not poll it
                task.restore_context_from_stk();
            }
        }
        // after the task is done, we need to set the task to unready(in bitmap) and also we need to find the next task to run
        // both of this process should be done in critical section
        loop {
            match critical_section::with(|_| {
                self.set_task_unready(task);
                // after set the task as unready, we need to revoke its stack if it has.
                if task.OSTCBStkPtr.is_some() {
                    dealloc_stack(task.OSTCBStkPtr.as_mut().unwrap());
                }
                // set the task's stack to None
                task.OSTCBStkPtr = None;
                self.find_highrdy_set_cur()
            }) {
                Some(t) => {
                    task = t;
                    if task.OSTCBStat.run_dequeue() {
                        // in the future, we should consider thread here
                        task.OS_POLL_FN.get().unwrap_unchecked()(task);
                    }
                }
                None => {
                    break;
                }
            }
        }
    }
    unsafe fn find_highrdy_set_cur(&self) -> Option<OS_TCB_REF> {
        let tmp = self.OSRdyGrp.get_unmut();
        // if there is no task in the ready queue, return None also set the current running task to the lowest priority
        if *tmp == 0 {
            self.OSPrioCur.set(OS_LOWEST_PRIO);
            return None;
        }
        let prio = tmp.trailing_zeros() as usize;
        let tmp = self.OSRdyTbl.get_unmut();
        let prio = prio * 8 + tmp[prio].trailing_zeros() as usize;
        // set the current running task
        self.OSPrioCur.set(prio as OS_PRIO);
        Some(self.os_prio_tbl.get_unmut()[prio])
    }
    unsafe fn set_task_unready(&self, task: OS_TCB_REF) {
        // added by liam: we have to make this process in critical section
        // because the bitmap is shared by all the tasks
        critical_section::with(|_| {
            let tmp = self.OSRdyTbl.get_mut();
            tmp[task.OSTCBY as usize] &= !task.OSTCBBitX;
            // when the group is empty, we need to set the corresponding bit in the OSRdyGrp to 0
            if tmp[task.OSTCBY as usize] == 0 {
                let tmp = self.OSRdyGrp.get_mut();
                *tmp &= !task.OSTCBBitY;
            }
        });
    }
    // check if an prio is exiting
    pub fn prio_exist(&self, prio: INT8U) -> bool {
        let prio_tbl: &[OS_TCB_REF; OS_LOWEST_PRIO + 1];
        prio_tbl = self.os_prio_tbl.get_unmut();
        prio_tbl[prio as USIZE].ptr.is_some()
    }

    pub fn reserve_bit(&self, prio: INT8U) {
        let prio_tbl: &mut [OS_TCB_REF; OS_LOWEST_PRIO + 1];
        prio_tbl = self.os_prio_tbl.get_mut();
        // use the dangling pointer(Some) to reserve the bit
        prio_tbl[prio as USIZE].ptr = Some(NonNull::dangling());
    }

    pub fn clear_bit(&self, prio: INT8U) {
        let prio_tbl: &mut [OS_TCB_REF; OS_LOWEST_PRIO + 1];
        prio_tbl = self.os_prio_tbl.get_mut();
        // use the dangling pointer(Some) to reserve the bit
        prio_tbl[prio as USIZE].ptr = None;
    }
}
