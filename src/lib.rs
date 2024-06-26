#![feature(box_patterns)]

// #[link_args = "-s EXPORTED_FUNCTIONS=['_coolrand','_makeIter','_next']"]
extern "C" {}

#[macro_use]
extern crate lazy_static;

extern crate serde;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate rand;

extern crate unicode_segmentation;

pub mod ops;

#[macro_use]
pub mod combinators;

pub mod paths;

pub mod compiler;
pub mod error;
pub mod indexes;
pub mod parser;
pub mod solver;

pub mod numerics;

pub mod watchers;

#[macro_use]
pub mod test_util;
