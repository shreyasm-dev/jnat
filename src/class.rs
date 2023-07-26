use crate::{env::Env, signature::Signature, value::Value};
use jni::objects::{JClass, JObject, JValueGen};

/// A struct wrapping a JClass
pub struct Class<'a> {
  env: &'a Env<'a>,
  class: JClass<'a>,
}

impl<'a> Class<'a> {
  /// Creates a new Class
  ///
  /// # Arguments
  ///
  /// * `env` - The environment
  /// * `class` - The JClass to wrap
  pub fn new(env: &'a Env<'a>, class: JClass<'a>) -> Class<'a> {
    Class { env, class }
  }

  /// Calls a static method on the class
  ///
  /// # Arguments
  ///
  /// * `name` - The name of the method
  /// * `signature` - The signature of the method
  /// * `args` - The arguments to pass to the method
  pub fn call_static_method(
    &self,
    name: &str,
    signature: Signature,
    args: &[Value],
  ) -> jni::errors::Result<JValueGen<JObject<'_>>> {
    let class = &self.class;
    let signature: String = signature.into();

    let mut jni_env = self.env.get_jni_env();
    jni_env.call_static_method(
      class,
      name,
      signature,
      args
        .iter()
        .map(|o| self.env.value(*o))
        .collect::<Vec<JValueGen<&JObject>>>()
        .as_slice(),
    )
  }

  /// Creates an instance of the class
  ///
  /// # Arguments
  ///
  /// * `signature` - The signature of the constructor
  /// * `args` - The arguments to pass to the constructor
  pub fn new_instance(&self, signature: Signature, args: &[Value]) -> jni::errors::Result<JObject> {
    let class = &self.class;
    let signature: String = signature.into();

    let mut jni_env = self.env.get_jni_env();
    jni_env.new_object(
      class,
      signature,
      args
        .iter()
        .map(|o| self.env.value(*o))
        .collect::<Vec<JValueGen<&JObject>>>()
        .as_slice(),
    )
  }

  /// Get the wrapped class
  pub fn get_class(self) -> JClass<'a> {
    self.class
  }
}
