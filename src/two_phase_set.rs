use crate::traits::{Grow, Shrink};
use std::collections::HashSet;
use std::hash::Hash;

/// A set that can add or delete values
///
/// # Panics
///
/// Any attempt to `del` an element that one did  not previously `add` will panic:
///
/// ```should_panic
/// // this will panic
/// use std::collections::HashSet;
/// use cvrdt_exposition::{Grow, Shrink, TwoPhaseSet};
/// let mut x = TwoPhaseSet::new((HashSet::new(), HashSet::new()));
/// x.del("this will panic");
/// ```
///
/// # Examples
///
/// Example usage, including demonstrating some properties:
///
/// ```
/// use std::collections::HashSet;
/// use cvrdt_exposition::{Grow, Shrink, TwoPhaseSet};
/// let mut x = TwoPhaseSet::new((HashSet::new(), HashSet::new()));
/// for c in "abc".chars() {
///     x.add(c);
/// }
/// x.del('c');
/// assert_eq!(x.query(&'a'), true);
/// assert_eq!(x.query(&'z'), false);
/// assert_eq!(x.query(&'c'), false);
/// let y = TwoPhaseSet::new(("abcdef".chars().collect(), HashSet::new()));
/// assert_eq!(x.merge(&y).payload(), y.merge(&x).payload());
/// let z = TwoPhaseSet::new(("8675309abcdefg".chars().collect(), "toremove".chars().collect()));
/// assert_eq!(x.merge(&y.merge(&z)).payload(), x.merge(&y).merge(&z).payload());
/// ```
#[derive(Debug, Clone)]
pub struct TwoPhaseSet<X: Clone + Eq + Hash> {
    /// The elements that have been added to this set
    pub added: HashSet<X>,
    /// The elements that have been removed from this set
    pub removed: HashSet<X>,
}

impl<X: Clone + Eq + Hash> Grow for TwoPhaseSet<X> {
    type Payload = (HashSet<X>, HashSet<X>);
    type Update = X;
    type Query = X;
    type Value = bool;

    fn new(payload: Self::Payload) -> Self {
        TwoPhaseSet {
            added: payload.0,
            removed: payload.1,
        }
    }
    fn payload(&self) -> Self::Payload {
        (self.added.clone(), self.removed.clone())
    }
    fn add(&mut self, update: Self::Update) {
        self.added.insert(update);
    }
    fn le(&self, other: &Self) -> bool {
        self.added.is_subset(&other.added) && self.removed.is_subset(&other.removed)
    }
    fn merge(&self, other: &Self) -> Self {
        TwoPhaseSet {
            added: self.added.union(&other.added).cloned().collect(),
            removed: self.removed.union(&other.removed).cloned().collect(),
        }
    }
    fn query(&self, query: &Self::Query) -> Self::Value {
        self.added.contains(query) && !self.removed.contains(query)
    }
}

impl<X: Clone + Eq + Hash> Shrink for TwoPhaseSet<X> {
    fn del(&mut self, x: X) {
        assert!(
            self.query(&x),
            "Only allowed for elements contained in 2PSet"
        );
        self.removed.insert(x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{grow_properties, shrink_properties};
    use proptest::prelude::*;

    static MAX_SIZE: usize = 100;

    fn cvrdt() -> impl Strategy<Value = TwoPhaseSet<String>> {
        (
            prop::collection::hash_set(any::<String>(), 0..MAX_SIZE),
            prop::collection::hash_set(any::<String>(), 0..MAX_SIZE),
        )
            .prop_map(|(added, removed)| TwoPhaseSet { added, removed })
    }

    fn cvrdt_and_addend() -> impl Strategy<Value = (TwoPhaseSet<String>, String)> {
        (cvrdt(), ".*")
    }

    fn cvrdt_and_subtrahend() -> impl Strategy<Value = (TwoPhaseSet<i8>, i8)> {
        (
            prop::collection::hash_set(any::<i8>(), 0..MAX_SIZE),
            prop::collection::hash_set(any::<i8>(), 0..MAX_SIZE),
            any::<i8>()
        )
            .prop_flat_map(|(mut added, mut removed, x)| {
                added.insert(x);
                removed.remove(&x);
                let t = TwoPhaseSet { added, removed };
                (Just(t), Just(x))
            })
    }

    grow_properties!(cvrdt, cvrdt_and_addend);
    shrink_properties!(cvrdt_and_subtrahend);
}
