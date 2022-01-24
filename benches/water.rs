use criterion::{criterion_group, criterion_main, Criterion};
use voxel::world_sector::grid::{GridWorld, SimpleVec};
use voxel::world_sector::pillar::{VoxelDescription, VoxelPillar};
use voxel::{voxel::VoxelPosition, voxel::VoxelRange};

fn water_bench(c: &mut Criterion) {
    /*
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
    */
}

fn world_initializatzion(c: &mut Criterion) {
    /*let mut group = c.benchmark_group("sector init");
    group.sample_size(10);
    group.bench_function("sector init", |b| {
        b.iter(|| {
            let mut sector = DefaultWorldSector::new(VoxelPosition::new(0, 0, 0));
            sector.insert_terrain();
        });
    });
    group.finish();
    */
}

fn iterate_through_world(c: &mut Criterion) {
    println!("start");
    let world = GridWorld::empty([0, 0]);
    println!("end");
    c.bench_function("iterate through world", |b| {
        b.iter(|| {
            world.iterate(1, 1, |c, _, _, _, _| {});
        })
    });
}

fn iterate_through_world_best_case(c: &mut Criterion) {
    let world = SimpleVec::empty();
    c.bench_function("iterate through world best case", |b| {
        b.iter(|| world.iterate(noop))
    });
}

fn iterate_though_world_mut(c: &mut Criterion) {
    let mut world = GridWorld::empty([0, 0]);

    c.bench_function("iterate though world mut", |b| {
        b.iter(|| world.iterate_mut(1, 1, noop_mut))
    });
}

fn noop(
    _: &VoxelPillar,
    _: Option<&VoxelPillar>,
    _: Option<&VoxelPillar>,
    _: Option<&VoxelPillar>,
    _: Option<&VoxelPillar>,
) {
}

fn noop_mut(
    _: &mut VoxelPillar,
    _: Option<&mut VoxelPillar>,
    _: Option<&mut VoxelPillar>,
    _: Option<&mut VoxelPillar>,
    _: Option<&mut VoxelPillar>,
) {
}

fn combine_water(c: &mut Criterion) {
    c.bench_function("combine_water", |b| {
        b.iter(|| {
            let mut pillar = VoxelPillar {
                voxel_heights: (0..4)
                    .into_iter()
                    .map(|i| VoxelDescription::water(i, 0.1))
                    .collect(),
            };

            pillar.merge();
        })
    });
}

criterion_group!(
    benches,
    water_bench,
    world_initializatzion,
    iterate_through_world,
    iterate_through_world_best_case,
    iterate_though_world_mut,
    combine_water
);
criterion_main!(benches);
