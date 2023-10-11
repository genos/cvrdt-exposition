//! # Understanding Convergent Replicated Data Types
//!
//! I wanted to understand CRDTs more, so I put together this Rust library for state-based CRDTs a.k.a. convergent replicated data types a.k.a. `CvRDTs`.
//! It aims to present an explicit (with strong types, etc.) and unified description of the `CvRDTs` presented in the _wonderful_ paper [A comprehensive study of Convergent and Commutative Replicated Data Types](https://hal.inria.fr/inria-00555588/).
//!
//! ## Do not use this!
//!
//! This code is solely for my own edification and is _not_ meant for production use.
//! There are already much better options for usable CRDTs in Rust; see the [`rust-crdt`](https://github.com/rust-crdt/rust-crdt) project.
//!
//! ## What makes a `CvRDT`?
//!
//! Quoting the [Wikipedia article on CRDTs](https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type),
//! > `CvRDTs` send their full local state to other replicas, where the states are merged by a function which must be commutative, associative, and idempotent.
//!
//! So suppose we've just written a brand new data type, and we'd like to demonstrate it's a `CvRDT`.
//! Suppose further that _x_, _y_, and _z_ are any arbitrary members of our data type, and that our data type has a merge function called _merge_; for our type to be a `CvRDT`, we need the following three things to be true for **any** values of _x_, _y_, and _z_:
//! 1. _merge(x, y) = merge(y, x)_
//! 2. _merge(x, merge(y, z)) = merge(merge(x, y), z)_
//! 3. _merge(merge(x, y), y) = merge(x, y)_
//!
//! [Wikipedia](https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type) continues,
//! > The merge function provides a join for any pair of replica states, so the set of all states forms a semilattice.
//!
//! Our merge function _merge_ induces a partial order on all the elements of our type, akin to Rust's [`PartialOrd` trait](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html), where _x ≤ y_ if _merge(x, y) = y_.
//! Since for any two elements _x_ and _y_, both _x ≤ merge(x, y)_ and _y ≤ merge(x, y)_, and _merge(x, y)_ is the "smallest" such elment for which this occurs, it is the least upper bound (or **join**) of _x_ and _y_.
//!
//! [The article](https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type) goes on
//! > The update function must monotonically increase the internal state, according to the same partial order rules as the semilattice.
//!
//! Suppose our type has an update function which we'll call _add_.
//! For any element _x_ and update value _u_, we need _x ≤ update(x, u)_; that is, we need _merge(x, update(x, u)) = update(x, u)_.
//!
//! Many types of CvRDT—in this library especially—are grow-only; they can add new elements/values/etc., but once added, these cannot be removed.
//! Some types allow removal as well; if our type does so, we **must** ensure that removal maintains the partial order given by our _merge_ function.
//! That is, we need the internal state to increase monotonically (as Wikipedia says in the above quote) even in the face of removals.
//! Suppose our type has a function
//!
//! Ideally we'd check these requirements for **every** possible configuration of our new `CvRDT` implementation.
//! This can be impossible to do via brute force, as the number of states to check can quickly get prohibitively large.
//! See the [penultimate section](#how-cvrdt-exposition-verifies-properties) for how `cvrdt-exposition` verifies these properties.
//!
//! ## Examples
//!
//! Per the [Wikipedia article](https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type), consider a one-way boolean flag that, once true, can never revert to false:
//! ```
//! use cvrdt_exposition::{Grow, OneWayBoolean};
//! let x = OneWayBoolean::new(false);
//! let mut y = OneWayBoolean::new(false);
//! y.add(());
//! assert_eq!(y.payload(), true);
//! assert_eq!(y.merge(&x).payload(), true);
//! ```
//!
//! As the internal state of a `OneWayBoolean` is only a single boolean value, we could verify the implementation fulfills the `CvRDT` requirements  by hand (though I'd rather get my computer to do it! see below).
//!
//! ## How `cvrdt-exposition` verifies properties
//!
//! In the absence of using formal methods like [TLA+](https://learntla.com/introduction/) (see also Hillel Wayne's [excellent book!](https://learntla.com/book/)), we resort to property-based testing via the [`proptest` crate](https://crates.io/crates/proptest).
//! This excellent crate is listed as a `dev-dependency` in [`cvrdt-exposition`'s `Cargo.tml` file](https://github.com/genos/cvrdt-exposition/blob/main/Cargo.toml), so if you can just use the stuff in this library (although [you shouldn't!](do-not-use-this)) without pulling in an extra dependency.
//! That said, I highly recommend learning to use `proptest`, [`quickcheck`](https://crates.io/crates/quickcheck), or some other [property testing framework](https://crates.io/search?q=property%20testing) for Rust.
//!
//! In the `cfg(test)`-only [`properties` module](https://github.com/genos/cvrdt/blob/main/src/properties.rs), `cvrdt-exposition` defines macros for automating checking `CvRDT` properties.
//! For instance, given a function `arb_cvrdt2` that yields an arbitrary pair of elements of our `CvRDT` type, `proptest`'s `proptest!` macro allows us to test that our `CvRDT`'s merge function is commutative via:
//!
//! ```ignore
//! proptest! {
//!     #[test]
//!     fn merge_commutative((x, y) in $arb_cvrdt2()) {
//!         prop_assert_eq!(
//!             Grow::payload(&Grow::merge(&x, &y)),
//!             Grow::payload(&Grow::merge(&y, &x))
//!         );
//!     }
//! };
//! ```
//!
//! Note that the above code uses the `cvrdt-exposition` nomenclature `Grow`, which we use to distinguish `CvRDTs` that are grow-only from those that can also remove elements.
//! See the [traits documentation](traits/index.html) for more.
//!
//! ## References
//!
//! - [A comprehensive study of Convergent and Commutative Replicated Data Types](https://hal.inria.fr/inria-00555588/)
//! - Wikipedia article on [Conflict-free replicated data type](https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type)
//! - [`rust-crdt`](https://github.com/rust-crdt/rust-crdt)
//! - [`meangirls`](https://github.com/aphyr/meangirls)
//! - [The `proptest` book](https://altsysrq.github.io/proptest-book/intro.html)
#![forbid(unsafe_code)]
#![forbid(missing_docs)]

/// Our two traits defining `CvRDTs`
pub mod traits;

/// Grow-Only Counter
pub mod g_counter;
/// Grow-Only Set
pub mod g_set;
/// Last-Writer-Wins Register
pub mod lww_register;
/// The simplest `CvRDT` example: a boolean flag that, once true, can never revert to false
pub mod one_way_boolean;
/// Positive-Negative Counter
pub mod pn_counter;
/// Two-Phase Set
pub mod two_phase_set;

/// Top-level re-exports for CRDT structures and traits
pub use crate::{
    g_counter::GCounter,
    g_set::GSet,
    lww_register::LWWRegister,
    one_way_boolean::OneWayBoolean,
    pn_counter::PNCounter,
    traits::{Grow, Shrink},
    two_phase_set::TwoPhaseSet,
};

/// PBT for `CvRDT` properties
#[cfg(test)]
pub(crate) mod properties;
