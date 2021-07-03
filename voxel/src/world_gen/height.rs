use noise::{Perlin, Seedable};

use super::noise_sampler::NoiseSampler;

#[derive(Clone)]
pub struct HeightGen {
    noises: Vec<NoiseSampler>,
}

impl HeightGen {
    pub fn new() -> HeightGen {
        HeightGen {
            noises: vec![
                NoiseSampler {
                    noise: Perlin::new().set_seed(123),
                    freqency: 500.0,
                    multiplier: 120.0,
                    squared: true,
                    offset: 0.0,
                },
                NoiseSampler {
                    noise: Perlin::new().set_seed(1235),
                    freqency: 500.0,
                    multiplier: 60.0,
                    squared: false,
                    offset: 0.0,
                },
                NoiseSampler {
                    noise: Perlin::new().set_seed(1234),
                    freqency: 30.0,
                    multiplier: 4.0,
                    squared: false,
                    offset: 0.0,
                },
            ],
        }
    }

    pub fn get_height_factor(&self, x: i32, z: i32) -> i32 {
        self.noises.iter().map(|s| s.sample(x, z)).sum::<f64>() as i32
    }
}
