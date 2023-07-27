use crate::{env::Env, signature::Signature, value::Value, Class, Type};
use jni::objects::{JObject, JValueGen};

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
        .map(|o| self.env.new_value(*o))
        .collect::<Vec<JValueGen<&JObject>>>()
        .as_slice(),
    )
  }

  /// Gets a field on the object
  ///
  /// # Arguments
  ///
  /// * `name` - The name of the field
  /// * `type` - The type of the field
  pub fn get_field(
    &self,
    name: &str,
    r#type: Type,
  ) -> jni::errors::Result<JValueGen<JObject<'_>>> {
    let r#type: String = r#type.into();

    let mut jni_env = self.env.get_jni_env();
    jni_env.get_field(self.object, name, r#type)
  }

  /// Sets a field on the object
  ///
  /// # Arguments
  ///
  /// * `name` - The name of the field
  /// * `value` - The value to set the field to
  pub fn set_field(
    &self,
    name: &str,
    r#type: Type,
    value: Value,
  ) -> jni::errors::Result<()> {
    let r#type: String = r#type.into();

    let mut jni_env = self.env.get_jni_env();
    jni_env.set_field(self.object, name, r#type, self.env.new_value(value))
  }

  /// Gets the wrapped object
  pub fn get_object(&self) -> &'a JObject<'a> {
    self.object
  }

  /// Gets the class of the object
  pub fn get_class(&self) -> jni::errors::Result<Class> {
    let jni_env = self.env.get_jni_env();
    Ok(Class::new(self.env, jni_env.get_object_class(self.object)?))
  }
}
