use core::convert::TryInto;
use core::time::Duration;

use syscalls::syscall2;

use crate::{clockid_t, timespec};
use crate::constants::*;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
struct Timespec {
    t: timespec,
}

impl Timespec {
    const fn zero() -> Timespec {
        Timespec {
            t: timespec { tv_sec: 0, tv_nsec: 0 },
        }
    }

    fn sub_timespec(&self, other: &Timespec) -> Result<Duration, Duration> {
        if self >= other {
            Ok(if self.t.tv_nsec >= other.t.tv_nsec {
                Duration::new((self.t.tv_sec - other.t.tv_sec) as u64,
                              (self.t.tv_nsec - other.t.tv_nsec) as u32)
            } else {
                Duration::new((self.t.tv_sec - 1 - other.t.tv_sec) as u64,
                              self.t.tv_nsec as u32 + (NSEC_PER_SEC as u32) -
                                  other.t.tv_nsec as u32)
            })
        } else {
            match other.sub_timespec(self) {
                Ok(d) => Err(d),
                Err(d) => Ok(d),
            }
        }
    }

    fn checked_add_duration(&self, other: &Duration) -> Option<Timespec> {
        let mut secs = other
            .as_secs()
            .try_into() // <- target type would be `libc::time_t`
            .ok()
            .and_then(|secs| self.t.tv_sec.checked_add(secs))?;

        // Nano calculations can't overflow because nanos are <1B which fit
        // in a u32.
        let mut nsec = other.subsec_nanos() + self.t.tv_nsec as u32;
        if nsec >= NSEC_PER_SEC as u32 {
            nsec -= NSEC_PER_SEC as u32;
            secs = secs.checked_add(1)?;
        }
        Some(Timespec {
            t: timespec {
                tv_sec: secs,
                tv_nsec: nsec as _,
            },
        })
    }

    fn checked_sub_duration(&self, other: &Duration) -> Option<Timespec> {
        let mut secs = other
            .as_secs()
            .try_into() // <- target type would be `libc::time_t`
            .ok()
            .and_then(|secs| self.t.tv_sec.checked_sub(secs))?;

        // Similar to above, nanos can't overflow.
        let mut nsec = self.t.tv_nsec as i32 - other.subsec_nanos() as i32;
        if nsec < 0 {
            nsec += NSEC_PER_SEC as i32;
            secs = secs.checked_sub(1)?;
        }
        Some(Timespec {
            t: timespec {
                tv_sec: secs,
                tv_nsec: nsec as _,
            },
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant {
    t: Timespec,
}

impl Instant {
    pub fn now() -> Instant {
        Instant { t: now(CLOCK_MONOTONIC) }
    }
    pub const fn zero() -> Instant {
        Instant {
            t: Timespec::zero(),
        }
    }
    pub fn actually_monotonic() -> bool { true } // for LINUX-LIKE SYSTEM

    pub fn checked_sub_instant(&self, other: &Instant) -> Option<Duration> {
        self.t.sub_timespec(&other.t).ok()
    }

    pub fn checked_add_duration(&self, other: &Duration) -> Option<Instant> {
        Some(Instant { t: self.t.checked_add_duration(other)? })
    }


    pub fn checked_sub_duration(&self, other: &Duration) -> Option<Instant> {
        Some(Instant { t: self.t.checked_sub_duration(other)? })
    }
}

fn now(clock: clockid_t) -> Timespec {
    use syscalls::SYS_clock_gettime;
    let mut t = Timespec {
        t: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        }
    };
    // TODO: wrap this
    unsafe {
        syscall2(SYS_clock_gettime, clock as _, &mut t.t as *mut _ as _).unwrap();
    }
    t
}

#[cfg(test)]
mod test {
    use crate::constants::CLOCK_MONOTONIC;

    #[test]
    fn now() {
        println!("{:?}", super::now(CLOCK_MONOTONIC))
    }
}