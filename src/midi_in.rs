use std::ffi::{c_void, CString};

use crate::api::RtMidiApi;
use crate::error::RtMidiError;
use crate::ffi;
use crate::midi;
use crate::RtMidiPort;

const DEFAULT_CLIENT_NAME: &str = "RtMidi Input Client";

/// Input arguments
///
/// Defines arguments used when constructing [`RtMidiIn`].
/// ```
/// use rtmidi::{RtMidiIn, RtMidiInArgs};
///
/// RtMidiIn::new(
///     RtMidiInArgs {
///         client_name: "My Input Client",
///         queue_size_limit: 256,
///         ..Default::default()
///     }
/// );
/// ```
pub struct RtMidiInArgs<'a> {
    /// API to use
    pub api: RtMidiApi,
    /// A client name used to group ports created by the application
    pub client_name: &'a str,
    /// Size of the MIDI input queue
    pub queue_size_limit: u32,
}

impl<'a> Default for RtMidiInArgs<'a> {
    fn default() -> Self {
        RtMidiInArgs {
            api: RtMidiApi::Unspecified,
            client_name: DEFAULT_CLIENT_NAME,
            queue_size_limit: 100,
        }
    }
}

/// Realtime MIDI input
///
/// This provides a common, platform-independent API for realtime MIDI input. It allows access to a
/// single MIDI input port. Incoming MIDI messages are either saved to a queue for retrieval using
/// [`RtMidiIn::message`] or immediately passed to a user-specified callback (which must be
/// "registered" using [`RtMidiIn::set_callback`]).
///
/// Create multiple instances to connect to more than one MIDI device at the same time.
///
/// With the macOS, Linux ALSA, and JACK MIDI APIs, it is also possible to open a virtual input
/// port to which other MIDI software clients can connect.
///
/// ```
/// use rtmidi::RtMidiIn;
///
/// // Initialise MIDI input with default arguments
/// let input = RtMidiIn::new(Default::default()).unwrap();
///
/// // List input ports
/// for port in 0..input.port_count().unwrap() {
///     println!("{}: {}", port+1, input.port_name(port).unwrap());
/// }
///
/// ```
pub struct RtMidiIn(*mut ffi::RtMidiWrapper);

impl RtMidiIn {
    /// Default constructor that allows an optional api, client name and queue size using the
    /// [`RtMidiInArgs`] type.
    ///
    /// An error will be returned if a MIDI system initialization error occurs. The queue size
    /// defines the maximum number of messages that can be held in the MIDI queue (when not using a
    /// callback). If the queue size limit is reached, incoming messages will be ignored.
    ///
    /// If no API argument is specified and multiple API support has been compiled, the default
    /// order of use is ALSA, JACK (Linux) and CORE, JACK (macOS).
    pub fn new(args: RtMidiInArgs) -> Result<Self, RtMidiError> {
        let client_name = CString::new(args.client_name)?;
        let ptr = unsafe {
            ffi::rtmidi_in_create(args.api as u32, client_name.as_ptr(), args.queue_size_limit)
        };
        match unsafe { Result::<(), RtMidiError>::from(*ptr) } {
            Ok(_) => Ok(RtMidiIn(ptr)),
            Err(e) => Err(e),
        }
    }

    /// Returns the MIDI API specifier for the current instance
    pub fn current_api(&self) -> RtMidiApi {
        let api = unsafe { ffi::rtmidi_in_get_current_api(self.0) };
        api.into()
    }

    /// Open a MIDI input connection given by enumeration number
    pub fn open_port<T: AsRef<str>>(
        &self,
        port_number: RtMidiPort,
        port_name: T,
    ) -> Result<(), RtMidiError> {
        midi::open_port(self.0, port_number, port_name)
    }

    /// Create a virtual input port, with a name, to allow software connections (macOS, JACK and
    /// ALSA only).
    ///
    /// This function creates a virtual MIDI input port to which other software applications can
    /// connect. This type of functionality is currently only supported by the macOS, any JACK,
    /// and Linux ALSA APIs (the function returns an error for the other APIs).
    pub fn open_virtual_port<T: AsRef<str>>(&self, port_name: T) -> Result<(), RtMidiError> {
        midi::open_virtual_port(self.0, port_name)
    }

    /// Close an open MIDI connection (if one exists)
    pub fn close_port(&self) -> Result<(), RtMidiError> {
        midi::close_port(self.0)
    }

    /// Return the number of available MIDI input ports
    pub fn port_count(&self) -> Result<RtMidiPort, RtMidiError> {
        midi::port_count(self.0)
    }

    /// Return a string identifier for the specified MIDI input port number
    pub fn port_name(&self, port_number: RtMidiPort) -> Result<&str, RtMidiError> {
        midi::port_name(self.0, port_number)
    }

