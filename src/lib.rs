#![no_builtins]
#![feature(core_intrinsics)]
#![feature(proc_macro_hygiene)]
#![feature(type_ascription)]
#![crate_type = "cdylib"]
#![allow(dead_code)]
#![feature(asm)]
#![feature(thread_local)]

#![cfg_attr(not(test), no_std)]
extern crate alloc;
#[macro_use]
extern crate lazy_static;

pub use complex::*;
pub use io::*;
pub use memory::*;
pub use panic::*;
pub use types::*;
pub use utils::*;

mod time;
mod panic;
mod complex;
mod memory;
mod types;
pub mod utils;
mod math;
mod io;
mod posix;
mod constants;
mod c_style;

