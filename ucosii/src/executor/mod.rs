//! the async scheduler(executor) of rust uC
/*
****************************************************************************************************************************************
*                                                      the scheduler of uC/OS-II
*                                                        code by liam and noah  
****************************************************************************************************************************************
*/
mod spawner;
mod raw;

// pub mod os_task;
// /// the executor's raw module
// pub mod raw;
// use executor::SpawnToken;
use core::future::Future;
use core::ops::Deref;
use core::ops::DerefMut;
use core::ptr::NonNull;

use alloc::string::String;
use raw::state::State;
// use spawner::SpawnToken;

use crate::port::*;
use crate::stk_allocator::ARENA;
use crate::ucosii::*;
use crate::cfg::*;
use crate::stk_allocator::OS_STK_REF;
use crate::util::SyncUnsafeCell;
use crate::util::UninitCell;

/*
****************************************************************************************************************************************
*                                                             type define  
****************************************************************************************************************************************
*/

/// the TCB of the task. It contains the task's info
#[allow(unused)]
// we put it in executor crate to use "pub(crate)" to make it can be used in the other mod in order to reduce coupling
pub struct OS_TCB{
    // it maybe None
    OSTCBStkPtr:Option<OS_STK_REF>, /* Pointer to current top of stack                         */
    // Task specific extension. If the OS_TASK_CREATE_EXT_EN feature is not active, it will be None
    OSTCBExtInfo:Option<OS_TCB_EXT>, 

    OSTCBNext:SyncUnsafeCell<Option<OS_TCB_REF>>, /* Pointer to next     TCB in the TCB list                 */
    OSTCBPrev:SyncUnsafeCell<Option<OS_TCB_REF>>, /* Pointer to previous TCB in the TCB list                 */

    // the poll fn that will be called by the executor. In the func, a waker will be create.
    OS_POLL_FN:SyncUnsafeCell<Option<unsafe fn(OS_TCB_REF)>>,
    
    #[cfg(feature="OS_EVENT_EN")]
    OSTCBEventPtr:Option<OS_EVENT_REF>, /* Pointer to event control block                */
    
    #[cfg(any(all(feature = "OS_Q_EN", feature = "OS_MAX_QS"), feature = "OS_MBOX_EN"))]
    OSTCBMsg:PTR, /* Message received from OSMboxPost() or OSQPost()         */
    
    OSTCBDly:INT32U,     /* Nbr ticks to delay task or, timeout waiting for event   */
    OSTCBStat:State,     /* Task      status                                        */
    OSTCBStatPend:INT8U, /* Task PEND status                                        */
    OSTCBPrio:INT8U,     /* Task priority (0 == highest)                            */
    
    OSTCBX:INT8U,      /* Bit position in group  corresponding to task priority   */
    OSTCBY:INT8U,      /* Index into ready table corresponding to task priority   */
    OSTCBBitX:OS_PRIO, /* Bit mask to access bit position in ready table          */
    OSTCBBitY:OS_PRIO, /* Bit mask to access bit position in ready group          */
    
    #[cfg(feature="OS_TASK_DEL_EN")]
    OSTCBDelReq:INT8U,  /* Indicates whether a task needs to delete itself         */
    
    #[cfg(feature="OS_TASK_PROFILE_EN")]
    OSTCBCtxSwCtr:INT32U,    /* Number of time the task was switched in                 */
    OSTCBCyclesTot:INT32U,   /* Total number of clock cycles the task has been running  */
    OSTCBCyclesStart:INT32U, /* Snapshot of cycle counter at start of task resumption   */
    OSTCBStkBase:OS_STK,    /* Pointer to the beginning of the task stack              */
    OSTCBStkUsed:INT32U,     /* Number of bytes used from the stack                     */
    
    #[cfg(feature="OS_TASK_REG_TBL_SIZE")]
    OSTCBRegTbl:[INT32U;OS_TASK_REG_TBL_SIZE],
    
    #[cfg(feature="OS_TASK_NAME_EN")]
    OSTCBTaskName: String,
    // #[cfg(feature="OS_TASK_CREATE_EXT_EN")]
    // OS_TLS OSTCBTLSTbl[OS_TLS_TBL_SIZE];
    
