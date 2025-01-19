pub use constant::*;
pub use field::*;

use crate::{EvaluationError, Extractable};

mod constant;
mod field;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Operand<V> {
    Field(Field),
    Constant(Constant<V>),
}

impl<V> Operand<V> {
    pub fn field(field: Field) -> Self {
        Self::Field(field)
    }

    pub fn constant(constant: Constant<V>) -> Self {
        Self::Constant(constant)
    }
}

impl<V: Clone> Operand<V> {
    pub fn get<'a, D: Extractable<V>>(
        &'a self,
        data: &'a D,
    ) -> Result<V, Box<dyn std::error::Error>> {
        match self {
            Self::Constant(c) => Ok(c.as_ref().clone()),
            Self::Field(f) => {
                let result = data
                    .extract(f.as_ref())?
                    .ok_or_else(|| EvaluationError::MissingField(f.to_string()))?;
                Ok(result)
            }
        }
    }
}

impl<V> From<Field> for Operand<V> {
    fn from(value: Field) -> Self {
        Self::field(value)
    }
}

impl<V> From<Constant<V>> for Operand<V> {
    fn from(value: Constant<V>) -> Self {
        Self::constant(value)
    }
}
