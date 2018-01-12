use rustler::{NifEnv, NifTerm, NifResult, NifEncoder};
use std::io::Write;
use std::cmp::Ordering;

pub fn term_debug<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let mut bytes: Vec<u8> = Vec::new();
    write!(&mut bytes, "{:?}", args[0]).expect("debug formatting should succeed");
    let s = String::from_utf8_lossy(&bytes).to_string();
    Ok(s.encode(env))
}

mod atoms {
    rustler_atoms! {
        atom equal;
        atom less;
        atom greater;
    }
}

pub fn term_eq<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    Ok((args[0] == args[1]).encode(env))
}
pub fn term_cmp<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    match Ord::cmp(&args[0], &args[1]) {
        Ordering::Equal => Ok(atoms::equal().encode(env)),
        Ordering::Less => Ok(atoms::less().encode(env)),
        Ordering::Greater => Ok(atoms::greater().encode(env)),
    }
}
