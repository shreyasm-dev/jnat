pub struct Signature {
  pub arguments: Vec<Type>,
  pub return_type: Type,
}

impl Signature {
  pub fn new(arguments: Vec<Type>, return_type: Type) -> Signature {
    Signature {
      arguments,
      return_type,
    }
  }
}

impl From<Signature> for String {
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

pub enum Type {
  Boolean,
  Byte,
  Char,
  Short,
  Int,
  Long,
  Float,
  Double,
  Void,
  Object(&'static str),
  Array(Box<Type>),
}

impl Type {
  fn to_string(&self) -> String {
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
      Type::Array(t) => format!("[{}", t.as_ref().to_string()),
    };

    result
  }
}
