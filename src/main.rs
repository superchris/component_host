use anyhow::Context;
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview2::{Table, WasiCtxBuilder};

wasmtime::component::bindgen!({
    path: "todo-list.wit",
    world: "todo-list"
});

fn main() -> anyhow::Result<()> {
    let mut config = Config::default();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let linker = Linker::new(&engine);

    let mut table = Table::new();

    // Add the command world (aka WASI CLI) to the linker
    // command::add_to_linker(&mut linker).context("Failed to link command world")?;
    let wasi = WasiCtxBuilder::new()
        .build(&mut table);
    let mut store = Store::new(&engine, wasi);

    let component = Component::from_file(&engine, "./todo-list.wasm").context("Component file not found")?;

    let (instance, _) = TodoList::instantiate(&mut store, &component, &linker)
        .context("Failed to instantiate the world")?;

    println!("after instantiation, before execution");

    let result = instance
        .call_init(&mut store)
        .context("Failed to call init function")?;
    
    println!("Got {} and {}", result[0], result[1]);

    let new_result = instance
        .call_add_todo(&mut store, "more", &result[..])
        .context("Failed to call init function")?;
    
    println!("Got {} and {} and {}", new_result[0], new_result[1], new_result[2]);
    
    Ok(())
}
