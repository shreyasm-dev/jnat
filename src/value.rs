use jni::{
  objects::{JObject, JValueGen},
  sys::{jboolean, jchar},
};

#[derive(Clone, Copy)]
pub enum Value {
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
  Object(Object),
}

impl Value {
  pub fn to_jvaluegen(self) -> JValueGen<&'static JObject<'static>> {
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
pub struct Object {
  object: &'static JObject<'static>,
}

impl Object {
  pub fn new(object: &'static JObject<'static>) -> Object {
    Object { object }
  }
}
