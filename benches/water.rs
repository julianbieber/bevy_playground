use criterion::{criterion_group, criterion_main, Criterion};
use voxel::water_simulation::WaterSimulation;
use voxel::{voxel::Voxel, voxel::VoxelPosition, world_sector::WorldSector};

fn water_bench(c: &mut Criterion) {
    let mut sector = WorldSector::<32, 8>::new(VoxelPosition::new(0, 0, 0));

    for x in -50..50 {
        for y in -50..50 {
            for z in -50..50 {
                sector.insert(
                    VoxelPosition { x, y, z },
                    Voxel::WaterVoxel {
                        fill: 0.5
                    },
                );
            }
        }
    }

    c.bench_function("free fallig water cube", |b| {
        b.iter(|| {
            sector.flow_water();
        })
    });

    dbg!(sector.water_count());
}

criterion_group!(benches, water_bench);
criterion_main!(benches);
