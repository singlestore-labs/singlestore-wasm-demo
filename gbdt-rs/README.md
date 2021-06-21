# Gradient boost decision tree library written in Safe Rust

MesaTEE GBDT-RS : a fast and secure GBDT library, supporting TEEs such as Intel SGX and ARM TrustZone

<https://github.com/mesalock-linux/gbdt-rs/>

## Gradient boosting

Gradient boosting is a machine learning technique for regression, classification and other tasks,
which produces a prediction model in the form of an ensemble of weak prediction models, typically
decision trees.

This simple example supports most [XGBoost](https://xgboost.readthedocs.io/en/latest/) models.

TODO show how to convert.
There's a bit of hackery around types and assumptions of length. TODO.

## WASM

```bash
cargo build --target wasm32-wasi --release

# optional: see predict is exported
wasm2wat target/wasm32-wasi/release/gbdt.wasm > target/wasm32-wasi/release/gbdt.wat
cat target/wasm32-wasi/release/gbdt.wat | grep "predict.command_export"

# run the library function
wasmtime --invoke predict target/wasm32-wasi/release/gbdt.wasm 1 2 3 4 5

# invoke is a little hokey, test with python instead:
python3 py/test.py

# load this into s2
echo -n '"' > target/wasm32-wasi/release/gbdt.base64
cat target/wasm32-wasi/release/gbdt.wasm | base64 >> target/wasm32-wasi/release/gbdt.base64
echo -n '"' >> target/wasm32-wasi/release/gbdt.base64

# copy entire string
xclip -sel c < target/wasm32-wasi/release/gbdt.base64
```
