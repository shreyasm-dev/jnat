extern crate pretty_env_logger;
#[macro_use]
extern crate log;

pub mod tests;

use std::{
  env::{self, current_exe, set_current_dir},
  fs::remove_file,
  path::Path,
  process::Command,
};

use crate::tests::IntegrationTest;

fn setup() {
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

// TODO: The dylib extension is harcoded, but it's macOS-specific (this code will not work on Windows or Linux)
// TODO: Better output instead of just logging
// TODO: Paths to JNI and Jnat rlibs are hardcoded, not sure if they change, look into it?
fn main() {
  setup();

  for t in inventory::iter::<IntegrationTest> {
    info!("Running {}", t.name);

    // Compile the Rust file to a dynamic library
    Command::new("rustc")
      .arg("--crate-type=cdylib")
      .arg("--out-dir")
      .arg("out")
      .arg("-L")
      .arg("dependency=../../target/debug/deps")
      .arg("--extern")
      .arg("jni=../../target/debug/deps/libjni-b23e366b6393286e.rlib")
      .arg("--extern")
      .arg("jnat=../../target/debug/deps/libjnat-618a78aa1aee40e4.rlib")
      .arg(format!("../tests/lib/{}.rs", t.lib))
      .output()
      .expect("Failed to spawn rustc");

    // Compile the Java class (outputs a class and a header file)
    Command::new("javac")
      .arg("-h")
      .arg("out")
      .arg("-d")
      .arg("out")
      .arg(format!("{}.java", t.java_class))
      .output()
      .expect("Failed to spawn javac");

    // Run the previously-compiled Java class
    let out = String::from_utf8(
      Command::new("java")
        .arg("-Djava.library.path=out")
        .arg("-cp")
        .arg("out")
        .arg(t.java_class)
        .output()
        .expect("Failed to spawn java")
        .stdout,
    )
    .expect("Failed to convert output to string");

    if (t.test_fn)(out) {
      info!("{} passed", t.name);
    } else {
      error!("{} failed", t.name);
    }

    remove_file(Path::new("out").join(format!("lib{}.dylib", t.lib)))
      .expect(format!("Failed to remove lib{}.dylib", t.lib).as_str());

    remove_file(Path::new("out").join(format!("{}.h", t.java_class)))
      .expect(format!("Failed to remove {}.h", t.java_class).as_str());

    remove_file(Path::new("out").join(format!("{}.class", t.java_class)))
      .expect(format!("Failed to remove {}.class", t.java_class).as_str());
  }

  teardown();
}
