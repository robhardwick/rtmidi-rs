use std::ffi::{CStr, CString};

use crate::error::RtMidiError;
use crate::ffi;
use crate::RtMidiPort;

pub fn open_port<T: AsRef<str>>(
    ptr: *mut ffi::RtMidiWrapper,
    port_number: RtMidiPort,
    port_name: T,
) -> Result<(), RtMidiError> {
    let port_name = CString::new(port_name.as_ref())?;
    unsafe {
        ffi::rtmidi_open_port(ptr, port_number, port_name.as_ptr());
        (*ptr).into()
    }
}

pub fn open_virtual_port<T: AsRef<str>>(
    ptr: *mut ffi::RtMidiWrapper,
    port_name: T,
) -> Result<(), RtMidiError> {
    let port_name = CString::new(port_name.as_ref())?;
    unsafe {
        ffi::rtmidi_open_virtual_port(ptr, port_name.as_ptr());
        (*ptr).into()
    }
}

pub fn close_port(ptr: *mut ffi::RtMidiWrapper) -> Result<(), RtMidiError> {
    unsafe {
        ffi::rtmidi_close_port(ptr);
        (*ptr).into()
    }
}

pub fn port_count(ptr: *mut ffi::RtMidiWrapper) -> Result<RtMidiPort, RtMidiError> {
    let port_count = unsafe { ffi::rtmidi_get_port_count(ptr) };
    match unsafe { Result::<(), RtMidiError>::from(*ptr) } {
        Ok(_) => Ok(port_count),
        Err(e) => Err(e),
    }
}

pub fn port_name<'a>(
    ptr: *mut ffi::RtMidiWrapper,
    port_number: RtMidiPort,
) -> Result<&'a str, RtMidiError> {
    let port_name = unsafe { ffi::rtmidi_get_port_name(ptr, port_number) };
    match unsafe { Result::<(), RtMidiError>::from(*ptr) } {
        Ok(_) if port_name.is_null() => Err(RtMidiError::NullPointer),
        Ok(_) => {
            let port_name = unsafe { CStr::from_ptr(port_name) };
            Ok(port_name.to_str()?)
        }
        Err(e) => Err(e),
    }
}
