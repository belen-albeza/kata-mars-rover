use crate::commands::{Command, ForwardCommand, Movable, NoOpCommand, Opcode};

#[derive(Debug)]
pub struct Cpu<'a> {
    commands: Vec<Box<dyn Command + 'a>>,
}

impl<'a> Cpu<'a> {
    pub fn parse(program: &str) -> Result<Vec<Opcode>, String> {
        let opcodes: Result<Vec<_>, _> = program.chars().map(|x| Opcode::try_from(x)).collect();
        opcodes
    }

    pub fn new(program: &[Opcode], target: &'a impl Movable) -> Self {
        let commands: Vec<Box<dyn Command>> = program
            .iter()
            .map(|opcode| Self::parse_opcode(*opcode, target))
            .collect();

        Self { commands }
    }

    fn parse_opcode(opcode: Opcode, target: &impl Movable) -> Box<dyn Command + '_> {
        match opcode {
            Opcode::NoOp => Box::new(NoOpCommand::new()),
            Opcode::Forward => Box::new(ForwardCommand::new(target)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::{mock, predicate::*};

    mock! {
        #[derive(Debug)]
        pub Vehicle {}
        impl Movable for Vehicle {
            fn advance(&self, dir: i32);
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
        let vehicle = MockVehicle::new();
        let cpu = Cpu::new(&program, &vehicle);

        assert_eq!(cpu.commands.len(), 2);
        assert_eq!(cpu.commands[0].opcode(), Opcode::Forward);
        assert_eq!(cpu.commands[1].opcode(), Opcode::NoOp);
    }
}
