use anyhow::{anyhow, Result};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Instruction {
    Addx(i32),
    Noop,
}

pub fn parse_instruction(line: &str) -> Result<Instruction> {
    let components: Vec<_> = line.split(" ").collect();

    match components[0] {
        "addx" => Ok(Instruction::Addx(components[1].parse::<i32>()?)),
        "noop" => Ok(Instruction::Noop),
        _ => Err(anyhow!("failed to parse instruction {}", line)),
    }
}

pub struct Cpu {
    /// The current value of the X register
    pub register: i32,
    ///  The current *in progress* cycle.
    pub current_cycle: usize,
    program: Vec<Instruction>,
    program_idx: usize,
    cycle_instruction_started: usize,
}

impl Cpu {
    pub fn new(program: Vec<Instruction>) -> Self {
        Self {
            register: 1,
            current_cycle: 0,
            cycle_instruction_started: 0,
            program_idx: 0,
            program: program,
        }
    }

    fn get_current_instruction(&self) -> Option<Instruction> {
        self.program.get(self.program_idx).map(|i_ref| *i_ref)
    }

    /// Completes the current cycle and starts the next cycle. Completing a cycle means applying all instruction side effects if one completed in the current cycle.
    pub fn step(&mut self) -> Option<()> {
        let current_instruction = self.get_current_instruction();
        let done = match current_instruction {
            None => true,
            Some(Instruction::Noop) => true,
            Some(Instruction::Addx(_)) => {
                (self.current_cycle - self.cycle_instruction_started) == 1
            }
        };

        if done {
            match current_instruction {
                None => (),
                Some(Instruction::Noop) => (),
                Some(Instruction::Addx(val)) => self.register += val,
            }
        }

        self.current_cycle += 1;

        if done {
            self.program_idx += 1;
            self.cycle_instruction_started = self.current_cycle;
        }

        match self.program.get(self.program_idx) {
            None => None,
            Some(_) => Some(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_step() -> Result<()> {
        let program = vec![
            Instruction::Noop,
            Instruction::Addx(3),
            Instruction::Addx(-5),
        ];
        let mut cpu = Cpu::new(program);

        let expected = vec![
            (1, Some(Instruction::Noop)),
            (1, Some(Instruction::Addx(3))),
            (1, Some(Instruction::Addx(3))),
            (4, Some(Instruction::Addx(-5))),
            (4, Some(Instruction::Addx(-5))),
            (-1, None),
        ];

        for (expected_register, expected_instruction_maybe) in expected.iter() {
            assert_eq!(cpu.get_current_instruction(), *expected_instruction_maybe);
            assert_eq!(cpu.register, *expected_register);
            cpu.step();
        }

        let program = vec![Instruction::Addx(5)];
        let mut cpu = Cpu::new(program);
        assert!(cpu.step().is_some());
        assert_eq!(cpu.register, 1);
        assert!(cpu.step().is_none());
        assert_eq!(cpu.register, 6);

        let program = vec![Instruction::Noop];
        let mut cpu = Cpu::new(program);
        assert!(cpu.step().is_none());
        assert_eq!(cpu.register, 1);

        Ok(())
    }

    #[test]
    fn test_parse_instruction() -> Result<()> {
        assert_eq!(parse_instruction("noop")?, Instruction::Noop);
        assert_eq!(parse_instruction("addx -10")?, Instruction::Addx(-10));
        assert!(parse_instruction("nonsense -1234").is_err());

        Ok(())
    }
}
