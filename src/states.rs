use serde::Deserialize;

#[derive(Deserialize,Clone)]
pub enum State {
    Offline,
    Online,
    InProgress,
    Pass,
    Fail,
    ENoFirmware,
    ENoRead,
    ENoWrite,
    EOpen,
    EUnknown,
}

impl State {

    pub fn code(&self) -> i32 {
        match self {
            Self::Offline => 0,
            Self::Online => 1,
            Self::InProgress => 2,
            Self::Pass=> 3,
            Self::Fail=> 4,
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
            Self::Pass => "The test inidicates the device passes",
            Self::Fail=> "The test indicates the device fails",
            Self::ENoFirmware => "No Firmware was found for the selected device",
            Self::ENoRead => "There was an error reading data from the microcontroller",
            Self::ENoWrite => "There was an error writing data to the microcontroller",
            Self::EOpen => "There was an error trying to communicate to the microcontroller",
            Self::EUnknown => "Something bad went wrong, check browser console",
        }
    }
}

