use super::IntegrationTest;

fn test_boolean_array(s: String) -> bool {
  return s == "0000000000-1000000000\n";
}

inventory::submit! {IntegrationTest {
  name: "boolean_array",
  lib: "boolean_array",
  java_class: "BooleanArray",
  test_fn: test_boolean_array,
}}
