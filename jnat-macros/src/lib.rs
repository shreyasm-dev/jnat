use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{
  parse::Parse, parse_macro_input, punctuated::Punctuated, spanned::Spanned, Ident, Path, Token,
  Type,
};

struct JavaNativeMethod {
  qualified_class_name: Path,
  method_name: Ident,
  signature: MethodSignature,
}

struct MethodSignature {
  return_type: Type,
  params: Vec<Type>,
}

impl Parse for JavaNativeMethod {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let qualified_class_name = input.parse()?;
    input.parse::<Token![,]>()?;
    let method_name = input.parse()?;
    input.parse::<Token![,]>()?;
    let signature = input.parse()?;
    Ok(JavaNativeMethod {
      qualified_class_name,
      method_name,
      signature,
    })
  }
}

impl Parse for MethodSignature {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let content;
    let _ = syn::parenthesized!(content in input);
    let params = Punctuated::<Type, Token![,]>::parse_terminated(&content)?;
    input.parse::<Token![->]>()?;
    let return_type = input.parse()?;
    Ok(MethodSignature {
      return_type,
      params: params.into_iter().collect(),
    })
  }
}

struct ParamWithName {
  name: Ident,
  ty: Type,
}

impl ToTokens for ParamWithName {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let name = &self.name;
    let ty = &self.ty;
    tokens.extend(quote::quote! { #name: #ty });
  }
}

#[proc_macro]
pub fn jnat(input: TokenStream) -> TokenStream {
  /*

  This method returns something like this:

  #[no_mangle]
  #[allow(non_snake_case)]
  pub extern "system" fn Java_com_example_jnat_HelloWorld_hello(env: jni::JNIEnv, class: jni::objects::JClass) -> jni::sys::jint {
    hello(env, class)
  }

  On the input:

  jnat!(hello)

  fn hello(env: jni::JNIEnv, class: jni::objects::JClass) -> jni::sys::jint {
    42
  }

  */

  let JavaNativeMethod {
    qualified_class_name,
    method_name,
    signature,
  } = parse_macro_input!(input as JavaNativeMethod);

  let method_name_str = &*method_name.to_string();

  let full_method_name = qualified_class_name.segments.iter().map(|s| s.ident.to_string()).collect::<Vec<String>>().join("_");
  let full_method_name = format!("Java_{}_{}", full_method_name, method_name_str);
  let full_method_name = Ident::new(&full_method_name, qualified_class_name.span());

  let return_type = signature.return_type;
  let params = signature.params.iter().enumerate().map(|(i, ty)| ParamWithName {
    name: Ident::new(&format!("arg{}", i), ty.span()),
    ty: ty.clone(),
  }).collect::<Vec<ParamWithName>>();

  let param_names = params.iter().map(|p| &p.name).collect::<Vec<&Ident>>();

  let output = quote::quote! {
    #[no_mangle]
    #[allow(non_snake_case)]
    pub extern "system" fn #full_method_name(#(#params),*) -> #return_type {
      #method_name(#(#param_names),*)
    }
  };

  output.into()
}
