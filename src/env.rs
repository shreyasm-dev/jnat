use jni::{
  errors::Error,
  objects::{JClass, JObject, JValueGen, JValueOwned},
  JNIEnv,
};

use crate::signature::Signature;

pub struct Env<'a> {
  jni_env: &'a mut JNIEnv<'a>,
}

impl<'a> Env<'a> {
  pub fn new(jni_env: &'a mut JNIEnv<'a>) -> Env<'a> {
    Env { jni_env }
  }

  pub fn class(&'a mut self, name: &str) -> Result<Class<'a>, Error> {
    let class = self.jni_env.find_class(name);

    match class {
      Ok(class) => Ok(Class::new(self, class)),
      Err(e) => Err(e),
    }
  }
}

pub struct Class<'a> {
  env: &'a mut Env<'a>,
  class: JClass<'a>,
}

impl<'a> Class<'a> {
  pub fn new(env: &'a mut Env<'a>, class: JClass<'a>) -> Class<'a> {
    Class { env, class }
  }

  pub fn call_static_method(
    &mut self,
    name: &str,
    signature: Signature,
    args: &[JValueGen<&JObject<'_>>],
  ) -> jni::errors::Result<JValueOwned<'_>> {
    let class = &self.class;
    let signature: String = signature.into();
    self
      .env
      .jni_env
      .call_static_method(class, name, signature, args)
  }
}
