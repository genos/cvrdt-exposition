use crate::traits::Grow;

#[derive(Debug, Clone)]
pub struct GCounter {
    id: usize,
    counts: Vec<u64>,
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
    use crate::grow_properties;
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

    grow_properties!(two, three, cvrdt_and_update);
}
