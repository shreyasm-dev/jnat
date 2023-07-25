#[derive(Clone)]
pub struct Signature<'a> {
  pub arguments: &'a [Type<'a>],
  pub return_type: Type<'a>,
}

impl<'a> Signature<'a> {
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

#[derive(Clone, Copy)]
pub enum Type<'a> {
  Boolean,
  Byte,
  Char,
  Short,
  Int,
  Long,
  Float,
  Double,
  Void,
  Object(&'a str),
  Array(&'a Type<'a>),
}

impl<'a> Type<'a> {
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
      Type::Array(t) => format!("[{}", t.to_string()),
    };

    result
  }
}
