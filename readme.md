# About    

This repository contains a copy of the source code for the `BTreeMap` and `BTreeSet` types from the Rust standard library. Minimal changes to the code were made to it to compile as a stand-alone library and pass all the tests. The original source code was downloaded on 2022-06-04 from the [Rust repository](https://github.com/rust-lang/rust/tree/master/library/alloc/src/collections/btree). Tests were run on Rust 1.61.0 stable. 

Note that all the work to implement these data structures has been done by the standard library authors. This library is intended to work as a reference and basis for implementing other tree data structures in Rust. 

I'm releasing this in case someone finds it useful, but I'm not planning to keep it updated and cannot provide any support.

## Example

You may want to clone or download the source and add the package as a local dependency. See the [Cargo book](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) for examples.

Usage is similar to the standard library:

```rust
use btree::map::BTreeMap; // Instead of std::collections::BTreeMap

let mut movie_reviews = BTreeMap::new();

// review some movies.
movie_reviews.insert("Office Space",       "Deals with real issues in the workplace.");
movie_reviews.insert("Pulp Fiction",       "Masterpiece.");
movie_reviews.insert("The Godfather",      "Very enjoyable.");
movie_reviews.insert("The Blues Brothers", "Eye lyked it a lot.");

// check for a specific one.
if !movie_reviews.contains_key("Les Misérables") {
    println!("We've got {} reviews, but Les Misérables ain't one.",
            movie_reviews.len());
}

// oops, this review has a lot of spelling mistakes, let's delete it.
movie_reviews.remove("The Blues Brothers");

// look up the values associated with some keys.
let to_find = ["Up!", "Office Space"];
for movie in &to_find {
    match movie_reviews.get(movie) {
    Some(review) => println!("{movie}: {review}"),
    None => println!("{movie} is unreviewed.")
    }
}

// Look up the value for a key (will panic if the key is not found).
println!("Movie review: {}", movie_reviews["Office Space"]);

// iterate over everything.
for (movie, review) in &movie_reviews {
    println!("{movie}: \"{review}\"");
} 
```
## Licence 

MIT (I think)
