use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // let out_dir = env::var("OUT_DIR").unwrap();
    // println!("cargo:rustc-link-search=native=/usr/include/arch-linux-gnu/c++/11/bits/");
    // println!("cargo:rustc-env=CXX=/usr/bin/clang");
    // println!("cargo:rustc-link-lib=font.o");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_dir.display());

    let out_asm = {
        let mut path = out_dir.clone();
        path.push("asm.o");
        path
    };
    Command::new("nasm")
        .args(["-f", "elf64", "-o", out_asm.to_str().unwrap()])
        .arg("asmfunc.asm")
        .status()
        .expect("asm.sが見つかりません。");
    Command::new("ar")
        .args(["crus", "libasm.a", "asm.o"])
        .current_dir(&out_dir)
        .status()
        .unwrap();
    println!("cargo:rustc-link-lib=static=asm");
    cc::Build::new()
        .cpp(true)
        .static_flag(true)
        .object("hankaku.o")
        .cpp_link_stdlib(None)
        // .include("include/arch-linux-gnu/c++/11/bits/")
        .file("font.cpp")
        .compile("font");
}
