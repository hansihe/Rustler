use crate::wrapper::{pid, ErlNifPid};
use crate::{Decoder, Encoder, Env, Error, NifResult, Term, Atom};
use std::mem::MaybeUninit;

#[derive(Clone)]
pub struct LocalPid {
    c: ErlNifPid,
}

impl LocalPid {
    pub fn as_c_arg(&self) -> &ErlNifPid {
        &self.c
    }

    ///
    /// Look up a local process by its registered name.
    ///
    pub fn whereis<'a>(env: Env<'a>, name: Atom) -> Option<Self> {
        unsafe { pid::whereis(env.as_c_arg(), name.as_c_arg()).map(|pid| LocalPid { c: pid }) }
    }
}

impl<'a> Decoder<'a> for LocalPid {
    fn decode(term: Term<'a>) -> NifResult<LocalPid> {
        unsafe { pid::get_local_pid(term.get_env().as_c_arg(), term.as_c_arg()) }
            .map(|pid| LocalPid { c: pid })
            .ok_or(Error::BadArg)
    }
}

impl Encoder for LocalPid {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        unsafe { Term::new(env, pid::make_pid(env.as_c_arg(), self.c)) }
    }
}

impl<'a> Env<'a> {
    /// Return the calling process's pid.
    ///
    /// # Panics
    ///
    /// Panics if this environment is process-independent.  (The only way to get such an
    /// environment is to use `OwnedEnv`.  The `Env` that Rustler passes to NIFs when they're
    /// called is always associated with the calling Erlang process.)
    pub fn pid(self) -> LocalPid {
        let mut pid = MaybeUninit::uninit();
        if unsafe { rustler_sys::enif_self(self.as_c_arg(), pid.as_mut_ptr()) }.is_null() {
            panic!("environment is process-independent");
        }
        LocalPid {
            c: unsafe { pid.assume_init() },
        }
    }
}
