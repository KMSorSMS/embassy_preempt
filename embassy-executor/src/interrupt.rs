//! Interrupt-mode executor.

use core::cell::UnsafeCell;
use core::mem::MaybeUninit;

use atomic_polyfill::{AtomicBool, Ordering};

use crate::raw::{self, OpaqueInterruptContext, Pender};

/// Architecture-specific interface for an interrupt-mode executor. This trait describes what data
/// should be passed to the [`InterruptExecutor`]'s pender, and how to enable the interrupt that
/// triggers polling the executor.
// TODO: Name pending
pub trait InterruptContext {
    /// A pointer-sized piece of data that is passed to the pender function.
    ///
    /// Usually, the context contains the interrupt that should be used to wake the executor.
    fn context(&self) -> OpaqueInterruptContext;

    /// Enabled the interrupt request.
    fn enable(&self);
}

/// Interrupt mode executor.
///
/// This executor runs tasks in interrupt mode. The interrupt handler is set up
/// to poll tasks, and when a task is woken the interrupt is pended from software.
///
/// This allows running async tasks at a priority higher than thread mode. One
/// use case is to leave thread mode free for non-async tasks. Another use case is
/// to run multiple executors: one in thread mode for low priority tasks and another in
/// interrupt mode for higher priority tasks. Higher priority tasks will preempt lower
/// priority ones.
///
/// It is even possible to run multiple interrupt mode executors at different priorities,
/// by assigning different priorities to the interrupts. For an example on how to do this,
/// See the 'multiprio' example for 'embassy-nrf'.
///
/// To use it, you have to pick an interrupt that won't be used by the hardware.
/// Some chips reserve some interrupts for this purpose, sometimes named "software interrupts" (SWI).
/// If this is not the case, you may use an interrupt from any unused peripheral.
///
/// It is somewhat more complex to use, it's recommended to use the
/// [`crate::thread::ThreadModeExecutor`] instead, if it works for your use case.
pub struct InterruptModeExecutor {
    started: AtomicBool,
    executor: UnsafeCell<MaybeUninit<raw::Executor>>,
}

unsafe impl Send for InterruptModeExecutor {}
unsafe impl Sync for InterruptModeExecutor {}

impl InterruptModeExecutor {
    /// Create a new, not started `InterruptExecutor`.
    #[inline]
    pub const fn new() -> Self {
        Self {
            started: AtomicBool::new(false),
            executor: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    /// Executor interrupt callback.
    ///
    /// # Safety
    ///
    /// You MUST call this from the interrupt handler, and from nowhere else.
    pub unsafe fn on_interrupt(&'static self) {
        let executor = unsafe { (&*self.executor.get()).assume_init_ref() };
        executor.poll();
    }

    /// Start the executor.
    ///
    /// This initializes the executor, enables the interrupt, and returns.
    /// The executor keeps running in the background through the interrupt.
    ///
    /// This returns a [`SendSpawner`] you can use to spawn tasks on it. A [`SendSpawner`]
    /// is returned instead of a [`Spawner`](embassy_executor::Spawner) because the executor effectively runs in a
    /// different "thread" (the interrupt), so spawning tasks on it is effectively
    /// sending them.
    ///
    /// To obtain a [`Spawner`](embassy_executor::Spawner) for this executor, use [`Spawner::for_current_executor()`](embassy_executor::Spawner::for_current_executor()) from
    /// a task running in it.
    ///
    /// # Interrupt requirements
    ///
    /// You must write the interrupt handler yourself, and make it call [`on_interrupt()`](Self::on_interrupt).
    ///
    /// This method already enables (unmasks) the interrupt, you must NOT do it yourself.
    ///
    /// You must set the interrupt priority before calling this method. You MUST NOT
    /// do it after.
    ///
    pub fn start(&'static self, irq: impl InterruptContext) -> crate::SendSpawner {
        if self
            .started
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            panic!("InterruptExecutor::start() called multiple times on the same executor.");
        }

        unsafe {
            (&mut *self.executor.get())
                .as_mut_ptr()
                .write(raw::Executor::new(Pender::Interrupt(irq.context())))
        }

        let executor = unsafe { (&*self.executor.get()).assume_init_ref() };

        irq.enable();

        executor.spawner().make_send()
    }

    /// Get a SendSpawner for this executor
    ///
    /// This returns a [`SendSpawner`] you can use to spawn tasks on this
    /// executor.
    ///
    /// This MUST only be called on an executor that has already been spawned.
    /// The function will panic otherwise.
    pub fn spawner(&'static self) -> crate::SendSpawner {
        if !self.started.load(Ordering::Acquire) {
            panic!("InterruptExecutor::spawner() called on uninitialized executor.");
        }
        let executor = unsafe { (&*self.executor.get()).assume_init_ref() };
        executor.spawner().make_send()
    }
}
