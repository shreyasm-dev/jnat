/// A representation of the arguments and return type of a method
#[derive(Clone, Copy)]
pub struct Signature<'a> {
  /// The arguments of the method
  pub arguments: &'a [Type<'a>],
  /// The return type of the method
  pub return_type: Type<'a>,
}

impl<'a> Signature<'a> {
  /// Creates a new Signature
  /// 
  /// # Arguments
  /// 
  /// * `arguments` - The arguments of the method
  /// * `return_type` - The return type of the method
  pub fn new(arguments: &'a [Type<'a>], return_type: Type<'a>) -> Signature<'a> {
    Signature {
      arguments,
      return_type,
    }
  }
}

impl<'a> From<Signature<'a>> for String {
  fn from(value: Signature) -> Self {
    let mut signature = String::new();

    signature.push('(');
    for arg in value.arguments {
      signature.push_str(&arg.to_string());
    }
    signature.push(')');
    signature.push_str(&value.return_type.to_string());

    signature
  }
}

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
      Type::Boolean => String::from("Z"),
      Type::Byte => String::from("B"),
      Type::Char => String::from("C"),
      Type::Short => String::from("S"),
      Type::Int => String::from("I"),
      Type::Long => String::from("J"),
      Type::Float => String::from("F"),
      Type::Double => String::from("D"),
      Type::Void => String::from("V"),
      Type::Object(s) => format!("L{};", s),
      Type::Array(t) => format!("[{}", t.to_string()),
    };

    result
  }
}
