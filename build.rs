extern crate bindgen;

use bindgen::builder;

const BINDINGS_DEST: &'static str = "src/bindings.rs";

fn main() {
    // Configure and generate bindings.
    let bindings = builder().header("simavr/simavr/sim/sim_avr.h")
                            .generate()
                            .expect("could not generate bindings");

    // Write the generated bindings to an output file.
    bindings.write_to_file(BINDINGS_DEST)
        .expect("could not write bindings to file");
}

