mod height;
mod noise_sampler;
mod type_decision;

use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use flume::{unbounded, Receiver, Sender};
use rand::{prelude::SmallRng, SeedableRng};

use crate::{
    player::PlayerPosition,
    voxel_world::{
        access::VoxelAccess,
        boundaries::ChunkBoundaries,
        chunk::VoxelChunk,
        voxel::{Voxel, VoxelPosition},
    },
};

use self::{height::HeightGen, type_decision::VoxelTypeDecision};

use super::AdditionalVoxels;

pub struct GeneratedChunks {
    generated: Vec<ChunkBoundaries>,
}

const CHUNKS_IN_EACH_DIRECTION: i32 = 3;

pub struct GenerationResult {
    boundaries: ChunkBoundaries,
    chunk: VoxelChunk,
    mesh: Mesh,
}

pub fn setup_world_gen(mut commands: Commands) {
    let (sender, receiver) = unbounded::<GenerationResult>();
    commands.insert_resource(VoxelTypeDecision::default());
    commands.insert_resource(HeightGen::new());
    commands.insert_resource(GeneratedChunks {
        generated: Vec::new(),
    });
    commands.insert_resource(sender);
    commands.insert_resource(receiver);
}

pub fn start_generation(
    sender: Res<Sender<GenerationResult>>,
    pool: Res<AsyncComputeTaskPool>,
    mut generated_chunks: ResMut<GeneratedChunks>,
    player_position: Res<PlayerPosition>,
    additional_voxels: Res<AdditionalVoxels>,
    voxel_type_decision: Res<VoxelTypeDecision>,
    height_gen: Res<HeightGen>,
) {
    let player_chunk =
        ChunkBoundaries::aligned(VoxelPosition::from_vec3(&player_position.position));

    for x in -CHUNKS_IN_EACH_DIRECTION..CHUNKS_IN_EACH_DIRECTION + 1 {
        for y in -1..4 + 1 {
            for z in -CHUNKS_IN_EACH_DIRECTION..CHUNKS_IN_EACH_DIRECTION + 1 {
                let boundaries = player_chunk.in_direction([x, y, z]);
                if let None = generated_chunks
                    .generated
                    .iter()
                    .find(|c| **c == boundaries)
                {
                    let cloned_boundary = boundaries.clone();
                    generated_chunks.generated.push(boundaries);
                    let cloned_sender = sender.clone();
                    let additional = additional_voxels
                        .voxels
                        .get(&cloned_boundary)
                        .map(|c| c.get_voxels())
                        .unwrap_or(vec![]);
                    let cloned_voxel_type_decision = voxel_type_decision.clone();
                    let cloned_height_gen = height_gen.clone();
                    pool.spawn(async move {
                        generate_chunk(
                            cloned_boundary,
                            cloned_voxel_type_decision,
                            cloned_height_gen,
                            cloned_sender,
                            additional,
                        );
                    })
                    .detach()
                }
            }
        }
    }
}

fn generate_chunk(
    boundaries: ChunkBoundaries,
    voxel_type_decision: VoxelTypeDecision,
    height_gen: HeightGen,
    sender: Sender<GenerationResult>,
    additional: Vec<Voxel>,
) {
    let mut rng = SmallRng::from_entropy();
    let mut chunk = VoxelChunk::empty();
    for x_i in boundaries.min[0]..boundaries.max[0] {
        for z_i in boundaries.min[2]..boundaries.max[2] {
            let total_y = height_gen.get_height_factor(x_i, z_i);
            let max_y = boundaries.max[1].min(total_y);
            if boundaries.min[1] <= max_y {
                for y in boundaries.min[1]..max_y {
                    let p = VoxelPosition { x: x_i, y, z: z_i };
                    chunk.set(Voxel {
                        position: p,
                        typ: voxel_type_decision.get_type(&mut rng, x_i, y, z_i, y >= total_y - 1),
                    });
                }
            }
        }
    }
    for v in additional {
        chunk.set(v);
    }
    let mesh = Mesh::from(&chunk);

    let result = GenerationResult {
        boundaries,
        chunk,
        mesh,
    };
    let _ = sender.send(result);
}

pub fn read_generation_results(
    mut commands: Commands,
    receiver: Res<Receiver<GenerationResult>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut chunk_access: ResMut<VoxelAccess>,
) {
    for generation in receiver.try_iter() {
        let chunk_texture = asset_server.load("world_texture_color.png");
        let chunk_mesh = meshes.add(generation.mesh);

        let chunk_material = materials.add(StandardMaterial {
            albedo_texture: Some(chunk_texture),
            ..Default::default()
        });
        let chunk_bundle = PbrBundle {
            mesh: chunk_mesh,
            material: chunk_material,
            transform: Transform::from_translation(Vec3::ZERO),
            ..Default::default()
        };
        let chunk_entity = commands
            .spawn(chunk_bundle)
            .with(generation.chunk)
            .current_entity()
            .unwrap();
        chunk_access.add_chunk(generation.boundaries, chunk_entity);
    }
}
