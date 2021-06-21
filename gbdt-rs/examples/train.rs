use gbdt::config::Config;
use gbdt::fitness::almost_equal_thrs;
use gbdt::gradient_boost::GBDT;
use gbdt::input::{load, InputFormat};

use gbdt::decision_tree::{Data, DataVec, PredVec, ValueType, VALUE_TYPE_UNKNOWN};

mod test;

fn train() {
    let train_file = "data/iris/train.txt";
    let mut cfg = Config::new();
    cfg.set_feature_size(4);
    cfg.set_max_depth(4);
    cfg.set_iterations(200);
    cfg.set_shrinkage(0.1);
    cfg.set_loss("LAD");
    cfg.set_debug(true);
    cfg.set_training_optimization_level(2);

    let mut input_format = InputFormat::csv_format();
    input_format.set_feature_size(4);
    input_format.set_label_index(4);
    let mut train_dv: DataVec =
        load(train_file, input_format).expect("failed to load training data");

    // train and save the model
    let mut gbdt = GBDT::new(&cfg);
    gbdt.fit(&mut train_dv);
    gbdt.save_model("gbdt.model")
        .expect("failed to save the model");

    // todo script
    // let mut file = File::create(filename)?;
    // let serialized = serde_json::to_string(self)?;
    // file.write_all(serialized.as_bytes())?;
    // const MODEL: &str = "
    //
}

fn load_test() -> DataVec {
    let mut label: ValueType = 0.0;
    let mut val: ValueType = 0.0;
    let mut input_format = InputFormat::csv_format();
    input_format.set_feature_size(4);
    input_format.set_label_index(4);
    let mut dv: DataVec = Vec::new();
    let test = std::str::from_utf8(test::TEST).unwrap();
    for line in test.lines() {
        let mut v: Vec<ValueType> = vec![VALUE_TYPE_UNKNOWN; input_format.feature_size];
        for token in line.split(input_format.delimeter) {
            if input_format.enable_unknown_value {
                v = line
                    .split(input_format.delimeter)
                    .map(|x| x.parse::<ValueType>().unwrap_or(VALUE_TYPE_UNKNOWN))
                    .collect();
            } else {
                v = line
                    .split(input_format.delimeter)
                    .map(|x| x.parse::<ValueType>().unwrap())
                    .collect();
            }
            dv.push(Data {
                label: v.swap_remove(input_format.label_idx),
                feature: v,
                target: 0.0,
                weight: 1.0,
                residual: 0.0,
                initial_guess: 0.0,
            })
        }
    }

    return dv;
}

fn main() {
    // train();

    // load test data
    // let test_file = "data/iris/test.txt";
    // let mut input_format = InputFormat::csv_format();
    // input_format.set_feature_size(4);
    // input_format.set_label_index(4);
    // let test_dv = load(test_file, input_format).expect("failed to load test data");

    // load the model and do inference
    // let model = GBDT::load_model("gbdt.model").expect("failed to load the model");
    // let model = serde_json::from_str(model::MODEL).unwrap();

    // if in memory:
    println!("loading model...");
    let model: GBDT = serde_json::from_slice(model::MODEL).unwrap();
    println!("model loaded");

    println!("loading tests...");
    let test_dv: DataVec = load_test();
    println!("test vec {}", test_dv.len());
    // println!("{:#?}", test_dv);

    let predicted: PredVec = model.predict(&test_dv);

    println!("print predictions...");
    assert_eq!(predicted.len(), test_dv.len());
    let mut correct = 0;
    let mut wrong = 0;
    for i in 0..predicted.len() {
        if almost_equal_thrs(test_dv[i].label, predicted[i], 0.0001) {
            correct += 1;
        } else {
            wrong += 1;
            println!("wrong: [{}]  {}  {}", i, test_dv[i].label, predicted[i]);
        };
    }

    println!("correct: {}", correct);
    println!("wrong:   {}", wrong);
}
