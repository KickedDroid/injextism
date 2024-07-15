use extism::*;

fn main() {
    let url =
        Wasm::url("http://127.0.0.1:8080/injextism.wasm");
    let manifest = Manifest::new([url]).with_memory_max(400);
    let mut plugin = Plugin::new(&manifest, [], true).unwrap();
    let res = plugin
        .call::<&str, &str>("greet", "from WebAssembly!")
        .unwrap();
    println!("{}", res);
}