    /// Set a callback function to be invoked for incoming MIDI messages.
    ///
    /// The callback function will be called whenever an incoming MIDI message is received. The
    /// callback is passed the event delta-time in seconds and a slice with the data bytes for the
    /// MIDI message.
    ///
    /// While not absolutely necessary, it is best to set the callback function before opening a
    /// MIDI port to avoid leaving some messages in the queue.
    pub fn set_callback<F: Fn(f64, &[u8])>(&self, callback: F) -> Result<(), RtMidiError> {
        let (callback, user_data) = ffi::create_callback(callback);
        unsafe {
            ffi::rtmidi_in_set_callback(self.0, Some(callback), user_data as *mut c_void);
            (*self.0).into()
        }
    }

    /// Cancel use of the current callback function (if one exists).
    ///
    /// Subsequent incoming MIDI messages will be written to the queue and can be retrieved with
    /// [`RtMidiIn::message`].
    pub fn cancel_callback(&self) -> Result<(), RtMidiError> {
        unsafe {
            ffi::rtmidi_in_cancel_callback(self.0);
            (*self.0).into()
        }
    }

    /// Specify whether certain MIDI message types should be queued or ignored during input.
    ///
    /// By default, MIDI timing and active sensing messages are ignored during message input
    /// because of their relative high data rates. MIDI sysex messages are ignored by default as
    /// well. Values of [`true`] imply that the respective message type will be ignored.
    pub fn ignore_types(
        &self,
        midi_sysex: bool,
        midi_time: bool,
        midi_sense: bool,
    ) -> Result<(), RtMidiError> {
        unsafe {
            ffi::rtmidi_in_ignore_types(self.0, midi_sysex, midi_time, midi_sense);
            (*self.0).into()
        }
    }

    /// Return a vector with the data bytes for the next available MIDI message in the input queue
    /// and the event delta-time in seconds.
    ///
    /// This function returns immediately whether a new message is available or not. A valid
    /// message is indicated by a non-zero vector size. An exception is thrown if an error occurs
    /// during message retrieval or an input connection was not previously established.
    pub fn message(&self) -> Result<(f64, Vec<u8>), RtMidiError> {
        let mut length = 0u64;
        let mut message = Vec::with_capacity(1024);
        let ptr = message.as_mut_ptr();
        let timestamp = unsafe { ffi::rtmidi_in_get_message(self.0, ptr, &mut length) };
        match unsafe { Result::<(), RtMidiError>::from(*self.0) } {
            Ok(_) => Ok((timestamp, message)),
            Err(e) => Err(e),
        }
    }
}

impl Drop for RtMidiIn {
    fn drop(&mut self) {
        unsafe { ffi::rtmidi_in_free(self.0) }
    }
}

#[cfg(test)]
mod tests {
    use super::{RtMidiIn, RtMidiInArgs};
    use crate::api::RtMidiApi;

    #[test]
    fn new() {
        assert!(RtMidiIn::new(RtMidiInArgs {
            client_name: "Test",
            ..Default::default()
        })
        .is_ok());
    }

    #[test]
    fn current_api() {
        assert_ne!(
            RtMidiIn::new(Default::default()).unwrap().current_api(),
            RtMidiApi::Unspecified
        );
    }

    #[test]
    #[cfg(not(target_os = "linux"))]
    fn open_port() {
        assert!(RtMidiIn::new(Default::default())
            .unwrap()
            .open_port(9999, "Test")
            .is_err());
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn open_port() {
        assert!(RtMidiIn::new(Default::default())
            .unwrap()
            .open_port(9999, "Test")
            .is_ok());
    }

    #[test]
    fn open_virtual_port() {
        assert!(RtMidiIn::new(Default::default())
            .unwrap()
            .open_virtual_port("Test")
            .is_ok());
    }

    #[test]
    fn close_port() {
        assert!(RtMidiIn::new(Default::default())
            .unwrap()
            .close_port()
            .is_ok());
    }

    #[test]
    fn port_count() {
        assert!(RtMidiIn::new(Default::default())
            .unwrap()
            .port_count()
            .is_ok());
    }

    #[test]
    fn port_name() {
        assert_eq!(
            RtMidiIn::new(Default::default())
                .unwrap()
                .port_name(9999)
                .unwrap(),
            ""
        );
    }

    #[test]
    fn set_callback() {
        assert!(RtMidiIn::new(Default::default())
            .unwrap()
            .set_callback(|_time, _message| {})
            .is_ok());
    }

    #[test]
    fn cancel_callback() {
        assert!(RtMidiIn::new(Default::default())
            .unwrap()
            .cancel_callback()
            .is_ok());
    }

    #[test]
    fn ignore_types() {
        assert!(RtMidiIn::new(Default::default())
            .unwrap()
            .ignore_types(false, false, false)
            .is_ok());
    }

    #[test]
    fn message() {
        assert!(RtMidiIn::new(Default::default()).unwrap().message().is_ok());
    }
}
