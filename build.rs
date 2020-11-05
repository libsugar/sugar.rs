use std::env;

#[cfg(feature = "code_gen")]
mod code_gen;
#[cfg(feature = "code_gen")]
use code_gen::*;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    #[cfg(feature = "code_gen")]
    code_gen(out_dir);

    println!("cargo:rerun-if-changed=build.rs");
}
