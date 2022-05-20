extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn find_vpp_lib_dir() -> String {
    /*
     * In the future there's more cleverness possibly to be added.
     * For now this will do.
     */
    let path = "/usr/lib/x86_64-linux-gnu/".to_string();
    path
}

fn git_version() -> String {
    let describe_output = Command::new("git")
        .arg("describe")
        .arg("--all")
        .arg("--long")
        .output()
        .unwrap();

    let mut describe = String::from_utf8_lossy(&describe_output.stdout).to_string();
    describe.pop();
    describe
}

fn main() {
    let dst = cmake::build("libmemif");
    println!("cargo:rustc-link-search=native={}", dst.display());
    // println!("cargo:rustc-link-lib=static=libmemif");
    // println!("cargo:rustc-link-lib=static=libmemif");

    println!("cargo:rustc-env=GIT_VERSION=version {}", &git_version());
    println!(
        "cargo:warning=libmemif directory: {}",
        &dst.to_str().unwrap()
    );

    let flags = format!("cargo:rustc-flags=-L{}/lib -lmemif", &dst.to_str().unwrap());

    // Tell cargo to tell rustc to link the VPP client library
    println!("{}", flags);

    let bindings = bindgen::Builder::default()
        .header("libmemif/src/libmemif.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_file_name = out_path.join("bindings.rs");
    bindings
        .write_to_file(out_file_name.clone())
        .expect("Couldn't write bindings!");

    Command::new("rustup")
        .args(&["run", "nightly", "rustfmt", out_file_name.to_str().unwrap()])
        .status()
        .unwrap();
}
