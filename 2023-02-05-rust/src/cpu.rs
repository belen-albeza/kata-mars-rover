use crate::commands::{Command, ForwardCommand, Movable, NoOpCommand, Opcode};

#[derive(Debug)]
pub struct Cpu {
    // commands: Vec<Box<dyn Command + 'a>>,
    program: Vec<Opcode>,
}

impl Cpu {
    pub fn parse(program: &str) -> Result<Vec<Opcode>, String> {
        let opcodes: Result<Vec<_>, _> = program.chars().map(|x| Opcode::try_from(x)).collect();
        opcodes
    }

    pub fn new(program: &[Opcode]) -> Self {
        Self {
            program: program.to_owned(),
        }
    }

    pub fn run(&self, target: &mut impl Movable) -> Result<(), String> {
        for opcode in &self.program {
            Self::step(*opcode, target)?
        }
        Ok(())
    }

    fn step(opcode: Opcode, target: &mut impl Movable) -> Result<(), String> {
        let mut command = Self::parse_opcode(opcode, target);
        command.execute()
    }

    fn parse_opcode(opcode: Opcode, target: &mut impl Movable) -> Box<dyn Command + '_> {
        match opcode {
            Opcode::NoOp => Box::new(NoOpCommand::new()),
            Opcode::Forward => Box::new(ForwardCommand::new(target)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::*;

    mock! {
        #[derive(Debug)]
        pub Vehicle {}
        impl Movable for Vehicle {
            fn advance(&mut self, dir: i32);
        }
    }

    #[test]
    pub fn test_parse_ok() {
        let result = Cpu::parse(" f");
        assert_eq!(result, Ok(vec![Opcode::NoOp, Opcode::Forward]));
    }

    #[test]
    pub fn test_parse_error() {
        let result = Cpu::parse("*");
        assert!(result.is_err());
    }

    #[test]
    pub fn test_new() {
        let program = vec![Opcode::Forward, Opcode::NoOp];
        let cpu = Cpu::new(&program);

        assert_eq!(cpu.program, program);
    }

    #[test]
    pub fn test_run_returns_ok_when_no_issues() {
        let program = vec![Opcode::Forward, Opcode::NoOp];
        let mut vehicle = MockVehicle::new();
        let cpu = Cpu::new(&program);

        vehicle
            .expect_advance()
            .with(predicate::eq(1))
            .return_const(())
            .times(1);

        let result = cpu.run(&mut vehicle);

        assert!(result.is_ok());
    }
}
