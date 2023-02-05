use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub enum Command {
    NoOp,
}

impl TryFrom<char> for Command {
    type Error = String;

    fn try_from(raw: char) -> Result<Self, Self::Error> {
        match raw {
            ' ' => Ok(Self::NoOp),
            _ => Err(format!("Unrecognized command `{}`", raw)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_command_from_char() {
        assert_eq!(Command::try_from(' '), Ok(Command::NoOp));
        assert!(Command::try_from('*').is_err());
    }
}
