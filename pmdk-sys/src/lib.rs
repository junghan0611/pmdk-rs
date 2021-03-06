//
// Copyright (c) 2019 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

#![doc(html_root_url = "https://docs.rs/pmdk-sys/0.9.2")]

use libc::{c_char, c_int, c_uint, c_void};
use libc::{mode_t, size_t};

mod base;

pub mod obj;

pub use base::PMEMoid;

#[link(name = "pmem", kind = "static")]
extern "C" {
    // Most commonly used functions:

    pub fn pmem_is_pmem(addr: *const c_void, len: size_t) -> c_int;
    pub fn pmem_persist(addr: *const c_void, len: size_t);
    pub fn pmem_msync(addr: *const c_void, len: size_t) -> c_int;
    pub fn pmem_map_file(
        path: *const c_char,
        len: size_t,
        flags: c_int,
        mode: mode_t,
        mapped_lenp: *mut size_t,
        is_pmemp: *mut c_int,
    ) -> *mut c_void;
    pub fn pmem_unmap(addr: *mut c_void, len: size_t) -> c_int;

    // Partial flushing operations:

    pub fn pmem_flush(addr: *const c_void, len: size_t);
    pub fn pmem_drain();
    pub fn pmem_has_hw_drain() -> c_int;

    // Copying to persistent memory:

    pub fn pmem_memmove_persist(
        pmemdest: *mut c_void,
        src: *const c_void,
        len: size_t,
    ) -> *mut c_void;
    pub fn pmem_memcpy_persist(
        pmemdest: *mut c_void,
        src: *const c_void,
        len: size_t,
    ) -> *mut c_void;
    pub fn pmem_memset_persist(pmemdest: *mut c_void, c: c_int, len: size_t) -> *mut c_void;
    pub fn pmem_memmove_nodrain(
        pmemdest: *mut c_void,
        src: *const c_void,
        len: size_t,
    ) -> *mut c_void;
    pub fn pmem_memcpy_nodrain(
        pmemdest: *mut c_void,
        src: *const c_void,
        len: size_t,
    ) -> *mut c_void;
    pub fn pmem_memset_nodrain(pmemdest: *mut c_void, c: c_int, len: size_t) -> *mut c_void;

    // Library API versioning:

    pub fn pmem_check_version(major_required: c_uint, minor_required: c_uint) -> *const c_char;

    // Error handling:

    pub fn pmem_errormsg() -> *const c_char;
}

#[link(name = "pmempool", kind = "static")]
extern "C" {
    // Utils

    pub fn pmempool_rm(path: *const c_char, flags: c_int) -> c_int;
}
