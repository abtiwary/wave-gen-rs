use anyhow::Result;

pub struct SineWaveOscillator {
    frequency: f64,
    amplitude: f64,
    angle: f64,
    phase: f64,
    sample_rate: usize,
}

impl SineWaveOscillator {
    pub fn new(frequency: f64, amplitude: f64, phase: f64, sample_rate: usize) -> Self {
        let mut s = SineWaveOscillator { 
            frequency, 
            amplitude,
            angle: 0.0,
            phase,
            sample_rate,
        };

        let tau = std::f64::consts::PI * 2.0;
        s.angle = tau * (s.frequency / sample_rate as f64);
        s
    }
    
    pub fn tick(&self, increment: usize) -> f64 {
        self.amplitude * f64::sin(self.angle * increment as f64) + self.phase
    }
}


#[cfg(test)]
mod test {
    use super::SineWaveOscillator;

    #[test]
    fn test_happy_case() {
        let sample_rate = 4;
        let duration = 1;
        let mut so = SineWaveOscillator::new(std::f64::consts::PI, 1.0, 0.0, sample_rate);
        let mut out: Vec<f64> = Vec::with_capacity(sample_rate * duration);
        //let max_amplitude_for_bit_depth: usize = (2 << (16-1)) - 1;
        for i in 0..sample_rate * duration {
            let o = so.tick(i); //* max_amplitude_for_bit_depth as f64;
            out.push(o);
        }
        //println!("{:#?}", &out);
        assert_eq!(out.len(), 4);
    }

}
