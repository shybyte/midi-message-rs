pub type U7 = u8;
pub type Note = U7;
pub type Channel = u8;
pub type Velocity = u8;

const NOTE_OFF:u8 = 0x8;
const NOTE_ON:u8 = 0x9;


#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MidiMessage {
    NoteOff(Channel, Note, Velocity),
    ProgramChange(Channel, U7),
    ControlChange(Channel, U7, U7),
    NoteOn(Channel, Note, Velocity),
    Unknown
}

impl MidiMessage {
    pub fn new(status_and_channel: u8, data1: U7, data2: U7) -> Self {
        let status = status_and_channel >> 4;
        let channel = status_and_channel & 0xf;
        match status {
            NOTE_OFF => MidiMessage::NoteOff(channel, data1, data2),
            NOTE_ON => MidiMessage::NoteOn(channel, data1, data2),
            _ => MidiMessage::Unknown
        };
        MidiMessage::NoteOn(0, data1, data2)
    }
}


