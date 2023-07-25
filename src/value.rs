use crate::{env::Env, signature::Signature};
use jni::{
  objects::{JObject, JValueGen, JValueOwned},
  sys::{jboolean, jchar},
};

/// An enum representing Java values
#[derive(Clone, Copy)]
pub enum Value<'a> {
  /// A boolean value
  Boolean(bool),
  /// A byte value
  Byte(i8),
  /// A char value
  Char(char),
  /// A short value
  Short(i16),
  /// An int value
  Int(i32),
  /// A long value
  Long(i64),
  /// A float value
  Float(f32),
  /// A double value
  Double(f64),
  // Null,
  /// A void value
  Void,
  /// An object value
  Object(Object<'a>),
}

impl<'a> Value<'a> {
  /// Converts a `Value` into a `JValueGen`
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

/// A struct wrapping a JObject
#[derive(Clone, Copy)]
pub struct Object<'a> {
  env: &'a Env<'a>,
  object: &'a JObject<'a>,
}

impl<'a> Object<'a> {
  /// Creates a new Object
  /// 
  /// # Arguments
  /// 
  /// * `env` - The environment
  /// * `object` - The JObject to wrap
  pub fn new(env: &'a Env<'a>, object: &'a JObject<'a>) -> Object<'a> {
    Object { env, object }
  }

  /// Calls a method on the object
  /// 
  /// # Arguments
  /// 
  /// * `name` - The name of the method
  /// * `signature` - The signature of the method
  /// * `args` - The arguments to pass to the method
  pub fn call_method(
    &self,
    name: &str,
    signature: Signature,
    args: &[Value],
  ) -> jni::errors::Result<JValueOwned<'_>> {
    let signature: String = signature.into();

    let mut jni_env = unsafe { jni::JNIEnv::from_raw(self.env.get_native_interface()) }.unwrap();
    jni_env.call_method(
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

impl<'a> AsRef<JObject<'a>> for Object<'a> {
  fn as_ref(&self) -> &JObject<'a> {
    self.object
  }
}
