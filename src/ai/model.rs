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