    // OS_EVENT **OSTCBEventMultiPtr; /* Pointer to multiple  event control blocks               */
    // OS_EVENT *OSTCBEventMultiRdy;  /* Pointer to the first event control block readied        */
    // OS_FLAG_NODE *OSTCBFlagNode; /* Pointer to event flag node                              */

    // OS_FLAGS OSTCBFlagsRdy; /* Event flags that made task ready to run                 */
}

#[cfg(feature="OS_TASK_CREATE_EXT_EN")]
#[allow(unused)]
pub(crate) struct OS_TCB_EXT{
    OSTCBExtPtr:PTR,      /* Pointer to user definable data for TCB extension        */
    OSTCBStkBottom:Option<OS_STK_REF>, /* Pointer to bottom of stack                              */
    OSTCBStkSize:INT32U,    /* Size of task stack (in number of stack elements)        */
    OSTCBOpt:INT16U,        /* Task options as passed by OSTaskCreateExt()             */
    OSTCBId:INT16U,         /* Task ID (0..65535)                                      */
}

/// the storage of the task. It contains the task's TCB and the future
#[allow(unused)]
pub struct OS_TASK_STORAGE<F: Future + 'static>{
    task_tcb:OS_TCB,
    // this part is invisible to other crate
    // by noah: maybe we need to use raw ptr
    future:UninitCell<F>,
}

/// the ref of the TCB. In other crate only it can be used to access the TCB
#[derive(Clone, Copy)]
#[allow(unused)]
pub struct OS_TCB_REF{
    /// the pointer to the TCB
    pub ptr:NonNull<OS_TCB>,
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

impl <F: Future + 'static>OS_TASK_STORAGE<F>{
    const NEW: Self = Self::new();
    /// create a new OS_TASK_STORAGE
    // Take a lazy approach, which means the TCB will be init when call the init func of TCB
    // this func will be used to init the global array
    pub const fn new()->Self{
        Self{
            task_tcb:OS_TCB{
                OSTCBStkPtr:None,
                OSTCBExtInfo:None,
                OSTCBNext:SyncUnsafeCell::new(None),
                OSTCBPrev:SyncUnsafeCell::new(None),
                OS_POLL_FN:SyncUnsafeCell::new(None),
                #[cfg(feature="OS_EVENT_EN")]
                OSTCBEventPtr:None,
                #[cfg(any(all(feature = "OS_Q_EN", feature = "OS_MAX_QS"), feature = "OS_MBOX_EN"))]
                OSTCBMsg:0 as PTR,
                OSTCBDly:0,
                OSTCBStat:State::new(),
                OSTCBStatPend:0,
                OSTCBPrio:0,
                OSTCBX:0,
                OSTCBY:0,
                OSTCBBitX:0,
                OSTCBBitY:0,
                #[cfg(feature="OS_TASK_DEL_EN")]
                OSTCBDelReq:0,
                #[cfg(feature="OS_TASK_PROFILE_EN")]
                OSTCBCtxSwCtr:0,
                OSTCBCyclesTot:0,
                OSTCBCyclesStart:0,
                OSTCBStkBase:0,
                OSTCBStkUsed:0,
                #[cfg(feature="OS_TASK_REG_TBL_SIZE")]
                OSTCBRegTbl:[0;OS_TASK_REG_TBL_SIZE],
                #[cfg(feature="OS_TASK_NAME_EN")]
                OSTCBTaskName:String::new(),
            },
            future:UninitCell::uninit(),
        }
    }

    /// init the storage of the task, just like the spawn in Embassy
    //  this func will be called by OS_TASK_CTREATE
    pub fn init(&'static self, _future:impl FnOnce() -> F){
         
    }

    /// the poll fun called by the executor
    pub fn poll(){

    }

    /// this func will be called to create a new task(TCB)
    // refer to the get of TaskPoolRef in embassy
    pub fn claim() -> OS_TCB_REF {
        // by noah: for we can create task after OSTaskCreate, so we need a cs
        critical_section::with(|cs| {
            let task_storage = ARENA.alloc::<OS_TASK_STORAGE<F>>(cs);
            // create a new task which is not init
            task_storage.write(OS_TASK_STORAGE::new());
            // by noah：no panic will occurred here because if the Arena is not enough, the program will panic when alloc
            OS_TCB_REF{ptr:NonNull::new(task_storage as *mut _ as _).unwrap()}
        })
    }
}

