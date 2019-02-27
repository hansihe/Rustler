use std::cmp::Ordering;
use std::fmt::{self, Debug};
use std::hash::{ Hash, Hasher };
use types::binary::OwnedBinary;
use wrapper::env::term_to_binary;
use wrapper::nif_interface::NIF_TERM;
use {Decoder, Env, NifResult};

/// Term is used to represent all erlang terms. Terms are always lifetime limited by a Env.
///
/// Term is cloneable and copyable, but it can not exist outside of the lifetime of the Env
/// that owns it.
#[derive(Clone, Copy)]
pub struct Term<'a> {
    term: NIF_TERM,
    env: Env<'a>,
}

impl<'a> Debug for Term<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        ::wrapper::term::fmt(self.as_c_arg(), f)
    }
}

impl<'a> Term<'a> {
    /// Create a `Term` from a raw `NIF_TERM`.
    ///
    /// # Unsafe
    /// The caller must ensure that `env` is the environment that `inner` belongs to,
    /// unless `inner` is an atom term.
    pub unsafe fn new(env: Env<'a>, inner: NIF_TERM) -> Self {
        Term {
            term: inner,
            env: env,
        }
    }
    /// This extracts the raw term pointer. It is usually used in order to obtain a type that can
    /// be passed to calls into the erlang vm.
    pub fn as_c_arg(&self) -> NIF_TERM {
        self.term
    }

    pub fn get_env(&self) -> Env<'a> {
        self.env
    }

    /// Returns a representation of self in the given Env.
    ///
    /// If the term is already is in the provided env, it will be directly returned. Otherwise
    /// the term will be copied over.
    pub fn in_env<'b>(&self, env: Env<'b>) -> Term<'b> {
        if self.get_env() == env {
            // It's safe to create a new Term<'b> without copying because we
            // just proved that the same environment is associated with both 'a
            // and 'b.  (They are either exactly the same lifetime, or the
            // lifetimes of two .run() calls on the same OwnedEnv.)
            unsafe { Term::new(env, self.as_c_arg()) }
        } else {
            unsafe { Term::new(env, ::wrapper::copy_term(env.as_c_arg(), self.as_c_arg())) }
        }
    }

    /// Decodes the Term into type T.
    ///
    /// This should be used as the primary method of extracting the value from a Term.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let term: Term = ...;
    /// let number: i32 = term.decode()?;
    /// ```
    pub fn decode<T>(self) -> NifResult<T>
    where
        T: Decoder<'a>,
    {
        Decoder::decode(self)
    }

    pub fn to_binary(self) -> OwnedBinary {
        let raw_binary = unsafe { term_to_binary(self.env.as_c_arg(), self.as_c_arg()) }.unwrap();
        unsafe { OwnedBinary::from_raw(raw_binary) }
    }

    /// Non-portable hash function that only guarantees the same hash for the same term within
    /// one Erlang VM instance.
    ///
    /// It takes 32-bit salt values and generates hashes within 0..2^32-1.
    pub fn hash_internal(&self, salt: u32) -> u32 {
        unsafe {
            ::wrapper::nif_interface::enif_hash(
                ::wrapper::nif_interface::NIF_HASH_TYPE::ERL_NIF_INTERNAL_HASH,
                self.as_c_arg(),
                salt as u64
            ) as u32
        }
    }

    /// Portable hash function that gives the same hash for the same Erlang term regardless of
    /// machine architecture and ERTS version.
    ///
    /// It generates hashes within 0..2^27-1.
    pub fn hash_phash2(&self) -> u32 {
        unsafe {
            ::wrapper::nif_interface::enif_hash(
                ::wrapper::nif_interface::NIF_HASH_TYPE::ERL_NIF_PHASH2,
                self.as_c_arg(),
                0
            ) as u32
        }
    }

}

impl<'a> PartialEq for Term<'a> {
    fn eq(&self, other: &Term) -> bool {
        unsafe {
            ::wrapper::nif_interface::enif_is_identical(self.as_c_arg(), other.as_c_arg()) == 1
        }
    }
}
impl<'a> Eq for Term<'a> {}

fn cmp(lhs: &Term, rhs: &Term) -> Ordering {
    let ord = unsafe { ::wrapper::nif_interface::enif_compare(lhs.as_c_arg(), rhs.as_c_arg()) };
    match ord {
        0 => Ordering::Equal,
        n if n < 0 => Ordering::Less,
        _ => Ordering::Greater,
    }
}

impl<'a> Ord for Term<'a> {
    fn cmp(&self, other: &Term) -> Ordering {
        cmp(self, other)
    }
}
impl<'a> PartialOrd for Term<'a> {
    fn partial_cmp(&self, other: &Term<'a>) -> Option<Ordering> {
        Some(cmp(self, other))
    }
}

impl<'a> Hash for Term<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // As far as I can see, there is really no way
        // to get a seed from the hasher. This is definitely
        // not optimal, but it's the best we can do for now.
        state.write_u32(self.hash_internal(0));
    }
}

unsafe impl<'a> Sync for Term<'a> {}
unsafe impl<'a> Send for Term<'a> {}
