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

impl<'a> Type<'a> {
  /// Converts a Type into a String
  pub fn to_string(&self) -> String {
    let result = match self {
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
      Type::Array(t) => format!("[{}", t.to_string()),
    };

    result
  }
}
