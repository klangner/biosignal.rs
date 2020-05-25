//! # Biosignals
//!
//! A biosignal is any signal in living beings that can be continually measured and monitored.
//!

pub mod io;
pub mod error;


/// Signal consists of:
/// * data
/// * sample_rate which is number of samples per second (Sampling frequency)
pub struct Signal {
    pub data: Vec<f64>,
    pub sample_rate: usize
}

impl Signal {

    /// create new signal with a given length and sample rate
    pub fn new(data: Vec<f64>, sample_rate: usize) -> Signal {
        Signal { data, sample_rate }
    }

    /// Length of the signal buffer
    pub fn len(&self) -> usize {
        self.data.len()
    }

}