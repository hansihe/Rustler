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

impl SchedulerFlags {
    #[inline]
    fn from(n: isize) -> Self {
        match n {
            _ if n == Self::Normal as isize => Self::Normal,
            _ if n == Self::DirtyCpu as isize => Self::DirtyCpu,
            _ if n == Self::DirtyIo as isize => Self::DirtyIo,
            _ => unreachable!(),
        }
    }
}

pub fn consume_timeslice(env: Env, percent: i32) -> bool {
    let success = unsafe { rustler_sys::enif_consume_timeslice(env.as_c_arg(), percent) };
    success == 1
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
/// Every other generic type is optional, but should reflect the non-`rustler`
/// arguments provided to the NIF, in the same order.
///
/// ## Example:
/// ```rust,ignore
/// #[nif]
/// fn factorial(input: u32, result: Option<u32>) -> Schedule<factorial, u32, u32, u32> {
///     let result = result.unwrap_or(1);
///     if input == 0 {
///         Schedule::Result(result)
///     } else {
///         // alternatively `Schedule::Next2(std::marker::PhantomData, input - 1, result * input)`
///         // alternatively `(input - 1, result * input).into()`
///         Schedule::next2(input - 1, result * input)
///     }
/// }
/// ```
pub enum Schedule<N: crate::Nif, T, A = (), B = (), C = (), D = (), E = (), F = (), G = ()> {
    /// The final result type to return back to the BEAM.
    Result(T),
    /// Single- and multiple-argument variants that should reflect the scheduled
    /// NIF's function signature.
    Next(PhantomData<N>, A),
    Next2(PhantomData<N>, A, B),
    Next3(PhantomData<N>, A, B, C),
    Next4(PhantomData<N>, A, B, C, D),
    Next5(PhantomData<N>, A, B, C, D, E),
    Next6(PhantomData<N>, A, B, C, D, E, F),
    Next7(PhantomData<N>, A, B, C, D, E, F, G),
}

macro_rules! impl_funcs {
    ($variant:ident $func_name:ident($($arg:ident : $ty:ty,)*)) => {
        impl<N: crate::Nif, T, A, B, C, D, E, F, G> Schedule<N, T, A, B, C, D, E, F, G> {
            #[allow(clippy::many_single_char_names)]
            #[inline]
            pub fn $func_name($($arg: $ty),*) -> Self {
                Self::$variant(PhantomData, $($arg),*)
            }
        }

        impl<N: crate::Nif, T, A, B, C, D, E, F, G> From<($($ty),*)> for Schedule<N, T, A, B, C, D, E, F, G> {
            #[allow(clippy::many_single_char_names)]
            #[inline]
            fn from(($($arg),*): ($($ty),*)) -> Self {
                Self::$func_name($($arg),*)
            }
        }
    };
}

impl<N: crate::Nif, T, A, B, C, D, E, F, G> Schedule<N, T, A, B, C, D, E, F, G> {
    #[inline]
    pub fn next(a: A) -> Self {
        Self::Next(PhantomData, a)
    }
}

impl<N: crate::Nif, T, A, B, C, D, E, F, G> From<A> for Schedule<N, T, A, B, C, D, E, F, G> {
    #[inline]
    fn from(a: A) -> Self {
        Self::next(a)
    }
}
impl_funcs! { Next2 next2(a: A, b: B,) }
impl_funcs! { Next3 next3(a: A, b: B, c: C,) }
impl_funcs! { Next4 next4(a: A, b: B, c: C, d: D,) }
impl_funcs! { Next5 next5(a: A, b: B, c: C, d: D, e: E,) }
impl_funcs! { Next6 next6(a: A, b: B, c: C, d: D, e: E, f: F,) }
impl_funcs! { Next7 next7(a: A, b: B, c: C, d: D, e: E, f: F, g: G,) }

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
        macro_rules! branch {
            ($($arg:tt),*) => (
                NifReturned::Reschedule {
                    fun_name: CStr::from_ptr(N::NAME as *const c_char).into(),
                    flags: SchedulerFlags::from(N::FLAGS as isize),
                    fun: N::RAW_FUNC,
                    args: vec![$($arg.encode(env).as_c_arg()),*],
                }
            )
        }

        #[allow(clippy::many_single_char_names)]
        match self {
            Self::Result(res) => NifReturned::Term(res.encode(env).as_c_arg()),
            Self::Next(_, a) => branch!(a),
            Self::Next2(_, a, b) => branch!(a, b),
            Self::Next3(_, a, b, c) => branch!(a, b, c),
            Self::Next4(_, a, b, c, d) => branch!(a, b, c, d),
            Self::Next5(_, a, b, c, d, e) => branch!(a, b, c, d, e),
            Self::Next6(_, a, b, c, d, e, f) => branch!(a, b, c, d, e, f),
            Self::Next7(_, a, b, c, d, e, f, g) => branch!(a, b, c, d, e, f, g),
        }
    }
}
