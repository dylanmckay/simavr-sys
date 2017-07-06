extern crate bindgen;
extern crate walkdir;

use bindgen::builder;
use walkdir::WalkDir;

const SIMAVR_INCLUDE_DIR: &'static str = "simavr/simavr/sim";
const SIMAVR_HEADER_EXT: &'static str = "h";
const BINDINGS_DEST: &'static str = "src/bindings.rs";

fn main() {
    println!("cargo:rerun-if-changed={}", BINDINGS_DEST);

    // Recurse through the simavr submodule and find all header files.
    let headers: Result<Vec<String>, _> =
        WalkDir::new(SIMAVR_INCLUDE_DIR)
            .into_iter()
            .filter(|entry| entry.as_ref().map(|e| e.path().extension().map(|e| e.to_str().unwrap()) == Some(SIMAVR_HEADER_EXT)).unwrap_or(false))
            .map(|entry| { entry.map(|e| e.path().to_str().unwrap().to_owned())})
            .collect();

    let headers = headers.expect("could not find bindgen headers in submodule");

    let builder = headers.into_iter().fold(builder(), |cur,header| cur.header(header));
    // Configure and generate bindings.
    let bindings = builder.generate()
                          .expect("could not generate bindings");

    // Write the generated bindings to an output file.
    bindings.write_to_file(BINDINGS_DEST)
        .expect("could not write bindings to file");
}

