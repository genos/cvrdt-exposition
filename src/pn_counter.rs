use crate::traits::{Grow, Shrink};

#[derive(Debug, Clone)]
pub struct PNCounter {
    pub id: usize,
    pub positive: Vec<u64>,
    pub negative: Vec<u64>,
}

impl PNCounter {
    fn compatible_len(&self, other: &Self) -> usize {
        assert_eq!(
            self.positive.len(),
            self.negative.len(),
            "Incompatible positive & negative lengths"
        );
        assert_eq!(
            other.positive.len(),
            other.negative.len(),
            "Incompatible positive & negative lengths"
        );
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
        PNCounter {
            id: payload.0,
            positive: payload.1,
            negative: payload.2,
        }
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
