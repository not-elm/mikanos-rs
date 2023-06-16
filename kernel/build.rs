use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
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
}