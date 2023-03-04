use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native=/usr/local/lib");

    cc::Build::new()
        .cpp(true)
        .warnings(true)
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-v")
        .flag("-g")
        .flag("-std=c++17")
        .file("font.cpp")

        .compile("font");
}
