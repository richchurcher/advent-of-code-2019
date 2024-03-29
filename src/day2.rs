use crate::intcode::Memory;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn converter(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn run_program(program: &[i32]) -> i32 {
    let mut memory = program.to_vec();
    memory[1] = 12;
    memory[2] = 2;
    memory.run(0).unwrap()
}

#[aoc(day2, part2)]
pub fn find_target_inputs(program: &[i32]) -> i32 {
    for noun in 0..99 {
        for verb in 0..99 {
            let memory = &mut program.to_vec();
            memory[1] = noun;
            memory[2] = verb;
            if memory.run(0).unwrap() == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    unreachable!()
}

