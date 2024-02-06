#![allow(dead_code)]

use std::fmt;
use std::time::Duration;
use wasmtimer::std::Instant;

use wasmtimer::tokio::Sleep;

pub trait TimerTrait {
    fn sleep(&self, duration: Duration) -> Sleep;
    fn sleep_until(&self, deadline: Instant) -> Sleep;
}

#[derive(Clone)]
pub(crate) struct Timer();

// =====impl Timer=====
impl Timer {
    pub(crate) fn new() -> Self {
        Self()
    }
}

impl fmt::Debug for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Timer").finish()
    }
}

impl TimerTrait for Timer {
    fn sleep(&self, duration: Duration) -> Sleep {
        wasmtimer::tokio::sleep(duration)
    }

    fn sleep_until(&self, deadline: Instant) -> Sleep {
        wasmtimer::tokio::sleep_until(deadline)
    }
}
