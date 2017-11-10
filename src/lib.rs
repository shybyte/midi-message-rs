pub type U7 = u8;
pub type Note = U7;
pub type Velocity = U7;
pub type Channel = u8; // 0-15
pub type ProgramNumber = U7;
pub type ControllerNumber = U7;
pub type ControllerValue = U7;

const NOTE_OFF:u8 = 0x8;
const NOTE_ON:u8 = 0x9;
const CONTROL_CHANGE:u8 = 0xB;
const PROGRAM_CHANGE:u8 = 0xC;


#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MidiMessage {
    NoteOff(Channel, Note, Velocity),
    ProgramChange(Channel, ProgramNumber),
    ControlChange(Channel, ControllerNumber, ControllerValue),
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
            PROGRAM_CHANGE => MidiMessage::ProgramChange(channel, data1),
            CONTROL_CHANGE => MidiMessage::ControlChange(channel, data1, data2),
            _ => MidiMessage::Unknown
        }
    }
}


