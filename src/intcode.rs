use failure::Fail;

pub trait Memory {
    fn add(&mut self, instruction: FullInstruction);
    fn get_full_instruction(
        &self,
        modes: (ParameterMode, ParameterMode, ParameterMode),
        ptr: &mut usize,
    ) -> Result<FullInstruction, IntCodeError>;
    fn get_simple_instruction(
        &self,
        modes: (ParameterMode, ParameterMode, ParameterMode),
        ptr: &mut usize,
    ) -> Result<SimpleInstruction, IntCodeError>;
    fn get_complex_instruction(
        &self,
        modes: (ParameterMode, ParameterMode, ParameterMode),
        ptr: &mut usize,
    ) -> Result<ComplexInstruction, IntCodeError>;
    fn get_parameter(&self, parameter: i32, mode: ParameterMode) -> i32;
    fn jump_true(&self, instruction: ComplexInstruction, ptr: &mut usize);
    fn jump_false(&self, instruction: ComplexInstruction, ptr: &mut usize);
    fn lt(&mut self, instruction: FullInstruction);
    fn eq(&mut self, instruction: FullInstruction);
    fn mul(&mut self, instruction: FullInstruction);
    fn put(&mut self, instruction: SimpleInstruction, input: i32);
    fn out(&self, instruction: SimpleInstruction);
    fn run(&mut self, input: i32) -> Result<i32, IntCodeError>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParameterMode {
    Position,
    Immediate,
}

pub enum InstructionLength {
    Full = 4,
    Complex = 3,
    Simple = 2,
}

impl ParameterMode {
    fn from_i32(val: i32) -> Result<ParameterMode, IntCodeError> {
        match val {
            0 => Ok(ParameterMode::Position),
            1 => Ok(ParameterMode::Immediate),
            _ => Err(IntCodeError::UnrecognisedParameterMode { val: val }),
        }
    }
}

#[derive(Debug, Fail)]
pub enum IntCodeError {
    #[fail(display = "Unexpected end of input")]
    UnexpectedEndOfInputError {},

    #[fail(display = "Instruction has an unknown opcode: {}", opcode)]
    UnknownOpcodeError { opcode: i32 },

