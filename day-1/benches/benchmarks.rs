use day_1::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1_oneliner() {
    part1_oneliner::process(divan::black_box(include_str!("../input1.txt",))).unwrap();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!("../input1.txt",))).unwrap();
}

#[divan::bench]
fn part1_nom() {
    part1_nom::process(divan::black_box(include_str!("../input1.txt",))).unwrap();
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(include_str!("../input2.txt",))).unwrap();
}
