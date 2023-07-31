extern crate jnat;

use jnat::{
  jnat_macros::jnat,
  jni::{
    objects::{JClass, JString},
    JNIEnv,
  },
  Array, Env, ObjectArray,
};

jnat!(ObjectArray, method, (JNIEnv, JClass) -> ());

fn method(env: JNIEnv, _: JClass) {
  let mut env = Env::new(&env);

  let object_array = env.new_object_array(10, "java/lang/String");
  print_array(env, &object_array);
  print!("-");

  for i in 0..object_array.length() {
    object_array
      .set(i, env.new_string("test").unwrap())
      .unwrap();
  }

  object_array
    .set(0, env.new_string("hello").unwrap())
    .unwrap();

  print_array(env, &object_array);
  println!();
}

fn print_array<'a>(env: Env, arr: &ObjectArray<'a>) {
  for i in 0..arr.length() {
    match env.get_string(JString::from(arr.get(i).unwrap())) {
      Ok(s) => print!("{}", s),
      Err(_) => print!("null"),
    }
  }
}
