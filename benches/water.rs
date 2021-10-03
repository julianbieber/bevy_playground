use criterion::{criterion_group, criterion_main, Criterion};
use voxel::{
    voxel::Voxel,
    voxel::VoxelPosition,
    world_sector::{water_simulation::WaterSimulation, DefaultWorldSector},
};

fn water_bench(c: &mut Criterion) {
    let mut sector = DefaultWorldSector::new(VoxelPosition::new(0, 0, 0));

    for x in -50..50 {
        for y in -50..50 {
            for z in -50..50 {
                sector.insert(VoxelPosition { x, y, z }, Voxel::WaterVoxel { fill: 0.5 });
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

fn world_initializatzion(c: &mut Criterion) {
    let mut group = c.benchmark_group("sector init");
    group.sample_size(10);
    group.bench_function("sector init", |b| {
        b.iter(|| {
            let mut sector = DefaultWorldSector::new(VoxelPosition::new(0, 0, 0));
            sector.insert_terrain();
        });
    });
    group.finish();
}

criterion_group!(benches, water_bench, world_initializatzion);
criterion_main!(benches);
