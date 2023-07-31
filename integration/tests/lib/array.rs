extern crate jnat;

use jnat::{
  jnat_macros::jnat,
  jni::{objects::JClass, JNIEnv},
  Array, Env,
};
use std::fmt::Display;

jnat!(Array, method, (JNIEnv, JClass) -> ());

fn method(mut env: JNIEnv, _: JClass) {
  let mut env = Env::new(&mut env);

  let boolean_array = env.new_boolean_array(10);
  print_array(&boolean_array);
  print!("-");
  boolean_array.set(0, true).unwrap();
  print_array(&boolean_array);
  println!();

  let byte_array = env.new_byte_array(10);
  print_array(&byte_array);
  print!("-");
  byte_array.set(0, 1).unwrap();
  print_array(&byte_array);
  println!();

  let char_array = env.new_char_array(10);
  print_array(&char_array);
  print!("-");
  char_array.set(0, 'a').unwrap();
  print_array(&char_array);
  println!();

  let double_array = env.new_double_array(10);
  print_array(&double_array);
  print!("-");
  double_array.set(0, 0.1).unwrap();
  print_array(&double_array);
  println!();

  let float_array = env.new_float_array(10);
  print_array(&float_array);
  print!("-");
  float_array.set(0, 0.1).unwrap();
  print_array(&float_array);
  println!();

  let int_array = env.new_int_array(10);
  print_array(&int_array);
  print!("-");
  int_array.set(0, 1).unwrap();
  print_array(&int_array);
  println!();

  let long_array = env.new_long_array(10);
  print_array(&long_array);
  print!("-");
  long_array.set(0, 1).unwrap();
  print_array(&long_array);
  println!();

  let short_array = env.new_short_array(10);
  print_array(&short_array);
  print!("-");
  short_array.set(0, 1).unwrap();
  print_array(&short_array);
  println!();
}

fn print_array<'a, T: Display, J>(arr: &impl Array<'a, T, J>) {
  for i in 0..arr.length() {
    print!("{}", arr.get(i).unwrap());
  }
}
