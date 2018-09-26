use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    // Build flatbuffers compiler
    build_flatc();

    // Generate flatbuffers rs code
    build_flatbuf_rs_from_schema();
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

