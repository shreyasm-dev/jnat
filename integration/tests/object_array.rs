use super::IntegrationTest;

fn test_object_array(s: String) -> bool {
  return s == "nullnullnullnullnullnullnullnullnullnull-hellotesttesttesttesttesttesttesttesttest\n";
}

inventory::submit! {IntegrationTest {
  name: "object_array",
  lib: "object_array",
  java_class: "ObjectArray",
  test_fn: test_object_array,
}}
