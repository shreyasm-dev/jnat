use crate::Env;
use jni::{errors::Result, objects::JBooleanArray};

pub trait Array<'a, T, J> {
  fn new(env: &'a Env<'a>, length: usize) -> Self;

  fn from(env: &'a Env<'a>, array: J) -> Self;

  fn length(&self) -> usize;

  fn get(&self, index: usize) -> Result<T>;

  fn set(&self, index: usize, value: T) -> Result<()>;
}

pub struct BooleanArray<'a> {
  env: &'a Env<'a>,
  array: JBooleanArray<'a>,
}

impl<'a> BooleanArray<'a> {
  pub fn new(env: &'a Env<'a>, length: usize) -> BooleanArray<'a> {
    let jni_env = env.get_jni_env();
    let array = jni_env.new_boolean_array(length as i32).unwrap();

    BooleanArray { env, array }
  }
}

impl<'a> Array<'a, bool, JBooleanArray<'a>> for BooleanArray<'a> {
  fn new(env: &'a Env<'a>, length: usize) -> Self {
    let jni_env = env.get_jni_env();
    let array = jni_env.new_boolean_array(length as i32).unwrap();

    BooleanArray { env, array }
  }

  fn from(env: &'a Env<'a>, array: JBooleanArray<'a>) -> Self {
    BooleanArray { env, array }
  }

  fn length(&self) -> usize {
    let jni_env = self.env.get_jni_env();
    jni_env.get_array_length(&self.array).unwrap() as usize
  }

  fn get(&self, index: usize) -> Result<bool> {
    let jni_env = self.env.get_jni_env();
    let index = index as i32;

    let mut buf: [u8; 1] = [0];
    jni_env.get_boolean_array_region(&self.array, index, &mut buf)?;

    Ok(buf[0] != 0)
  }

  fn set(&self, index: usize, value: bool) -> Result<()> {
    let jni_env = self.env.get_jni_env();
    let index = index as i32;

    let buf: [u8; 1] = [value as u8];
    jni_env.set_boolean_array_region(&self.array, index, &buf)?;

    Ok(())
  }
}
