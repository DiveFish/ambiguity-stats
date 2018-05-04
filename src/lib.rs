extern crate conllx;

extern crate petgraph;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

mod ambiguities;
pub use ambiguities::pp_attachment;

mod comps;
pub use comps::{heads_and_deprels_equal, heads_equal, deprels_equal, postags_equal, check_deprel, check_deprels, check_postag, check_postags};

mod data_preps;
pub use data_preps::read_gng_data;