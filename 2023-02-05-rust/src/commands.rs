use std::cmp::PartialEq;
use std::convert::TryFrom;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Opcode {
    NoOp,
    Forward,
    Backward,
    Left,
    Right,
}

impl TryFrom<char> for Opcode {
    type Error = String;

    fn try_from(raw: char) -> Result<Self, Self::Error> {
        match raw {
            ' ' => Ok(Self::NoOp),
            'f' | 'F' => Ok(Self::Forward),
            'b' | 'B' => Ok(Self::Backward),
            'l' | 'L' => Ok(Self::Left),
            'r' | 'R' => Ok(Self::Right),
            _ => Err(format!("Unrecognized command `{}`", raw)),
        }
    }
}

pub trait Command: Debug {
    fn execute(&mut self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct NoOpCommand {}

impl NoOpCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for NoOpCommand {}

pub trait Movable: Debug {
    fn advance(&mut self, dir: i32);
    fn turn(&mut self, dir: i32);
}

#[derive(Debug)]
pub struct ForwardCommand<'a> {
    target: &'a mut dyn Movable,
}

impl<'a> ForwardCommand<'a> {
    pub fn new(target: &'a mut impl Movable) -> Self {
        Self { target }
    }
}

impl<'a> Command for ForwardCommand<'a> {
    fn execute(&mut self) -> Result<(), String> {
        self.target.advance(1);
        Ok(())
    }
}

#[derive(Debug)]
pub struct BackwardCommand<'a> {
    target: &'a mut dyn Movable,
}

impl<'a> BackwardCommand<'a> {
    pub fn new(target: &'a mut impl Movable) -> Self {
        Self { target }
    }
}

impl<'a> Command for BackwardCommand<'a> {
    fn execute(&mut self) -> Result<(), String> {
        self.target.advance(-1);
        Ok(())
    }
}

#[derive(Debug)]
pub struct LeftCommand<'a> {
    target: &'a mut dyn Movable,
}

impl<'a> LeftCommand<'a> {
    pub fn new(target: &'a mut impl Movable) -> Self {
        Self { target }
    }
}

impl<'a> Command for LeftCommand<'a> {
    fn execute(&mut self) -> Result<(), String> {
        self.target.turn(-1);
        Ok(())
    }
}

#[derive(Debug)]
pub struct RightCommand<'a> {
    target: &'a mut dyn Movable,
}

impl<'a> RightCommand<'a> {
    pub fn new(target: &'a mut impl Movable) -> Self {
        Self { target }
    }
}

impl<'a> Command for RightCommand<'a> {
    fn execute(&mut self) -> Result<(), String> {
        self.target.turn(1);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_command_from_char() {
        assert_eq!(Opcode::try_from(' '), Ok(Opcode::NoOp));
        assert_eq!(Opcode::try_from('f'), Ok(Opcode::Forward));
        assert_eq!(Opcode::try_from('b'), Ok(Opcode::Backward));
        assert_eq!(Opcode::try_from('l'), Ok(Opcode::Left));
        assert_eq!(Opcode::try_from('r'), Ok(Opcode::Right));
        assert!(Opcode::try_from('*').is_err());
    }
}
