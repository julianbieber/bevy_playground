use bevy::prelude::*;

pub struct DelayedDespawnsPlugin;

impl Plugin for DelayedDespawnsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<DelayedDespawns>()
            .add_system(evaluate_delayed_despanws.system());
    }
}

#[derive(Default)]
pub struct DelayedDespawns {
    pub despawns: Vec<(Timer, Entity)>,
}

fn evaluate_delayed_despanws(
    mut despanws_res: ResMut<DelayedDespawns>,
    commands: &mut Commands,
    time: Res<Time>,
) {
    let mut at_least_one = false;
    for (timer, entity) in despanws_res.despawns.iter_mut() {
        if timer.tick(time.delta_seconds()).just_finished() {
            commands.despawn(entity.clone());
            at_least_one = true;
        }
    }

    if at_least_one {
        let remaining: Vec<(Timer, Entity)> = despanws_res
            .despawns
            .iter()
            .filter(|(t, _)| !t.just_finished())
            .map(|(t, e)| (t.clone(), e.clone()))
            .collect();

        despanws_res.despawns = remaining;
    }
}
