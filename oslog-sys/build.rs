//
//  build.rs
//  oslog-sys
//
//  Created by Søren Mortensen on 28/07/2018.
//  Copyright © 2018 Søren Mortensen. All rights reserved.
//

extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let liboslog_dir = "liboslog";

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir_argument = format!("OUT_DIR={}", out_dir);
    let obj_dir_argument = format!("OBJ_DIR={}/obj", out_dir);

    Command::new("make")
        .args(&[&*out_dir_argument, &*obj_dir_argument])
        .current_dir(PathBuf::from(liboslog_dir))
        .status()
        .unwrap();

    println!("cargo:rustc-link-lib=static=oslog");
    println!("cargo:rustc-link-search=native={}", out_dir);

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("include/oslog-sys.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("os_log.rs"))
        .expect("Couldn't write bindings!");
}
