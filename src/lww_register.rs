use crate::traits::Grow;
use std::time::Instant;

/// A last-write-wins register
///
/// # Panics
///
/// Any attempt to `add` a new element to this register will panic if the register's `timestamp` is
/// greater than `Instant::now()` (no time-traveling allowed) at the time of calling `add`:
///
/// ```should_panic
/// // This will panic
/// use std::time::{Duration, Instant};
/// use cvrdt_exposition::{Grow, LWWRegister};
/// let mut x = LWWRegister::new(('a', Instant::now() + Duration::from_secs(1729)));
/// x.add('b');
/// ```
///
/// # Difference from references
///
/// In the [comprehensive study paper](https://hal.inria.fr/inria-00555588/), timestamps are
/// unsigned integers, whereas we use
/// [`std::time::Instant`s](https://doc.rust-lang.org/std/time/struct.Instant.html).
///
/// # Examples
///
/// ```
/// use std::time::Instant;
/// use cvrdt_exposition::{Grow, LWWRegister};
/// let mut x = LWWRegister::new(('a', Instant::now()));
/// x.add('b');
/// x.add('c');
/// assert_eq!(x.query(&()), 'c');
/// let y = LWWRegister::new(('z', Instant::now()));
/// assert!(x.le(&y));
/// let z = x.merge(&y);
/// assert_eq!(y.merge(&x).payload(), z.payload());
/// assert_eq!(z.query(&()), 'z');
/// assert_eq!(z.payload().0, 'z');
/// ```
#[derive(Debug, Clone)]
pub struct LWWRegister<X: Clone + Eq> {
    pub value: X,
    pub timestamp: Instant,
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
        assert!(self.timestamp <= now, "Time should be monotonic");
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
