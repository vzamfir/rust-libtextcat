fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let pwd = std::env::var("CARGO_MANIFEST_DIR").unwrap();

   /* // store senna path
    std::process::Command::new("sh")
        .args(&["set_senna_path.sh", &format!("{}/senna/", pwd)])
        .status().unwrap();*/

    // build c stuff
    std::process::Command::new("make")
        .args(&["static_lib", "-f", "makefile", &format!("OUT_DIR={}", out_dir), "LIBTEXTCAT_DIR=libtextcat-2.2/src"])
        .status().unwrap();

    // `make clean` (only removes *.o files)
    std::process::Command::new("make")
        .args(&["clean", "-f", "makefile", &format!("OUT_DIR={}", out_dir), "LIBTEXTCAT_DIR=libtextcat-2.2/src"])
        .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=language_detection");
}
