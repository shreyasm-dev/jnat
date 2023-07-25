extern crate jnat;

use jnat::{
  env::Env,
  jni::{
    objects::{JClass, JObject},
    JNIEnv,
  },
  signature::{Signature, Type},
  value::Value,
};

#[no_mangle]
pub extern "system" fn Java_CallMethod_caller(env: JNIEnv, _: JClass, instance: JObject) {
  let mut env = env;
  let mut env = Env::new(&mut env);

  // Alternatively:
  // let mut instance = Object::new(&mut env, &instance);
  let mut instance = env.object(&instance);

  instance
    .call_method(
      "callback",
      Signature::new(&[Type::Int], Type::Void),
      &[Value::Int(0)],
    )
    .expect("Failed to call static method");
}
