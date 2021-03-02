# RtMidi [![crates.io](https://img.shields.io/crates/v/rtmidi.svg)](https://crates.io/crates/rtmidi) [![Test](https://github.com/robhardwick/rtmidi-rs/actions/workflows/test.yml/badge.svg)](https://github.com/robhardwick/rtmidi-rs/actions)

A safe wrapper around [RtMidi](https://www.music.mcgill.ca/~gary/rtmidi/) that provides a
common API (Application Programming Interface) for realtime MIDI input/output across Linux
(ALSA & JACK), macOS (CoreMIDI & JACK), and Windows (Multimedia Library) operating systems.