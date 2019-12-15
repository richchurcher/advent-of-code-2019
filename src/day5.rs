use crate::intcode::Memory;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
pub fn converter(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn air_conditioner_diagnostic(program: &[i32]) -> i32 {
    let mut memory = program.to_vec();
    memory.run(1).unwrap()
}

#[aoc(day5, part2)]
pub fn thermal_radiator_diagnostic(program: &[i32]) -> i32 {
    let mut memory = program.to_vec();
    memory.run(5).unwrap()
}

