use std::ffi::{self, c_char, CString, CStr};
use crate::renderer::RendererError;

/// Converts most types to a CString
pub fn convert_to_cstring<T: Into<Vec<u8>>>(str: T) -> Result<CString, RendererError> {
    match CString::new(str) {
        Ok(e) => Ok(e),
        Err(e) => Err(RendererError::from(e)),
    }
}

/// Converts a String to a CString as String does not fit in the bound given above
pub fn string_to_cstring(str: &String) -> Result<CString, RendererError> {
    convert_to_cstring(str.as_str())
}

/// Converts a character array to a CStr
pub unsafe fn char_array_to_cstr(array: &[c_char; 256]) -> &CStr {
    unsafe {CStr::from_ptr(array.as_ptr())}
}

/// Converts an entire vector of CStrings into a vector of character pointers
pub fn string_vector_to_char_vector(array: &[String]) -> Result<CharVector, RendererError> {
    let mut chars = Vec::new();
    let mut storage = Vec::new();
    for str in array {
        let current = string_to_cstring(str)?;
        chars.push(current.as_ptr());
        storage.push(current);
    };
    Ok(CharVector{
        chars,
        _storage: storage,
    })
}

/// Stores the CStrings with the character pointers to keep things safe
pub(crate) struct CharVector {
    pub chars: Vec<*const c_char>,
    _storage: Vec<CString>,
}

impl From<ffi::NulError> for RendererError {
    fn from(error: ffi::NulError) -> Self {
        RendererError::StringContainingNullCharError(error)
    }
}

impl From<ffi::FromBytesUntilNulError> for RendererError {
    fn from(_: ffi::FromBytesUntilNulError) -> Self {
        RendererError::CStringDidNotContainTerminatingNullButeError
    }
}