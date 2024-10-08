use serde::Deserialize;

#[derive(Deserialize,Clone)]
pub enum State {
    Offline,
    Online,
    InProgress,
    Done,
    ENoFirmware,
    ENoRead,
    ENoWrite,
    EOpen,
    EUnknown,
}

impl State {
    pub fn from_i32(n: i32) -> State {
        match n {
            0 => Self::Offline,
            1 => Self::Online,
            2 => Self::InProgress,
            3 => Self::Done,
            -1 => Self::ENoFirmware,
            -2 => Self::ENoRead,
            -3 => Self::ENoWrite,
            -4 => Self::EOpen,
            _ => Self:: EUnknown,
        }
    }
    pub fn code(&self) -> i32 {
        match self {
            Self::Offline => 0,
            Self::Online => 1,
            Self::InProgress => 2,
            Self::Done => 3,
            Self::ENoFirmware=> -1,
            Self::ENoRead=> -2,
            Self::ENoWrite=> -3,
            Self::EOpen=> -4,
            Self::EUnknown=> -6,

        }
    }

    pub fn message(&self) -> &str{
        match self {
            Self::Offline => "Trying to connect to server...",
            Self::Online => "Server is up. Waiting for test to begin.",
            Self::InProgress => "Microcontroller has began testing",
            Self::Done => "Test results are available",
            Self::ENoFirmware => "No Firmware was found for the selected device",
            Self::ENoRead => "There was an error reading data from the microcontroller",
            Self::ENoWrite => "There was an error writing data to the microcontroller",
            Self::EOpen => "There was an error trying to communicate to the microcontroller",
            Self::EUnknown => "Something bad went wrong, check browser console",
        }
    }
}

