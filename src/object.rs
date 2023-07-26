use jni::objects::{JObject, JValueGen};

use crate::{env::Env, signature::Signature, value::Value};

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
  ) -> jni::errors::Result<JValueGen<JObject<'_>>> {
    let signature: String = signature.into();

    let mut jni_env = self.env.get_jni_env();
    jni_env.call_method(
      self.object,
      name,
      signature,
      args
        .iter()
        .map(|o| self.env.value(*o))
        .collect::<Vec<JValueGen<&JObject>>>()
        .as_slice(),
    )
  }

  /// Gets the wrapped object
  pub fn get_object(&self) -> &'a JObject<'a> {
    self.object
  }
}
