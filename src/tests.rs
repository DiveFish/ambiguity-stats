extern crate conllx;
use conllx::{Token, TokenBuilder, Features};

lazy_static!{
	pub static ref TOKEN_1: Token = TokenBuilder::new("Die")
		                .lemma("die")
		                .cpos("TEST")
		                .pos("ART")
		                .features(Features::from_string("test"))
		                .head(2)
		                .head_rel("TEST")
		                .p_head(3)
		                .p_head_rel("TEST")
		                .token();

	pub static ref TOKEN_2: Token = TokenBuilder::new("Haus")
		                .lemma("Haus")
		                .cpos("TEST")
		                .pos("NN")
		                .features(Features::from_string("test"))
		                .head(2)
		                .head_rel("TEST")
		                .p_head(3)
		                .p_head_rel("TEST")
		                .token();
}