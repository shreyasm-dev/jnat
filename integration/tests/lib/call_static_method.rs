extern crate jnat;

use jnat::{
  jnat_macros::jnat,
  jni::{objects::JClass, JNIEnv},
  Env, Object, Signature, Type, Value,
};

jnat!(CallStaticMethod, caller, (JNIEnv, JClass) -> ());

fn caller(mut env: JNIEnv, _: JClass) {
  let mut env = Env::new(&mut env);

  // Alternatively (remember to rename the second parameter of caller to `class`):
  // let mut class = Class::new(&env, class);
  let mut class = env.class("CallStaticMethod").expect("Failed to find class");

  let s = env.string(" - Hello, world!").unwrap();

  class
    .call_static_method(
      "callback",
      Signature::new(&[Type::Int, Type::Object("java/lang/String")], Type::Void),
      &[Value::Int(0), Value::Object(Object::new(&env, &s))],
    )
    .expect("Failed to call static method");
}
