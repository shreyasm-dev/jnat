use crate::Env;
use jni::{
  errors::Result,
  objects::{
    JBooleanArray, JByteArray, JCharArray, JDoubleArray, JFloatArray, JIntArray, JLongArray,
    JObject, JObjectArray, JShortArray,
  },
};

/// A trait for wrapped JNI arrays
pub trait Array<'a, T, J> {
  /// Create a new array with the given length
  ///
  /// # Arguments
  ///
  /// * `env` - The environment
  /// * `length` - The length of the array
  fn new(env: &'a Env<'a>, length: usize) -> Self;

  /// Create a new array from a JNI array
  ///
  /// # Arguments
  ///
  /// * `env` - The environment
  /// * `array` - The JNI array
  fn from(env: &'a Env<'a>, array: J) -> Self;

  /// Get the length of the array
  fn length(&self) -> usize;

  /// Get the value at the given index
  ///
  /// # Arguments
  ///
  /// * `index` - The index
  fn get(&self, index: usize) -> Result<T>;

  /// Set the value at the given index
  ///
  /// # Arguments
  ///
  /// * `index` - The index
  /// * `value` - The value
  fn set(&self, index: usize, value: T) -> Result<()>;
}

/// A struct wrapping a JNI boolean array
pub struct BooleanArray<'a> {
  env: &'a Env<'a>,
  array: JBooleanArray<'a>,
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

/// A struct wrapping a JNI byte array
pub struct ByteArray<'a> {
  env: &'a Env<'a>,
  array: JByteArray<'a>,
}

impl<'a> Array<'a, i8, JByteArray<'a>> for ByteArray<'a> {
  fn new(env: &'a Env<'a>, length: usize) -> Self {
    let jni_env = env.get_jni_env();
    let array = jni_env.new_byte_array(length as i32).unwrap();

    ByteArray { env, array }
  }

  fn from(env: &'a Env<'a>, array: JByteArray<'a>) -> Self {
    ByteArray { env, array }
  }

  fn length(&self) -> usize {
    let jni_env = self.env.get_jni_env();
    jni_env.get_array_length(&self.array).unwrap() as usize
  }

  fn get(&self, index: usize) -> Result<i8> {
    let jni_env = self.env.get_jni_env();
    let index = index as i32;

    let mut buf: [i8; 1] = [0];
    jni_env.get_byte_array_region(&self.array, index, &mut buf)?;

    Ok(buf[0])
  }

  fn set(&self, index: usize, value: i8) -> Result<()> {
    let jni_env = self.env.get_jni_env();
    let index = index as i32;

    let buf: [i8; 1] = [value];
    jni_env.set_byte_array_region(&self.array, index, &buf)?;

    Ok(())
  }
}

/// A struct wrapping a JNI char array
pub struct CharArray<'a> {
  env: &'a Env<'a>,
  array: JCharArray<'a>,
}

impl<'a> Array<'a, char, JCharArray<'a>> for CharArray<'a> {
  fn new(env: &'a Env<'a>, length: usize) -> Self {
    let jni_env = env.get_jni_env();
    let array = jni_env.new_char_array(length as i32).unwrap();

    CharArray { env, array }
  }

  fn from(env: &'a Env<'a>, array: JCharArray<'a>) -> Self {
    CharArray { env, array }
  }

  fn length(&self) -> usize {
    let jni_env = self.env.get_jni_env();
    jni_env.get_array_length(&self.array).unwrap() as usize
  }

  fn get(&self, index: usize) -> Result<char> {
    let jni_env = self.env.get_jni_env();
    let index = index as i32;

    let mut buf: [u16; 1] = [0];
    jni_env.get_char_array_region(&self.array, index, &mut buf)?;

    Ok(buf[0] as u8 as char)
  }

  fn set(&self, index: usize, value: char) -> Result<()> {
    let jni_env = self.env.get_jni_env();
    let index = index as i32;

    let buf: [u16; 1] = [value as u16];
    jni_env.set_char_array_region(&self.array, index, &buf)?;

    Ok(())
  }
}

