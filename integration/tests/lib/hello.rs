extern crate jni;
extern crate jnat;

use jni::{
  objects::{JClass, JString},
  sys::jstring,
  JNIEnv,
};

// use jnat::Env;

#[no_mangle]
pub extern "system" fn Java_Hello_hello(mut env: JNIEnv, _: JClass, name: JString) -> jstring {
  let name: String = env.get_string(&name).expect("Failed to get name").into();

  let output = env
    .new_string(format!("Hello, {}!", name))
    .unwrap();

  output.into_raw()
}
