//! Drone Stream.
//!
//! This module implements standard output/error interface, which mimics Rust's
//! standard library.
//! 
//! Drone Stream definitions.

#![warn(missing_docs, unsafe_op_in_unsafe_fn)]
#![warn(clippy::pedantic)]

#![cfg_attr(feature = "host", allow(unused_imports, dead_code, unreachable_code, unused_variables))]
pub mod linked_list;
pub mod soft_atomic;