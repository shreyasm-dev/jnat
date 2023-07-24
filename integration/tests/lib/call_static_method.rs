extern crate jnat;
extern crate jni;

pub fn jstring_to_string(env: &mut JNIEnv, jstring: &JString) -> String {
  env.get_string(jstring).unwrap().into()
}

use jni::{
  objects::{JClass, JObject, JString, JValue},
  sys::jstring,
  JNIEnv,
};

use jnat::{
  env::{Class, Env},
  signature::{Signature, Type},
};

#[no_mangle]
pub extern "system" fn Java_CallStaticMethod_caller(
  env: JNIEnv,
  _: JClass,
) {
  let mut env = env;
  let mut env = Env::new(&mut env);

  // Alternative (remember to rename the second parameter of Java_CallStaticMethod_caller to `class`):
  // let mut class = Class::new(&mut env, class);
  let mut class = env.class("CallStaticMethod").expect("Failed to find class");

  let res = class
    .call_static_method("callback", Signature::new(vec![], Type::Void), &[])
    .expect("Failed to call static method");
}
