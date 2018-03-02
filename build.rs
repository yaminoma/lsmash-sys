extern crate bindgen;

use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

fn format_write(builder: bindgen::Builder, output: &str) {
    let s = builder
        .generate()
        .unwrap()
        .to_string()
        .replace("/**", "/*")
        .replace("/*!", "/*");

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(output)
        .unwrap();

    let _ = file.write(s.as_bytes());
}

fn common_builder() -> bindgen::Builder {
    bindgen::builder()
        .raw_line("#![allow(dead_code)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("#![allow(non_upper_case_globals)]")
}

fn find_dir(default: &'static str, env_key: &'static str) -> PathBuf {
    match env::var_os(env_key) {
        Some(val) => PathBuf::from(&val),
        _ => PathBuf::from(default),
    }
}

fn main() {
<<<<<<< HEAD
    let root_include = find_dir("l-smash", "ROOT_INCLUDE_PATH");
    let codecs_include = find_dir("l-smash/codecs", "CODECS_INCLUDE_PATH");
    let lsmash_include = find_dir("l-smash/l-smash", "CORE_INCLUDE_PATH");
    let importer_include = find_dir("l-smash/importer", "IMPORTER_INCLUDE_PATH");

    //println!("cargo:rustc-link-lib=dylib={}", "lsmash");
=======
    let root_include = find_dir("lsmash", "ROOT_INCLUDE_PATH");
    let codecs_include = find_dir("lsmash/codecs", "CODECS_INCLUDE_PATH");
    let lsmash_include = find_dir("lsmash/lsmash", "CORE_INCLUDE_PATH");
    let importer_include = find_dir("lsmash/importer", "IMPORTER_INCLUDE_PATH");

    // println!("cargo:rustc-link-lib=dylib={}", "lsmash");
>>>>>>> a6f6979... first commit

    let builder = common_builder()
        .clang_arg(format!("-I{}", root_include.to_string_lossy()))
        .clang_arg(format!("-I{}", codecs_include.to_string_lossy()))
        .clang_arg(format!("-I{}", lsmash_include.to_string_lossy()))
        .clang_arg(format!("-I{}", importer_include.to_string_lossy()))
        .header(root_include.join("lsmash.h").to_string_lossy());

    // Manually fix the comment so rustdoc won't try to pick them
    format_write(builder, "src/lsmash.rs");
}
