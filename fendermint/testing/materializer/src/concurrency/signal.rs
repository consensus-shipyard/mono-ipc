// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

pub struct Signal(tokio::sync::Semaphore);

impl Signal {
    pub fn new() -> Self {
        Self(tokio::sync::Semaphore::new(0))
    }

    pub fn send(&self) {
        self.0.close();
    }

    pub fn received(&self) -> bool {
        self.0.is_closed()
    }
}

impl Default for Signal {
    fn default() -> Self {
        Self::new()
    }
}
