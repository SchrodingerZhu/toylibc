#![allow(non_camel_case_types)]

pub type errno_t = u8;
pub type char_t = i8;
pub type uchar_t = u8;
pub type short_t = i16;
pub type ushort_t = u16;
pub type int_t = i32;
pub type uint_t = u32;
pub type long_t = i64;
pub type longlong_t = i64;
pub type ulong_t = u64;
pub type ulonglong_t = u64;
pub type off_t = i64;
pub type key_t = int_t;
pub type rlim_t = u64;
// stddef
pub type ssize_t = long_t;
pub type size_t = ulong_t;
pub type ptrdiff_t = long_t;

// stdint
pub type int8_t = char_t;
pub type int16_t = short_t;
pub type int32_t = int_t;
pub type int64_t = longlong_t;
pub type uint8_t = uchar_t;
pub type uint16_t = ushort_t;
pub type uint32_t = uint_t;
pub type uint64_t = ulonglong_t;
pub type int_least8_t = int8_t;
pub type int_least16_t = int16_t;
pub type int_least32_t = int32_t;
pub type int_least64_t = int64_t;
pub type uint_least8_t = uint8_t;
pub type uint_least16_t = uint16_t;
pub type uint_least32_t = uint32_t;
pub type uint_least64_t = uint64_t;
pub type int_fast8_t = int8_t;
pub type int_fast16_t = int16_t;
pub type int_fast32_t = int32_t;
pub type int_fast64_t = int64_t;
pub type uint_fast8_t = uint8_t;
pub type uint_fast16_t = uint16_t;
pub type uint_fast32_t = uint32_t;
pub type uint_fast64_t = uint64_t;
pub type intptr_t = long_t;
pub type uintptr_t = ulong_t;
pub type intmax_t = long_t;
pub type uintmax_t = ulong_t;

#[repr(u8)]
pub enum void_t {
    __variant1,
    __variant2,
}

pub type __kernel_size_t = ulong_t;
pub type __kernel_ssize_t = long_t;
pub type __kernel_ptrdiff_t = long_t;

pub const NSIG_BPW: usize = 64;
#[no_mangle]
pub static _NSIG_BPW: usize = NSIG_BPW;

#[repr(packed)]
pub struct epoll_event {
    pub events: u32,
    pub data: u64,
}

pub struct stat {
    pub std_dev: ulong_t,
    pub st_ino: ulong_t,
    pub st_nlink: ulong_t,

    pub st_mode: uint_t,
    pub st_uid: uint_t,
    pub st_gid: uint_t,
    pub __pad0: uint_t,
    pub st_rdev: ulong_t,
    pub st_size: long_t,
    pub st_blksize: long_t,
    pub st_blocks: long_t,

    pub st_atime: ulong_t,
    pub st_atime_nsec: ulong_t,
    pub st_mtime: ulong_t,
    pub st_mtime_nsec: ulong_t,
    pub st_ctime: ulong_t,
    pub st_ctime_nsec: ulong_t,
    pub __unused: [long_t; 3],
}

pub struct iocb {
    pub aio_data: u64,
    pub aio_key: u32,
    pub aio_reserved1: u32,
    pub aio_lio_opcode: u16,
    pub aio_reqprio: i16,
    pub aio_fildes: u32,
    pub aio_buf: u64,
    pub aio_nbytes: u64,
    pub aio_offset: i64,
    pub aio_reserved2: u64,
    pub aio_flags: u32,
    pub aio_resfd: u32,
}

pub struct pt_regs {
    pub r15: ulong_t,
    pub r14: ulong_t,
    pub r13: ulong_t,
    pub r12: ulong_t,
    pub bp: ulong_t,
    pub bx: ulong_t,
    pub r11: ulong_t,
    pub r10: ulong_t,
    pub r9: ulong_t,
    pub r8: ulong_t,
    pub ax: ulong_t,
    pub cx: ulong_t,
    pub dx: ulong_t,
    pub si: ulong_t,
    pub di: ulong_t,
    pub orig_ax: ulong_t,
    pub ip: ulong_t,
    pub cs: ulong_t,
    pub flags: ulong_t,
    pub sp: ulong_t,
    pub ss: ulong_t,
}