/// A struct wrapping a JNI double array
pub struct DoubleArray<'a> {
  env: &'a Env<'a>,
  array: JDoubleArray<'a>,
}

impl<'a> Array<'a, f64, JDoubleArray<'a>> for DoubleArray<'a> {
  fn new(env: &'a Env<'a>, length: usize) -> Self {
    let jni_env = env.get_jni_env();
    let array = jni_env.new_double_array(length as i32).unwrap();

    DoubleArray { env, array }
  }

  fn from(env: &'a Env<'a>, array: JDoubleArray<'a>) -> Self {
    DoubleArray { env, array }
  }

  fn length(&self) -> usize {
    let jni_env = self.env.get_jni_env();
    jni_env.get_array_length(&self.array).unwrap() as usize
  }

  fn get(&self, index: usize) -> Result<f64> {
    let jni_env = self.env.get_jni_env();
    let index = index as i32;

    let mut buf: [f64; 1] = [0.0];
    jni_env.get_double_array_region(&self.array, index, &mut buf)?;

    Ok(buf[0])
  }

  fn set(&self, index: usize, value: f64) -> Result<()> {
    let jni_env = self.env.get_jni_env();
    let index = index as i32;

    let buf: [f64; 1] = [value];
    jni_env.set_double_array_region(&self.array, index, &buf)?;

    Ok(())
  }
}

/// A struct wrapping a JNI float array
pub struct FloatArray<'a> {
  env: &'a Env<'a>,
  array: JFloatArray<'a>,
}

impl<'a> Array<'a, f32, JFloatArray<'a>> for FloatArray<'a> {
  fn new(env: &'a Env<'a>, length: usize) -> Self {
    let jni_env = env.get_jni_env();
    let array = jni_env.new_float_array(length as i32).unwrap();

    FloatArray { env, array }
  }

  fn from(env: &'a Env<'a>, array: JFloatArray<'a>) -> Self {
    FloatArray { env, array }
  }

  fn length(&self) -> usize {
    let jni_env = self.env.get_jni_env();
    jni_env.get_array_length(&self.array).unwrap() as usize
  }

  fn get(&self, index: usize) -> Result<f32> {
    let jni_env = self.env.get_jni_env();
    let index = index as i32;

    let mut buf: [f32; 1] = [0.0];
    jni_env.get_float_array_region(&self.array, index, &mut buf)?;

    Ok(buf[0])
  }

  fn set(&self, index: usize, value: f32) -> Result<()> {
    let jni_env = self.env.get_jni_env();
    let index = index as i32;

    let buf: [f32; 1] = [value];
    jni_env.set_float_array_region(&self.array, index, &buf)?;

    Ok(())
  }
}

/// A struct wrapping a JNI int array
pub struct IntArray<'a> {
  env: &'a Env<'a>,
  array: JIntArray<'a>,
}

impl<'a> Array<'a, i32, JIntArray<'a>> for IntArray<'a> {
  fn new(env: &'a Env<'a>, length: usize) -> Self {
    let jni_env = env.get_jni_env();
    let array = jni_env.new_int_array(length as i32).unwrap();

    IntArray { env, array }
  }

  fn from(env: &'a Env<'a>, array: JIntArray<'a>) -> Self {
    IntArray { env, array }
  }

  fn length(&self) -> usize {
    let jni_env = self.env.get_jni_env();
    jni_env.get_array_length(&self.array).unwrap() as usize
  }

  fn get(&self, index: usize) -> Result<i32> {
    let jni_env = self.env.get_jni_env();
    let index = index as i32;

    let mut buf: [i32; 1] = [0];
    jni_env.get_int_array_region(&self.array, index, &mut buf)?;

    Ok(buf[0])
  }

  fn set(&self, index: usize, value: i32) -> Result<()> {
    let jni_env = self.env.get_jni_env();
    let index = index as i32;

    let buf: [i32; 1] = [value];
    jni_env.set_int_array_region(&self.array, index, &buf)?;

    Ok(())
  }
}

/// A struct wrapping a JNI long array
pub struct LongArray<'a> {
  env: &'a Env<'a>,
  array: JLongArray<'a>,
}

