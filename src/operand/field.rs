#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Field(String);

impl Field {
    pub fn new(value: String) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Field {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl From<String> for Field {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Field {
    fn from(value: &str) -> Self {
        value.to_owned().into()
    }
}
