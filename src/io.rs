use std::fs::File;
use matfile;
use dsp::signal::Signal;
use crate::error::{Error, ErrorKind, Result};


/// Read data from mat file
pub fn read_mat(fname: &str, array_name: &str, sample_rate: usize) -> Result<Signal> {
    let file = File::open(fname)?;
    let mat_file = matfile::MatFile::parse(file).unwrap();
    let array = mat_file.find_by_name(array_name).unwrap();
    if let matfile::NumericData::Double { real, imag: None} = array.data() {
        let data = real.iter().map(|&v| v as f32).collect();
        Ok(Signal::new(data, sample_rate))
    } else {
        Err(Error::new(ErrorKind::NoData(array_name.to_owned())))
    }
} 