use jni::{
  objects::{JObject, JValueGen, JValueOwned},
  sys::{jboolean, jchar},
};

use crate::{env::Env, signature::Signature};

#[derive(Clone, Copy)]
pub enum Value<'a> {
  Boolean(bool),
  Byte(i8),
  Char(char),
  Short(i16),
  Int(i32),
  Long(i64),
  Float(f32),
  Double(f64),
  // Null,
  Void,
  Object(Object<'a>),
}

impl<'a> Value<'a> {
  pub fn to_jvaluegen(self) -> JValueGen<&'a JObject<'a>> {
    match self {
      Self::Boolean(b) => JValueGen::Bool(b as jboolean),
      Self::Byte(b) => JValueGen::Byte(b),
      Self::Char(c) => JValueGen::Char(c as jchar),
      Self::Short(s) => JValueGen::Short(s),
      Self::Int(i) => JValueGen::Int(i),
      Self::Long(l) => JValueGen::Long(l),
      Self::Float(f) => JValueGen::Float(f),
      Self::Double(d) => JValueGen::Double(d),
      // Self::Null => JValueGen::Object(JObject::null()),
      Self::Void => JValueGen::Void,
      Self::Object(object) => JValueGen::Object(object.object),
    }
  }
}

#[derive(Clone, Copy)]
pub struct Object<'a> {
  env: *mut Env<'a>,
  object: &'a JObject<'a>,
}

impl<'a> Object<'a> {
  pub fn new(env: *mut Env<'a>, object: &'a JObject<'a>) -> Object<'a> {
    Object { env, object }
  }

  pub fn call_method(
    &mut self,
    name: &str,
    signature: Signature,
    args: &[Value],
  ) -> jni::errors::Result<JValueOwned<'_>> {
    let signature: String = signature.into();

    unsafe { // TODO: Find a way to avoid this unsafe block
      (*self.env).jni_env.call_method(
        *self,
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
}

impl<'a> AsRef<JObject<'a>> for Object<'a> {
  fn as_ref(&self) -> &JObject<'a> {
    self.object
  }
}

impl<'a> From<Object<'a>> for &JObject<'a> {
  fn from(value: Object<'a>) -> Self {
    value.object
  }
}
