use arctk::util::ProgressBar;
use rand::thread_rng;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::sync::{Arc, Mutex};

use crate::{data::Data, engine::Engine, model::Model};

/// Run a multi-threaded simulation.
/// # Errors
/// if the progress bar can not be locked.
#[allow(clippy::expect_used)]
#[inline]
pub fn multi_thread<'a>(
    mut data: Data,
    num_neutrons: usize,
    block_size: usize,
    model: &'a Model,
    engine: Engine,
) -> Data {
    let pb = ProgressBar::new("Simulating", num_neutrons);
    let pb = Arc::new(Mutex::new(pb));

    let num_threads = num_cpus::get().min(model.num_threads);
    let threads: Vec<_> = (0..num_threads).collect();
    let mut out: Vec<_> = threads
        .par_iter()
        .map(|_id| {
            worker(
                block_size,
                model,
                Data::new(model.grid.num_voxels),
                &pb,
                engine,
            )
        })
        .collect();
    pb.lock()
        .expect("Failed to lock progress bar.")
        .finish_with_message("Simulation complete.");

    // let mut data = out.pop().expect("No data received.");
    while let Some(o) = out.pop() {
        data += &o;
    }

    data
}

/// Run a single-threaded simulation.
/// # Errors
/// if the progress bar can not be locked.
#[allow(clippy::expect_used)]
#[inline]
fn worker<'a>(
    block_size: usize,
    model: &'a Model,
    mut data: Data,
    pb: &Arc<Mutex<ProgressBar>>,
    engine: Engine,
) -> Data {
    let mut rng = thread_rng();

    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(block_size);
        std::mem::drop(pb);
        b
    } {
        for n in start..end {
            engine(n, &mut rng, &model, &mut data);
        }
    }

    data
}
