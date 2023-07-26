use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse::Parser, parse_macro_input, punctuated::Punctuated, Ident, ItemFn, Path, Token};

#[proc_macro_attribute]
pub fn jnat(args: TokenStream, item: TokenStream) -> TokenStream {
  let args = Punctuated::<Path, Token![.]>::parse_terminated
    .parse(args)
    .unwrap();
  let item = parse_macro_input!(item as ItemFn);

  let method_name = args
    .iter()
    .map(|x| x.to_token_stream().to_string())
    .collect::<Vec<String>>()
    .join("_");
  let method_name = format!("Java_{}_{}", method_name, item.sig.ident.to_string());

  let item = item.clone();
  let mut native_fn = item.clone();

  native_fn.sig.ident = Ident::new(&method_name, native_fn.sig.ident.span());
  native_fn.sig.abi = Some(syn::parse_quote!(extern "system"));

  // Changing visibility and params is probably unexpected behavior, just leaving this here for reference

  /* item
    .sig
    .inputs
    .insert(0, syn::parse_quote!(mut env: jnat::jni::JNIEnv));
  item
    .sig
    .inputs
    .insert(1, syn::parse_quote!(class: jnat::jni::objects::JClass)); */

  /* native_fn.vis = Visibility::Public(Pub {
    span: Span::call_site(),
  }); */

  // We want #[no_mangle] and #[allow(non_snake_case)]

  let out = "#[no_mangle]\n#[allow(non_snake_case)]\n".to_string()
    + &native_fn.to_token_stream().to_string()
    + "\n"
    + &item.to_token_stream().to_string();

  out.parse().unwrap()
}
