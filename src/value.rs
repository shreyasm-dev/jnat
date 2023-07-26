use crate::object::Object;

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
  // Null, // TODO: Implement this
  /// A void value
  Void,
  /// An object value
  Object(Object<'a>),
}
