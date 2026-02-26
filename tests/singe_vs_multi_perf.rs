use fractale::arguments::Args;
use fractale::multi_core::multi_core_generate;
use fractale::single_core::single_core_generate;
use std::time::Instant;

fn run_single_bench(args: &Args) -> u128 {
    let start = Instant::now();

    let _img = single_core_generate(args);

    start.elapsed().as_millis()
}

fn run_multi_bench(args: &Args) -> u128 {
    let start = Instant::now();

    let _img = multi_core_generate(args);

    start.elapsed().as_millis()
}

#[test]
fn compare_performances() {
    let args = Args {
        width: 1000,
        height: 800,
        center_x: -0.75,
        center_y: 0.0,
        zoom: 1.0,
        max_iter: 2000,
        output: String::from("test_output"),
    };

    let time_single = run_single_bench(&args);
    println!("Single-core: {} ms", time_single);

    let time_multi = run_multi_bench(&args);
    println!("Multi-core:  {} ms", time_multi);

    let speedup = time_single as f64 / time_multi as f64;
    println!("Speedup:     {:.2}x", speedup);

    assert!(speedup > 1.0, "Multi-core should be faster!");
}
