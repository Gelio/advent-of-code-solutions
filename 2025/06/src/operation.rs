use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operation {
    Add,
    Multiply,
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 1 {
            return Err(format!(
                "operation should be a single character, got \"{s}\""
            ));
        }

        s.chars()
            .nth(0)
            .ok_or(format!("received empty string"))?
            .try_into()
    }
}

impl TryFrom<char> for Operation {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '*' => Ok(Operation::Multiply),
            '+' => Ok(Operation::Add),
            _ => Err(format!("unsupported operation \"{value}\"")),
        }
    }
}
