use ::{ NifEnv, NifTerm, NifError, NifResult, NifDecoder, NifEncoder };
use ::wrapper::nif_interface::{ self, ErlNifPid };
use ::wrapper::pid;
use std::mem;

#[derive(Clone)]
pub struct NifPid {
    c: ErlNifPid
}

impl NifPid {
    pub fn as_c_arg(&self) -> &ErlNifPid {
        &self.c
    }
}

impl<'a> NifDecoder<'a> for NifPid {
    fn decode(term: NifTerm<'a>) -> NifResult<NifPid> {
        unsafe { pid::get_local_pid(term.get_env().as_c_arg(), term.as_c_arg()) }
            .map(|pid| NifPid { c: pid })
            .ok_or(NifError::BadArg)
    }
}

impl NifEncoder for NifPid {
    fn encode<'a>(&self, env: NifEnv<'a>) -> NifTerm<'a> {
        unsafe { NifTerm::new(env, pid::make_pid(env.as_c_arg(), &self.c)) }
    }
}

impl<'a> NifEnv<'a> {
    /// Return the calling process's pid.
    ///
    /// # Panics
    ///
    /// Panics if this environment is process-independent.  (The only way to get such an
    /// environment is to use `OwnedEnv`.  The `NifEnv` that Rustler passes to NIFs when they're
    /// called is always associated with the calling Erlang process.)
    pub fn pid(self) -> NifPid {
        let mut pid: ErlNifPid = unsafe { mem::uninitialized() };
        if unsafe { nif_interface::enif_self(self.as_c_arg(), &mut pid) }.is_null() {
            panic!("environment is process-independent");
        }
        NifPid { c: pid }
    }
}
