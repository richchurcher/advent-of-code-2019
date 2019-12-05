#[aoc_generator(day2)]
pub fn converter(input: &str) -> Vec<i32> {
    input
        .split(",")
        .enumerate()
        .map(|(i, n)| match i {
            1 => 12,
            2 => 2,
            _ => n.parse::<i32>().unwrap(),
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn execute(program: &[i32]) -> i32 {
    let mut memory = program.to_vec();
    let mut ptr = program.iter();
    while let Some(opcode) = ptr.next() {
        if *opcode == 99 {
            break;
        }
        let lhs = ptr.next().unwrap();
        let rhs = ptr.next().unwrap();
        let store = ptr.next().unwrap();
        memory[*store as usize] = match *opcode {
            1 => memory[*lhs as usize] + memory[*rhs as usize],
            2 => memory[*lhs as usize] * memory[*rhs as usize],
            _ => 0,
        };
    }

    memory[0]
}

#[aoc(day2, part2)]
pub fn find_target_inputs(program: &[i32]) -> i32 {
    for noun in 0..99 {
        for verb in 0..99 {
            let memory = &mut program.to_vec();
            memory[1] = noun;
            memory[2] = verb;
            if execute(memory) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    -1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn execute_is_correct_for_simple_programs() {
        assert_eq!(execute(&vec![1, 1, 2, 0, 99]), 3)
    }

    #[test]
    fn execute_is_correct_for_longer_programs() {
        assert_eq!(execute(&vec![1, 1, 2, 0, 2, 2, 2, 0, 99]), 4)
    }
}
