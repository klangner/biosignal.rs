use gnuplot::{Figure, Color};
use biosignal::io;
use dsp::Signal;


fn main() {
    let fname = "examples/data/ecg.mat";
    let signal = io::read_mat(fname, "ecg", 500).unwrap();
    // Plot the input signal
    plot(&signal);
    // Filtering out the body movement
    // Remove 50 Hz noise from the power outlet
    // Increase the signal-to-noise ratio
    // Find heart rate
}


/// Plot signal
fn plot(signal: &Signal) {
    let idx: Vec<usize> = (0..signal.len()).collect();
    let mut fg = Figure::new();
    fg.axes2d().lines(&idx, &signal.data, &[Color("red")]);
    let _ = fg.show();
}