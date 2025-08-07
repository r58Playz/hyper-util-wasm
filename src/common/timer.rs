#![allow(dead_code)]

use std::fmt;
use std::pin::Pin;
use std::time::Duration;

use wasmtimer::std::Instant;
use wasmtimer::tokio::Sleep;

#[derive(Clone)]
pub(crate) struct Timer;

// =====impl Timer=====
impl Timer {
    pub(crate) fn new() -> Self {
        Self
    }

    pub fn sleep(&self, duration: Duration) -> Sleep {
        wasmtimer::tokio::sleep(duration)
    }

    pub fn sleep_until(&self, deadline: Instant) -> Sleep {
        wasmtimer::tokio::sleep_until(deadline)
    }

    pub fn reset(&self, sleep: &mut Sleep, deadline: Instant) {
        Pin::new(sleep).reset(deadline)
    }
}

impl fmt::Debug for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Timer").finish()
    }
}
