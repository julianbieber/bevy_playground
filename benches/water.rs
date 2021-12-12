use criterion::{criterion_group, criterion_main, Criterion};
use voxel::world_sector::grid::{GridWorld, SimpleVec};
use voxel::world_sector::pillar::VoxelPillar;
use voxel::{voxel::Voxel, voxel::VoxelPosition};

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

fn neighbor_index_calculation(c: &mut Criterion) {
    c.bench_function("right/up", |b| {
        b.iter(|| {
            let mut x1 = 0;
            let mut x2 = 0;
            let mut x3 = 0;
            for _ in 0..(L1DIMENSION * L2DIMENSION * L3DIMENSION) - 1 {
                if let Some((new_x3, new_x2, new_x1)) = right_or_up(x3, x2, x1) {
                    x3 = new_x3;
                    x2 = new_x2;
                    x1 = new_x1;
                } else {
                    dbg!(x3, x2, x1);
                }
            }
        })
    });
}

fn neighbor_index_calculation_back(c: &mut Criterion) {
    c.bench_function("left/down", |b| {
        b.iter(|| {
            let mut x1 = L1DIMENSION - 1;
            let mut x2 = L2DIMENSION - 1;
            let mut x3 = L3DIMENSION - 1;

            for _ in 0..(L1DIMENSION * L2DIMENSION * L3DIMENSION) - 1 {
                if let Some((new_x3, new_x2, new_x1)) = left_or_down(x3, x2, x1) {
                    x3 = new_x3;
                    x2 = new_x2;
                    x1 = new_x1;
                } else {
                    dbg!(x3, x2, x1);
                }
            }
        })
    });
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
        b.iter(||  world.iterate_mut(1, 1, noop_mut) )
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

criterion_group!(
    benches,
    water_bench,
    world_initializatzion,
    neighbor_index_calculation,
    neighbor_index_calculation_back,
    iterate_through_world,
    iterate_through_world_best_case,
    iterate_though_world_mut
);
criterion_main!(benches);

use voxel::world_sector::consts::{L1DIMENSION, L2DIMENSION, L3DIMENSION};

pub fn left_or_down(x3: usize, x2: usize, x1: usize) -> Option<(usize, usize, usize)> {
    if x1 == 0 && x2 == 0 && x3 == 0 {
        return None;
    }
    let (right_x1, overflow_to_2) = {
        let next = x1 - 1;
        (
            next & (L1DIMENSION - 1),
            (next & (2 as usize).pow(usize::BITS - 1)) >> (usize::BITS - 1),
        )
    };

    let (right_x2, overflow_to_3) = {
        let next = x2 - overflow_to_2;
        (
            next & (L2DIMENSION - 1),
            (next & (2 as usize).pow(usize::BITS - 1)) >> (usize::BITS - 1),
        )
    };

    let right_x3 = x3 - overflow_to_3;
    Some((right_x3, right_x2, right_x1))
}

pub fn right_or_up(x3: usize, x2: usize, x1: usize) -> Option<(usize, usize, usize)> {
    if x1 == L1DIMENSION - 1 && x2 == L2DIMENSION - 1 && x3 == L3DIMENSION - 1 {
        return None;
    }

    let (right_x1, overflow_to_2) = {
        let next = x1 + 1;
        (next % L1DIMENSION, next / L1DIMENSION)
    };

    let (right_x2, overflow_to_3) = {
        let next = x2 + overflow_to_2;
        (next % L2DIMENSION, next / L2DIMENSION)
    };

    let right_x3 = x3 + overflow_to_3;
    Some((right_x3, right_x2, right_x1))
}
