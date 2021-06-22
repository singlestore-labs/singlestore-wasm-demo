# Gradient boost decision tree library written in Safe Rust

MesaTEE GBDT-RS : a fast and secure GBDT library, supporting TEEs such as Intel SGX and ARM TrustZone

MesaTEE GBDT-RS provides the training and inference capabilities. And it can use the models trained by xgboost to do inference tasks.

<https://github.com/mesalock-linux/gbdt-rs/>

## Gradient boosting

Gradient boosting is a machine learning technique for regression, classification and other tasks,
which produces a prediction model in the form of an ensemble of weak prediction models, typically
decision trees.

This simple example supports most [XGBoost](https://xgboost.readthedocs.io/en/latest/) models.

TODO show how to convert.
There's a bit of hackery around types and assumptions of length. TODO.

## Usage

```bash
cargo rustc --target wasm32-wasi --release -- -Z wasi-exec-model=reactor

# optional: see predict is exported
wasm2wat target/wasm32-wasi/release/gbdt.wasm > target/wasm32-wasi/release/gbdt.wat
cat target/wasm32-wasi/release/gbdt.wat | grep "predict"

# run the library function
wasmtime --invoke predict target/wasm32-wasi/release/gbdt.wasm 1 2 3 4 5

# invoke is a little hokey, test with python instead:
python3 py/test.py

# load this into s2
echo -n 'create function predict(f1 float not null, f2 float not null, f3 float not null, f4 float not null, f5 float not null) returns float not null as wasm  "' > target/wasm32-wasi/release/gbdt.sql
base64 -w0 target/wasm32-wasi/release/gbdt.wasm >> target/wasm32-wasi/release/gbdt.sql
echo -n '";' >> target/wasm32-wasi/release/gbdt.sql

memsql x_db < /home/bhayes/repos/oss/singlestore-wasm-demo/gbdt-rs/target/wasm32-wasi/release/gbdt.sql

memsql
show functions;
```
