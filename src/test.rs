#[cfg(test)]
mod signature {
  use crate::signature::{Signature, Type};

  #[test]
  fn signature_from() {
    let signature: String = Signature::new(vec![], Type::Void).into();
    assert_eq!(signature, "()V");

    let signature: String = Signature::new(vec![Type::Boolean], Type::Boolean).into();
    assert_eq!(signature, "(Z)Z");

    let signature: String = Signature::new(
      vec![Type::Array(Box::new(Type::Object("java/lang/String"))), Type::Array(Box::new(Type::Char))],
      Type::Char,
    )
    .into();
    assert_eq!(signature, "([Ljava/lang/String;[C)C");
  }
}
