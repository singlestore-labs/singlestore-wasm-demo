# Rust sum with WASM

This is the simplest example of building a User Defined Function with WASM.

## Usage

```bash
cargo rustc --target wasm32-wasi --release

# optional: see predict is exported
wasm2wat target/wasm32-wasi/release/sum.wasm > target/wasm32-wasi/release/sum.wat
cat target/wasm32-wasi/release/sum.wat | grep "wasm_add"

# run the library function
wasmtime --invoke wasm_add target/wasm32-wasi/release/sum.wasm 1 2
# prints: 3

# load this into s2
echo -n 'create function wasm_add(n1 int not null, n2 int not null) returns int not null as wasm  "' > target/wasm32-wasi/release/sum.sql
base64 -w0 target/wasm32-wasi/release/sum.wasm >> target/wasm32-wasi/release/sum.sql
echo -n '";' >> target/wasm32-wasi/release/sum.sql

memsql x_db < /home/bhayes/repos/oss/singlestore-wasm-demo/sum/target/wasm32-wasi/release/sum.sql

memsql
show functions;
select wasm_add(1, 2);
select wasm_add(1, -2);
```
