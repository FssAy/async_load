use std::ffi::{CStr, CString, NulError};
use std::os::raw::c_char;
use std::ptr::null;
use std::str::Utf8Error;

/// Alias to the number type supported by GMS2.
pub type GMLDouble = f64;

/// Alias to the string type supported by GMS2
pub type GMLString = *const c_char;

/// Trait used to convert `GMLString` into a readable type.
pub trait GMLStringRead {
    /// Converts itself into a more Rust-friendly C string.
    fn as_cstr(&self) -> &CStr;

    /// Converts itself into `&str`.
    ///
    /// Can fail if it's not a valid string.
    fn as_str(&self) -> Result<&str, Utf8Error>;
}

/// Trait used to convert `Self` into a `GMLString`.
pub trait GMLStringWrite {
    /// Converts itself into a `GMLString`.
    ///
    /// Fails if it contains byte `0`.
    fn into_gml(self) -> Result<GMLString, NulError>;
}

/// Trait used to create empty version of `Self`.
pub trait None {
    fn none() -> Self;
}

impl GMLStringRead for GMLString {
    fn as_cstr(&self) -> &CStr {
        unsafe { &CStr::from_ptr(*self) }
    }

    fn as_str(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(*self).to_str() }
    }
}

impl GMLStringWrite for &str {
    fn into_gml(self) -> Result<GMLString, NulError> {
        CString::new(self)
            .map(|c_string| c_string.into_raw() as GMLString)
    }
}

impl GMLStringWrite for String {
    fn into_gml(self) -> Result<GMLString, NulError> {
        self.as_str().into_gml()
    }
}

impl None for GMLString {
    fn none() -> Self {
        null()
    }
}

impl None for GMLDouble {
    fn none() -> Self {
        0.0
    }
}
