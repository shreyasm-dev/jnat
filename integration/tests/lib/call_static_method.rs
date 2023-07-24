extern crate jnat;

use jnat::{
  env::Env,
  jni::{objects::JClass, JNIEnv},
  signature::{Signature, Type},
};

#[no_mangle]
pub extern "system" fn Java_CallStaticMethod_caller(env: JNIEnv, _: JClass) {
  let mut env = env;
  let mut env = Env::new(&mut env);

  // Alternatively (remember to rename the second parameter of Java_CallStaticMethod_caller to `class`):
  // let mut class = Class::new(&mut env, class);
  let mut class = env.class("CallStaticMethod").expect("Failed to find class");

  class
    .call_static_method("callback", Signature::new(vec![], Type::Void), &[])
    .expect("Failed to call static method");
}
