#![allow(dead_code)]

use crate::types::*;

pub const NULL: intptr_t = 0;
pub const O_RDONLY: int_t = 0;
pub const ENOMEM: int_t = 12; /* Cannot allocate memory */
pub const EACCES: int_t = 13; /* Permission denied */
pub const EEXIST: int_t = 17; /* File exists */
pub const EISDIR: int_t = 21; /* Is a directory */
pub const EINVAL: int_t = 22; /* Invalid argument */
pub const ERANGE: int_t = 34; /* Result too large */
pub const FUTEX_WAIT: int_t = 0;
pub const FUTEX_WAKE: int_t = 1;
pub const FUTEX_FD: int_t = 2;
pub const FUTEX_REQUEUE: int_t = 3;
pub const FUTEX_CMP_REQUEUE: int_t = 4;
pub const FUTEX_WAKE_OP: int_t = 5;
pub const FUTEX_LOCK_PI: int_t = 6;
pub const FUTEX_UNLOCK_PI: int_t = 7;
pub const FUTEX_TRYLOCK_PI: int_t = 8;
pub const FUTEX_WAIT_BITSET: int_t = 9;
pub const FUTEX_WAKE_BITSET: int_t = 10;
pub const FUTEX_WAIT_REQUEUE_PI: int_t = 11;
pub const FUTEX_CMP_REQUEUE_PI: int_t = 12;

pub const FUTEX_PRIVATE_FLAG: int_t = 128;
pub const FUTEX_CLOCK_REALTIME: int_t = 256;
pub const FUTEX_CMD_MASK: int_t =
    !(FUTEX_PRIVATE_FLAG | FUTEX_CLOCK_REALTIME);

pub const NSEC_PER_SEC: u64 = 1_000_000_000;

pub const CLOCK_REALTIME: clockid_t = 0;
pub const CLOCK_MONOTONIC: clockid_t = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: clockid_t = 2;
pub const CLOCK_THREAD_CPUTIME_ID: clockid_t = 3;
pub const CLOCK_MONOTONIC_RAW: clockid_t = 4;
pub const CLOCK_REALTIME_COARSE: clockid_t = 5;
pub const CLOCK_MONOTONIC_COARSE: clockid_t = 6;
pub const CLOCK_BOOTTIME: clockid_t = 7;
pub const CLOCK_REALTIME_ALARM: clockid_t = 8;
pub const CLOCK_BOOTTIME_ALARM: clockid_t = 9;

pub const INT_MAX: int_t = 2147483647;