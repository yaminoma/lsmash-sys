extern crate bindgen;

use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=lsmash");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .blacklist_type("max_align_t")
        .generate()
        .expect("Unable to generate liblsmash bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").expect("Environment variable `OUT_DIR' is not defined"));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write liblsmash bindings");
}
