#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DoNotUse;

impl std::fmt::Display for DoNotUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("You must use your own Evaluable implementation for custom operator. See crate documentation")
    }
}

impl std::error::Error for DoNotUse {}

#[allow(dead_code)]
struct Dummy;

impl Evaluable<String> for Dummy {
    fn evaluate<'a, D: Extractable<String>>(
        &self,
        _: &D,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        Err(Box::new(DoNotUse))
    }
}

/// This trait must be implemented by any struct that need to run against the boolean AST.
/// It allows AST to get values.Extract
pub trait Extractable<V: Clone> {
    /// Extract the value for the given key.
    ///
    /// # Parameter
    ///
    /// * key : a key that the implementor need to interpret to get the data.
    ///
    /// It depend on what operator you use but common implementation should be for
    /// `Extractable<String>` and `Extractable<[Number]>`.
    ///
    /// It must return `None` if the `key` is irrelevent for the struct and an error
    /// if conversion fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use boulette::Extractable;
    /// use rust_decimal::Decimal;
    ///
    /// struct Person {
    ///     name: String,
    ///     age: u8,
    /// }
    ///
    /// #[derive(Debug, thiserror::Error)]
    /// pub enum ConversionError {
    ///     #[error("Field {0} isn't a number")]
    ///     NotANumber(String),
    /// }
    ///
    /// impl Extractable<String> for Person {
    ///     fn extract(&self, key: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    ///         match key {
    ///             "name" => Ok(Some(self.name.clone())),
    ///             "age" => Ok(Some(self.age.to_string())),
    ///             // Field doesn't exist we return None
    ///             _ => Ok(None),
    ///         }
    ///     }
    /// }
    ///
    /// impl Extractable<Decimal> for Person {
    ///     fn extract<'a>(&self, key: &str) -> Result<Option<Decimal>, Box<dyn std::error::Error>> {
    ///         match key {
    ///             // Makes no sense to convert the name into a number, returning an error
    ///             "name" => Err(Box::new(ConversionError::NotANumber(key.to_string()))),
    ///             "age" => Ok(Some(self.age.into())),
    ///             // Field doesn't exist we return None
    ///             _ => Ok(None),
    ///         }
    ///     }
    /// }
    /// ```
    ///
    ///
    fn extract(&self, key: &str) -> Result<Option<V>, Box<dyn std::error::Error>>;
}

pub trait Evaluable<V: Clone> {
    fn evaluate<D: Extractable<V>>(&self, data: &D) -> Result<bool, Box<dyn std::error::Error>>;
}
