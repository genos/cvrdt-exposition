use crate::traits::Grow;

/// A boolean flag that, once true, can never revert to false
///
/// # Examples
///
/// ```
/// use cvrdt_exposition::{Grow, OneWayBoolean};
/// let mut x = OneWayBoolean::new(false);
/// x.add(());
/// assert_eq!(x.payload(), true);
/// assert_eq!(x.query(&()), true);
/// for payload in vec![true, false] {
///     let y = OneWayBoolean::new(payload);
///     assert_eq!(x.merge(&y).payload(), y.merge(&x).payload());
///     assert!(y.le(&x));
/// }
/// ```
#[derive(Debug, Clone)]
pub struct OneWayBoolean {
    /// The internal state of a `OneWayBoolean` is a single boolean flag
    pub flag: bool,
}

impl Grow for OneWayBoolean {
    type Payload = bool;
    type Update = ();
    type Query = ();
    type Value = bool;

    fn new(payload: Self::Payload) -> Self {
        OneWayBoolean { flag: payload }
    }
    fn payload(&self) -> Self::Payload {
        self.flag
    }
    fn add(&mut self, _update: Self::Update) {
        self.flag = true;
    }
    fn le(&self, other: &OneWayBoolean) -> bool {
        self.flag <= other.flag
    }
    fn merge(&self, other: &OneWayBoolean) -> Self {
        OneWayBoolean {
            flag: self.flag || other.flag,
        }
    }
    fn query(&self, _query: &Self::Query) -> Self::Value {
        self.flag
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grow_properties;
    use proptest::prelude::*;

    fn cvrdt() -> impl Strategy<Value = OneWayBoolean> {
        any::<bool>().prop_map(|flag| OneWayBoolean { flag })
    }

    fn cvrdt_and_update() -> impl Strategy<Value = (OneWayBoolean, ())> {
        (cvrdt(), Just(()))
    }

    grow_properties!(cvrdt, cvrdt_and_update);
}
