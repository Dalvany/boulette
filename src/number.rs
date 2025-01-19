use rust_decimal::Decimal;

use crate::{Evaluable, Operand};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum NumberOperator {
    Equals {
        left: Operand<Decimal>,
        right: Operand<Decimal>,
    },
    LessThan {
        left: Operand<Decimal>,
        right: Operand<Decimal>,
    },
    LessThanOrEquals {
        left: Operand<Decimal>,
        right: Operand<Decimal>,
    },
    GreaterThan {
        left: Operand<Decimal>,
        right: Operand<Decimal>,
    },
    GreaterThanOrEquals {
        left: Operand<Decimal>,
        right: Operand<Decimal>,
    },
}

impl Evaluable<Decimal> for NumberOperator {
    fn evaluate<D: crate::Extractable<Decimal>>(
        &self,
        data: &D,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let (left, right) = match self {
            Self::Equals { left, right }
            | Self::LessThan { left, right }
            | Self::LessThanOrEquals { left, right }
            | Self::GreaterThan { left, right }
            | Self::GreaterThanOrEquals { left, right } => {
                let left = left.get(data)?;
                let right = right.get(data)?;

                (left, right)
            }
        };

        match self {
            Self::Equals { left: _, right: _ } => Ok(left == right),
            Self::LessThan { left: _, right: _ } => Ok(left < right),
            Self::LessThanOrEquals { left: _, right: _ } => Ok(left <= right),
            Self::GreaterThan { left: _, right: _ } => Ok(left > right),
            Self::GreaterThanOrEquals { left: _, right: _ } => Ok(left >= right),
        }
    }
}
