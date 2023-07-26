use crate::{
  signature::Signature,
  value::{Object, Value},
};
use jni::{
  errors::Error,
  objects::{JClass, JObject, JValueGen, JValueOwned, JString},
  sys::JNINativeInterface_,
  JNIEnv,
};

/// A wrapper around the JNI environment
#[derive(Clone, Copy)]
pub struct Env<'a> {
  jni_env: &'a JNIEnv<'a>,
}

impl<'a> Env<'a> {
  /// Creates a new Env
  /// 
  /// # Arguments
  /// 
  /// * `jni_env` - The JNI environment
  pub fn new(jni_env: &'a JNIEnv<'a>) -> Env<'a> {
    Env { jni_env: jni_env }
  }

  /// Gets the native interface
  pub fn get_native_interface(&self) -> *mut *const JNINativeInterface_ {
    self.jni_env.get_native_interface()
  }

  /// Gets a class, given a qualified name
  /// 
  /// # Arguments
  /// 
  /// * `name` - The qualified name of the class
  pub fn class(&'a self, name: &str) -> Result<Class<'a>, Error> {
    let mut jni_env = unsafe { JNIEnv::from_raw(self.get_native_interface()) }.unwrap();

    let class = jni_env.find_class(name);

    match class {
      Ok(class) => Ok(Class::new(self, class)),
      Err(e) => Err(e),
    }
  }

  /// Converts a JObject into an Object
  /// 
  /// # Arguments
  /// 
  /// * `object` - The JObject to wrap
  pub fn object(&'a self, object: &'a JObject<'a>) -> Object<'a> {
    Object::new(self, object)
  }

  /// Converts a string into a JObject
  /// 
  /// # Arguments
  /// 
  /// * `string` - The string to convert
  pub fn string(&'a self, string: &'a str) -> Result<JObject<'a>, Error> {
    let string = self.jni_env.new_string(string);

    match string {
      Ok(string) => Ok(JObject::from(string)),
      Err(e) => Err(e),
    }
  }

  /// Gets a string from the JVM, given a JString
  /// 
  /// # Arguments
  /// 
  /// * `string` - The JString to convert
  pub fn get_string(&'a self, string: &'a JString<'a>) -> Result<String, Error> {
    let mut jni_env = unsafe { JNIEnv::from_raw(self.get_native_interface()) }.unwrap();

    let string = jni_env.get_string(&string);

    match string {
      Ok(string) => Ok(string.into()),
      Err(e) => Err(e),
    }
  }
}

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
