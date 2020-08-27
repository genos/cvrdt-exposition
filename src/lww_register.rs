use crate::traits::Grow;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct LWWRegister<X: Clone + Eq> {
    value: X,
    timestamp: Instant,
}

impl<X: Clone + Eq> Grow for LWWRegister<X> {
    type Payload = (X, Instant);
    type Update = X;
    type Query = ();
    type Value = X;

    fn new(payload: Self::Payload) -> Self {
        LWWRegister {
            value: payload.0,
            timestamp: payload.1,
        }
    }
    fn payload(&self) -> Self::Payload {
        (self.value.clone(), self.timestamp)
    }
    fn add(&mut self, update: Self::Update) {
        let now = Instant::now();
        assert!(now >= self.timestamp, "Time should be monotonic");
        self.value = update;
        self.timestamp = now;
    }
    fn le(&self, other: &Self) -> bool {
        self.timestamp <= other.timestamp
    }
    fn merge(&self, other: &Self) -> Self {
        if self.timestamp < other.timestamp {
            other.clone()
        } else {
            self.clone()
        }
    }
    fn query(&self, _query: &Self::Query) -> Self::Value {
        self.value.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grow_properties;
    use proptest::prelude::*;

    fn cvrdt() -> impl Strategy<Value = LWWRegister<String>> {
        any::<String>().prop_map(|value| LWWRegister {
            value: value,
            timestamp: Instant::now(),
        })
    }

    fn cvrdt_and_update() -> impl Strategy<Value = (LWWRegister<String>, String)> {
        (cvrdt(), ".*")
    }

    grow_properties!(cvrdt, cvrdt_and_update);
}
