use jni::{
  errors::Error,
  objects::{JClass, JObject, JValueGen, JValueOwned},
  sys::JNINativeInterface_,
  JNIEnv,
};

use crate::{
  signature::Signature,
  value::{Object, Value},
};

#[derive(Clone, Copy)]
pub struct Env<'a> {
  jni_env: &'a JNIEnv<'a>,
}

impl<'a> Env<'a> {
  pub fn new(jni_env: &'a JNIEnv<'a>) -> Env<'a> {
    Env { jni_env: jni_env }
  }

  pub fn get_native_interface(&self) -> *mut *const JNINativeInterface_ {
    self.jni_env.get_native_interface()
  }

  pub fn class(&'a self, name: &str) -> Result<Class<'a>, Error> {
    let mut jni_env = unsafe { JNIEnv::from_raw(self.get_native_interface()) }.unwrap();

    let class = jni_env.find_class(name);

    match class {
      Ok(class) => Ok(Class::new(self, class)),
      Err(e) => Err(e),
    }
  }

  pub fn object(&'a self, object: &'a JObject<'a>) -> Object<'a> {
    Object::new(self, object)
  }

  pub fn string(&'a self, string: &'a str) -> Result<JObject<'a>, Error> {
    let string = self.jni_env.new_string(string);

    match string {
      Ok(string) => Ok(JObject::from(string)),
      Err(e) => Err(e),
    }
  }
}

pub struct Class<'a> {
  env: &'a Env<'a>,
  class: JClass<'a>,
}

impl<'a> Class<'a> {
  pub fn new(env: &'a Env<'a>, class: JClass<'a>) -> Class<'a> {
    Class { env, class }
  }

  pub fn call_static_method(
    &self,
    name: &str,
    signature: Signature,
    args: &[Value],
  ) -> jni::errors::Result<JValueOwned<'_>> {
    let class = &self.class;
    let signature: String = signature.into();
    let mut jni_env = unsafe { JNIEnv::from_raw(self.env.get_native_interface()) }.unwrap();
    jni_env.call_static_method(
      class,
      name,
      signature,
      args
        .iter()
        .map(|o| o.to_jvaluegen())
        .collect::<Vec<JValueGen<&JObject>>>()
        .as_slice(),
    )
  }
}
