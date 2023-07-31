use crate::{class::Class, object::Object, value::Value, Array, BooleanArray, ByteArray, CharArray, DoubleArray, FloatArray, IntArray, LongArray, ShortArray, ObjectArray};
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
    unsafe { JNIEnv::unsafe_clone(&self.jni_env) }
  }

  /// Gets a class, given a qualified name
  ///
  /// # Arguments
  ///
  /// * `name` - The qualified name of the class
  pub fn get_class(&'a self, name: &str) -> Result<Class<'a>, Error> {
    let mut jni_env = self.get_jni_env();
    Ok(Class::new(self, jni_env.find_class(name)?))
  }

  // We won't be using this for consistency reasons
  /* /// Converts a JObject into an Object
  ///
  /// # Arguments
  ///
  /// * `object` - The JObject to wrap
  pub fn get_object(&'a self, object: &'a JObject<'a>) -> Object<'a> {
    Object::new(self, object)
  } */

  /// Converts a string into a JObject
  ///
  /// # Arguments
  ///
  /// * `string` - The string to convert
  pub fn new_string(&'a self, string: &'a str) -> Result<JObject<'a>, Error> {
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
  pub fn new_value(self, value: Value<'a>) -> JValueGen<&'a JObject<'a>> {
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

  /// Creates a new boolean array
  ///
  /// # Arguments
  ///
  /// * `length` - The length of the array
  pub fn new_boolean_array(&'a self, length: usize) -> BooleanArray<'a> {
    BooleanArray::new(self, length)
  }

  /// Creates a new byte array
  ///
  /// # Arguments
  ///
  /// * `length` - The length of the array
  pub fn new_byte_array(&'a self, length: usize) -> ByteArray<'a> {
    ByteArray::new(self, length)
  }

  /// Creates a new char array
  /// 
  /// # Arguments
  /// 
  /// * `length` - The length of the array
  pub fn new_char_array(&'a self, length: usize) -> CharArray<'a> {
    CharArray::new(self, length)
  }

  /// Creates a new double array
  /// 
  /// # Arguments
  /// 
  /// * `length` - The length of the array
  pub fn new_double_array(&'a self, length: usize) -> DoubleArray<'a> {
    DoubleArray::new(self, length)
  }

  /// Creates a new float array
  /// 
  /// # Arguments
  /// 
  /// * `length` - The length of the array
  pub fn new_float_array(&'a self, length: usize) -> FloatArray<'a> {
    FloatArray::new(self, length)
  }

  /// Creates a new int array
  /// 
  /// # Arguments
  /// 
  /// * `length` - The length of the array
  pub fn new_int_array(&'a self, length: usize) -> IntArray<'a> {
    IntArray::new(self, length)
  }

  /// Creates a new long array
  /// 
  /// # Arguments
  /// 
  /// * `length` - The length of the array
  pub fn new_long_array(&'a self, length: usize) -> LongArray<'a> {
    LongArray::new(self, length)
  }

  /// Creates a new short array
  /// 
  /// # Arguments
  /// 
  /// * `length` - The length of the array
  pub fn new_short_array(&'a self, length: usize) -> ShortArray<'a> {
    ShortArray::new(self, length)
  }

  /// Creates a new object array
  /// 
  /// # Arguments
  /// 
  /// * `length` - The length of the array
  /// * `class` - The class of the array
  pub fn new_object_array(&'a self, length: usize, class: &str) -> ObjectArray<'a> {
    ObjectArray::new_with(self, length, class)
  }
}
