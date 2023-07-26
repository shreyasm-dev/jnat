use crate::{class::Class, object::Object, value::Value};
use jni::{
  errors::Error,
  objects::{JObject, JString, JValueGen},
  sys::{jboolean, jchar},
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
    Env { jni_env }
  }

  /// Gets the native interface
  pub fn get_jni_env(&self) -> JNIEnv<'_> {
    unsafe { JNIEnv::from_raw(self.jni_env.get_native_interface()) }.unwrap()
  }

  /// Gets a class, given a qualified name
  ///
  /// # Arguments
  ///
  /// * `name` - The qualified name of the class
  pub fn class(&'a self, name: &str) -> Result<Class<'a>, Error> {
    let mut jni_env = self.get_jni_env();
    Ok(Class::new(self, jni_env.find_class(name)?))
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
    Ok(JObject::from(self.jni_env.new_string(string)?))
  }

  /// Gets a string from the JVM, given a JString
  ///
  /// # Arguments
  ///
  /// * `string` - The JString to convert
  pub fn get_string(&'a self, string: JString<'a>) -> Result<String, Error> {
    let mut jni_env = self.get_jni_env();
    Ok(jni_env.get_string(&string)?.into())
  }

  /// Gets a JValueGen<JObject>, given a Value
  ///
  /// # Arguments
  ///
  /// * `value` - The Value to convert
  pub fn value(self, value: Value<'a>) -> JValueGen<&'a JObject<'a>> {
    match value {
      Value::Boolean(b) => JValueGen::Bool(b as jboolean),
      Value::Byte(b) => JValueGen::Byte(b),
      Value::Char(c) => JValueGen::Char(c as jchar),
      Value::Short(s) => JValueGen::Short(s),
      Value::Int(i) => JValueGen::Int(i),
      Value::Long(l) => JValueGen::Long(l),
      Value::Float(f) => JValueGen::Float(f),
      Value::Double(d) => JValueGen::Double(d),
      Value::Void => JValueGen::Void,
      Value::Object(object) => JValueGen::Object(object.get_object()),
    }
  }

  /// Gets a Value, given a JValueGen<JObject>
  ///
  /// # Arguments
  ///
  /// * `object` - The JValueGen<JObject> to convert
  pub fn get_value(&self, jvaluegen: JValueGen<&'a JObject<'a>>) -> Value {
    match jvaluegen {
      JValueGen::Bool(b) => Value::Boolean(b != 0),
      JValueGen::Byte(b) => Value::Byte(b),
      JValueGen::Char(c) => Value::Char(c as u8 as char),
      JValueGen::Short(s) => Value::Short(s),
      JValueGen::Int(i) => Value::Int(i),
      JValueGen::Long(l) => Value::Long(l),
      JValueGen::Float(f) => Value::Float(f),
      JValueGen::Double(d) => Value::Double(d),
      JValueGen::Object(o) => Value::Object(Object::new(self, o)),
      JValueGen::Void => Value::Void,
    }
  }
}
