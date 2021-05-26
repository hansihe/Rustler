use rustler_sys::c_char;

use crate::codegen_runtime::{NifReturnable, NifReturned};
use crate::wrapper::ErlNifTaskFlags;
use crate::Env;

pub enum SchedulerFlags {
    Normal = ErlNifTaskFlags::ERL_NIF_NORMAL_JOB as isize,
    DirtyCpu = ErlNifTaskFlags::ERL_NIF_DIRTY_JOB_CPU_BOUND as isize,
    DirtyIo = ErlNifTaskFlags::ERL_NIF_DIRTY_JOB_IO_BOUND as isize,
}

impl SchedulerFlags {
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
/// fn factorial(input: u32, result: Option<u32>) -> Schedule<factorial, u32, u32> {
///     let result = result.unwrap_or(1);
///     if input == 0 {
///         Schedule::Result(result)
///     } else {
///         Schedule::Next2(factorial, input - 1, result * input)
///     }
/// }
/// ```
pub enum Schedule<N: crate::Nif, T, A = (), B = (), C = (), D = (), E = (), F = (), G = ()> {
    /// The final result type to return back to the BEAM.
    Result(T),
    /// Single- and multiple-argument variants that should reflect the scheduled
    /// NIF's function signature.
    Next(N, A),
    Next2(N, A, B),
    Next3(N, A, B, C),
    Next4(N, A, B, C, D),
    Next5(N, A, B, C, D, E),
    Next6(N, A, B, C, D, E, F),
    Next7(N, A, B, C, D, E, F, G),
}

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
                    fun_name: std::ffi::CStr::from_ptr(N::NAME as *const c_char).into(),
                    flags: SchedulerFlags::from(N::FLAGS as isize),
                    fun: N::RAW_FUNC,
                    args: vec![$($arg.encode(env).as_c_arg()),*],
                }
            )
        }

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
