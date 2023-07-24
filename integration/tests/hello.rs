use super::IntegrationTest;

fn test_hello(s: String) -> bool {
  return s == "Hello, world!\n";
}

inventory::submit! {IntegrationTest {
  name: "hello",
  lib: "hello",
  java_class: "Hello",
  test_fn: test_hello,
}}
