use dsp::signal::Signal;
use dsp::spectrum::Spectrum;
use dsp::fft::{ForwardFFT, InverseFFT};
use dsp::num_complex::Complex32;


/// Filter signal by zeroing bins in Frequency domain
pub fn filter(signal: &Signal, start_freq: f32, end_freq: f32) -> Signal {
    let mut fft = ForwardFFT::new(signal.len());
    let mut ift = InverseFFT::new(signal.len());
    let spectrum = fft.process(&signal);
    let mut output = spectrum.data.to_vec();
    for i in 0..spectrum.len() {
        let f = spectrum.item_freq(i);
        if f >= start_freq && f <= end_freq {
            output[i] = Complex32::new(0.0, 0.0);
        }
    }
    let filtered_spectrum = Spectrum::new(output, spectrum.sample_rate);
    let output_signal = ift.process(&filtered_spectrum);
    output_signal
}