use crate::intcode::Memory;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn converter(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn run_program(program: &[usize]) -> usize {
    let mut memory = program.to_vec();
    memory[1] = 12;
    memory[2] = 2;
    program.to_vec().run().unwrap()
}

#[aoc(day2, part2)]
pub fn find_target_inputs(program: &[usize]) -> usize {
    for noun in 0..99 {
        for verb in 0..99 {
            let memory = &mut program.to_vec();
            memory[1] = noun;
            memory[2] = verb;
            if memory.run().unwrap() == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    unreachable!()
}

