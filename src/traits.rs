/// CvRDTs that can only grow, i.e. only add items
pub trait Grow: Clone {
    type Payload: Eq;
    type Update;
    type Query;
    type Value;

    fn new(payload: Self::Payload) -> Self;
    fn payload(&self) -> Self::Payload;
    fn add(&mut self, update: Self::Update);
    fn le(&self, other: &Self) -> bool;
    fn merge(&self, other: &Self) -> Self;
    fn query(&self, query: &Self::Query) -> Self::Value;
}

/// CvRDTs that can also shrink, i.e. remove items
pub trait Shrink: Grow {
    fn del(&mut self, update: Self::Update);
}
