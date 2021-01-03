use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug)]
pub struct NPC {
    pub behaviour: NPCBehaviours,
    pub velocity: f32,
}

#[derive(Debug)]
pub enum NPCBehaviours {
    FOLLOW,
    RANDOM,
    EXPLODE,
}

impl Distribution<NPCBehaviours> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> NPCBehaviours {
        match rng.gen_range(0, 2) {
            0 => NPCBehaviours::FOLLOW,
            _ => NPCBehaviours::RANDOM,
        }
    }
}
