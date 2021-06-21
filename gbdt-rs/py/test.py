import wasmtime

from wasmtime import Engine, Store, Module, Instance, Linker, WasiConfig

engine = Engine()
linker = Linker(engine)
linker.define_wasi()

store = Store(engine)
wasi = WasiConfig()
wasi.inherit_stdout()
store.set_wasi(wasi)

# todo find abs path instead of assuming repo root
module = Module.from_file(store.engine, 'target/wasm32-wasi/release/gbdt.wasm')

instance = linker.instantiate(store, module)
predict = instance.exports(store)["predict"]
prediction = predict(store, 1.0, 2.0, 3.0, 4.0, 5.0)
print("prediction: %d" % prediction)
