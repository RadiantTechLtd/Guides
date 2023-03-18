use arctk::args;
use std::env::current_dir;
use std::fs::create_dir;
use std::path::PathBuf;

use phase::{run, Model, Parameters};

/// Entrypoint.
fn main() {
    // Parse command line arguments.
    args!(_bin_path: PathBuf, params_path: PathBuf);

    // Setup directories.
    let cwd = current_dir().expect("Failed to determine current working directory");
    let (in_dir, out_dir) = (cwd.join("input"), cwd.join("output"));
    init_dirs(&in_dir, &out_dir);

    // Read input parameters.
    let params =
        Parameters::load(&in_dir.join(&params_path)).expect("Failed to load parameters file");

    // Build model.
    let model = Model::new(&params);

    // Run the simulation.
    run(model);
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