    #[fail(display = "Not a recognised ParameterMode: {}", val)]
    UnrecognisedParameterMode { val: i32 },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FullInstruction {
    pub address: i32,
    pub modes: (ParameterMode, ParameterMode, ParameterMode),
    pub noun: i32,
    pub verb: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComplexInstruction {
    pub address: i32,
    pub noun: i32,
    pub modes: (ParameterMode, ParameterMode, ParameterMode),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimpleInstruction {
    pub address: i32,
    pub modes: (ParameterMode, ParameterMode, ParameterMode),
}

fn get_modes(head: i32) -> Result<(ParameterMode, ParameterMode, ParameterMode), IntCodeError> {
    let first = ParameterMode::from_i32((head % 1_000) / 100)?;
    let second = ParameterMode::from_i32((head % 10_000) / 1_000)?;
    let third = ParameterMode::from_i32((head % 100_000) / 10_000)?;

    Ok((first, second, third))
}

impl Memory for Vec<i32> {
    fn add(&mut self, instruction: FullInstruction) {
        let (noun_mode, verb_mode, _) = instruction.modes;
        let noun = self.get_parameter(instruction.noun, noun_mode);
        let verb = self.get_parameter(instruction.verb, verb_mode);
        self[instruction.address as usize] = noun + verb;
    }

    fn eq(&mut self, instruction: FullInstruction) {
        let (noun_mode, verb_mode, _) = instruction.modes;
        let noun = self.get_parameter(instruction.noun, noun_mode);
        let verb = self.get_parameter(instruction.verb, verb_mode);
        if noun == verb {
            self[instruction.address as usize] = 1;
        } else {
            self[instruction.address as usize] = 0;
        }
    }

    fn get_full_instruction(
        &self,
        modes: (ParameterMode, ParameterMode, ParameterMode),
        ptr: &mut usize,
    ) -> Result<FullInstruction, IntCodeError> {
        if (*ptr + 3) >= self.len() {
            return Err(IntCodeError::UnexpectedEndOfInputError {});
        }

        let noun = self[*ptr + 1];
        let verb = self[*ptr + 2];
        let address = self[*ptr + 3];
        *ptr += InstructionLength::Full as usize;

        Ok(FullInstruction {
            address,
            modes,
            noun,
            verb,
        })
    }

    fn get_complex_instruction(
        &self,
        modes: (ParameterMode, ParameterMode, ParameterMode),
        ptr: &mut usize,
    ) -> Result<ComplexInstruction, IntCodeError> {
        if (*ptr + 2) >= self.len() {
            return Err(IntCodeError::UnexpectedEndOfInputError {});
        }

        let noun = self[*ptr + 1];
        let address = self[*ptr + 2];
        *ptr += InstructionLength::Complex as usize;

        Ok(ComplexInstruction {
            address,
            modes,
            noun,
        })
    }

    fn get_simple_instruction(
        &self,
        modes: (ParameterMode, ParameterMode, ParameterMode),
        ptr: &mut usize,
    ) -> Result<SimpleInstruction, IntCodeError> {
        if (*ptr + 1) >= self.len() {
            return Err(IntCodeError::UnexpectedEndOfInputError {});
        }

        let address = self[*ptr + 1];
        *ptr += InstructionLength::Simple as usize;

        Ok(SimpleInstruction { address, modes })
    }

    fn get_parameter(&self, parameter: i32, mode: ParameterMode) -> i32 {
        match mode {
            ParameterMode::Position => self[parameter as usize],
            ParameterMode::Immediate => parameter,
        }
    }

    fn jump_true(&self, instruction: ComplexInstruction, ptr: &mut usize) {
        if self.get_parameter(instruction.noun, instruction.modes.0) != 0 {
            *ptr = self.get_parameter(instruction.address, instruction.modes.1) as usize;
        }
    }

    fn jump_false(&self, instruction: ComplexInstruction, ptr: &mut usize) {
        if self.get_parameter(instruction.noun, instruction.modes.0) == 0 {
            *ptr = self.get_parameter(instruction.address, instruction.modes.1) as usize;
        }
    }

    fn lt(&mut self, instruction: FullInstruction) {
        let (noun_mode, verb_mode, _) = instruction.modes;
        let noun = self.get_parameter(instruction.noun, noun_mode);
        let verb = self.get_parameter(instruction.verb, verb_mode);
        if noun < verb {
            self[instruction.address as usize] = 1;
        } else {
            self[instruction.address as usize] = 0;
        }
    }

    fn mul(&mut self, instruction: FullInstruction) {
        let (noun_mode, verb_mode, _) = instruction.modes;
        let noun = self.get_parameter(instruction.noun, noun_mode);
        let verb = self.get_parameter(instruction.verb, verb_mode);
        self[instruction.address as usize] = noun * verb;
    }

    fn out(&self, instruction: SimpleInstruction) {
        println!(
            "{}",
            self.get_parameter(instruction.address, instruction.modes.0)
        );
    }

    fn put(&mut self, instruction: SimpleInstruction, input: i32) {
        self[instruction.address as usize] = input;
    }

    fn run(&mut self, input: i32) -> Result<i32, IntCodeError> {
        let mut ptr = 0;
        while ptr < self.len() {
            let opcode = self[ptr] % 100;
            let modes = get_modes(self[ptr])?;

            match opcode {
                1 => self.add(self.get_full_instruction(modes, &mut ptr)?),
                2 => self.mul(self.get_full_instruction(modes, &mut ptr)?),
                3 => self.put(self.get_simple_instruction(modes, &mut ptr)?, input),
                4 => self.out(self.get_simple_instruction(modes, &mut ptr)?),
                5 => self.jump_true(self.get_complex_instruction(modes, &mut ptr)?, &mut ptr),
                6 => self.jump_false(self.get_complex_instruction(modes, &mut ptr)?, &mut ptr),
                7 => self.lt(self.get_full_instruction(modes, &mut ptr)?),
                8 => self.eq(self.get_full_instruction(modes, &mut ptr)?),
                99 => break,
                _ => return Err(IntCodeError::UnknownOpcodeError { opcode }),
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
        assert_eq!(vec![1, 1, 2, 0, 99].run(0).unwrap(), 3)
    }

    #[test]
    fn run_is_correct_for_longer_programs() {
        assert_eq!(vec![1, 1, 2, 0, 2, 2, 2, 0, 99].run(0).unwrap(), 4)
    }

    #[test]
    fn get_modes_11101() {
        assert_eq!(
            get_modes(11101).unwrap(),
            (
                ParameterMode::Immediate,
                ParameterMode::Immediate,
                ParameterMode::Immediate,
            )
        )
    }

    #[test]
    fn get_modes_2() {
        assert_eq!(
            get_modes(2).unwrap(),
            (
                ParameterMode::Position,
                ParameterMode::Position,
                ParameterMode::Position,
            )
        )
    }

    #[test]
    fn get_modes_10104() {
        assert_eq!(
            get_modes(10104).unwrap(),
            (
                ParameterMode::Immediate,
                ParameterMode::Position,
                ParameterMode::Immediate,
            )
        )
    }

    #[test]
    fn get_modes_1002() {
        assert_eq!(
            get_modes(1002).unwrap(),
            (
                ParameterMode::Position,
                ParameterMode::Immediate,
                ParameterMode::Position,
            )
        )
    }
}
