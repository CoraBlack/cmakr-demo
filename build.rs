use cmakr;

fn main() {
    let rx = cmakr::Cmd::default()
        .set_path("./sdl")
        .set_binary_path("./build")
        .set_output_path("./target/debug")
        .spawn();

    let output = bindgen::Builder::default()
        .header("./sdl/include/SDL3/SDL.h")
        .clang_arg("-I./sdl/include/")
        .raw_line("#![allow(non_upper_case_globals)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("#![allow(dead_code)]")
        .generate_cstr(true)
        .generate_comments(true)
        .generate()
        .expect("Failed to generate rust binding for SDL3");

    output.write_to_file("./src/sdl.rs").expect("Failed to write sdl.rs");

    let result = rx.recv().unwrap();
    result.expect("Failed to build SDL");

    println!("cargo:rustc-link-search=native=./target/debug");
    println!("cargo:rustc-link-lib=dylib=SDL3");
}