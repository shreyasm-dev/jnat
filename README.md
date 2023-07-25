# jnat

A wrapper around the [jni](https://crates.io/crates/jni) crate

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
  env::{Class, Env},
  jni::{objects::JClass, JNIEnv}, // jni crate, re-exported by jnat
  signature::{Signature, Type},
};

#[no_mangle]
pub extern "system" fn Java_HelloWorld_caller(env: JNIEnv, class: JClass) {
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
