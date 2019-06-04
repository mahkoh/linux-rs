// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::{mem};
use crate::{
    kty::{c_int, itimerspec, TFD_TIMER_ABSTIME},
    syscall::{close, timerfd_settime, timerfd_gettime, read},
    fd::{FdContainer},
    util::retry::{retry},
    time::{Time, time_to_timespec, time_from_timespec},
    result::{Result},
    lmem,
};

/// A timer.
pub struct Timer {
    fd: c_int,
    owned: bool,
}

impl Timer {
    /// Disables the timer.
    pub fn disable(&self) -> Result {
        let arg = lmem::zeroed();
        rv!(timerfd_settime(self.fd, 0, &arg, None))
    }

    /// Sets the timer to expire every `iv` time units.
    pub fn interval(&self, iv: Time) -> Result {
        let arg = itimerspec {
            it_interval: time_to_timespec(iv),
            it_value: time_to_timespec(iv),
        };
        rv!(timerfd_settime(self.fd, 0, &arg, None))
    }

    /// Sets the timer to expire every `iv` time units, starting at the absolute `start`.
    pub fn interval_from(&self, iv: Time, start: Time) -> Result {
        let arg = itimerspec {
            it_interval: time_to_timespec(iv),
            it_value: time_to_timespec(start),
        };
        rv!(timerfd_settime(self.fd, TFD_TIMER_ABSTIME, &arg, None))
    }

    /// Sets the timer to expire every `iv` time units, starting in `when` units.
    pub fn interval_in(&self, iv: Time, when: Time) -> Result {
        let arg = itimerspec {
            it_interval: time_to_timespec(iv),
            it_value: time_to_timespec(when),
        };
        rv!(timerfd_settime(self.fd, 0, &arg, None))
    }

    /// Sets the timer to expire once at the absolute `when`.
    pub fn once_at(&self, when: Time) -> Result {
        let arg = itimerspec {
            it_interval: lmem::zeroed(),
            it_value: time_to_timespec(when),
        };
        rv!(timerfd_settime(self.fd, TFD_TIMER_ABSTIME, &arg, None))
    }

    /// Sets the timer to expire in `when` time units.
    pub fn once_in(&self, when: Time) -> Result {
        let arg = itimerspec {
            it_interval: lmem::zeroed(),
            it_value: time_to_timespec(when),
        };
        rv!(timerfd_settime(self.fd, 0, &arg, None))
    }

    /// Returns the status of the timer.
    ///
    /// TODO: Document this.
    pub fn status(&self) -> Result<(Time, Time)> {
        let mut arg = lmem::zeroed();
        rv!(timerfd_gettime(self.fd, &mut arg))?;
        Ok((time_from_timespec(arg.it_interval), time_from_timespec(arg.it_value)))
    }

    /// Returns the number of times the timer expired since this function was last called.
    pub fn ticks(&self) -> Result<u64> {
        let mut buf = 0;
        retry(|| read(self.fd, lmem::as_mut_data(&mut buf)))?;
        Ok(buf)
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        if self.owned {
            close(self.fd);
        }
    }
}

impl From<Timer> for c_int {
    fn from(timer: Timer) -> Self {
        let fd = timer.fd;
        mem::forget(timer);
        fd
    }
}

impl FdContainer for Timer {
    fn is_owned(&self) -> bool {
        self.owned
    }

    fn borrow(&self) -> c_int {
        self.fd
    }

    fn from_owned(fd: c_int) -> Timer {
        Timer { fd, owned: true }
    }

    fn from_borrowed(fd: c_int) -> Timer {
        Timer { fd, owned: false }
    }
}
