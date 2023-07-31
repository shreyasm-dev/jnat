extern crate jnat;

use jnat::{
  jnat_macros::jnat,
  jni::{objects::JClass, JNIEnv},
  Array, BooleanArray, Env,
};

jnat!(BooleanArray, method, (JNIEnv, JClass) -> ());

fn method(mut env: JNIEnv, _: JClass) {
  let mut env = Env::new(&mut env);
  let array = env.new_boolean_array(10);

  print_array(&array);
  print!("-");

  array.set(0, true).unwrap();
  print_array(&array);
  println!();
}

fn print_array(arr: &BooleanArray) {
  for i in 0..arr.length() {
    print!("{:?}", arr.get(i).unwrap() as u8);
  }
}
