extern crate midi_message;

use midi_message::raw_midi_message::RawMidiMessage;

#[test]
fn new_() {
    let raw_midi_message = RawMidiMessage::new(144, 50, 50);
    assert_eq!(raw_midi_message.status, 144);
    assert_eq!(raw_midi_message.data1, 50);
    assert_eq!(raw_midi_message.data2, 50);
}
