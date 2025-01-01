from wasmtime import Store, Module, Instance, Func, FuncType

# データや関数を管理するStoreを作成
store = Store()
# 
module = Module.from_file(store.engine, './main_bg.wasm')

# 必須インポートを定義
def init_externref_table():
    pass
func_type = FuncType([], [])
init_func = Func(store, func_type, init_externref_table)

instance = Instance(store, module, [init_func])
add = instance.exports(store)['add']

print(add(store, 1, 2))