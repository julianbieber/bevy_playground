use bevy::{prelude::*, render::renderer::RenderResources};

#[derive(RenderResources)]
pub struct CustomMaterial {
    pub time: f32,
}

impl CustomMaterial {
    pub fn add(&mut self) {
        self.time += 1.0f32;
    }
}

pub fn update_material_time(mut material: ResMut<Assets<CustomMaterial>>, handle: Mut<Handle<CustomMaterial>>) {
    for m in material.get_mut(&handle).iter_mut(){
        m.add();
    }

}