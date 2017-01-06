use super::nif_interface;
use super::nif_interface::{ NIF_ENV, NIF_TERM, c_uint };
use {NifResult, NifError};

#[allow(dead_code)]
pub fn make_atom(env: NIF_ENV, name: &str) -> NIF_TERM {
    unsafe { nif_interface::enif_make_atom_len(env, name.as_ptr() as *const u8, name.len() as usize) }
}

/// Get the contents of this atom as a string.
///
/// If you only need to test for equality, comparing the terms directly
/// is much faster.
///
/// # Errors
///
/// `NifError::BadArg` if `term` is not an atom.
///
pub fn get_atom(env: NIF_ENV, term: NIF_TERM) -> NifResult<String> {
    // Determine the length of the atom, in bytes.
    let mut len = 0;
    let success = unsafe {
        nif_interface::enif_get_atom_length_latin1(env, term, &mut len)
    };
    if success == 0 {
        return Err(NifError::BadArg);
    }

    // Get the bytes from the atom into a buffer.
    // enif_get_atom() writes a null terminated string,
    // so add 1 to the atom's length to make room for it.
    let mut bytes: Vec<u8> = Vec::with_capacity(len as usize + 1);
    let nbytes = unsafe {
        nif_interface::enif_get_atom_latin1(env, term, bytes.as_mut_ptr(), len + 1)
    };
    assert!(nbytes as c_uint == len + 1);
    unsafe {
        // This is safe unless the VM is lying to us.
        bytes.set_len(len as usize);  // drop the null byte
    }

    // Convert from Latin-1 bytes to a String.
    let nonascii_count = bytes.iter().filter(|&&b| b >= 128).count();
    if nonascii_count == 0 {
        // The string is ASCII, so it is safe to convert without copying.
        Ok(unsafe { String::from_utf8_unchecked(bytes) })
    } else {
        // Transcode from Latin-1 to UTF-8.
        let mut out = String::with_capacity(bytes.len() + nonascii_count);
        for b in bytes {
            out.push(b as char);
        }
        Ok(out)
    }
}