impl<'a> Array<'a, i64, JLongArray<'a>> for LongArray<'a> {
  fn new(env: &'a Env<'a>, length: usize) -> Self {
    let jni_env = env.get_jni_env();
    let array = jni_env.new_long_array(length as i32).unwrap();

    LongArray { env, array }
  }

  fn from(env: &'a Env<'a>, array: JLongArray<'a>) -> Self {
    LongArray { env, array }
  }

  fn length(&self) -> usize {
    let jni_env = self.env.get_jni_env();
    jni_env.get_array_length(&self.array).unwrap() as usize
  }

  fn get(&self, index: usize) -> Result<i64> {
    let jni_env = self.env.get_jni_env();
    let index = index as i32;

    let mut buf: [i64; 1] = [0];
    jni_env.get_long_array_region(&self.array, index, &mut buf)?;

    Ok(buf[0])
  }

  fn set(&self, index: usize, value: i64) -> Result<()> {
    let jni_env = self.env.get_jni_env();
    let index = index as i32;

    let buf: [i64; 1] = [value];
    jni_env.set_long_array_region(&self.array, index, &buf)?;

    Ok(())
  }
}

/// A struct wrapping a JNI short array
pub struct ShortArray<'a> {
  env: &'a Env<'a>,
  array: JShortArray<'a>,
}

impl<'a> Array<'a, i16, JShortArray<'a>> for ShortArray<'a> {
  fn new(env: &'a Env<'a>, length: usize) -> Self {
    let jni_env = env.get_jni_env();
    let array = jni_env.new_short_array(length as i32).unwrap();

    ShortArray { env, array }
  }

  fn from(env: &'a Env<'a>, array: JShortArray<'a>) -> Self {
    ShortArray { env, array }
  }

  fn length(&self) -> usize {
    let jni_env = self.env.get_jni_env();
    jni_env.get_array_length(&self.array).unwrap() as usize
  }

  fn get(&self, index: usize) -> Result<i16> {
    let jni_env = self.env.get_jni_env();
    let index = index as i32;

    let mut buf: [i16; 1] = [0];
    jni_env.get_short_array_region(&self.array, index, &mut buf)?;

    Ok(buf[0])
  }

  fn set(&self, index: usize, value: i16) -> Result<()> {
    let jni_env = self.env.get_jni_env();
    let index = index as i32;

    let buf: [i16; 1] = [value];
    jni_env.set_short_array_region(&self.array, index, &buf)?;

    Ok(())
  }
}

/// A struct wrapping a JNI object array
pub struct ObjectArray<'a> {
  env: &'a Env<'a>,
  array: JObjectArray<'a>,
}

impl<'a> ObjectArray<'a> {
  /// Create a new object array with the given length and class
  ///
  /// # Arguments
  ///
  /// * `env` - The environment
  /// * `length` - The length of the array
  /// * `class` - The qualified class
  pub fn new_with(env: &'a Env<'a>, length: usize, class: &str) -> Self {
    let mut jni_env = env.get_jni_env();
    let array = jni_env
      .new_object_array(length as i32, class, JObject::null())
      .unwrap();

    ObjectArray { env, array }
  }
}

impl<'a> Array<'a, JObject<'a>, JObjectArray<'a>> for ObjectArray<'a> {
  fn new(env: &'a Env<'a>, length: usize) -> Self {
    Self::new_with(env, length, "java/lang/Object")
  }

  fn from(env: &'a Env<'a>, array: JObjectArray<'a>) -> Self {
    ObjectArray { env, array }
  }

  fn length(&self) -> usize {
    let jni_env = self.env.get_jni_env();
    jni_env.get_array_length(&self.array).unwrap() as usize
  }

  fn get(&self, index: usize) -> Result<JObject<'a>> {
    let mut jni_env = self.env.get_jni_env();
    let index = index as i32;

    jni_env.get_object_array_element(&self.array, index)
  }

  fn set(&self, index: usize, value: JObject<'a>) -> Result<()> {
    let jni_env = self.env.get_jni_env();
    let index = index as i32;

    jni_env.set_object_array_element(&self.array, index, value)?;

    Ok(())
  }
}
