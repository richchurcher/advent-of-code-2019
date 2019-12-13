use failure::Fail;

// we have:
// - memory
// - instruction
//   - opcode
//   - noun
//   - verb
//   - address

pub trait Memory {
    // reset
    // run
    // execute
    fn execute(&mut self, instruction: Instruction) -> Result<usize, IntCodeError>;
    fn run(&mut self) -> Result<usize, IntCodeError>;
}

#[derive(Debug, Fail)]
pub enum IntCodeError {
    #[fail(display = "Instruction has an unknown opcode: {}", opcode)]
    UnknownOpcodeError { opcode: usize },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instruction {
    pub opcode: usize,
    pub noun: usize,
    pub verb: usize,
    pub address: usize,
}

impl Memory for Vec<usize> {
    fn execute(&mut self, instruction: Instruction) -> Result<usize, IntCodeError> {
        self[instruction.address] = match instruction.opcode {
            1 => self[instruction.noun] + self[instruction.verb],
            2 => self[instruction.noun] * self[instruction.verb],
            _ => {
                return Err(IntCodeError::UnknownOpcodeError {
                    opcode: instruction.opcode,
                })
            }
        };

        Ok(self[instruction.address])
    }

    fn run(&mut self) -> Result<usize, IntCodeError> {
        let program = self.to_vec();
        let mut ptr = program.iter().cloned();
        while let Some(opcode) = ptr.next() {
            if opcode == 99 {
                break;
            }

            let result = self.execute(Instruction {
                opcode: opcode,
                noun: ptr.next().unwrap(),
                verb: ptr.next().unwrap(),
                address: ptr.next().unwrap(),
            });

            if result.is_err() {
                panic!("All is not well in intcode land.");
            }
        }

        Ok(self[0])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_is_correct_for_simple_programs() {
        assert_eq!(vec![1, 1, 2, 0, 99].run().unwrap(), 3)
    }

    #[test]
    fn run_is_correct_for_longer_programs() {
        assert_eq!(vec![1, 1, 2, 0, 2, 2, 2, 0, 99].run().unwrap(), 4)
    }
}
