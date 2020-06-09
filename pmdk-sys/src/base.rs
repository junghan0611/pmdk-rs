//
// Copyright (c) 2019 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use serde::{Deserialize, Serialize};
use std::fmt;

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct pmemoid {
    pool_uuid_lo: u64,
    off: u64,
}

impl Default for pmemoid {
    #[inline(always)]
    fn default() -> Self {
        Self {
            pool_uuid_lo: 0,
            off: 0,
        }
    }
}

impl fmt::Debug for pmemoid {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "pmemoid pool: {}, off: {:x}",
            self.pool_uuid_lo, self.off
        )
    }
}

impl pmemoid {
    pub fn new(pool_uuid_lo: u64, off: u64) -> Self {
        Self { pool_uuid_lo, off }
    }

    pub fn off(&self) -> u64 {
        self.off
    }

    pub fn pool_uuid_lo(&self) -> u64 {
        self.pool_uuid_lo
    }

    pub fn is_null(&self) -> bool {
        self.off == 0 && self.pool_uuid_lo == 0
    }
}

// struct of 2 u64, (pool-id, offset): (u64, u64)
// PMEM holds a global variable in memory and lookup offset in pool (obj, block, or misc)
// must be translated into a pointer
pub type PMEMoid = pmemoid;
