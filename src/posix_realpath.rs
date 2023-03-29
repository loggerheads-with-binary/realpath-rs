extern crate libc;
use libc::realpath as libc_realpath;

extern crate path_absolutize;
use path_absolutize::Absolutize;

use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Error {
    NullError,
    PathConversionError,
}

pub fn realpath(path: &PathBuf) -> Result<PathBuf, Error> {
    //Convert path to realpath using libc

    let path = path.absolutize().unwrap();

    let path = path.to_str().ok_or_else(|| Error::PathConversionError)?;
    let path = path.to_string();
    let path = path.as_bytes();
    let path = path.as_ptr();

    let path = path as *const i8;

    let mut buf = [0; libc::PATH_MAX as usize];
    let buf = buf.as_mut_ptr();
    let buf = buf as *mut i8;

    let result = unsafe { libc_realpath(path, buf) };

    if result.is_null() {
        return Err(Error::NullError);
    } else {
        let result = unsafe { std::ffi::CStr::from_ptr(result) };
        let result = result
            .to_str()
            .or_else(|_| Err(Error::PathConversionError))?;
        let result = result.to_string();
        let result = PathBuf::from(result);
        Ok(result)
    }
}
