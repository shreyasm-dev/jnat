use super::IntegrationTest;

fn test_static_call(s: String) -> bool {
  return s == "Static callback\n";
}

inventory::submit! {IntegrationTest {
  name: "static_call",
  lib: "static_call",
  java_class: "StaticCall",
  test_fn: test_static_call,
}}
