use super::IntegrationTest;

fn test_method(s: String) -> bool {
  return s == "Static callback: 0 - Hello, world!\n";
}

inventory::submit! {IntegrationTest {
  name: "method",
  lib: "method",
  java_class: "Method",
  test_fn: test_method,
}}
