
fn main() {
    // let out_dir = env::var("OUT_DIR").unwrap();
    // println!("cargo:rustc-link-search=native=/usr/include/x86_64-linux-gnu/c++/11/bits/");
    // println!("cargo:rustc-env=CXX=/usr/bin/clang");
    // println!("cargo:rustc-link-lib=font.o");
    cc::Build::new()
        .cpp(true)
        .static_flag(true)
        .object("hankaku.o")
        .cpp_link_stdlib(None)
        // .include("include/x86_64-linux-gnu/c++/11/bits/")
        .file("font.cpp")
        .compile("font");
}
