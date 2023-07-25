extern crate jnat;

use jnat::{
  env::Env,
  jni::{
    objects::{JClass, JObject},
    JNIEnv,
  },
  signature::{Signature, Type},
  value::{Value, Object},
};

#[no_mangle]
pub extern "system" fn Java_CallMethod_caller(env: JNIEnv, _: JClass, instance: JObject) {
  let mut env = env;
  let mut env = Env::new(&mut env);

  // Alternatively:
  // let mut instance = Object::new(&env, &instance);
  let mut instance = env.object(&instance);

  let s = env.string(" - Hello, world!").unwrap();

  instance
    .call_method(
      "callback",
      Signature::new(&[Type::Int, Type::Object("java/lang/String")], Type::Void),
      &[Value::Int(0), Value::Object(Object::new(&env, &s))],
    )
    .expect("Failed to call static method");
}
