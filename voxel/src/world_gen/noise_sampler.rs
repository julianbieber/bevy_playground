use noise::{NoiseFn, Perlin};

#[derive(Clone, Debug)]
pub struct NoiseSampler {
    pub noise: Perlin,
    pub freqency: f64,
    pub multiplier: f64,
    pub squared: bool,
    pub offset: f64,
}

impl NoiseSampler {
    pub fn sample(&self, x: i32, z: i32) -> f64 {
        let v = self
            .noise
            .get([x as f64 / self.freqency, z as f64 / self.freqency]);
        if self.squared {
            ((v + 0.2).powi(5)) * self.multiplier + self.offset
        } else {
            v * self.multiplier + self.offset
        }
    }
}
