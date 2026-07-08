use std::ffi::{self, c_char, CString, CStr};
use crate::renderer::RendererError;

/// Converts most types to a CString.
pub fn convert_to_cstring<T: Into<Vec<u8>>>(str: T) -> Result<CString, RendererError> {
    match CString::new(str) {
        Ok(e) => Ok(e),
        Err(e) => Err(RendererError::from(e)),
    }
}

/// Converts a String to a CString as String does not fit in the bound given above.
pub fn string_to_cstring(str: &String) -> Result<CString, RendererError> {
    convert_to_cstring(str.as_str())
}

/// Converts a character array to a CStr.
/// The array must have a valid nul terminator at the end.
pub unsafe fn char_array_to_cstr(array: &[c_char]) -> &CStr {
    unsafe {CStr::from_ptr(array.as_ptr())}
}

/// Converts an entire vector of CStrings into a vector of character pointers.
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

/// Stores the CStrings with the character pointers to keep things safe.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_empty_str_to_cstring() {
        assert_eq!(convert_to_cstring("").unwrap(), CString::new("").unwrap());
    }

    #[test]
    fn convert_valid_str_to_cstring() {
        assert_eq!(convert_to_cstring("Hello World!").unwrap(),
                   CString::new("Hello World!").unwrap());
    }

    #[test]
    fn convert_invalid_str_to_cstring() {
        assert!(convert_to_cstring("Hello\0World!").is_err());
    }

    #[test]
    fn convert_empty_string_to_cstring() {
        assert_eq!(string_to_cstring(&String::from("")).unwrap(), CString::new("").unwrap());
    }

    #[test]
    fn convert_valid_string_to_cstring() {
        assert_eq!(string_to_cstring(&String::from("Hello World!")).unwrap(),
                   CString::new("Hello World!").unwrap());
    }

    #[test]
    fn convert_invalid_string_to_cstring() {
        assert!(string_to_cstring(&String::from("Hello\0World!")).is_err());
    }

    #[test]
    fn convert_empty_char_array_to_cstring() {
        let c_char_text = Vec::new();
        let c_char_array: [c_char; 0] = c_char_text.try_into().unwrap();
        unsafe{ assert_eq!(char_array_to_cstr(&c_char_array),
                           CString::new("").unwrap().as_c_str()); }
    }

    #[test]
    fn convert_valid_char_array_to_cstring() {
        let text = b"Hello World!";
        let mut c_char_text = Vec::new();
        for char in text {
            c_char_text.push(*char as c_char)
        }
        let c_char_array: [c_char; 12] = c_char_text.try_into().unwrap();

        unsafe { assert_eq!(char_array_to_cstr(&c_char_array),
                            CString::new("Hello World!").unwrap().as_c_str()) }
    }
}