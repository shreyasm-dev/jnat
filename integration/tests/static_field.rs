use super::IntegrationTest;

fn test_static_field(s: String) -> bool {
  return s == "0\n";
}

inventory::submit! {IntegrationTest {
  name: "static_field",
  lib: "static_field",
  java_class: "StaticField",
  test_fn: test_static_field,
}}
