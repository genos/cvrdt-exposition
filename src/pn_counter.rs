use crate::traits::{Grow, Shrink};

/// A vectorized counter than can grow or shrink
///
/// # Panics
///
/// Like [`GCounter`s](../g_counter/struct.GCounter.html), ny function involving two or more
/// `PNCounter`s (viz. `le` and `merge`) will panic (via `assert_eq!`) if their counts vectors are
/// not the same length. What's more, since `PNCounter`s involve _two_ vectorized counts, any
/// instantiation (via `new`) will also panic if the lengths of the positive and negative count
/// vectors differ. I'd prefer to check this at compile time (as much as possible) instead, but
///
/// - avoiding C++'s template mess is part of what makes Rust great
/// - Rust doesn't have [const generics](https://rust-lang.github.io/rfcs/2000-const-generics.html)
/// yet
/// - this library is meant to be as simple and expository as possible, so I'd like to avoid
/// fancier things like [`generic_array`](https://docs.rs/generic-array/0.14.4/generic_array/)
///
/// # Difference from references
///
/// In the [comprehensive study paper](https://hal.inria.fr/inria-00555588/) and the [Wikipedia
/// article](https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type), the vectorized
/// `PNCounter` presumes a local `myID()` function that tells our local `PNCounter` the index to
/// update in its counts array. This detail isn't necessary for understanding how their pseudocode
/// works, but it _is_ required if you're trying to implement a `PNCounter` in real code. As such,
/// we explicitly include the `id` as a member of our `PNCounter` struct, and make the _arbitrary_
/// choice that when merging two `PNCounter`s, we take the minimum of their two `id`s as the new
/// one.
///
/// # Example
///
/// Example usage, including demonstrating some properties:
///
/// ```
/// use cvrdt_exposition::{Grow, PNCounter, Shrink};
/// let mut x = PNCounter::new((0, vec![0; 2], vec![0; 2]));
/// x.add(());
/// x.del(());
/// x.add(());
/// x.add(());
/// assert_eq!(x.payload(), (0, vec![3, 0], vec![1, 0]));
/// assert_eq!(x.query(&()), 2);
/// let y = PNCounter::new((1, vec![0, 3], vec![0, 0]));
/// let z = x.merge(&y);
/// assert_eq!(z.payload(), (0, vec![3, 3], vec![1, 0]));
/// assert_eq!(z.payload(), y.merge(&x).payload());
/// assert_eq!(z.query(&()), 5);
/// ```
///
/// As mentioned above, operations panic when trying dealing with two or more `PNCounter`s of
/// incompatible sizes:
///
/// ```should_panic
/// // This will panic
/// use cvrdt_exposition::{PNCounter, Grow};
/// let x = PNCounter::new((0, vec![0], vec![0]));
/// let y = PNCounter::new((1, vec![0, 0], vec![0, 0]));
/// x.merge(&y);
/// ```
///
/// We will also get panics if we try to create a new `PNCounter` with differing `positive` and
/// `negative` lengths:
///
/// ```should_panic
/// // This will panic
/// use cvrdt_exposition::{PNCounter, Grow};
/// let x = PNCounter::new((0, vec![0], vec![0, 0]));
/// ```
///
/// Or if we specify an `id` outside the length of the `positive` or `negative` counts:
///
/// ```should_panic
/// // This will panic
/// use cvrdt_exposition::{PNCounter, Grow};
/// let x = PNCounter::new((17, vec![0], vec![0]));
/// ```
#[derive(Debug, Clone)]
pub struct PNCounter {
    /// The index for this local `PNCounter` where all updates occur
    pub id: usize,
    /// The vector of positive counts (additions)
    pub positive: Vec<u64>,
    /// The vector of negative counts (deletions)
    pub negative: Vec<u64>,
}

impl PNCounter {
    fn consistent(&self) {
        assert_eq!(
            self.positive.len(),
            self.negative.len(),
            "Incompatible positive & negative lengths"
        );
        assert!(self.id < self.positive.len(), "ID too large");
        assert!(self.id < self.negative.len(), "ID too large");
    }
    fn compatible_len(&self, other: &Self) -> usize {
        self.consistent();
        other.consistent();
        assert_eq!(
            self.positive.len(),
            other.positive.len(),
            "Incompatible positive lengths"
        );
        assert_eq!(
            self.negative.len(),
            other.negative.len(),
            "Incompatible negative lengths"
        );
        self.positive.len()
    }
}

impl Grow for PNCounter {
    type Payload = (usize, Vec<u64>, Vec<u64>);
    type Update = ();
    type Query = ();
    type Value = u64;

    fn new(payload: Self::Payload) -> Self {
        let pn = PNCounter {
            id: payload.0,
            positive: payload.1,
            negative: payload.2,
        };
        pn.consistent();
        pn
    }
    fn payload(&self) -> Self::Payload {
        (self.id, self.positive.clone(), self.negative.clone())
    }
    fn add(&mut self, _update: Self::Update) {
        self.positive[self.id] += 1;
    }
    fn le(&self, other: &Self) -> bool {
        let n = self.compatible_len(other);
        (0..n)
            .all(|i| self.positive[i] <= other.positive[i] && self.negative[i] <= other.negative[i])
    }
    fn merge(&self, other: &Self) -> Self {
        let n = self.compatible_len(other);
        PNCounter {
            id: self.id.min(other.id), // arbitrary
            positive: (0..n)
                .map(|i| self.positive[i].max(other.positive[i]))
                .collect(),
            negative: (0..n)
                .map(|i| self.negative[i].max(other.negative[i]))
                .collect(),
        }
    }
    fn query(&self, _query: &Self::Query) -> Self::Value {
        self.positive.iter().sum::<u64>() - self.negative.iter().sum::<u64>()
    }
}

impl Shrink for PNCounter {
    fn del(&mut self, _update: Self::Update) {
        self.negative[self.id] += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{grow_properties, shrink_properties};
    use proptest::prelude::*;

    static MAX_SIZE: usize = 100;

    fn sized(n: usize) -> impl Strategy<Value = PNCounter> {
        (
            prop::collection::vec(any::<u64>(), n),
            prop::collection::vec(any::<u64>(), n),
        )
            .prop_flat_map(|(positive, negative)| {
                let len = positive.len();
                (0..len, Just(positive), Just(negative))
            })
            .prop_map(|(id, positive, negative)| PNCounter {
                id,
                positive,
                negative,
            })
    }

    fn two() -> impl Strategy<Value = (PNCounter, PNCounter)> {
        (1..MAX_SIZE).prop_flat_map(|n| (sized(n), sized(n)))
    }
    fn three() -> impl Strategy<Value = (PNCounter, PNCounter, PNCounter)> {
        (1..MAX_SIZE).prop_flat_map(|n| (sized(n), sized(n), sized(n)))
    }
    fn cvrdt_and_update() -> impl Strategy<Value = (PNCounter, ())> {
        (1..MAX_SIZE).prop_flat_map(sized).prop_map(|p| (p, ()))
    }

    grow_properties!(two, three, cvrdt_and_update);
    shrink_properties!(cvrdt_and_update);
}
