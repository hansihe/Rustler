//! Contains basic wrappers for the Erlang NIF api. Should not be used directly.
//!
//! While the nif_interface module should directly export unsafe nif helper functions,
//! this module should preform validation and make them (reasonably) safe and easy to
//! use from rust. This module should try to be as nonopinionated as possible, and
//! should try to stick as close as possible to the original C api.
//! 
//! Making the apis nice to use from rust should be done in the root rustler crate.


pub mod nif_interface;

pub mod tuple;
pub use self::tuple::{ get_tuple };

pub mod map;

pub mod atom;

pub mod exception;

pub mod resource;

pub mod list;

pub mod check;

use ::wrapper::nif_interface::{ NIF_ENV, NIF_TERM, enif_make_copy };
pub fn copy_term(dest: NIF_ENV, term: NIF_TERM) -> NIF_TERM {
    unsafe { enif_make_copy(dest, term) }
}

/*macro_rules! wrap_number {
    (
}*/
