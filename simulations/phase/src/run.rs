use crate::model::Model;

pub fn run(model: Model) -> i32 {
    println!("Interaction coefficient : {}", model.interaction_coeff);
    println!("Single scattering albedo: {}", model.albedo);
    return 123;
}
