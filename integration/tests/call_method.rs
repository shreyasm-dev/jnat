use super::IntegrationTest;

fn test_call_method(s: String) -> bool {
  return s == "Static callback: 0 - Hello, world!\n";
}

inventory::submit! {IntegrationTest {
  name: "call_method",
  lib: "call_method",
  java_class: "CallMethod",
  test_fn: test_call_method,
}}
