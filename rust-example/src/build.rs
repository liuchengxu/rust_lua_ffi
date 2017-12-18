extern crate lua_c_ffi_marshalling;

use std::env;

fn main() {
    let rust_output = ::std::path::Path::new(&env::var("OUT_DIR").unwrap()).join("ffi.rs");

    let output = lua_c_ffi_marshalling::generate(
        &env::current_dir().unwrap().as_path().join("src/ffi.rs"), "rust_example");

    use std::io::Write;
    std::fs::File::create(rust_output.clone()).unwrap().write_all(output.as_bytes()).unwrap();

    assert!(rust_output.exists());
}
