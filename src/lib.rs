// This library contains copies of the implementations of the BTreeMap and BTreeSet types from the Rust 
// standard library. Minimal changes to the code was made to it to compile and pass the tests. Tests were 
// run on Rust 1.61.0 stable. The original source code was downloaded on 2022-06-04 from 
// https://github.com/rust-lang/rust/tree/master/library/alloc/src/collections/btree 
// 
// Note that the I'm not affiliated with the Rust project and cannot provide any support. The library 
// is intended to work as a reference and basis for implementing other tree data structures in Rust. 
// I'm releasing this in case someone find's it useful, but I'm not planning to keep it updated.

mod append;
mod borrow;
mod dedup_sorted_iter;
mod fix;
pub mod map;
mod mem;
mod merge_iter;
mod navigate;
mod node;
mod remove;
mod search;
pub mod set;
mod split;
#[cfg(test)]
mod testing;

#[doc(hidden)]
trait Recover<Q: ?Sized> {
    type Key;

    fn get(&self, key: &Q) -> Option<&Self::Key>;
    fn take(&mut self, key: &Q) -> Option<Self::Key>;
    fn replace(&mut self, key: Self::Key) -> Option<Self::Key>;
}

pub use self::map::*;
pub use self::set::*;
