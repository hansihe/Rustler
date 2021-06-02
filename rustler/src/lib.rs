#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]

//! [Github](https://github.com/rusterlium/rustler)
//! [Example](https://github.com/hansihe/Rustler_Example)
//!
//! Rustler is a library for writing Erlang NIFs in safe Rust code. That means there should be no
//! ways to crash the BEAM (Erlang VM). The library provides facilities for generating the
//! boilerplate for interacting with the BEAM, handles encoding and decoding of Erlang terms, and
//! catches rust panics before they unwind into C.
//!
//! The library provides functionality for both Erlang and Elixir, however Elixir is favored as of
//! now.
//!
//! This crate provides the entire runtime library for rustler. Code generators are located in the
//! rustler_codegen library.
//!
//! # Getting Started
//! There is a [`:rustler`](https://hex.pm/packages/rustler) package on hex.pm that provides
//! functionality which makes working with Rustler easier, including project generators, an
//! automatic NIF compiler for Mix, and utilities for loading the compiled NIF.
//!
//! For more information about this, see [the documentation for
//! rustler_mix](https://hexdocs.pm/rustler/basics.html).

#[macro_use(enif_snprintf)]
extern crate rustler_sys;

#[doc(hidden)]
pub mod wrapper;

#[doc(hidden)]
pub mod codegen_runtime;

pub extern crate lazy_static;

#[macro_use]
pub mod types;

mod term;

pub use crate::term::Term;
pub use crate::types::{
    Atom, Binary, Decoder, Encoder, ListIterator, LocalPid, MapIterator, OwnedBinary,
};
pub mod resource;
pub use crate::resource::ResourceArc;

#[doc(hidden)]
pub mod dynamic;
pub use crate::dynamic::TermType;

pub mod schedule;
pub use crate::schedule::{Schedule, SchedulerFlags};
pub mod env;
pub use crate::env::{Env, OwnedEnv};
pub mod thread;
pub use crate::thread::{spawn, JobSpawner, ThreadSpawner};

pub mod error;
pub mod export;
pub use crate::error::Error;

pub mod r#return;
pub use crate::r#return::Return;

#[doc(hidden)]
mod nif;
pub use nif::Nif;

pub type NifResult<T> = Result<T, Error>;

#[cfg(feature = "derive")]
pub use rustler_codegen::{
    init, nif, NifMap, NifRecord, NifStruct, NifTuple, NifUnitEnum, NifUntaggedEnum,
};
