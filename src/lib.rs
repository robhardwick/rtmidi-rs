//! # RtMidi
//!
//! A safe wrapper around [RtMidi](https://www.music.mcgill.ca/~gary/rtmidi/) that provides a
//! common API (Application Programming Interface) for realtime MIDI input/output across Linux
//! (ALSA & JACK), macOS (CoreMIDI & JACK), and Windows (Multimedia Library) operating systems.
//!
//! Where applicable, multiple API support can be compiled and a particular API specified when
//! creating an RtMidi instance.
//!
//! MIDI input and output functionality are separated into two structs, [`RtMidiIn`] and
//! [`RtMidiOut`]. Each instance supports only a single MIDI connection. RtMidi does not provide
//! timing functionality (i.e., output messages are sent immediately). Input messages are
//! timestamped with delta times in seconds (via an [`f64`] type). MIDI data is passed to the user as
//! raw bytes using a `&[u8]`.
//!
//! ## Probing Ports / Devices
//!
//! A client generally must query the available MIDI ports before deciding which to use. The
//! following example outlines how this can be done.
//!
//! ```
//! use rtmidi::{RtMidiIn, RtMidiOut, RtMidiError};
//!
//! fn main() -> Result<(), RtMidiError> {
//!     // Initialise MIDI input
//!     let input = RtMidiIn::new(Default::default())?;
//!
//!     // Get number of input ports
//!     let input_ports = input.port_count()?;
//!     println!("There are {} MIDI input sources available.", input_ports);
//!
//!     // List input ports
//!     for port in 0..input_ports {
//!         println!("\tInput Port #{}: {}", port+1, input.port_name(port)?);
//!     }
//!
//!     // Initialise MIDI output
//!     let output = RtMidiOut::new(Default::default())?;
//!
//!     // Get number of output ports
//!     let output_ports = output.port_count()?;
//!     println!("There are {} MIDI output ports available.", output_ports);
//!
//!     // List output ports
//!     for port in 0..output_ports {
//!         println!("\tOutput Port #{}: {}", port+1, output.port_name(port)?);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! Note that the port enumeration is system specific and will change if any devices are unplugged
//! or plugged (or a new virtual port opened or closed) by the user. Thus, the port numbers should
//! be verified immediately before opening a port. As well, if a user unplugs a device (or closes
//! a virtual port) while a port connection exists to that device/port, a MIDI system error will be
//! generated.
//!
//! ## MIDI Output
//!
//! RtMidiOut provides simple functionality to immediately send messages over a MIDI connection. No
//! timing functionality is provided.
//!
//! ```
//! use std::process::exit;
//! use std::thread::sleep;
//! use std::time::Duration;
//! use rtmidi::{RtMidiOut, RtMidiError};
//!
//! fn main() -> Result<(), RtMidiError> {
//!     // Initialise MIDI output
//!     let output = RtMidiOut::new(Default::default())?;
//!
//!     // Check available ports
//!     let ports = output.port_count()?;
//!     if ports < 1 {
//!         eprintln!("No ports available!");
//!         exit(1);
//!     }
//!
//!     // Open first available port
//!     output.open_port(0, "RtMidi Output")?;
//!
//!     // Program change: 192, 5
//!     output.message(&[192, 5])?;
//!
//!     // Control Change: 176, 7, 100 (volume)
//!     output.message(&[176, 7, 100])?;
//!
//!     // Note On: 144, 64, 90
//!     output.message(&[144, 64, 90])?;
//!
//!     sleep(Duration::from_millis(500));
//!
//!     // Note Off: 128, 64, 40
//!     output.message(&[128, 64, 40])?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## MIDI Input
//!
//! [`RtMidiIn`] uses an internal callback function or thread to receive incoming MIDI messages
//! from a port or device. These messages are then either queued and read by the user via calls to
//! [`RtMidiIn::message`] or immediately passed to a user-specified callback function (which must
//! be "registered" using [`RtMidiIn::set_callback`]). Note that if you have multiple instances of
//! [`RtMidiIn`], each may have its own thread.
//!
//! [`RtMidiIn`] provides [`RtMidiIn::ignore_types`] to specify that certain MIDI message types be
//! ignored. By default, system exclusive, timing, and active sensing messages are ignored.
//!
//! It is necessary to set the callback immediately after opening the port to avoid having incoming
//! messages written to the queue (which is not emptied when a callback function is set). If you
//! are worried about this happening, you can check the queue using [`RtMidiIn::message`] to verify
//! it is empty (after the callback is set).
//!
//! ```
//! use std::process::exit;
//! use std::io::{stdin, Read};
//! use rtmidi::{RtMidiIn, RtMidiError};
//!
//! fn main() -> Result<(), RtMidiError> {
//!     // Initialise MIDI input
//!     let input = RtMidiIn::new(Default::default())?;
//!
//!     // Check available ports
//!     let ports = input.port_count()?;
//!     if ports < 1 {
//!         eprintln!("No ports available!");
//!         exit(1);
//!     }
//!
//!     // Open first available port
//!     input.open_port(0, "RtMidi Input")?;
//!
//!     // Set our callback function.  This should be done immediately after
//!     // opening the port to avoid having incoming messages written to the
//!     // queue.
//!     input.set_callback(|timestamp, message| {
//!         for (index, byte) in message.iter().enumerate() {
//!             println!("Byte {} = 0x{:02x}, ", index, byte);
//!         }
//!     })?;
//!
//!     // Don't ignore sysex, timing, or active sensing messages.
//!     input.ignore_types(false, false, false)?;
//!
//!     println!("Reading MIDI input ...");
//!     stdin().read(&mut [0]).unwrap();
//!
//!     Ok(())
//! }
//! ```

mod api;
mod error;
mod ffi;
mod midi;
mod midi_in;
mod midi_out;

/// A MIDI input/output port identifier
pub type RtMidiPort = u32;

pub use api::RtMidiApi;
pub use error::RtMidiError;
pub use midi_in::{RtMidiIn, RtMidiInArgs};
pub use midi_out::{RtMidiOut, RtMidiOutArgs};
