use jni::JNIEnv;

pub struct Env<'a> {
  pub jni_env: *mut JNIEnv<'a>, // TODO: Temporarily pub, remove later
}

impl Env<'_> {
  pub fn new(jni_env: *mut JNIEnv) -> Env {
    Env { jni_env }
  }
}
