use std::{env, path::PathBuf};

fn main() {
    cc::Build::new()
        .file("src/uinput_wrapper.c")
        .compile("uinput_wrapper");

    let binding = bindgen::Builder::default()
        .header("./src/uinput_wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_macro_fallback()
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("uinput.rs");

    binding.write_to_file(out_path).unwrap()
}
