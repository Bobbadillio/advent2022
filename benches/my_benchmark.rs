use criterion::{black_box, criterion_group, criterion_main, Criterion};
use solutions::*;

pub fn day_01_01_benchmark(c: &mut Criterion) {
    let puzzle = include_str!("../AOCDay01.txt");
    c.bench_function("day 01 pt 1", |b| b.iter(|| {
        aocday01::solve_part_1(black_box(puzzle));
    }));
}

pub fn day_01_02_benchmark(c: &mut Criterion) {
    let puzzle = include_str!("../AOCDay01.txt");
    c.bench_function("day 01 pt 2", |b| b.iter(|| {
        aocday01::solve_part_2(black_box(puzzle))
    }));
}

pub fn day_01_both_benchmark(c: &mut Criterion) {
    let puzzle = include_str!("../AOCDay01.txt");
    c.bench_function("day 01 both", |b| b.iter(|| {
        aocday01::solve_both(black_box(puzzle))
    }));
}

pub fn day_02_01_benchmark(c: &mut Criterion) {
    let puzzle = include_str!("../AOCDay02.txt");

    c.bench_function("day 02 pt 1", |b| b.iter(|| {
        aocday02::solve_part_1(black_box(puzzle));
    }));
}

pub fn day_02_02_benchmark(c: &mut Criterion) {
    let puzzle = include_str!("../AOCDay02.txt");

    c.bench_function("day 02 pt 2", |b| b.iter(|| {
        aocday02::solve_part_2(black_box(puzzle));
    }));
}

pub fn day_03_01_benchmark(c: &mut Criterion) {
    let puzzle = include_str!("../AOCDay03.txt");

    c.bench_function("day 03 pt 1", |b| b.iter(|| {
        aocday03::solve_part_1(black_box(puzzle));
    }));
}

pub fn day_03_02_benchmark(c: &mut Criterion) {
    let puzzle = include_str!("../AOCDay03.txt");

    c.bench_function("day 03 pt 2", |b| b.iter(|| {
        aocday03::solve_part_2(black_box(puzzle));
    }));
}

pub fn day_04_01_benchmark(c: &mut Criterion) {

    let puzzle = include_str!("../AOCDay04.txt");
    c.bench_function("day 04 pt 1", |b| b.iter(|| aocday04::fold_lines_to_containments(black_box(puzzle))));
}
pub fn day_04_02_benchmark(c: &mut Criterion) {

    let puzzle = include_str!("../AOCDay04.txt");
    c.bench_function("day 04 pt 2", |b| b.iter(|| aocday04::fold_lines_to_overlaps(black_box(puzzle))));
}

pub fn day_05_parse_benchmark(c: &mut Criterion) {
    let mut puzzle = include_str!("../AOCDay05.txt");
    c.bench_function("day 05 parsing", |b| b.iter(|| aocday05::parse_fully(black_box(puzzle))));
}
pub fn day_05_01_benchmark(c: &mut Criterion) {
    let mut puzzle = include_str!("../AOCDay05.txt");
    if let Ok((leftover, (mut stack, orders ))) = aocday05::parse_fully(black_box(puzzle)){
        c.bench_function("day 05 pt 1", |b| b.iter(|| aocday05::solve_pt1(black_box(&mut stack), black_box(&orders))));
    }
}
pub fn day_05_02_benchmark(c: &mut Criterion) {

    let puzzle = include_str!("../AOCDay05.txt");
    if let Ok((leftover, (mut stack, orders ))) = aocday05::parse_fully(black_box(puzzle)) {
        c.bench_function("day 05 pt 2", |b| b.iter(|| aocday05::solve_pt2(black_box(&mut stack), black_box(&orders))));
    }
}
pub fn day_06_01_benchmark(c: &mut Criterion) {
    let puzzle = include_str!("../AOCDay06.txt");
    c.bench_function("day 06 pt 1", |b| b.iter(|| aocday06::solve_pt1(black_box(puzzle.as_bytes()))));

}
pub fn day_06_02_benchmark(c: &mut Criterion) {
    let puzzle = include_str!("../AOCDay06.txt");
    c.bench_function("day 06 pt 2", |b| b.iter(|| aocday06::solve_pt2(black_box(puzzle.as_bytes()))));
}
pub fn day_06_both_benchmark(c: &mut Criterion) {
    let puzzle = include_str!("../AOCDay06.txt");
    c.bench_function("day 06 both", |b| b.iter(|| aocday06::solve_both(black_box(puzzle.as_bytes()))));
}
pub fn day_10_both_benchmark(c: &mut Criterion) {
    let puzzle = include_str!("../AOCDay10.txt");
    c.bench_function("day 10", |b| b.iter(|| aocday10::solve_both(black_box(puzzle))));
}


criterion_group!(benches,
    // day_01_01_benchmark, day_01_02_benchmark, day_01_both_benchmark,
    // day_02_01_benchmark, day_02_02_benchmark,
    // day_03_01_benchmark, day_03_02_benchmark,
    // day_04_01_benchmark, day_04_02_benchmark,
    // day_05_01_benchmark, day_05_02_benchmark,day_05_parse_benchmark,
    // day_06_01_benchmark, day_06_02_benchmark, day_06_both_benchmark
    day_10_both_benchmark
);
criterion_main!(benches);