#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[cfg(rtmidi_version = "v4_0_0")]
mod lib {
    use std::ffi::c_void;
    use std::slice;

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    pub fn create_callback<F: Fn(f64, &[u8])>(
        f: F,
    ) -> (
        unsafe extern "C" fn(f64, *const u8, u64, *mut c_void),
        *mut F,
    ) {
        unsafe extern "C" fn trampoline<F: Fn(f64, &[u8])>(
            timestamp: f64,
            data: *const u8,
            size: u64,
            func: *mut c_void,
        ) {
            let messages = slice::from_raw_parts(data, size as usize);
            (*(func as *mut F))(timestamp, messages)
        }
        (trampoline::<F>, Box::into_raw(Box::new(f)))
    }
}

#[cfg(rtmidi_version = "v4_0_0")]
pub use lib::*;

#[cfg(rtmidi_version = "v3_0_0")]
mod lib {
    use std::ffi::c_void;
    use std::os::raw::{c_char, c_uchar};
    use std::ptr;
    use std::slice;

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    pub const RtMidiApi_RTMIDI_API_UNSPECIFIED: RtMidiApi = RtMidiApi_RT_MIDI_API_UNSPECIFIED;
    pub const RtMidiApi_RTMIDI_API_MACOSX_CORE: RtMidiApi = RtMidiApi_RT_MIDI_API_MACOSX_CORE;
    pub const RtMidiApi_RTMIDI_API_LINUX_ALSA: RtMidiApi = RtMidiApi_RT_MIDI_API_LINUX_ALSA;
    pub const RtMidiApi_RTMIDI_API_UNIX_JACK: RtMidiApi = RtMidiApi_RT_MIDI_API_UNIX_JACK;
    pub const RtMidiApi_RTMIDI_API_WINDOWS_MM: RtMidiApi = RtMidiApi_RT_MIDI_API_WINDOWS_MM;
    pub const RtMidiApi_RTMIDI_API_RTMIDI_DUMMY: RtMidiApi = RtMidiApi_RT_MIDI_API_RTMIDI_DUMMY;

    pub fn rtmidi_api_display_name(_api: u32) -> *const c_char {
        ptr::null()
    }

    pub fn create_callback<F: Fn(f64, &[u8])>(
        f: F,
    ) -> (unsafe extern "C" fn(f64, *const u8, *mut c_void), *mut F) {
        unsafe extern "C" fn trampoline<F: Fn(f64, &[u8])>(
            timestamp: f64,
            data: *const u8,
            func: *mut c_void,
        ) {
            let messages = slice::from_raw_parts(data, 3);
            (*(func as *mut F))(timestamp, messages)
        }
        (trampoline::<F>, Box::into_raw(Box::new(f)))
    }

    pub unsafe fn wrap_rtmidi_in_get_message(
        device: RtMidiInPtr,
        mut message: *mut c_uchar,
        size: *mut size_t,
    ) -> f64 {
        rtmidi_in_get_message(device, &mut message, size)
    }
}

#[cfg(rtmidi_version = "v3_0_0")]
pub use lib::{wrap_rtmidi_in_get_message as rtmidi_in_get_message, *};
