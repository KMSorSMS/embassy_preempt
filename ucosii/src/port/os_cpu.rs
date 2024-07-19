//! about the cpu

use crate::heap::init_heap;

/// finish the init part of the CPU/MCU
pub fn OSInitHookBegin(){
    init_heap();
}