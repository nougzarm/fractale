use clap::Parser;

use fractale::arguments::Args;
use fractale::multi_core::multi_core_generate;

fn main() {
    let args = Args::parse();

    let img = multi_core_generate(&args);

    img.save(&args.output).expect("Saving failure");
}
