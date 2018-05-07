extern crate conllx;

extern crate petgraph;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

mod ambiguities;
pub use ambiguities::{n_pp_attachments, n_pp_objps, n_obj_frontings, n_verb_particles, n_coordinations};

mod comps;
pub use comps::{heads_and_deprels_equal, heads_equal, deprels_equal, postags_equal, check_deprel, check_deprels, check_postag, check_postags};

mod data_preps;
pub use data_preps::read_gng_data;