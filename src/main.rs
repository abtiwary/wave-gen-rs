use std::fs::File;
use std::io::prelude::*;

use clap::{Arg, Command};
use anyhow::Result;

use wave_gen_rs::{oscillator, wave};

fn main() -> Result<()> {
    let matches = Command::new("Simple WAVe Generator")
        .version("0.1")
        .author("Ab Tiwary")
        .about("A simple wave generator in Rust")
        .arg(Arg::new("frequency"))
            .long_flag("frequency")
        .arg(Arg::new("amplitude"))
            .long_flag("amplitude")
        .arg(Arg::new("phase"))
            .long_flag("phase")
        .arg(Arg::new("sample_rate"))
            .long_flag("sample_rate")
        .arg(Arg::new("bit_depth"))
            .long_flag("bit_depth")
        .arg(Arg::new("duration"))
            .long_flag("duration")
        .arg(Arg::new("outfile"))
            .long_flag("outfile")
        .get_matches();

    // example values:
    // frequency: 440.0
    // amplitude: 0.5
    // phase: 0.0
    // sample_rate: 44100
    // bit_depth: 8

    let frequency: f64 = matches.value_of("frequency").unwrap().parse::<f64>().unwrap();
    let amplitude: f64 = matches.value_of("amplitude").unwrap().parse::<f64>().unwrap();
    let phase: f64 = matches.value_of("phase").unwrap().parse::<f64>().unwrap();
    let sample_rate: usize = matches.value_of("sample_rate").unwrap().parse::<usize>().unwrap();
    let bit_depth: u8 = matches.value_of("bit_depth").unwrap().parse::<u8>().unwrap();
    let max_amplitude_for_bit_depth: u8 = (2 << (bit_depth - 1)) - 1;
    let duration: usize = matches.value_of("duration").unwrap().parse::<usize>().unwrap();
    
    let outfile: String = matches.value_of("outfile").unwrap().parse::<String>().unwrap();

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

    let mut file = File::create(&outfile)
        .expect("fatal error: expected to create a file");
    let wave_header = wave::generate_wave_header(samples.len(), sample_rate as u32);
    file.write_all(&wave_header).expect("fatal error: expected to write wave header to file");;
    file.write_all(&samples).expect("fatal error: expected to write samples to file");

    Ok(())
}

// example usage: ./wave-gen-rs 440.0 0.5 0.0 44100 8 4 /Users/abtiwary/temp/test4.wav

