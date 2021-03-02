use std::ffi::CStr;
use std::fmt;

use crate::ffi;

/// MIDI API specifier
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtMidiApi {
    Unspecified = ffi::RtMidiApi_RTMIDI_API_UNSPECIFIED,
    MacOSXCore = ffi::RtMidiApi_RTMIDI_API_MACOSX_CORE,
    LinuxALSA = ffi::RtMidiApi_RTMIDI_API_LINUX_ALSA,
    UnixJack = ffi::RtMidiApi_RTMIDI_API_UNIX_JACK,
    WindowsMM = ffi::RtMidiApi_RTMIDI_API_WINDOWS_MM,
    RtMidiDummy = ffi::RtMidiApi_RTMIDI_API_RTMIDI_DUMMY,
}

impl From<u32> for RtMidiApi {
    fn from(api: u32) -> Self {
        match api {
            ffi::RtMidiApi_RTMIDI_API_UNSPECIFIED => RtMidiApi::Unspecified,
            ffi::RtMidiApi_RTMIDI_API_MACOSX_CORE => RtMidiApi::MacOSXCore,
            ffi::RtMidiApi_RTMIDI_API_LINUX_ALSA => RtMidiApi::LinuxALSA,
            ffi::RtMidiApi_RTMIDI_API_UNIX_JACK => RtMidiApi::UnixJack,
            ffi::RtMidiApi_RTMIDI_API_WINDOWS_MM => RtMidiApi::WindowsMM,
            ffi::RtMidiApi_RTMIDI_API_RTMIDI_DUMMY => RtMidiApi::RtMidiDummy,
            _ => panic!("Invalid API value"),
        }
    }
}

impl fmt::Display for RtMidiApi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_name = unsafe { CStr::from_ptr(ffi::rtmidi_api_display_name(*self as u32)) };
        write!(f, "{}", display_name.to_str().map_err(|_| fmt::Error)?)
    }
}
