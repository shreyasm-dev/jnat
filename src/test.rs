#[cfg(test)]
mod signature {
  use crate::signature::{Signature, Type};

  #[test]
  fn signature_from() {
    let signature: String = Signature::new(&[], Type::Void).into();
    assert_eq!(signature, "()V");

    let signature: String = Signature::new(&[Type::Boolean], Type::Boolean).into();
    assert_eq!(signature, "(Z)Z");

    let signature: String = Signature::new(
      &[
        Type::Array(&Type::Object("java/lang/String")),
        Type::Array(&Type::Char),
      ],
      Type::Char,
    )
    .into();
    assert_eq!(signature, "([Ljava/lang/String;[C)C");
  }
}
