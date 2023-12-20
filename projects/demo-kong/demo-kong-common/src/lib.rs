#![no_std]
use serde::{Serialize, Deserialize};


#[repr(C)]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct BackendPorts {
    pub ports: [u16; 16],
    pub index: usize,
}

#[cfg(feature = "user")]
unsafe impl aya::Pod for BackendPorts {}
