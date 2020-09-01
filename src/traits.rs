/// CvRDTs that can only grow, i.e. only add items
pub trait Grow: Clone {
    /// The internal state of our CvRDT; sufficient to build a new copy via [`new`](#tymethod.new).
    /// Required to implement `Eq` for testing and verification.
    type Payload: Eq;

    /// Message to update our internal state
    type Update;

    /// Message to query the [`Value`](#associatedtype.Value) of our CvRDT
    type Query;

    /// The response to a [`Query`](#associatedtype.Query)
    type Value;

    /// Create a new version of our data structure from the given
    /// [`Payload`](#associatedtype.Payload)
    ///
    /// # Parameters
    ///
    /// - `payload`: a payload which fully specifies all information needed to instantiate our data
    /// structure
    ///
    /// # Returns
    ///
    /// A new instance of this CvRDT
    fn new(payload: Self::Payload) -> Self;

    /// Retrieve the [`Payload`](#associatedtype.Payload) (internal state) of this CvRDT
    ///
    /// # Parameters
    ///
    /// - a borrowed reference to `self`
    ///
    /// # Returns
    ///
    /// The payload of this CvRDT
    fn payload(&self) -> Self::Payload;

    /// Add an item to the data structure, mutating this CvRDT in place
    ///
    /// # Parameters
    ///
    /// - a mutably borrowed reference to `self`
    /// - an [`Update`](#associatedtype.Update) message
    ///
    /// # Returns
    ///
    /// Nothing; this data structure is updated in-place
    fn add(&mut self, update: Self::Update);

    /// Is this CvRDT ≤ another in the semilattice's partial order?
    ///
    /// # Parameters
    ///
    /// - a borrowed reference to `self`
    /// - a borrowed reference to teh other structure to compare
    ///
    /// # Returns
    ///
    /// `true` if and only if this CvRDT is less than or equal to the `other` (in terms of
    /// the semilattice induced by [`merge`](#tymethod.merge))
    fn le(&self, other: &Self) -> bool;

    /// Merge this data structure and another into a new CvRDT
    ///
    /// # Parameters
    ///
    /// - a borrowed reference to `self`
    /// - a borrowed reference to the other structure to merge
    ///
    /// # Returns
    ///
    /// A new instance of this CvRDT
    ///
    /// # Notes
    /// Per [the top-level documentation](../index.html#what-makes-a-cvrdt), this function must be
    /// commutative, associative, and idempotent
    fn merge(&self, other: &Self) -> Self;

    /// Query the data structure to get some [`Value`](#associatedtype.Value)
    ///
    /// # Parameters
    ///
    /// - a borrowed reference to `self`
    /// - a borrowed reference to a [`Query`](#associatedtype.Query) message
    ///
    /// # Returns
    ///
    /// The [`Value`](#associatedtype.Value) determined by the current internal state
    fn query(&self, query: &Self::Query) -> Self::Value;
}

/// CvRDTs that can also shrink, i.e. delete items
pub trait Shrink: Grow {
    /// Delete an item from the data structure, mutating this CvRDT in place
    ///
    /// # Parameters
    /// - a mutably borrowed reference to `self`
    /// - an [`Update`](#associatedtype.Update) message
    ///
    /// # Returns
    /// Nothing; this data structure is updated in-place
    fn del(&mut self, update: Self::Update);
}
