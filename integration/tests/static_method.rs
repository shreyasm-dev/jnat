use super::IntegrationTest;

fn test_static_method(s: String) -> bool {
  return s == "Static callback: 0 - Hello, world!\n";
}

inventory::submit! {IntegrationTest {
  name: "static_method",
  lib: "static_method",
  java_class: "StaticMethod",
  test_fn: test_static_method,
}}
