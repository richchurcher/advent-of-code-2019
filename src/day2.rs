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

pub fn set_inputs(memory: &mut [i32], noun: i32, verb: i32) -> () {
    memory[1] = noun;
    memory[2] = verb;
}

#[aoc(day2, part2)]
pub fn find_target_inputs(program: &[i32]) -> i32 {
    let mut result = 0;
    let mut noun = 0;
    let mut verb = 0;

    for n in 0..99 {
        for v in 0..99 {
            let memory = &mut program.to_vec();
            set_inputs(memory, n, v);
            result = execute(memory);
            if result == 19690720 {
                noun = n;
                verb = v;
                break;
            }
        }

        if result == 19690720 {
            break;
        }
    }

    100 * noun + verb
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

    #[test]
    fn set_inputs_modifies_indices_1_and_2() {
        let memory = &mut vec![1, 2, 3, 4];
        set_inputs(memory, 20, 30);
        assert_eq!(memory, &mut vec![1, 20, 30, 4])
    }
}
