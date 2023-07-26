extern crate jnat;

use jnat::{
  env::Env,
  jnat_macros::jnat,
  jni::{
    objects::{JClass, JObject},
    JNIEnv,
  },
  signature::{Signature, Type},
  value::{Object, Value},
};

#[jnat(CallMethod)]
fn caller(env: JNIEnv, _: JClass, instance: JObject) {
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
