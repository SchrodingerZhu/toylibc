#![allow(dead_code)]
#![allow(non_upper_case_globals)]

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



pub const IPCOP_semop: int_t = 1;
pub const IPCOP_semget: int_t = 2;
pub const IPCOP_semctl: int_t = 3;
pub const IPCOP_msgsnd: int_t = 11;
pub const IPCOP_msgrcv: int_t = 12;
pub const IPCOP_msgget: int_t = 13;
pub const IPCOP_msgctl: int_t = 14;
pub const IPCOP_shmat: int_t = 21;
pub const IPCOP_shmdt: int_t = 22;
pub const IPCOP_shmget: int_t = 23;
pub const IPCOP_shmctl: int_t = 24;
pub const IPC_STAT: int_t = 2;
pub const IPC_64: int_t = 0;
pub const IPC_MODERN: int_t = 0x100;
pub const IPC_TIME64: int_t = (IPC_STAT & 0x100);


pub const INT_MAX: int_t = 2147483647;
pub const INT8_MIN: int8_t = core::i8::MIN;
pub const INT16_MIN: int16_t = core::i16::MIN;
pub const INT32_MIN: int32_t = core::i32::MIN;
pub const INT64_MIN: int64_t = core::i64::MIN;

pub const INT8_MAX: int8_t = core::i8::MAX;
pub const INT16_MAX: int16_t = core::i16::MAX;
pub const INT32_MAX: int32_t = core::i32::MAX;
pub const INT64_MAX: int64_t = core::i64::MAX;
pub const UINT8_MAX: uint8_t = core::u8::MAX;
pub const UINT16_MAX: uint16_t = core::u16::MAX;
pub const UINT32_MAX: uint32_t = core::u32::MAX;
pub const UINT64_MAX: uint64_t = core::u64::MAX;


pub const INT_FAST16_MIN: int_fast16_t = INT32_MIN as _;
pub const INT_FAST32_MIN: int_fast32_t = INT32_MIN;

pub const INT_FAST16_MAX: int_fast16_t = INT32_MAX as _;
pub const INT_FAST32_MAX: int_fast32_t = INT32_MAX;

pub const UINT_FAST16_MAX: uint_fast16_t = UINT32_MAX as _;
pub const UINT_FAST32_MAX: uint_fast32_t = UINT32_MAX;

pub const INTPTR_MIN: intptr_t = INT64_MIN;
pub const INTPTR_MAX: intptr_t = INT64_MAX;
pub const UINTPTR_MAX: uintptr_t = UINT64_MAX as _;
pub const PTRDIFF_MIN: ptrdiff_t = INT64_MIN;
pub const PTRDIFF_MAX: ptrdiff_t = INT64_MAX;
pub const SIZE_MAX: size_t = UINT64_MAX as _;
pub const IOV_MAX: size_t = INT_MAX as u64;