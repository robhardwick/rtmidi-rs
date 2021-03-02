use std::ffi::CString;

use crate::api::RtMidiApi;
use crate::error::RtMidiError;
use crate::ffi;
use crate::midi;
use crate::RtMidiPort;

const DEFAULT_CLIENT_NAME: &str = "RtMidi Output Client";

/// Output arguments
///
/// Defines arguments used when constructing [`RtMidiOut`].
/// ```
/// use rtmidi::{RtMidiOut, RtMidiOutArgs};
///
/// RtMidiOut::new(
///     RtMidiOutArgs {
///         client_name: "My Output Client",
///         ..Default::default()
///     }
/// );
/// ```
pub struct RtMidiOutArgs<'a> {
    pub api: RtMidiApi,
    pub client_name: &'a str,
}

impl<'a> Default for RtMidiOutArgs<'a> {
    fn default() -> Self {
        RtMidiOutArgs {
            api: RtMidiApi::Unspecified,
            client_name: DEFAULT_CLIENT_NAME,
        }
    }
}

/// Realtime MIDI output
///
/// This provides a common, platform-independent API for MIDI output. It allows one to probe
/// available MIDI output ports, to connect to one such port, and to send MIDI bytes immediately
/// over the connection. Create multiple instances to connect to more than one MIDI device at the
/// same time. With the macOS, Linux ALSA and JACK MIDI APIs, it is also possible to open a virtual
/// port to which other MIDI software clients can connect.
///
/// ```
/// use rtmidi::RtMidiOut;
///
/// // Initialise MIDI output with default arguments
/// let output = RtMidiOut::new(Default::default()).unwrap();
///
/// // List output ports
/// for port in 0..output.port_count().unwrap() {
///     println!("{}: {}", port+1, output.port_name(port).unwrap());
/// }
///
/// ```
pub struct RtMidiOut(*mut ffi::RtMidiWrapper);

impl RtMidiOut {
    /// Default constructor that allows an optional api and client name using the
    /// [`RtMidiOutArgs`] type.
    ///
    /// An exception will be thrown if a MIDI system initialization error occurs.
    ///
    /// If no API argument is specified and multiple API support has been compiled, the default
    /// order of use is ALSA, JACK (Linux) and CORE, JACK (macOS).
    pub fn new(args: RtMidiOutArgs) -> Result<Self, RtMidiError> {
        let client_name = CString::new(args.client_name)?;
        let ptr = unsafe { ffi::rtmidi_out_create(args.api as u32, client_name.as_ptr()) };
        match unsafe { Result::<(), RtMidiError>::from(*ptr) } {
            Ok(_) => Ok(RtMidiOut(ptr)),
            Err(e) => Err(e),
        }
    }

    /// Returns the MIDI API specifier for the current instance
    pub fn current_api(&self) -> RtMidiApi {
        let api = unsafe { ffi::rtmidi_out_get_current_api(self.0) };
        api.into()
    }

    /// Open a MIDI output connection
    pub fn open_port<T: AsRef<str>>(
        &self,
        port_number: RtMidiPort,
        port_name: T,
    ) -> Result<(), RtMidiError> {
        midi::open_port(self.0, port_number, port_name)
    }

    /// Create a virtual output port, with a name, to allow software connections (macOS, JACK and
    /// ALSA only).
    ///
    /// This function creates a virtual MIDI output port to which other software applications can
    /// connect. This type of functionality is currently only supported by the macOS, Linux ALSA
    /// and JACK APIs (the function does nothing with the other APIs). An error is returned if an
    /// error occurs while attempting to create the virtual port.
    pub fn open_virtual_port<T: AsRef<str>>(&self, port_name: T) -> Result<(), RtMidiError> {
        midi::open_virtual_port(self.0, port_name)
    }

    /// Close an open MIDI connection (if one exists)
    pub fn close_port(&self) -> Result<(), RtMidiError> {
        midi::close_port(self.0)
    }

    /// Return the number of available MIDI output ports
    pub fn port_count(&self) -> Result<RtMidiPort, RtMidiError> {
        midi::port_count(self.0)
    }

    /// Return a string identifier for the specified MIDI output port number
    pub fn port_name(&self, port_number: RtMidiPort) -> Result<&str, RtMidiError> {
        midi::port_name(self.0, port_number)
    }

    /// Immediately send a single message out an open MIDI output port.
    ///
    /// An error is returned if an error occurs during output or an output connection was not
    /// previously established.
    pub fn message(&self, message: &[u8]) -> Result<(), RtMidiError> {
        let length = message.len();
        unsafe {
            ffi::rtmidi_out_send_message(self.0, message.as_ptr(), length as i32);
            (*self.0).into()
        }
    }
}

impl Drop for RtMidiOut {
    fn drop(&mut self) {
        unsafe { ffi::rtmidi_out_free(self.0) }
    }
}

#[cfg(test)]
mod tests {
    use super::{RtMidiOut, RtMidiOutArgs};
    use crate::RtMidiApi;

    #[test]
    fn new() {
        assert!(RtMidiOut::new(RtMidiOutArgs {
            client_name: "Test",
            ..Default::default()
        })
        .is_ok());
    }

    #[test]
    fn current_api() {
        assert_ne!(
            RtMidiOut::new(Default::default()).unwrap().current_api(),
            RtMidiApi::Unspecified
        );
    }

    #[test]
    fn open_port() {
        assert!(RtMidiOut::new(Default::default())
            .unwrap()
            .open_port(9999, "Test")
            .is_err());
    }

    #[test]
    fn open_virtual_port() {
        assert!(RtMidiOut::new(Default::default())
            .unwrap()
            .open_virtual_port("Test")
            .is_ok());
    }

    #[test]
    fn close_port() {
        assert!(RtMidiOut::new(Default::default())
            .unwrap()
            .close_port()
            .is_ok());
    }

    #[test]
    fn port_count() {
        assert!(RtMidiOut::new(Default::default())
            .unwrap()
            .port_count()
            .is_ok());
    }

    #[test]
    fn port_name() {
        assert_eq!(
            RtMidiOut::new(Default::default())
                .unwrap()
                .port_name(9999)
                .unwrap(),
            ""
        );
    }

    #[test]
    fn message() {
        assert!(RtMidiOut::new(Default::default())
            .unwrap()
            .message(&[0, 0, 0])
            .is_ok());
    }
}
