use std::ffi::{CStr, NulError};
use std::str::Utf8Error;

use crate::ffi;

/// MIDI error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RtMidiError {
    Error(String),
    Utf8(Utf8Error),
    NullString(NulError),
    NullPointer,
}

impl From<ffi::RtMidiWrapper> for Result<(), RtMidiError> {
    fn from(e: ffi::RtMidiWrapper) -> Self {
        if e.ok {
            Ok(())
        } else if e.msg.is_null() {
            Err(RtMidiError::Error("Invalid error".to_string()))
        } else if let Ok(message) = unsafe { CStr::from_ptr(e.msg) }.to_str() {
            Err(RtMidiError::Error(message.to_string()))
        } else {
            Err(RtMidiError::Error("Unknown error".to_string()))
        }
    }
}

impl From<Utf8Error> for RtMidiError {
    fn from(e: Utf8Error) -> Self {
        RtMidiError::Utf8(e)
    }
}

impl From<NulError> for RtMidiError {
    fn from(e: NulError) -> Self {
        RtMidiError::NullString(e)
    }
}
