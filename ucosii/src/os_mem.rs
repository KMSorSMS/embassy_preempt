/*
*********************************************************************************************************
*                                              uC/OS-II
*                                        The Real-Time Kernel
*
*                    Copyright 1992-2021 Silicon Laboratories Inc. www.silabs.com
*
*                                 SPDX-License-Identifier: APACHE-2.0
*
*               This software is subject to an open source license and is distributed by
*                Silicon Laboratories Inc. pursuant to the terms of the Apache License,
*                    Version 2.0 available at www.apache.org/licenses/LICENSE-2.0.
*                                    rewrite by liam and noah
*
*********************************************************************************************************
*/

/*
*********************************************************************************************************
*
*                                           MEMORY MANAGEMENT
*
* Filename : os_mem.rs
* Version  : V0.0.1
*********************************************************************************************************
*/

use crate::ucosii::*;
use crate::*;
use core::mem::size_of;

/*
*********************************************************************************************************
*                                      CREATE A MEMORY PARTITION
*
* Description : Create a fixed-sized memory partition that will be managed by uC/OS-II.
*
* Arguments   : addr     is the starting address of the memory partition
*
*               nblks    is the number of memory blocks to create from the partition.
*
*               blksize  is the size (in bytes) of each block in the memory partition.
*
*               perr     is a pointer to a variable containing an error message which will be set by
*                        this function to either:
*
*                        OS_ERR_NONE                     if the memory partition has been created correctly.
*                        OS_ERR_ILLEGAL_CREATE_RUN_TIME  if you tried to create a memory partition after
*                                                        safety critical operation started.
*                        OS_ERR_MEM_INVALID_ADDR         if you are specifying an invalid address for the memory
*                                                        storage of the partition or, the block does not align
*                                                        on a pointer boundary
*                        OS_ERR_MEM_INVALID_PART         no free partitions available
*                        OS_ERR_MEM_INVALID_BLKS         user specified an invalid number of blocks (must be >= 2)
*                        OS_ERR_MEM_INVALID_SIZE         user specified an invalid block size
*                                                          - must be greater than the size of a pointer
*                                                          - must be able to hold an integral number of pointers
* Returns    : != (OS_MEM *)0  is the partition was created
*              == (OS_MEM *)0  if the partition was not created because of invalid arguments or, no
*                              free partition is available.
*********************************************************************************************************
*/
#[allow(unused)]
#[no_mangle]
pub fn OSMemCreate(addr: VoidPtr, nblks: u32, blksize: u32, perr: *mut OS_ERR_STATE) -> *mut OS_MEM {
    let pmem: *mut OS_MEM;
    let mut pblk: VoidPtr;
    let plink: *mut VoidPtr;
    let mut loops: u32;
    let mut i: u32;
    #[cfg(feature = "OS_SAFETY_CRITICAL")]
    {
        if (perr.is_null()) {
            // you should provide this function below
            OS_SAFETY_CRITICAL_EXCEPTION();
            return (0 as *mut OS_MEM);
        }
    }
    // escape OS_SAFETY_CRITICAL_IEC61508
    #[cfg(feature = "OS_ARG_CHK_EN")]
    {
        if addr.is_null() {
            /* Must pass a valid address for the memory part.*/
            *perr = OS_ERR_MEM_INVALID_ADDR;
            return (0 as *mut OS_MEM);
        }
        if (addr as u32) & (size_of(VoidPtr) - 1) != 0 {
            /* Must be pointer size aligned.*/
            *perr = OS_ERR_MEM_INVALID_ADDR;
            return (0 as *mut OS_MEM);
        }
        if (nblks < 2) {
            /* Must have at least 2 blocks.*/
            *perr = OS_ERR_MEM_INVALID_BLKS;
            return (0 as *mut OS_MEM);
        }
        if blksize < size_of(VoidPtr) {
            /* Must be able to hold at least a pointer.*/
            *perr = OS_ERR_MEM_INVALID_SIZE;
            return (0 as *mut OS_MEM);
        }
    }
    // enter critical to protect the memory partition
    if critical_section::with(|cs|{
        
    }){

    }
}
