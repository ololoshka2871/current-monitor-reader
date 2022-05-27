use serde::{Deserialize, Serialize};

use structmap::{value::Value, GenericMap, StringMap, ToMap};
use structmap_derive::ToMap;

/// Ina219 input report (RAW values)
#[derive(ToMap, Deserialize, Serialize, Default)]
pub struct Ina219Report {
    pub voltage: u16,
    pub shunt_voltage: i16,
    pub current: i16,
    pub power: i16,
}

/// Ina219 input report (converted)
#[derive(ToMap, Deserialize, Serialize, Default)]
pub struct Ina219Result {
    // f64 по тому что Ina219Result::to_genericmap() не наботает с f32
    pub voltage: f64,
    pub shunt_voltage: f64,
    pub current: f64,
    pub power: f64,
}

impl From<[u8; 8]> for Ina219Report {
    fn from(raw_data: [u8; 8]) -> Ina219Report {
        match ssmarshal::deserialize(&raw_data) {
            Ok((r, _)) => r,
            Err(e) => panic!("{}", e),
        }
    }
}

impl Ina219Report {
    pub fn to_result(&self) -> Ina219Result {
        Ina219Result {
            voltage: self.voltage as f64 / 1000.0,               // mV -> V
            shunt_voltage: self.shunt_voltage as f64 / 100000.0, // 0.01mV -> V
            current: self.current as f64,
            power: self.power as f64,
        }
    }
}
