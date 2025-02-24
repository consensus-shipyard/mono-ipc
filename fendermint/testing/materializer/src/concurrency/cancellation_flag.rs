// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use std::sync::atomic::{AtomicBool, Ordering};

/// A simple cancellation flag using `AtomicBool`.
#[derive(Debug)]
pub struct CancellationFlag {
    inner: AtomicBool,
}

impl CancellationFlag {
    pub fn new() -> Self {
        Self {
            inner: AtomicBool::new(false),
        }
    }

    pub fn cancel(&self) {
        self.inner.store(true, Ordering::SeqCst);
    }

    pub fn is_cancelled(&self) -> bool {
        self.inner.load(Ordering::SeqCst)
    }
}

impl Default for CancellationFlag {
    fn default() -> Self {
        Self::new()
    }
}