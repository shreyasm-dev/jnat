/// An enum representing Java types
#[derive(Clone, Copy)]
pub enum Type<'a> {
  /// A boolean type
  Boolean,
  /// A byte type
  Byte,
  /// A char type
  Char,
  /// A short type
  Short,
  /// An int type
  Int,
  /// A long type
  Long,
  /// A float type
  Float,
  /// A double type
  Double,
  /// A void type
  Void,
  /// An object type
  Object(&'a str),
  /// An array type
  Array(&'a Type<'a>),
}

impl<'a> From<Type<'a>> for String {
  fn from(r#type: Type) -> Self {
    let result = match r#type {
      Type::Boolean => "Z".into(),
      Type::Byte => "B".into(),
      Type::Char => "C".into(),
      Type::Short => "S".into(),
      Type::Int => "I".into(),
      Type::Long => "J".into(),
      Type::Float => "F".into(),
      Type::Double => "D".into(),
      Type::Void => "V".into(),
      Type::Object(s) => format!("L{};", s),
      Type::Array(t) => format!("[{}", <Type as Into<String>>::into(*t)),
    };

    result
  }
}
