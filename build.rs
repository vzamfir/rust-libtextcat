//! Compile libtextcat and language_detection.c
//! The actual compilation is managed by the make file `Makefile`

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let pwd = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // store libtextcat path
    std::process::Command::new("sh")
        .args(&["set_libtextcat_path.sh", &format!("{}/libtextcat-2.2/langclass/LM/", pwd)])
        .status().unwrap();

    // build c stuff
    std::process::Command::new("make")
        .args(&["static_lib", "-f", "Makefile", &format!("OUT_DIR={}", out_dir), "LIBTEXTCAT_DIR=libtextcat-2.2/src"])
        .status().unwrap();

    // `make clean` (only removes *.o files)
    std::process::Command::new("make")
        .args(&["clean", "-f", "Makefile", &format!("OUT_DIR={}", out_dir), "LIBTEXTCAT_DIR=libtextcat-2.2/src"])
        .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=language_detection");
}
