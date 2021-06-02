use rustler_sys::c_char;

use crate::codegen_runtime::{NifReturnable, NifReturned};
use crate::wrapper::ErlNifTaskFlags;
use crate::Env;
use std::{ffi::CStr, marker::PhantomData};

pub enum SchedulerFlags {
    Normal = ErlNifTaskFlags::ERL_NIF_NORMAL_JOB as isize,
    DirtyCpu = ErlNifTaskFlags::ERL_NIF_DIRTY_JOB_CPU_BOUND as isize,
    DirtyIo = ErlNifTaskFlags::ERL_NIF_DIRTY_JOB_IO_BOUND as isize,
}

pub fn consume_timeslice(env: Env, percent: i32) -> bool {
    let success = unsafe { rustler_sys::enif_consume_timeslice(env.as_c_arg(), percent) };
    success == 1
}

/// Convenience macro for scheduling a future invokation of a NIF.
#[macro_export]
macro_rules! reschedule {
    ($flags:expr, $($arg:expr),*) => (
        rustler::schedule::Schedule::from(($flags, $($arg,)*))
    )
}

/// Convenience type for scheduling a future invokation of a NIF.
///
/// ## Usage:
///
/// The first generic type should be the NIF that will be scheduled, with a
/// current limitation being that it must be same throughout the lifetime of the
/// NIF.
///
/// The second generic type defined should be the type of the return value.
///
/// Every other generic type is optional, but should reflect the argument types
/// of the scheduled NIF, in the same order.
///
/// ## Example:
/// ```rust,ignore
/// #[nif]
/// fn factorial(input: u32, result: Option<u32>) -> Schedule<factorial, u32, u32, u32> {
///     let result = result.unwrap_or(1);
///     if input == 0 {
///         Schedule::Return(result)
///     } else {
///         // alternatively `Schedule::Continue2(std::marker::PhantomData, SchedulerFlags::Normal, input - 1, result * input)`
///         // alternatively `Schedule::continue2(SchedulerFlags::Normal, input - 1, result * input)`
///         // alternatively `Schedule::from((SchedulerFlags::Normal, input - 1, result * input))`
///         // alternatively `(SchedulerFlags::Normal, input - 1, result * input).into()`
///         reschedule!(SchedulerFlags::Normal, input - 1, result * input)
///     }
/// }
/// ```
pub enum Schedule<N: crate::Nif, T, A = (), B = (), C = (), D = (), E = (), F = (), G = ()> {
    /// The final result type to return back to the BEAM.
    Return(T),
    /// Single- and multiple-argument variants that should reflect the scheduled
    /// NIF's function signature.
    Continue1(PhantomData<N>, SchedulerFlags, A),
    Continue2(PhantomData<N>, SchedulerFlags, A, B),
    Continue3(PhantomData<N>, SchedulerFlags, A, B, C),
    Continue4(PhantomData<N>, SchedulerFlags, A, B, C, D),
    Continue5(PhantomData<N>, SchedulerFlags, A, B, C, D, E),
    Continue6(PhantomData<N>, SchedulerFlags, A, B, C, D, E, F),
    Continue7(PhantomData<N>, SchedulerFlags, A, B, C, D, E, F, G),
}

macro_rules! impls {
    ($($variant:ident $func_name:ident($($arg:ident : $ty:ty,)*);)*) => {
        impl<N: crate::Nif, T, A, B, C, D, E, F, G> Schedule<N, T, A, B, C, D, E, F, G> {
            $(#[allow(clippy::many_single_char_names)]
            #[inline]
            pub fn $func_name(flags: SchedulerFlags, $($arg: $ty),*) -> Self {
                Self::$variant(PhantomData, flags, $($arg),*)
            })*
        }

        $(impl<N: crate::Nif, T, A, B, C, D, E, F, G> From<(SchedulerFlags, $($ty),*)> for Schedule<N, T, A, B, C, D, E, F, G> {
            #[allow(clippy::many_single_char_names)]
            #[inline]
            fn from((flags, $($arg),*): (SchedulerFlags, $($ty),*)) -> Self {
                Self::$func_name(flags, $($arg),*)
            }
        })*

        unsafe impl<N, T, A, B, C, D, E, F, G> NifReturnable for Schedule<N, T, A, B, C, D, E, F, G>
        where
            N: crate::Nif,
            T: crate::Encoder,
            A: crate::Encoder,
            B: crate::Encoder,
            C: crate::Encoder,
            D: crate::Encoder,
            E: crate::Encoder,
            F: crate::Encoder,
            G: crate::Encoder,
        {
            #[inline]
            unsafe fn into_returned(self, env: Env) -> NifReturned {
                #[allow(clippy::many_single_char_names)]
                match self {
                    Self::Return(res) => NifReturned::Term(res.encode(env).as_c_arg()),
                    $(Self::$variant(_, flags, $($arg),*) => NifReturned::Reschedule {
                        fun_name: CStr::from_ptr(N::NAME as *const c_char).into(),
                        flags,
                        fun: N::RAW_FUNC,
                        args: vec![$($arg.encode(env).as_c_arg()),*],
                    },)*
                }
            }
        }
    };
}

impls! {
    Continue1 continue1(a: A,);
    Continue2 continue2(a: A, b: B,);
    Continue3 continue3(a: A, b: B, c: C,);
    Continue4 continue4(a: A, b: B, c: C, d: D,);
    Continue5 continue5(a: A, b: B, c: C, d: D, e: E,);
    Continue6 continue6(a: A, b: B, c: C, d: D, e: E, f: F,);
    Continue7 continue7(a: A, b: B, c: C, d: D, e: E, f: F, g: G,);
}
