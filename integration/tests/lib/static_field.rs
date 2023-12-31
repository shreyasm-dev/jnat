extern crate jnat;

use jnat::{
  jnat_macros::jnat,
  jni::{objects::JClass, JNIEnv},
  Env, Type, Value,
};

jnat!(StaticField, method, (JNIEnv, JClass) -> ());

fn method(env: JNIEnv, _: JClass) {
  let mut env = Env::new(&env);
  let mut class = env.get_class("StaticField").unwrap();

  if let Value::Int(value) = env.get_value(
    class
      .get_static_field("staticField", Type::Int)
      .unwrap()
      .borrow(),
  ) {
    println!("{}", value);
  } else {
    unreachable!();
  }

  class
    .set_static_field("staticField", Type::Int, Value::Int(1))
    .unwrap();

  if let Value::Int(value) = env.get_value(
    class
      .get_static_field("staticField", Type::Int)
      .unwrap()
      .borrow(),
  ) {
    println!("{}", value);
  } else {
    unreachable!();
  }
}
