use arctk::{args, rt::Side::Outside};
use rand::rngs::ThreadRng;
use std::env::current_dir;
use std::fs::create_dir;
use std::path::PathBuf;

use phase::{run, Data, Model, Neutron, Parameters};

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
    println!("Total samples >> {}", data.total);
    println!("Escaped grid  >> {}", data.escaped);
    println!("Absorbed      >> {}", data.absorbed);
    println!("Scatters      >> {}", data.scatters.sum());
    data.save(&model.colour_map, &out_dir);
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

/// Inject a neutron into the model
fn my_engine(_i: usize, rng: &mut ThreadRng, model: &Model, data: &mut Data) {
    // Generate a random neutron.
    let mut neutron = model.generate_neutron(rng);

    // Inject the neutron into the model.
    let dist_side = model.grid.boundary.dist_side(&neutron.ray);
    if let Some((dist, side)) = dist_side {
        if !side.is_inside() {
            neutron.travel(dist + model.bump_dist);
        }
    } else {
        panic!("Failed to inject neutron into the grid.")
    }

    // Sample the model.
    sample(neutron, rng, model, data);

    // Sample complete.
    data.total += 1;
}

/// Sample the model.
/// # Parameters
/// * `i`: Current (unique) neutron index.
/// * `rng`: Random number generator.
/// * `model`: Complete information about the environment.
/// * `data`: Mutable reference to cumulative output data to store the results.
fn sample(mut neutron: Neutron, rng: &mut ThreadRng, model: &Model, data: &mut Data) {
    while let Some(index) = model.grid.voxel_index(&neutron.ray.pos) {
        let voxel = model.grid.generate_voxel(index);
        loop {
            if let Some(voxel_dist) = voxel.dist(&neutron.ray) {
                let r = rand::random::<f64>();
                let scatter_dist = -(r.ln()) / model.interaction_coeff;

                if scatter_dist < voxel_dist {
                    // Scattering event.
                    neutron.travel(scatter_dist);
                    neutron.scatter(rng);
                    neutron.weight *= model.albedo;
                    data.scatters[index] += neutron.weight;

                    if neutron.weight < model.min_weight {
                        // Neutron has been absorbed.
                        data.absorbed += 1;
                        return;
                    }
                } else {
                    neutron.travel(voxel_dist + model.bump_dist);
                    break;
                }
            } else {
                println!(
                    "[WARN!] Neutron escaped the grid at: {}\t{}\t{}",
                    neutron.ray.pos.x, neutron.ray.pos.y, neutron.ray.pos.z
                );
            }
        }
    }

    // Neutron escaped the grid.
    data.escaped += 1;
}
