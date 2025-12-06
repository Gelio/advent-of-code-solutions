use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Add,
    Multiply,
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Operation::Multiply),
            "+" => Ok(Operation::Add),
            _ => Err(format!("unsupported operation \"{s}\"")),
        }
    }
}
