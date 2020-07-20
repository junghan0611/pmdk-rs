//
// Copyright (c) 2019 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use libc::{c_char, c_int, c_void, mode_t};

use crate::PMEMoid;

pub enum PMEMobjpool {}

#[allow(non_camel_case_types)]
pub type pmemobj_constr =
    unsafe extern "C" fn(pop: *mut PMEMobjpool, ptr: *mut c_void, arg: *mut c_void) -> c_int;

#[link(name = "pmemobj", kind = "static")]
extern "C" {
    pub fn pmemobj_open(path: *const c_char, layout: *const c_char) -> *mut PMEMobjpool;
    pub fn pmemobj_create(
        path: *const c_char,
        layout: *const c_char,
        poolsize: usize,
        mode: mode_t,
    ) -> *mut PMEMobjpool;
    pub fn pmemobj_close(pop: *mut PMEMobjpool);

    // Object

    pub fn pmemobj_alloc(
        pop: *mut PMEMobjpool,
        oidp: *mut PMEMoid,
        size: usize,
        type_num: u64,
        constructor: Option<pmemobj_constr>,
        arg: *mut c_void,
    ) -> c_int;
    pub fn pmemobj_alloc_usable_size(oid: PMEMoid) -> usize;
    pub fn pmemobj_free(oidp: *mut PMEMoid);

    pub fn pmemobj_memcpy_persist(
        pop: *mut PMEMobjpool,
        dest: *mut c_void,
        src: *const c_void,
        len: usize,
    );
    pub fn pmemobj_memset_persist(pop: *mut PMEMobjpool, dest: *mut c_void, c: c_int, len: usize);
    pub fn pmemobj_persist(pop: *mut PMEMobjpool, addr: *const c_void, len: usize);
    pub fn pmemobj_flush(pop: *mut PMEMobjpool, addr: *const c_void, len: usize);
    pub fn pmemobj_drain(pop: *mut PMEMobjpool);

    // Error handling:

    pub fn pmemobj_errormsg() -> *const c_char;

    // translates persistent (pool-id, offset) to transient pointer
    pub fn pmemobj_direct(oid: PMEMoid) -> *mut c_void;

    // translates pointer to (pool-id, offset)
    pub fn pmemobj_oid(addr: *const c_void) -> PMEMoid;

    // extra u64 payload per object
    pub fn pmemobj_type_num(oid: PMEMoid) -> u64;

    // iterator, no gurantee over consitency
    pub fn pmemobj_first(pop: *const PMEMobjpool) -> PMEMoid;
    pub fn pmemobj_next(oid: PMEMoid) -> PMEMoid;

    // control memory allocation ect
    pub fn pmemobj_ctl_exec(pop: *mut PMEMobjpool, name: *const c_char, arg: *mut c_void) -> c_int;
    pub fn pmemobj_ctl_get(pop: *mut PMEMobjpool, name: *const c_char, arg: *mut c_void) -> c_int;
    pub fn pmemobj_ctl_set(pop: *mut PMEMobjpool, name: *const c_char, arg: *mut c_void) -> c_int;
}

// an optimization can be
// 1. that pool can be loaded to the same memory space
// 2. if we know that a key belongs to a specific volume, and it owns its pool, than we don't need pool id in key.
// 3. pointer-to-offset calculation
