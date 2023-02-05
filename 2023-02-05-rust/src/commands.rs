use std::cmp::PartialEq;
use std::convert::TryFrom;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Opcode {
    NoOp,
    Forward,
}

impl TryFrom<char> for Opcode {
    type Error = String;

    fn try_from(raw: char) -> Result<Self, Self::Error> {
        match raw {
            ' ' => Ok(Self::NoOp),
            'f' | 'F' => Ok(Self::Forward),
            _ => Err(format!("Unrecognized command `{}`", raw)),
        }
    }
}

pub trait Command: Debug {
    fn execute(&self) -> Result<(), String> {
        Ok(())
    }

    fn opcode(&self) -> Opcode;
}

impl PartialEq for dyn Command {
    fn eq(&self, other: &Self) -> bool {
        self.opcode() == other.opcode()
    }
}

#[derive(Debug, PartialEq)]
pub struct NoOpCommand {}
impl Command for NoOpCommand {
    fn opcode(&self) -> Opcode {
        Opcode::NoOp
    }
}

impl NoOpCommand {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait Movable: Debug {
    fn advance(&self, dir: i32);
}

#[derive(Debug)]
pub struct ForwardCommand<'a> {
    target: &'a dyn Movable,
}

impl<'a> ForwardCommand<'a> {
    pub fn new(target: &'a impl Movable) -> Self {
        Self { target }
    }
}

impl<'a> Command for ForwardCommand<'a> {
    fn execute(&self) -> Result<(), String> {
        println!("Trying to move its target");
        Ok(())
    }

    fn opcode(&self) -> Opcode {
        Opcode::Forward
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_command_from_char() {
        assert_eq!(Opcode::try_from(' '), Ok(Opcode::NoOp));
        assert_eq!(Opcode::try_from('f'), Ok(Opcode::Forward));
        assert!(Opcode::try_from('*').is_err());
    }
}
