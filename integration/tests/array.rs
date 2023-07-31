use super::IntegrationTest;

fn test_array(s: String) -> bool {
  return s == "falsefalsefalsefalsefalsefalsefalsefalsefalsefalse-truefalsefalsefalsefalsefalsefalsefalsefalsefalse
0000000000-1000000000
\0\0\0\0\0\0\0\0\0\0-a\0\0\0\0\0\0\0\0\0
0000000000-0.1000000000
0000000000-0.1000000000
0000000000-1000000000
0000000000-1000000000
0000000000-1000000000\n";
}

inventory::submit! {IntegrationTest {
  name: "array",
  lib: "array",
  java_class: "Array",
  test_fn: test_array,
}}
