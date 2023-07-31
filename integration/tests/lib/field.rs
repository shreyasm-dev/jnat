extern crate jnat;

use jnat::{
  jnat_macros::jnat,
  jni::{
    objects::{JClass, JObject},
    JNIEnv,
  },
  Env, Object, Type, Value,
};

jnat!(Field, method, (JNIEnv, JClass, JObject) -> ());

fn method(env: JNIEnv, _: JClass, instance: JObject) {
  let mut env = Env::new(&env);
  let instance = Object::new(&env, &instance);

  if let Value::Int(value) = env.get_value(instance.get_field("field", Type::Int).unwrap().borrow())
  {
    println!("{}", value);
  } else {
    unreachable!();
  }

  instance
    .set_field("field", Type::Int, Value::Int(1))
    .unwrap();

  if let Value::Int(value) = env.get_value(instance.get_field("field", Type::Int).unwrap().borrow())
  {
    println!("{}", value);
  } else {
    unreachable!();
  }
}
