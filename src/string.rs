use crate::{Evaluable, Extractable, Operand};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StringOperator {
    Equals {
        left: Operand<String>,
        right: Operand<String>,
    },
    Contains {
        left: Operand<String>,
        right: Operand<String>,
    },
    StartsWith {
        left: Operand<String>,
        right: Operand<String>,
    },
    EndsWith {
        left: Operand<String>,
        right: Operand<String>,
    },
}

impl Evaluable<String> for StringOperator {
    fn evaluate<D: Extractable<String>>(
        &self,
        data: &D,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let (left, right) = match self {
            Self::Equals { left, right }
            | Self::Contains { left, right }
            | Self::StartsWith { left, right }
            | Self::EndsWith { left, right } => {
                let left = left.get(data)?;
                let right = right.get(data)?;

                (left, right)
            }
        };

        match self {
            Self::Equals { left: _, right: _ } => Ok(left == right),
            Self::Contains { left: _, right: _ } => Ok(left.contains(&right)),
            Self::StartsWith { left: _, right: _ } => Ok(left.starts_with(&right)),
            Self::EndsWith { left: _, right: _ } => Ok(left.ends_with(&right)),
        }
    }
}
