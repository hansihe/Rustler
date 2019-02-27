use rustler::{Env, Term, NifResult, Encoder};
use std::io::Write;
use std::cmp::Ordering;

pub fn term_debug<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let mut bytes: Vec<u8> = Vec::new();
    write!(&mut bytes, "{:?}", args[0]).expect("debug formatting should succeed");
    let s = String::from_utf8_lossy(&bytes).to_string();
    Ok(s.encode(env))
}

mod atoms {
    rustler::rustler_atoms! {
        atom equal;
        atom less;
        atom greater;
    }
}

pub fn term_eq<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    Ok((args[0] == args[1]).encode(env))
}

pub fn term_cmp<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    match Ord::cmp(&args[0], &args[1]) {
        Ordering::Equal => Ok(atoms::equal().encode(env)),
        Ordering::Less => Ok(atoms::less().encode(env)),
        Ordering::Greater => Ok(atoms::greater().encode(env)),
    }
}

pub fn term_internal_hash<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let salt: u32 = args[1].decode()?;
    Ok(args[0].hash_internal(salt).encode(env))
}

pub fn term_phash2_hash<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    Ok(args[0].hash_phash2().encode(env))
}
