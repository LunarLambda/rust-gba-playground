use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    if cfg!(docs_rs) {
        return;
    }

    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("runtime");
    let build = PathBuf::from(env::var("OUT_DIR").unwrap()).join("build");

    let setup_output = Command::new("meson")
        .args([
            "setup",
            "--cross-file=meson/arm-none-eabi.ini",
            "--cross-file=meson/arm7tdmi.ini",
            "-Drt_lang=rust",
            build.to_str().unwrap(),
        ])
        .current_dir(&src)
        .output()
        .expect("sdk-seven/runtime: failed to setup");

    if !setup_output.status.success() {
        panic!("{}", String::from_utf8_lossy(&setup_output.stderr));
    }

    let build_output = Command::new("ninja")
        .current_dir(&build)
        .output()
        .expect("sdk-seven/runtime: failed to build");

    if !build_output.status.success() {
        panic!("{}", String::from_utf8_lossy(&build_output.stderr));
    }

    let minrt_lib = src.join("rt/lib/ldscripts");
    let minrt_build = build.join("rt");

    println!(
        "cargo:link-search={}:{}",
        minrt_lib.display(),
        minrt_build.display()
    );
    println!("cargo:link-args=-Trom.mem:-Tgba.x");
    println!("cargo:link-libs=minrt_rom");
}
