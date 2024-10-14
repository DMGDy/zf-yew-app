use serde::{Serialize,Deserialize};
use std::{
    fmt,
    str::FromStr,
};


#[derive(Serialize,Deserialize,Clone,PartialEq)]
pub enum Device {
    BST,
    CWS,
    PrS,
    ESCM,
    None,
}

impl Device {
    pub fn abbrev(&self) -> &str {
        match self {
            Device::BST => "BST",
            Device::CWS => "CWS",
            Device::PrS => "PrS",
            Device::ESCM => "ESCM",
            Device::None => "N/A"
        }
    }
}

impl FromStr for Device {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Break Signal Transmitter" =>Ok(Device::BST),
            "Continuious Wear Sensor" => Ok(Device::CWS),
            "Pressure Sensor" => Ok(Device::PrS),
            "Electronic Stability Control Module" => Ok(Device::ESCM),
            "None" => Ok(Device::None),
            _ =>Err(())
        }
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Device::BST => write!(f, "Brake Signal Transmitter"),
            Device::CWS => write!(f, "Continuous Wear Sensor"),
            Device::PrS => write!(f, "Pressure Sensor"),
            Device::ESCM=> write!(f, "Electronic Stability Control Module"),
            Device::None=> write!(f, "No Device Selected"),
        }
    }
}

#[derive(Default,Serialize,Clone,PartialEq)]
pub struct TestData {
    pub device: String,
    pub check: bool,
}
