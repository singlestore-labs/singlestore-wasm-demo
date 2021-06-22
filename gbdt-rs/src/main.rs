use gbdt::decision_tree::{Data, DataVec, PredVec};

#[macro_use]
extern crate lazy_static;

pub mod model;

fn main() {}

#[no_mangle]
pub extern "C" fn predict(label: f64, f1: f64, f2: f64, f3: f64, f4: f64) -> f64 {
    let mut dv: DataVec = Vec::new();
    dv.push(Data {
        label: label as f32,
        feature: vec![f1 as f32, f2 as f32, f3 as f32, f4 as f32],
        target: 0.0,
        weight: 1.0,
        residual: 0.0,
        initial_guess: 0.0,
    });

    let predicted: PredVec = model::COMPILED_MODEL.predict(&dv);
    return predicted[0] as f64;
}
