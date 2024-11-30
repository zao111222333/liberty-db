use quote::quote;
use syn::{
  parse::{Parse, ParseStream},
  punctuated::Punctuated,
  DeriveInput, Expr, LitStr, Token,
};

struct KeyValue {
  key: LitStr,
  value: Expr,
}

impl Parse for KeyValue {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let key: LitStr = input.parse()?;
    input.parse::<Token![=]>()?;
    let value: Expr = input.parse()?;

    Ok(KeyValue { key, value })
  }
}

pub(crate) struct MacroArgs {
  pairs: Punctuated<KeyValue, Token![,]>,
}

impl Parse for MacroArgs {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let pairs = Punctuated::parse_terminated(input)?;
    Ok(MacroArgs { pairs })
  }
}

pub(crate) fn inner(
  args: MacroArgs,
  input: DeriveInput,
) -> syn::Result<proc_macro2::TokenStream> {
  let name = if let syn::Data::Enum(_) = &input.data {
    &input.ident
  } else {
    return Err(syn::Error::new(
      proc_macro2::Span::call_site(),
      "can only be used on Enum".to_string(),
    ));
  };

  let q = crate::enum_token::enum_quote(
    name,
    args
      .pairs
      .into_iter()
      .map(|pair| {
        let exp = pair.value;
        (quote! {#exp}, vec![pair.key.value()])
      })
      .collect(),
  );
  Ok(quote! {
    #input
    #q
  })
}

#[test]
fn test() {
  use syn::parse_str;
  let input = r#"
    "input_voltage" = Self::Voltage(VoltageVariable::InputVoltage),
    "input_transition_time" = Self::Time(TimeVariable::InputTransitionTime),
    "input_net_transition" = Self::Time(TimeVariable::InputNetTransition),
    "input_noise_height" = Self::Voltage(VoltageVariable::InputNoiseHeight),
    "input_noise_width" = Self::Time(TimeVariable::InputNoiseWidth),
    "driver_slew" = Self::Time(TimeVariable::DriverSlew),
    "#;
  let name: syn::Ident = syn::Ident::new("MyStructName", proc_macro2::Span::call_site());
  let args: MacroArgs = parse_str(input).unwrap();
  let trie_tree = crate::trie_tree::build_tree(
    &args
      .pairs
      .into_iter()
      .map(|pair| {
        let exp = pair.value;
        (quote! {#exp}, pair.key.value())
      })
      .collect::<Vec<_>>(),
  );
  let parse_match_quote = crate::trie_tree::quote_tree(&name, &trie_tree);
  println!("{}", parse_match_quote)
}
