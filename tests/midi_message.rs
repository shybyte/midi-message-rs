extern crate midi_message;

use midi_message::MidiMessage;

#[test]
fn note_on() {
    assert_eq!(MidiMessage::new(0x90, 24, 100), MidiMessage::NoteOn(0, 24, 100));
    assert_eq!(MidiMessage::new(0x91, 24, 100), MidiMessage::NoteOn(1, 24, 100));
}

#[test]
fn note_off() {
    assert_eq!(MidiMessage::new(0x80, 24, 100), MidiMessage::NoteOff(0, 24, 100));
    assert_eq!(MidiMessage::new(0x8f, 24, 100), MidiMessage::NoteOff(0xf, 24, 100));
}

#[test]
fn program_change() {
    assert_eq!(MidiMessage::new(0xC0, 23, 100), MidiMessage::ProgramChange(0, 23));
}

#[test]
fn control_change() {
    assert_eq!(MidiMessage::new(0xB0, 23, 100), MidiMessage::ControlChange(0, 23, 100));
}

#[test]
fn unknown_message() {
    assert_eq!(MidiMessage::new(0xff, 24, 100), MidiMessage::Unknown);
}

