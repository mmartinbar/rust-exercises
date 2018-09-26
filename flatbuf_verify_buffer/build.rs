extern crate bindgen;
extern crate cc;

use std::process::Command;
use std::env;
use std::path::Path;

use bindgen::Builder;

fn main() {
    // Path to flatcc libraries
    println!("cargo:rustc-link-search=third_party/dvidelabs_flatcc/lib/");
    println!("cargo:rustc-link-lib=static=flatccrt_d");

    // Build flatbuffers rust compiler
    build_flatc();

    // Generate flatbuffers rs code
    build_flatbuf_rs_from_schema();

    // Build flatbuffers C compiler
    build_flatcc();

    // Generate flatbuffers C code
    build_flatbuf_c_from_schema();

    cc::Build::new()
        .file("src/messages_in_c/handshake_verifier.c")
        .include("src/messages_in_c/")
        .include("third_party/dvidelabs_flatcc/include/")
        .compile("libhandshake.a");

    // Generate bindings for flatbuffers C code
    generate_c_bindings();
}

fn build_flatc() {
    println!("* Building flatc compiler (it may take several minutes the first time)...");

    // Move to google flatbuffers source code directory
    let google_flatbuffers_dir = Path::new("third_party/google_flatbuffers");
    env::set_current_dir(google_flatbuffers_dir).unwrap();

    // Call cmake to generate the makefile
    Command::new("sh")
        .arg("-c")
        .arg("cmake -G 'Unix Makefiles' -DCMAKE_BUILD_TYPE=Release")
        .output()
        .expect("failed to execute process");

    // Call make to build flatc compiler
    Command::new("sh")
        .arg("-c")
        .arg("make")
        .output()
        .expect("failed to execute process");

    // Move back to root directory
    let main_dir = Path::new("../..");
    env::set_current_dir(main_dir).unwrap();

    println!("* Building flatc compiler... [DONE]")
}

fn build_flatbuf_rs_from_schema() {
    println!("* Compiling flatbuffer schemas to rs code...");

    // Call make to build flatc compiler
    Command::new("sh")
        .arg("-c")
        .arg("third_party/google_flatbuffers/flatc --rust -o src/messages schemas/*.fbs")
        .output()
        .expect("failed to execute process");

    println!("* Compiling flatbuffer schemas to rs code... [DONE]");
}

fn build_flatcc() {
    println!("* Building flatcc compiler (it may take several minutes the first time)...");

    // Move to dvidelabs flatcc source code directory
    let dvidelabs_flatcc= Path::new("third_party/dvidelabs_flatcc/scripts");
    env::set_current_dir(dvidelabs_flatcc).unwrap();

    // Call cmake to generate the makefile
    Command::new("sh")
        .arg("-c")
        .arg("./build.sh")
        .output()
        .expect("failed to execute process");

    // Move back to root directory
    let main_dir = Path::new("../../..");
    env::set_current_dir(main_dir).unwrap();

    println!("* Building flatcc compiler... [DONE]")
}

fn build_flatbuf_c_from_schema() {
    println!("* Compiling flatbuffer schemas to C code...");

    // Call make to build flatc compiler
    Command::new("sh")
        .arg("-c")
        .arg("third_party/dvidelabs_flatcc/bin/flatcc -a -o src/messages_in_c/ schemas/*.fbs")
        .output()
        .expect("failed to execute process");

    println!("* Compiling flatbuffer schemas to C code... [DONE]");
}

fn generate_c_bindings() {
    println!("* Generating C bindings...");

    // Get bindings to verifier header files
    let bindings = Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/messages_in_c/handshake_verifier.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the src/messages/bindings.rs file.
    let out_path = Path::new("src/messages/bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");

    println!("* Generating C bindings... [DONE]");
}
