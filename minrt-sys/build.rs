/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use std::path::PathBuf;
use std::env;

fn main() {
    // we skip assembling the runtime for docs.rs builds.
    if !cfg!(docs_rs) {
      let root_dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), ""));
      let sdk_seven_dir = root_dir.join(PathBuf::from("vendor/sdk-seven"));
      let build_path = PathBuf::from(env::var("OUT_DIR").unwrap());
      let build_path = build_path.join("build");
      let libseven_build_dir = build_path.to_str().unwrap();

      let setup_output = std::process::Command::new("meson")
        .args([
          "setup",
          "--cross-file=cross/arm-none-eabi.txt",
          "--cross-file=cross/arm7tdmi.txt",
          "-Dminrt_lang=rust",
          libseven_build_dir
        ])
        .current_dir(sdk_seven_dir.to_str().unwrap())
        .output()
        .expect("libseven: failed to setup");

      if !setup_output.status.success() {
        panic!("{}", String::from_utf8_lossy(&setup_output.stderr));
      }

      let build_output = std::process::Command::new("ninja")
        .arg("-C")
        .arg("build")
        .current_dir(PathBuf::from(env::var("OUT_DIR").unwrap()))
        .output()
        .expect("libseven: failed to build");

      if !build_output.status.success() {
        panic!("{}", String::from_utf8_lossy(&build_output.stderr));
      }

      let minrt_lib = sdk_seven_dir.join("minrt/lib/ldscripts");
      let minrt_libs = minrt_lib.to_str().unwrap();
    let minrt_build = build_path.join("minrt");
      let minrt_builds = minrt_build.to_str().unwrap();
      // Downstream stuff
      println!("cargo:link-search={}:{}", minrt_libs, minrt_builds);
      println!("cargo:link-args=-Trom.mem:-Tgba.x");
    }
}
