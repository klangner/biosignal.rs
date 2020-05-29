use gnuplot::{Figure};
use dsp::signal::Signal;
use biosignal::io;
use biosignal::filter;


fn main() {
    let fname = "examples/data/ecg.mat";
    let signal = io::read_mat(fname, "ecg", 500).unwrap();
    plot(&signal, "Input signal", "input");
    // Filtering out the body movement
    let signal2 = filter::filter(&signal, 0.0, 0.5);
    plot(&signal2, "Filtered signal", "filtered1");
    // Remove 50 Hz noise from the power outlet
    let signal3 = filter::filter(&signal2, 45.0, 55.0);
    plot(&signal3, "Filtered signal", "filtered2");
    // Increase the signal-to-noise ratio
    // Find heart rate
    // Plot results
}


/// Save plot to the file in the output folder
fn plot(signal: &Signal, title: &str, fname: &str) {
    let idx: Vec<usize> = (0..signal.len()).collect();
    let mut fg = Figure::new();
    fg.set_title(title);
    fg.axes2d().lines(&idx, &signal.data, &[]);
    // fg.show().unwrap();
    let path = format!("output/{}.svg", fname);
    fg.save_to_svg(path, 900, 300).unwrap();
}