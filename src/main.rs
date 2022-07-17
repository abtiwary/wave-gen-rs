use std::fs::File;
use std::io::prelude::*;

use anyhow::Result;

use wave_gen_rs::{oscillator, wave};

fn main() -> Result<()> {
    let frequency: f64 = 440.0;
    let amplitude: f64 = 0.5;
    let phase: f64 = 0.0;
    let sample_rate: usize = 44100;
    let bit_depth: u16 = 8;
    let max_amplitude_for_bit_depth: u16 = (2 << (bit_depth - 1)) - 1;
    let duration: usize = 2;

    let new_oscillator = oscillator::SineWaveOscillator::new(
        frequency, 
        amplitude, 
        phase, 
        sample_rate,
    );

    let mut samples: Vec<u8> = vec![0; sample_rate * duration];
    for i in 0..(sample_rate*duration) {
        samples[i] = (&new_oscillator.tick(i) * (max_amplitude_for_bit_depth as f64)) as u8;
    }

    let mut file = File::create("/Users/abtiwary/temp/test.wav")
        .expect("fatal error: expected to create a file");
    let wave_header = wave::generate_wave_header(samples.len(), sample_rate as u32);
    file.write_all(&wave_header).expect("fatal error: expected to write wave header to file");;
    file.write_all(&samples).expect("fatal error: expected to write samples to file");

    Ok(())
}
