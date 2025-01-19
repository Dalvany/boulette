#[cfg(feature = "number")]
use number::NumberOperator;
pub use operand::*;
pub use r#trait::*;
use string::StringOperator;

#[cfg(feature = "number")]
pub mod number;
mod operand;
pub mod string;
mod r#trait;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EvaluationError {
    MissingField(String),
}

impl std::fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingField(field) => write!(f, "Unable to find {field}"),
        }
    }
}

impl std::error::Error for EvaluationError {}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Node<C> {
    And(Vec<Box<Node<C>>>),
    Or(Vec<Box<Node<C>>>),
    Not(Box<Node<C>>),
    String(StringOperator),
    #[cfg(feature = "number")]
    Number(NumberOperator),
    Custom(C),
}
