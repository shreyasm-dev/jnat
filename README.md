# jnat

A wrapper around the [jni](https://crates.io/crates/jni) crate

View the [documentation](https://docs.rs/jnat)

## Example

Create a new Cargo lib project with `cargo new --lib mylib` and add the following to `Cargo.toml`:

```toml
[dependencies]
jnat = [latest version]

[lib]
crate-type = ["cdylib"]
```

Add the following to `src/lib.rs`:

```rust
use jnat::{
  jnat_macros::jnat,
  jni::{objects::JClass, JNIEnv}, // jni crate, re-exported by jnat
  Class,
  Env,
  Signature,
  Type,
};

#[jnat(HelloWorld)]
fn caller(env: JNIEnv, class: JClass) {
  let mut env = env;
  let mut env = Env::new(&mut env);
  let mut class = Class::new(&mut env, class);

  class
    .call_static_method("hello", Signature::new(&[], Type::Void), &[])
    .unwrap();
}
```

Then, run `cargo build`. Create a new file called `HelloWorld.java` and add the following:

```java
public class HelloWorld {
  private static native void caller();

  static {
    System.loadLibrary("mylib");
  }

  public static void main(String[] args) {
    HelloWorld.caller();
  }

  public static void hello() {
    System.out.println("Hello, world!");
  }
}
```

Compile the java file with `javac -h . HelloWorld.java`. Then, run `java -Djava.library.path=path/to/target/debug HelloWorld`. You should see `Hello, world!` printed to the console.

## Notes

- Jnat re-exports jni by default. If you want to use a different version of jni, you can disable either the default features or the `jni` feature.
- Jnat exports a macro, `jnat::jnat_macros::jnat` (seen in the example above), which is used to generate the `Java_HelloWorld_caller` function. This macro can be disabled by disabling either the default features or the `jni-macros` feature. Note that the macro keeps the original function to prevent unintuitive behavior (so you can, in your Rust code, call just `example()` instead of `Java_org_example_Class_example()` while still allowing Java to call it).
