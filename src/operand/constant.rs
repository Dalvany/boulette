#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Constant<V>(V);

impl<V> AsRef<V> for Constant<V> {
    fn as_ref(&self) -> &V {
        &self.0
    }
}

impl<V> From<V> for Constant<V> {
    fn from(value: V) -> Self {
        Self(value)
    }
}
