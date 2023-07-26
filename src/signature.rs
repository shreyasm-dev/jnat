use crate::r#type::Type;

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
