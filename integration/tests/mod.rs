pub mod boolean_array;
pub mod field;
pub mod hello;
pub mod method;
pub mod static_field;
pub mod static_method;

#[derive(Debug)]
pub struct IntegrationTest {
  pub name: &'static str,
  pub lib: &'static str,
  pub java_class: &'static str,
  pub test_fn: fn(String) -> bool,
}

inventory::collect!(IntegrationTest);