unsafe impl Sync for OS_TCB_REF{}
unsafe impl Send for OS_TCB_REF{}

impl Default for OS_TCB_REF{
    // this func will not be called
    fn default()->Self{
        // by noah:dangling is used to create a dangling pointer, which is just like the null pointer in C
        OS_TCB_REF{ptr:NonNull::dangling()}
    }
}

// impl deref for OS_TCB_REF
impl Deref for OS_TCB_REF{
    type Target = OS_TCB;
    fn deref(&self)->&Self::Target{
        unsafe{self.ptr.as_ref()}
    }
}
// impl deref for mut OS_TCB_REF
impl DerefMut for OS_TCB_REF{
    fn deref_mut(&mut self)->&mut Self::Target{
        unsafe{self.ptr.as_mut()}
    }
}

impl OS_TCB_REF {
    fn new<F: Future + 'static>(task: &'static OS_TASK_STORAGE<F>) -> Self {
        Self {
            ptr: NonNull::from(task).cast(),
        }
    }

    /// Safety: The pointer must have been obtained with `Task::as_ptr`
    pub(crate) unsafe fn from_ptr(ptr: *const OS_TCB) -> Self {
        Self {
            ptr: NonNull::new_unchecked(ptr as *mut OS_TCB),
        }
    }

    pub(crate) fn header(self) -> &'static OS_TCB {
        unsafe { self.ptr.as_ref() }
    }

    /// The returned pointer is valid for the entire OS_TASK_STORAGE.
    pub(crate) fn as_ptr(self) -> *const OS_TCB {
        self.ptr.as_ptr()
    }
}

// impl<F: Future + 'static, const N: usize> TaskPool<F, N> {
//     /// Create a new TaskPool, with all tasks in non-spawned state.
//     // by noah：this func will be called to init OSTCBTbl by using lazy_static
//     pub const fn new() -> Self {
//         Self {
//             // in uC, "N" will be set as OS_MAX_TASKS + OS_N_SYS_TASKS, which is the number of the TCB
//             pool: [OS_TASK_STORAGE::NEW; N],
//         }
//     }

//     // fn spawn_impl<T>(&'static self, future: impl FnOnce() -> F) -> SpawnToken<T> {
//     //     match self.pool.iter().find_map(AvailableTask::claim) {
//     //         Some(task) => task.initialize_impl::<T>(future),
//     //         None => SpawnToken::new_failed(),
//     //     }
//     // }

//     // // Try to spawn a task in the pool.
//     // //
//     // // See [`OS_TASK_STORAGE::spawn()`] for details.
//     // //
//     // // This will loop over the pool and spawn the task in the first storage that
//     // // is currently free. If none is free, a "poisoned" SpawnToken is returned,
//     // // which will cause [`Spawner::spawn()`](super::Spawner::spawn) to return the error.
//     // pub fn spawn(&'static self, future: impl FnOnce() -> F) -> SpawnToken<impl Sized> {
//     //     self.spawn_impl::<F>(future)
//     // }

//     // // Like spawn(), but allows the task to be send-spawned if the args are Send even if
//     // // the future is !Send.
//     // //
//     // // Not covered by semver guarantees. DO NOT call this directly. Intended to be used
//     // // by the Embassy macros ONLY.
//     // //
//     // // SAFETY: `future` must be a closure of the form `move || my_async_fn(args)`, where `my_async_fn`
//     // // is an `async fn`, NOT a hand-written `Future`.
//     // #[doc(hidden)]
//     // pub unsafe fn _spawn_async_fn<FutFn>(&'static self, future: FutFn) -> SpawnToken<impl Sized>
//     // where
//     //     FutFn: FnOnce() -> F,
//     // {
//     //     // See the comment in AvailableTask::__initialize_async_fn for explanation.
//     //     self.spawn_impl::<FutFn>(future)
//     // }
// }

// /// by noah：we need to impl Send and Sync for TaskPoolRef
// unsafe impl Send for TaskPoolRef{}
// unsafe impl Sync for TaskPoolRef{}

