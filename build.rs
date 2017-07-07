extern crate bindgen;
extern crate walkdir;

use bindgen::builder;
use walkdir::WalkDir;

use std::path::Path;
use std::process::Command;
use std::ffi::OsStr;

const SIMAVR_INCLUDE_DIR: &'static str = "simavr/simavr/sim";
const SIMAVR_HEADER_EXT: &'static str = "h";
const SIMAVR_ARCHIVE_NAME: &'static str = "libsimavr.a";
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

    compile_simavr()
}

fn compile_simavr() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let simavr_dir = manifest_dir.join("simavr");

    // note that there are a number of downsides to this approach, the comments
    // below detail how to improve the portability of these commands.
    let mut cmd = Command::new("make");
    cmd.current_dir(&simavr_dir);

    if cfg!(trace) {
        cmd.env("CFLAGS", "-DCONFIG_SIMAVR_TRACE=1");
    }

    cmd.status()
       .expect("failed to compile simavr");

    let archive_file = WalkDir::new(&simavr_dir)
                          .into_iter()
                          .map(|e| e.unwrap().path().to_owned())
                          .find(|path| path.file_name() == Some(OsStr::new(SIMAVR_ARCHIVE_NAME)));

    if let Some(archive_file) = archive_file {
        let parent = simavr_dir.join(archive_file.parent().unwrap());
        let lib_name = archive_file.file_stem().unwrap().to_str().unwrap().replacen("lib", "", 1);

        println!("cargo:rustc-link-search={}", parent.display());
        println!("cargo:rustc-link-lib={}", lib_name);
    } else {
        panic!("could not find simavr archive file");
    }
}

