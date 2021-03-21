use noise::{Perlin, Seedable};
use rand::prelude::SmallRng;

use crate::voxel_world::voxel::VoxelTypes;
use rand::seq::SliceRandom;

use super::noise_sampler::NoiseSampler;

#[derive(Clone, Debug)]
pub struct VoxelTypeDecision {
    type_boundaries: Vec<VoxelTypeBoundary>,
    temperature_sampler: NoiseSampler,
}

impl VoxelTypeDecision {
    pub fn default() -> VoxelTypeDecision {
        VoxelTypeDecision {
            type_boundaries: vec![
                VoxelTypeBoundary::moss(),
                VoxelTypeBoundary::dark_rock1(),
                VoxelTypeBoundary::grey_rock1(),
                VoxelTypeBoundary::grey_rock2(),
                VoxelTypeBoundary::brown_rock(),
                VoxelTypeBoundary::dark_rock2(),
                VoxelTypeBoundary::ground_rock1(),
                VoxelTypeBoundary::snow(),
            ],
            temperature_sampler: NoiseSampler {
                noise: Perlin::new().set_seed(123456),
                freqency: 10000.0,
                multiplier: 50.0,
                squared: false,
                offset: 10.0,
            },
        }
    }
}

impl VoxelTypeDecision {
    pub fn get_type(&self, rng: &mut SmallRng, x: i32, y: i32, z: i32, ground: bool) -> VoxelTypes {
        let mut temperature = self.temperature_sampler.sample(x, z);
        if y > -10 {
            temperature -= y as f64 / 1.5;
        }
        let valid: Vec<_> = self
            .type_boundaries
            .iter()
            .filter(|b| {
                y >= b.min_y
                    && y <= b.max_y
                    && ground == b.only_ground
                    && temperature >= b.min_termperature
                    && temperature <= b.max_temperature
            })
            .collect();
        valid.choose(rng).unwrap().typ.clone()
    }
}

#[derive(Clone, Debug)]
struct VoxelTypeBoundary {
    min_y: i32,
    max_y: i32,
    min_termperature: f64,
    max_temperature: f64,
    typ: VoxelTypes,
    only_ground: bool,
}

impl VoxelTypeBoundary {
    fn moss() -> VoxelTypeBoundary {
        VoxelTypeBoundary {
            min_y: 0,
            max_y: 50,
            typ: VoxelTypes::Moss,
            only_ground: true,
            min_termperature: -5.0,
            max_temperature: 30.0,
        }
    }

    fn dark_rock1() -> VoxelTypeBoundary {
        VoxelTypeBoundary {
            min_y: i32::MIN,
            max_y: 10,
            typ: VoxelTypes::DarkRock1,
            only_ground: false,
            min_termperature: f64::NEG_INFINITY,
            max_temperature: f64::INFINITY,
        }
    }

    fn grey_rock1() -> VoxelTypeBoundary {
        VoxelTypeBoundary {
            min_y: -10,
            max_y: i32::MAX,
            typ: VoxelTypes::GreyRock1,
            only_ground: false,
            min_termperature: f64::NEG_INFINITY,
            max_temperature: f64::INFINITY,
        }
    }

    fn grey_rock2() -> VoxelTypeBoundary {
        VoxelTypeBoundary {
            min_y: 0,
            max_y: i32::MAX,
            typ: VoxelTypes::GreyRock2,
            only_ground: false,
            min_termperature: f64::NEG_INFINITY,
            max_temperature: f64::INFINITY,
        }
    }

    fn brown_rock() -> VoxelTypeBoundary {
        VoxelTypeBoundary {
            min_y: -40,
            max_y: 40,
            typ: VoxelTypes::BrownRock,
            only_ground: false,
            min_termperature: f64::NEG_INFINITY,
            max_temperature: f64::INFINITY,
        }
    }

    fn dark_rock2() -> VoxelTypeBoundary {
        VoxelTypeBoundary {
            min_y: i32::MIN,
            max_y: 20,
            typ: VoxelTypes::DarkRock2,
            only_ground: false,
            min_termperature: f64::NEG_INFINITY,
            max_temperature: f64::INFINITY,
        }
    }

    fn ground_rock1() -> VoxelTypeBoundary {
        VoxelTypeBoundary {
            min_y: i32::MIN,
            max_y: i32::MAX,
            typ: VoxelTypes::GroundRock1,
            only_ground: true,
            min_termperature: f64::NEG_INFINITY,
            max_temperature: f64::INFINITY,
        }
    }

    fn snow() -> VoxelTypeBoundary {
        VoxelTypeBoundary {
            min_y: i32::MIN,
            max_y: i32::MAX,
            typ: VoxelTypes::Snow,
            only_ground: true,
            min_termperature: f64::NEG_INFINITY,
            max_temperature: 0.0,
        }
    }
}
