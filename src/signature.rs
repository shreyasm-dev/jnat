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
      signature.push(arg.to_char());
    }
    signature.push(')');
    signature.push(value.return_type.to_char());

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
  fn to_char(&self) -> char {
    match self {
      Type::Boolean => 'Z',
      Type::Byte => 'B',
      Type::Char => 'C',
      Type::Short => 'S',
      Type::Int => 'I',
      Type::Long => 'J',
      Type::Float => 'F',
      Type::Double => 'D',
      Type::Void => 'V',
      Type::Object(_) => 'L',
      Type::Array(_) => '[',
    }
  }
}
