# Understanding Convergent Replicated Data Types

[![Build Status](https://github.com/genos/cvrdt-exposition/actions/workflows/ci.yml/badge.svg)]
[![Crates.io](https://img.shields.io/crates/v/cvrdt-exposition)](https://crates.io/crates/cvrdt-exposition)
[![Docs.rs](https://docs.rs/cvrdt-exposition/badge.svg)](https://docs.rs/cvrdt-exposition/)

I wanted to understand CRDTs more, so I put together this Rust library for state-based CRDTs a.k.a. convergent replicated data types a.k.a. CvRDTs.
It aims to present an explicit (with strong types, etc.) and unified description of the CvRDTs presented in the _wonderful_ paper [A comprehensive study of Convergent and Commutative Replicated Data Types](https://hal.inria.fr/inria-00555588/).

## Do not use this!

This code is solely for my own edification and is _not_ meant for production use.
There are already much better options for usable CRDTs in Rust; see the [`rust-crdt`](https://github.com/rust-crdt/rust-crdt) project.

## References

- [A comprehensive study of Convergent and Commutative Replicated Data Types](https://hal.inria.fr/inria-00555588/)
- Wikipedia article on [Conflict-free replicated data type](https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type)
- [`rust-crdt`](https://github.com/rust-crdt/rust-crdt)
- [`meangirls`](https://github.com/aphyr/meangirls)
- [The `proptest` book](https://altsysrq.github.io/proptest-book/intro.html); `cvrdt-exposition` uses `proptest` for testing that implementations fulfill required CvRDT properties. 
