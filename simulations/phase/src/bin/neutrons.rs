use arctk::args;
use rand::rngs::ThreadRng;
use std::env::current_dir;
use std::fs::create_dir;
use std::path::PathBuf;

use phase::{run, Data, Model, Parameters};

/// Entrypoint.
/// # Parameters
/// * `_bin_path`: Path to the binary. Included by default.
/// * `params_path`: Path to the parameters file relative, to the "input/" directory.
fn main() {
    // Parse command line arguments.
    args!(_bin_path: PathBuf, params_path: PathBuf);

    // Setup directories.
    let cwd = current_dir().expect("Failed to determine current working directory");
    let (in_dir, out_dir) = (cwd.join("input"), cwd.join("output"));
    init_dirs(&in_dir, &out_dir);

    // Read input parameters.
    let params = Parameters::load(&in_dir.join(&params_path));

    // Build model.
    let model = Model::new(&params);

    // Run the simulation.
    let data = run::multi_thread(params.num_neutrons, params.block_size, &model, my_engine);
    println!("SUM >> {}", data.total);
}

/// Initialise the input and output directories.
fn init_dirs(input: &PathBuf, output: &PathBuf) {
    if !input.exists() {
        create_dir(&input).expect("Failed to create input directory");
    }

    if !output.exists() {
        create_dir(&output).expect("Failed to create output directory");
    }
}

/// Sample the model.
/// # Parameters
/// * `n`: Number of neutrons to simulate.
/// * `rng`: Random number generator.
/// * `model`: Complete information about the environment.
/// * `data`: Mutable reference to cumulative output data to store the results.
fn my_engine(_n: usize, _rng: &mut ThreadRng, _model: &Model, data: &mut Data) {
    data.total += 1;
}
