//! Contains basic wrappers for the Erlang NIF API. Should not be used directly.
//!
//! This module should preform validation and make them (reasonably) safe and easy to
//! use from Rust. This module should try to be as non-opinionated as possible, and
//! should try to stick as close as possible to the original C API.
//!
//! Making the APIs nice to use from Rust should be done in the root `rustler` crate.
#![allow(clippy::upper_case_acronyms)]

pub mod atom;
pub mod binary;
pub mod check;
pub mod env;
pub mod exception;
pub mod list;
pub mod map;
pub mod pid;
pub mod resource;
pub mod term;
pub mod tuple;

pub use rustler_sys::{
    enif_clear_env, enif_free_env, enif_get_local_pid, enif_make_pid, enif_map_iterator_create,
    enif_map_iterator_destroy, enif_map_iterator_get_pair, enif_map_iterator_next, enif_self,
    enif_whereis_pid, ErlNifMapIterator, ErlNifMapIteratorEntry, ErlNifPid,
    ERL_NIF_THR_DIRTY_CPU_SCHEDULER, ERL_NIF_THR_DIRTY_IO_SCHEDULER, ERL_NIF_THR_NORMAL_SCHEDULER,
    ERL_NIF_THR_UNDEFINED,
};

pub use std::os::raw::{c_double, c_int, c_uchar, c_uint, c_void};
pub type size_t = usize;

pub type NIF_ENV = *mut rustler_sys::ErlNifEnv;
pub type NIF_TERM = size_t;
pub type NIF_RESOURCE_TYPE = *const rustler_sys::ErlNifResourceType;

pub fn get_nif_resource_type_init_size() -> usize {
    std::mem::size_of::<rustler_sys::ErlNifResourceTypeInit>()
}

pub type NIF_RESOURCE_HANDLE = *const c_void;
pub type MUTABLE_NIF_RESOURCE_HANDLE = *mut c_void;

pub type NifResourceDtor =
    unsafe extern "C" fn(r_env: NIF_ENV, obj: MUTABLE_NIF_RESOURCE_HANDLE) -> ();
pub type NifResourceFlags = rustler_sys::ErlNifResourceFlags;

pub enum NIF_ERROR {
    BAD_ARG,
}

pub type DEF_NIF_FUNC = rustler_sys::ErlNifFunc;
pub type DEF_NIF_ENTRY = rustler_sys::ErlNifEntry;
pub use rustler_sys::ErlNifResourceFlags as NIF_RESOURCE_FLAGS;
pub use rustler_sys::NIF_MAJOR_VERSION;
pub use rustler_sys::NIF_MINOR_VERSION;

pub use rustler_sys::ErlNifBinaryToTerm as NIF_BINARY_TO_TERM_OPTS;
pub use rustler_sys::ERL_NIF_BIN2TERM_SAFE;

#[repr(C)]
pub enum ErlNifTaskFlags {
    ERL_NIF_NORMAL_JOB = 0,
    ERL_NIF_DIRTY_JOB_CPU_BOUND = 1,
    ERL_NIF_DIRTY_JOB_IO_BOUND = 2,
}
