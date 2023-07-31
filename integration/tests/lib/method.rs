extern crate jnat;

use jnat::{
  jnat_macros::jnat,
  jni::{
    objects::{JClass, JObject},
    JNIEnv,
  },
  Env, Object, Signature, Type, Value,
};

jnat!(Method, caller, (JNIEnv, JClass, JObject) -> ());

fn caller(env: JNIEnv, _: JClass, instance: JObject) {
  let mut env = Env::new(&env);
  let mut instance = Object::new(&env, &instance);

  let s = env.new_string(" - Hello, world!").unwrap();

  instance
    .call_method(
      "callback",
      Signature::new(&[Type::Int, Type::Object("java/lang/String")], Type::Void),
      &[Value::Int(0), Value::Object(Object::new(&env, &s))],
    )
    .expect("Failed to call static method");
}