// /// this part is copied from Embassy
// impl TaskPoolRef {
//     /// new a TaskPoolRef. the ptr is null
//     pub const fn new() -> Self {
//         Self {
//             ptr: Mutex::new(RefCell::new(null_mut())),
//         }
//     }

//     /// Get the pool for this ref, allocating it from the arena the first time.
//     ///
//     /// safety: for a given TaskPoolRef instance, must always call with the exact
//     /// same generic params.
//     pub unsafe fn get<F: Future, const N: usize>(&'static self) -> &'static TaskPool<F, N> {
//         critical_section::with(|cs| {
//             let mut ptr = self.ptr.borrow_ref_mut(cs);
//             if ptr.is_null() {
//                 // by noah：we won't use ARENA.alloc as embassy. We just define a TaskPool
//                 let pool = ARENA.alloc::<TaskPool<F, N>>(cs);
//                 pool.write(TaskPool::new());
//                 *ptr = pool as *mut _ as _;
//             }

//             unsafe { &*(*ptr as *const _) }
//         })
//     }
// }

impl<F: Future + 'static> AvailableTask<F> {
    // // Try to claim a [`OS_TASK_STORAGE`].
    // //
    // // This function returns `None` if a task has already been spawned and has not finished running.
    // pub fn claim(task: &'static OS_TASK_STORAGE<F>) -> Option<Self> {
    //     task.raw.state.spawn().then(|| Self { task })
    // }

    // fn initialize_impl<S>(self, future: impl FnOnce() -> F) -> SpawnToken<S> {
    //     unsafe {
    //         self.task.raw.poll_fn.set(Some(OS_TASK_STORAGE::<F>::poll));
    //         self.task.future.write_in_place(future);

    //         let task = OS_TCB_REF::new(self.task);

    //         SpawnToken::new(task)
    //     }
    // }

    // /// Initialize the [`OS_TASK_STORAGE`] to run the given future.
    // pub fn initialize(self, future: impl FnOnce() -> F) -> SpawnToken<F> {
    //     self.initialize_impl::<F>(future)
    // }

    // // Initialize the [`OS_TASK_STORAGE`] to run the given future.
    // //
    // // # Safety
    // //
    // // `future` must be a closure of the form `move || my_async_fn(args)`, where `my_async_fn`
    // // is an `async fn`, NOT a hand-written `Future`.
    // #[doc(hidden)]
    // pub unsafe fn __initialize_async_fn<FutFn>(self, future: impl FnOnce() -> F) -> SpawnToken<FutFn> {
    //     // When send-spawning a task, we construct the future in this thread, and effectively
    //     // "send" it to the executor thread by enqueuing it in its queue. Therefore, in theory,
    //     // send-spawning should require the future `F` to be `Send`.
    //     //
    //     // The problem is this is more restrictive than needed. Once the future is executing,
    //     // it is never sent to another thread. It is only sent when spawning. It should be
    //     // enough for the task's arguments to be Send. (and in practice it's super easy to
    //     // accidentally make your futures !Send, for example by holding an `Rc` or a `&RefCell` across an `.await`.)
    //     //
    //     // We can do it by sending the task args and constructing the future in the executor thread
    //     // on first poll. However, this cannot be done in-place, so it'll waste stack space for a copy
    //     // of the args.
    //     //
    //     // Luckily, an `async fn` future contains just the args when freshly constructed. So, if the
    //     // args are Send, it's OK to send a !Send future, as long as we do it before first polling it.
    //     //
    //     // (Note: this is how the generators are implemented today, it's not officially guaranteed yet,
    //     // but it's possible it'll be guaranteed in the future. See zulip thread:
    //     // https://rust-lang.zulipchat.com/#narrow/stream/187312-wg-async/topic/.22only.20before.20poll.22.20Send.20futures )
    //     //
    //     // The `FutFn` captures all the args, so if it's Send, the task can be send-spawned.
    //     // This is why we return `SpawnToken<FutFn>` below.
    //     //
    //     // This ONLY holds for `async fn` futures. The other `spawn` methods can be called directly
    //     // by the user, with arbitrary hand-implemented futures. This is why these return `SpawnToken<F>`.
    //     self.initialize_impl::<FutFn>(future)
    // }
}

/*
****************************************************************************************************************************************
*                                                             type define  
****************************************************************************************************************************************
*/