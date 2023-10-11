use crate::traits::Grow;

/// A vectorized counter that can only grow
///
/// # Panics
///
/// Any function involving two or more `GCounter`s (viz. `le` and `merge`) will panic (via
/// `assert_eq!`) if their counts vectors are not the same length. I'd prefer to check this at
/// compile time (as much as possible) instead, but
///
/// - avoiding C++'s template mess is part of what makes Rust great
/// - Rust doesn't have [const generics](https://rust-lang.github.io/rfcs/2000-const-generics.html)
/// yet
/// - this library is meant to be as simple and expository as possible, so I'd like to avoid
/// fancier things like [`generic_array`](https://docs.rs/generic-array/0.14.4/generic_array/)
///
/// As mentioned above, operations panic when trying dealing with two or more `GCounter`s of
/// incompatible sizes:
///
/// ```should_panic
/// // this will panic
/// use cvrdt_exposition::{GCounter, Grow};
/// let x = GCounter::new((0, vec![0]));
/// let y = GCounter::new((1, vec![0, 0]));
/// x.merge(&y);
/// ```
///
/// # Difference from references
///
/// In the [comprehensive study paper](https://hal.inria.fr/inria-00555588/) and the [Wikipedia
/// article](https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type), the vectorized
/// `GCounter` presumes a local `myID()` function that tells our local `GCounter` the index to
/// update in its counts array. This detail isn't necessary for understanding how their pseudocode
/// works, but it _is_ required if you're trying to implement a `GCounter` in real code. As such,
/// we explicitly include the `id` as a member of our `GCounter` struct, and make the _arbitrary_
/// choice that when merging two `GCounter`s, we take the minimum of their two `id`s as the new
/// one.
///
/// # Examples
///
/// Example usage, including demonstrating some properties:
///
/// ```
/// use cvrdt_exposition::{GCounter, Grow};
/// let mut x = GCounter::new((0, vec![0; 3]));
/// x.add(());
/// assert_eq!(x.payload(), (0, vec![1, 0, 0]));
/// assert_eq!(x.query(&()), 1);
/// let mut y = GCounter::new((1, vec![0; 3]));
/// y.add(());
/// y.add(());
/// assert_eq!(x.merge(&y).payload(), (0, vec![1, 2, 0]));
/// let z = GCounter::new((2, vec![0, 0, 3]));
/// assert!(x.le(&x.merge(&y).merge(&z)));
/// assert_eq!(x.merge(&y).merge(&z).payload(), (0, vec![1, 2, 3]));
/// assert_eq!(x.merge(&y.merge(&z)).payload(), x.merge(&y).merge(&z).payload());
/// ```
#[derive(Debug, Clone)]
pub struct GCounter {
    /// The index for this local `GCounter` where all increments occur
    pub id: usize,
    /// The vector of counts
    pub counts: Vec<u64>,
}

impl GCounter {
    fn compatible_len(&self, other: &Self) -> usize {
        assert_eq!(
            self.counts.len(),
            other.counts.len(),
            "Incompatible lengths"
        );
        self.counts.len()
    }
}

impl Grow for GCounter {
    type Payload = (usize, Vec<u64>);
    type Update = ();
    type Query = ();
    type Value = u64;

    fn new(payload: Self::Payload) -> Self {
        GCounter {
            id: payload.0,
            counts: payload.1,
        }
    }
    fn payload(&self) -> Self::Payload {
        (self.id, self.counts.clone())
    }
    fn add(&mut self, _update: Self::Update) {
        self.counts[self.id] += 1;
    }
    fn le(&self, other: &Self) -> bool {
        let n = self.compatible_len(other);
        (0..n).all(|i| self.counts[i] <= other.counts[i])
    }
    fn merge(&self, other: &Self) -> Self {
        let n = self.compatible_len(other);
        GCounter {
            id: self.id.min(other.id), // arbitrary
            counts: (0..n)
                .map(|i| self.counts[i].max(other.counts[i]))
                .collect(),
        }
    }
    fn query(&self, _query: &Self::Query) -> Self::Value {
        self.counts.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::properties::grow;
    use proptest::prelude::*;

    static MAX_SIZE: usize = 100;

    fn sized(n: usize) -> impl Strategy<Value = GCounter> {
        prop::collection::vec(any::<u64>(), n)
            .prop_flat_map(|counts| {
                let len = counts.len();
                (0..len, Just(counts))
            })
            .prop_map(|(id, counts)| GCounter { id, counts })
    }

    fn two() -> impl Strategy<Value = (GCounter, GCounter)> {
        (1..MAX_SIZE).prop_flat_map(|n| (sized(n), sized(n)))
    }
    fn three() -> impl Strategy<Value = (GCounter, GCounter, GCounter)> {
        (1..MAX_SIZE).prop_flat_map(|n| (sized(n), sized(n), sized(n)))
    }
    fn cvrdt_and_update() -> impl Strategy<Value = (GCounter, ())> {
        (1..MAX_SIZE).prop_flat_map(sized).prop_map(|g| (g, ()))
    }

    grow!(two, three, cvrdt_and_update);
}
