use bevy::prelude::*;

use crate::pickups::Energy;

pub struct DelayedUnitEffectsPlugin;

impl Plugin for DelayedUnitEffectsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<DelayedEffects>()
            .add_system(evaluate_delayed_effects.system());
    }
}

#[derive(Default)]
pub struct DelayedEffects {
    pub effects: Vec<(Timer, Effect)>,
}

#[derive(Clone)]
pub struct Effect {
    pub range: f32,
    pub center: Vec3,
    pub typ: Effects,
}

#[derive(Clone)]
pub enum Effects {
    Damage { amount: f32 },
}

fn evaluate_delayed_effects(
    mut effects_res: ResMut<DelayedEffects>,
    time: Res<Time>,
    mut units_query: Query<(&Transform, &mut Energy)>,
) {
    let mut at_least_one = false;
    for (timer, effect) in effects_res.effects.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            at_least_one = true;
            for (unit_transform, mut energy) in units_query.iter_mut() {
                if unit_transform.translation.distance_squared(effect.center)
                    < effect.range * effect.range
                {
                    match effect.typ {
                        Effects::Damage { amount } => {
                            energy.amount -= amount;
                            if energy.amount < 0.0 {
                                energy.amount = 0.0;
                            }
                        }
                    }
                }
            }
        }
    }

    if at_least_one {
        let remaining: Vec<(Timer, Effect)> = effects_res
            .effects
            .iter()
            .filter(|(t, _)| !t.just_finished())
            .map(|(t, e)| (t.clone(), e.clone()))
            .collect();

        effects_res.effects = remaining;
    }
}
