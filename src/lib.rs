extern crate conllx;

extern crate petgraph;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

mod ambiguities;
pub use ambiguities::*;

mod comps;
pub use comps::*;

mod data_preps;
pub use data_preps::read_gng_data;