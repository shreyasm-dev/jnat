extern crate pretty_env_logger;
#[macro_use]
extern crate log;

pub mod tests;

use std::{
  env::{self, current_exe, set_current_dir},
  fs::{read_dir, remove_file},
  path::Path,
  process::{exit, Command},
};

use crate::tests::IntegrationTest;

#[cfg(target_os = "macos")]
static DYLIB_EXTENSION: &str = "dylib";
#[cfg(target_os = "linux")]
static DYLIB_EXTENSION: &str = "so";
#[cfg(target_os = "windows")]
static DYLIB_EXTENSION: &str = "dll";

// TODO: Ensure this works correctly
fn get_dylib_name(lib: &str) -> String {
  format!("lib{}.{}", lib, DYLIB_EXTENSION)
}

fn setup() {
  println!("Remember to run cargo build before running this!");

  env::set_var("RUST_APP_LOG", "debug");
  pretty_env_logger::init_custom_env("RUST_APP_LOG");

  let mut dir = current_exe().expect("Failed to get current exe");
  dir.pop();
  dir.pop();
  dir.pop();
  dir.pop();
  dir.push("integration");
  dir.push("java");

  set_current_dir(dir).expect("Failed to set current dir");
}

// TODO: Is there anything that needs to be done here?
fn teardown() {}

// TODO: Better output instead of just logging
fn main() {
  setup();

  let mut libjnat: Option<String> = None;

  let paths = read_dir(Path::new("../../target/debug/deps")).expect("Failed to read directory");

  for path in paths {
    let path = path.expect("Failed to get path");
    let path = path.path();

    if path.is_file() {
      let path = path;
      let filename = path
        .file_name()
        .expect("Failed to get filename")
        .to_str()
        .expect("Failed to convert filename to string");

      if filename.starts_with("libjnat") && filename.ends_with("rlib") {
        libjnat = Some(
          path
            .to_str()
            .expect("Failed to convert path to string")
            .to_string(),
        );
      }
    }
  }

  let libjnat = libjnat.expect("Failed to find libjnat rlib");

  for t in inventory::iter::<IntegrationTest> {
    println!();
    info!("Running {}", t.name);

    // Compile the Rust file to a dynamic library
    let output = Command::new("rustc")
      .arg("--crate-type=cdylib")
      .arg("--out-dir")
      .arg("out")
      .arg("-L")
      .arg("dependency=../../target/debug/deps")
      .arg("--extern")
      .arg(format!("jnat={}", libjnat))
      .arg(format!("../tests/lib/{}.rs", t.lib))
      .output()
      .expect("Failed to spawn rustc");

    let code = output.status.code();

    if code != Some(0) {
      error!(
        "Failed to compile {} (rustc exited with code {:?})",
        t.lib, code
      );
      error!(
        "{}",
        String::from_utf8(output.stderr).expect("Failed to convert stderr to string")
      );
      exit(code.unwrap_or(1));
    }

    // Compile the Java class (outputs a class and a header file)
    let output = Command::new("javac")
      .arg("-h")
      .arg("out")
      .arg("-d")
      .arg("out")
      .arg(format!("{}.java", t.java_class))
      .output()
      .expect("Failed to spawn javac");

    let code = output.status.code();

    if code != Some(0) {
      error!(
        "Failed to compile {} (javac exited with code {:?})",
        t.java_class, code
      );
      error!(
        "{}",
        String::from_utf8(output.stderr).expect("Failed to convert stderr to string")
      );
      exit(code.unwrap_or(1));
    }

    // Run the previously-compiled Java class
    let output = Command::new("java")
      .arg("-Djava.library.path=out")
      .arg("-cp")
      .arg("out")
      .arg(t.java_class)
      .output()
      .expect("Failed to spawn java");

    let code = output.status.code();

    if code != Some(0) {
      error!(
        "Failed to run {} (java exited with code {:?})",
        t.java_class, code
      );
      error!(
        "{}",
        String::from_utf8(output.stderr).expect("Failed to convert stderr to string")
      );
      exit(code.unwrap_or(1));
    }

    let out = String::from_utf8(output.stdout).expect("Failed to convert stdout to string");

    if (t.test_fn)(out) {
      info!("{} passed", t.name);
    } else {
      error!("{} failed", t.name);
    }

    remove_file(Path::new("out").join(get_dylib_name(t.lib)))
      .expect(format!("Failed to remove {}", get_dylib_name(t.lib)).as_str());

    remove_file(Path::new("out").join(format!("{}.h", t.java_class)))
      .expect(format!("Failed to remove {}.h", t.java_class).as_str());

    remove_file(Path::new("out").join(format!("{}.class", t.java_class)))
      .expect(format!("Failed to remove {}.class", t.java_class).as_str());
  }

  teardown();
}
