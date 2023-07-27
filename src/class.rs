use crate::{env::Env, signature::Signature, value::Value, Type};
use jni::objects::{JClass, JObject, JStaticFieldID, JValueGen};

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
        .map(|o| self.env.new_value(*o))
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
  pub fn create(&self, signature: Signature, args: &[Value]) -> jni::errors::Result<JObject> {
    let class = &self.class;
    let signature: String = signature.into();

    let mut jni_env = self.env.get_jni_env();
    jni_env.new_object(
      class,
      signature,
      args
        .iter()
        .map(|o| self.env.new_value(*o))
        .collect::<Vec<JValueGen<&JObject>>>()
        .as_slice(),
    )
  }

  /// Gets a static field on the class
  ///
  /// # Arguments
  ///
  /// * `name` - The name of the field
  /// * `type` - The type of the field
  pub fn get_static_field(
    &self,
    name: &str,
    r#type: Type,
  ) -> jni::errors::Result<JValueGen<JObject<'_>>> {
    let class = &self.class;
    let r#type: String = r#type.into();

    let mut jni_env = self.env.get_jni_env();
    jni_env.get_static_field(class, name, r#type)
  }

  /// Sets a static field on the class
  ///
  /// # Arguments
  ///
  /// * `name` - The name of the field
  /// * `value` - The value to set the field to
  pub fn set_static_field(
    &self,
    name: &str,
    r#type: Type,
    value: Value,
  ) -> jni::errors::Result<()> {
    let class = &self.class;
    let value = self.env.new_value(value);
    let field = self.get_static_field_id(name, r#type)?;

    let mut jni_env = self.env.get_jni_env();
    jni_env.set_static_field(class, field, value)
  }

  /// Get a static field ID on the class
  ///
  /// # Arguments
  ///
  /// * `name` - The name of the field
  /// * `signature` - The signature of the field
  pub fn get_static_field_id(
    &self,
    name: &str,
    r#type: Type,
  ) -> jni::errors::Result<JStaticFieldID> {
    let class = &self.class;
    let r#type: String = r#type.into();

    let mut jni_env = self.env.get_jni_env();
    jni_env.get_static_field_id(class, name, r#type)
  }

  /// Get the wrapped class
  pub fn get_class(self) -> JClass<'a> {
    self.class
  }
}
