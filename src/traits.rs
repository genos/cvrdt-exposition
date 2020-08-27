/// CvRDTs that can only grow, i.e. only add items
pub trait Grow: Clone {
    /// The internal state of our CvRDT; sufficient to build a new copy via [`new`](#tymethod.new)
    type Payload;

    /// Message to update our internal state; received by [`add`](#tymethod.add)
    type Update;

    /// Message to query the [`Value`](#associatedtype.Value) of our CvRDT
    type Query;

    /// The response to a [`Query`](#associatedtype.Query)
    type Value;

    /// Create a new version of our data structure from the given
    /// [`Payload`](#associatedtype.Payload)
    fn new(payload: Self::Payload) -> Self;

    /// Retrieve the [`Payload`](#associatedtype.Payload) (internal state) of this CvRDT
    fn payload(&self) -> Self::Payload;

    /// Add an item to the data structure, mutating it in place
    fn add(&mut self, update: Self::Update);

    /// `true` if and only if this CvRDT is less than or equal to the `other` (in terms of
    /// the semilattice induced by [`merge`](#tymethod.merge))
    fn le(&self, other: &Self) -> bool;

    /// Merge this data structure and another into a new CvRDT; per [the top-level
    /// documentation](../index.html#what-makes-a-cvrdt), this function must be commutative,
    /// associative, and idempotent
    fn merge(&self, other: &Self) -> Self;

    /// Query the data structure to get some [`Value`](#associatedtype.Value)
    fn query(&self, query: &Self::Query) -> Self::Value;
}

/// CvRDTs that can also shrink, i.e. delete items
pub trait Shrink: Grow {
    /// Update the CvRDT by deleting/removing an item
    fn del(&mut self, update: Self::Update);
}