pub struct stack_t {
    pub ss_sp: *mut void_t,
    pub ss_flags: int_t,
    pub ss_size: size_t,
}

pub struct cap_user_data_t {
    pub effective: u32,
    pub permitted: u32,
    pub inheritable: u32,
}

pub struct cap_user_header_t {
    pub version: u32,
    pub pid: int_t,
}

pub struct msg {
    pub byte3: u8,
    pub byte2: u8,
    pub eth_id: u8,
    pub cmd: u8,
    pub byte7: u8,
    pub byte6: u8,
    pub byte5: u8,
    pub byte4: u8,
}

pub type __statfs_word = long_t;

pub const FD_SETSIZE: usize = 1024;

#[no_mangle]
pub static __FD_SETSIZE: usize = FD_SETSIZE;

pub struct __kernel_fd_set {
    // XXX size_of
    pub fds_bits: [ulong_t; (FD_SETSIZE / (8 * 8))],
}

pub struct getcpu_cache {
    // XXX size_of
    pub blob: [ulong_t; 128 / 8],
}

pub struct sysinfo {
    pub uptime: long_t,
    pub loads: [ulong_t; 3],
    pub totalram: ulong_t,
    pub freeram: ulong_t,
    pub sharedram: ulong_t,
    pub bufferram: ulong_t,
    pub totalswap: ulong_t,
    pub freeswap: ulong_t,
    pub procs: u16,
    pub pad: u16,
    pub totalhigh: ulong_t,
    pub freehigh: ulong_t,
    pub mem_unit: u32,
    // XXX size_of
    pub _f: [char_t; 20 - 2 * 8 - 4],
}

#[repr(C)]
pub struct tm {
    pub tm_sec: int_t,
    pub tm_min: int_t,
    pub tm_hour: int_t,
    pub tm_mday: int_t,
    pub tm_mon: int_t,
    pub tm_year: int_t,
    pub tm_wday: int_t,
    pub tm_yday: int_t,
    pub tm_isdst: int_t,
    pub tm_gmtoff: long_t,
    pub tm_zone: *const char_t,
}

pub type clock_t = longlong_t;
pub type time_t = i64;
pub type clockid_t = int_t;

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: long_t,
}

pub type tv_nsec_t = long_t;

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct rlimit {
    rlim_cur: rlim_t,
    rlim_max: rlim_t,
}


pub type pid_t = i32;
pub type uid_t = u32;
pub type gid_t = u32;
pub type in_addr_t = u32;
pub type in_port_t = u16;
pub type sighandler_t = size_t;
pub type cc_t = uchar_t;
pub type useconds_t = u32;
pub type dev_t = u64;
pub type socklen_t = u32;
pub type mode_t = u32;
pub type ino64_t = u64;
pub type off64_t = i64;
pub type blkcnt64_t = i64;
pub type rlim64_t = u64;
pub type mqd_t = int_t;
pub type nfds_t = ulong_t;
pub type nl_item = int_t;
pub type idtype_t = uint_t;
pub type loff_t = longlong_t;
pub type pthread_key_t = uint_t;

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct ipc_perm {
    pub __ipc_perm_key: key_t,
    pub uid: uid_t,
    pub gid: gid_t,
    pub cuid: uid_t,
    pub cgid: gid_t,
    pub mode: mode_t,
    pub __ipc_perm_seq: int_t,
    __pad1: long_t,
    __pad2: long_t,
}

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct shmid_ds {
    pub shm_perm: ipc_perm,
    pub shm_segsz: size_t,
    pub shm_atime: time_t,
    pub shm_dtime: time_t,
    pub shm_ctime: time_t,
    pub shm_cpid: pid_t,
    pub shm_lpid: pid_t,
    pub shm_nattch: ulong_t,
    __pad1: long_t,
    __pad2: long_t,
}

