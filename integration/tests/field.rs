use super::IntegrationTest;

fn test_field(s: String) -> bool {
  return s == "0\n1\n";
}

inventory::submit! {IntegrationTest {
  name: "field",
  lib: "field",
  java_class: "Field",
  test_fn: test_field,
}}
