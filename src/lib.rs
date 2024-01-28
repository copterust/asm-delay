//! no_std implementation of DelayMs and DelayUs for cortex-m

#![deny(missing_docs)]
#![no_std]

pub use bitrate;
use bitrate::*;
use cortex_m::asm::delay;
use embedded_hal::delay::DelayNs;

/// asm::delay based Timer
pub struct AsmDelay {
    freq_base_ns: u32,
}

impl AsmDelay {
    /// Consturct new delay timer of CPU frequency (Hertz)
    pub fn new<F>(freq: F) -> Self
    where
        F: Into<Hertz<u32>>,
    {
        let freq_hz = freq.into().0;
        AsmDelay {
            freq_base_ns: (freq_hz / 1_000_000_000) / 2,
        }
    }
}

impl DelayNs for AsmDelay
{
    fn delay_ns(&mut self, ns: u32) {
        delay(self.freq_base_ns * ns)
    }
}

/// Convert cycles back to time
pub struct CyclesToTime {
    freq_base_ms: f32,
    freq_base_us: f32,
    freq_base_ns: f32,
}

impl CyclesToTime {
    /// Consturct new cycles to time converter of CPU frequency (Hertz)
    pub fn new<F>(freq: F) -> Self
    where
        F: Into<Hertz<u32>>,
    {
        let freq_hz = freq.into().0;
        CyclesToTime {
            freq_base_ms: (freq_hz / 1_000) as f32,
            freq_base_us: (freq_hz / 1_000_000) as f32,
            freq_base_ns: (freq_hz / 1_000_000_00) as f32,
        }
    }

    /// Converts cycles to ms
    pub fn to_ms<U>(&self, cycles: U) -> f32
    where
        U: Into<u32>,
    {
        (cycles.into() as f32) / self.freq_base_ms
    }

    /// Converts cycles to us
    pub fn to_us<U>(&self, cycles: U) -> f32
    where
        U: Into<u32>,
    {
        (cycles.into() as f32) / self.freq_base_us
    }

    /// Converts cycles to ns
    pub fn to_ns<U>(&self, cycles: U) -> f32
    where
        U: Into<u32>,
    {
        (cycles.into() as f32) / self.freq_base_ns
    }
}
