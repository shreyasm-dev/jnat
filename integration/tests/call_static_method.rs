use super::IntegrationTest;

fn test_call_static_method(s: String) -> bool {
  return s == "Static callback: 0\n";
}

inventory::submit! {IntegrationTest {
  name: "call_static_method",
  lib: "call_static_method",
  java_class: "CallStaticMethod",
  test_fn: test_call_static_method,
}}
