use std::env;
use std::path::{PathBuf};
use std::process::{Command, Stdio};

fn run(cmd: &mut Command) {
  let res = cmd.stdout(Stdio::inherit())
               .stderr(Stdio::inherit())
               .status()
               .unwrap()
               .success();
  assert!(res);
}

fn main() {
  let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  let out_dir = env::var("OUT_DIR").unwrap();

  let mut quadpack_root = PathBuf::from(&manifest_dir);
  quadpack_root.push("slatec_quadpack");

  run(Command::new("make")
      .current_dir(&out_dir)
      .arg("-f").arg(&quadpack_root.join("Makefile"))
      .arg(&format!("SRC_PREFIX={}", quadpack_root.to_str().unwrap()))
  );

  println!("cargo:rustc-link-search=native={}", out_dir);
}
