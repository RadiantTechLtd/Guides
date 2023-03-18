use rand::rngs::ThreadRng;

use crate::{data::Data, model::Model};

pub type Engine = fn(usize, &mut ThreadRng, &Model, &mut Data);
