extern crate conllx;
extern crate ambiguity_stats;

use ambiguity_stats::pp_attachment;
use ambiguity_stats::read_gng_data;

static GOLD_DATA: &str = "data/validation.conll";
static NONGOLD_DATA: &str = "data/validation-nongold.conll";

fn main() {
	let (golddata, nongolddata) = read_gng_data(GOLD_DATA, NONGOLD_DATA);

    let mut idx = 0;
    let mut errors = 0;
	for sent in &golddata {
        errors += pp_attachment(&sent, &nongolddata.get(idx).expect("No Token"));
        idx += 1;
    }

    println!("Number of errors: {:?}", errors);
	println!("Done with analysis");
}