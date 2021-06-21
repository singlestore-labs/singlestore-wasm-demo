use gbdt::decision_tree::{Data, DataVec, PredVec};
use gbdt::gradient_boost::GBDT;

pub mod model;

#[no_mangle]
pub extern "C" fn predict(label: f32, f1: f32, f2: f32, f3: f32, f4: f32) -> f32 {
    let mut dv: DataVec = Vec::new();
    dv.push(Data {
        label: label,
        feature: vec![f1, f2, f3, f4],
        target: 0.0,
        weight: 1.0,
        residual: 0.0,
        initial_guess: 0.0,
    });

    println!("loading model...");
    let model: GBDT = serde_json::from_slice(model::MODEL).unwrap();
    println!("model loaded");

    let predicted: PredVec = model.predict(&dv);
    return predicted[0];
}
