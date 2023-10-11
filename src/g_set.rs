use crate::traits::Grow;
use std::collections::HashSet;
use std::hash::Hash;

/// A set that can only add values
///
/// # Examples
///
/// Example usage, including demonstrating some properties:
///
/// ```
/// use std::collections::HashSet;
/// use cvrdt_exposition::{Grow, GSet};
/// let mut x = GSet::new(HashSet::new());
/// for c in "abc".chars() {
///     x.add(c);
/// }
/// assert_eq!(x.query(&'a'), true);
/// assert_eq!(x.query(&'z'), false);
/// assert!(x.le(&GSet::new("abcdef".chars().collect())));
/// let y = GSet::new("cdefg".chars().collect());
/// assert_eq!(x.merge(&y).payload(), y.merge(&x).payload());
/// let z = GSet::new("8675309abcdefg".chars().collect());
/// assert_eq!(x.merge(&y.merge(&z)).payload(), x.merge(&y).merge(&z).payload());
/// ```
#[derive(Debug, Clone)]
pub struct GSet<X: Clone + Eq + Hash> {
    /// The contents of this set
    pub values: HashSet<X>,
}

impl<X: Clone + Eq + Hash> Grow for GSet<X> {
    type Payload = HashSet<X>;
    type Update = X;
    type Query = X;
    type Value = bool;

    fn new(payload: Self::Payload) -> Self {
        GSet { values: payload }
    }
    fn payload(&self) -> Self::Payload {
        self.values.clone()
    }
    fn add(&mut self, update: Self::Update) {
        self.values.insert(update);
    }
    fn le(&self, other: &Self) -> bool {
        self.values.is_subset(&other.values)
    }
    fn merge(&self, other: &Self) -> Self {
        GSet {
            values: self.values.union(&other.values).cloned().collect(),
        }
    }
    fn query(&self, query: &Self::Query) -> Self::Value {
        self.values.contains(query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::properties::grow;
    use proptest::prelude::*;

    static MAX_SIZE: usize = 100;

    fn cvrdt() -> impl Strategy<Value = GSet<String>> {
        prop::collection::hash_set(any::<String>(), 0..MAX_SIZE).prop_map(|values| GSet { values })
    }

    fn cvrdt_and_update() -> impl Strategy<Value = (GSet<String>, String)> {
        (cvrdt(), ".*")
    }

    grow!(cvrdt, cvrdt_and_update);
}
