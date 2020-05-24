use std::fs::File;
use std::io;
use dsp::Signal;


/// read data from mat file
fn read_data(fname: &str, array_name: &str, sample_rate: usize) -> io::Result<Signal> {
    let file = File::open(fname)?;
    let mat_file = matfile::MatFile::parse(file).unwrap();
    let array = mat_file.find_by_name(array_name).unwrap();
    if let matfile::NumericData::Double { real, imag: None} = array.data() {
        let data: Vec<f32> = real.iter().map(|&v| v as f32).collect();
        Ok(Signal::new(data, sample_rate))
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "No data"))
    }
} 


fn main() {
    let fname = "examples/data/ecg.mat";
    let signal = read_data(fname, "ecg", 500).unwrap();
    println!("Length: {:?}", signal.len());
}
