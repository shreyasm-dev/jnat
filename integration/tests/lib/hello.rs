extern crate jnat;

use jnat::{
  env::Env,
  jni::{
    objects::{JClass, JString},
    sys::jstring,
    JNIEnv,
  },
};

#[no_mangle]
pub extern "system" fn Java_Hello_hello(mut env: JNIEnv, _: JClass, name: JString) -> jstring {
  let mut env = Env::new(&env);

  let message: String = env.get_string(&name).expect("Failed to get name").into();
  let message = format!("Hello, {}!", message);

  let output = env.string(&message).unwrap();

  output.into_raw()
}
