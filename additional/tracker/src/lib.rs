use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(TrackChanges, attributes(track))]
pub fn track_changes_derive(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let struct_name = &input.ident;
  let enum_name = format_ident!("{}Field", struct_name);

  let fields = match &input.data {
    Data::Struct(data) => match &data.fields {
      Fields::Named(named) => &named.named,
      _ => panic!("TrackChanges supports only named fields"),
    },
    _ => panic!("TrackChanges can only be derived for structs"),
  };

  let mut variants = Vec::new();
  let mut setters = Vec::new();

  for field in fields {
    let field_name = field.ident.as_ref().unwrap();

    if field_name == "changes" {
      continue;
    }

    let skip = field.attrs.iter().any(|attr| {
      attr.path().is_ident("track")
        && attr
          .parse_args::<Ident>()
          .map(|ident| ident == "skip")
          .unwrap_or(false)
    });
    if skip {
      continue;
    }

    let variant_name = format_ident!("{}", to_pascal_case(&field_name.to_string()));
    let marker_name = format_ident!("changed_{}", field_name);

    variants.push(quote! { #variant_name });
    setters.push(quote! {
        pub fn #marker_name(&mut self) {
            self.changes.push(#enum_name::#variant_name);
        }
    });
  }

  let expanded = quote! {
      #[derive(Debug, Clone, Copy, PartialEq, Eq)]
      pub enum #enum_name {
          #(#variants),*
      }

      impl #struct_name {
          #(#setters)*
      }
  };

  expanded.into()
}

fn to_pascal_case(s: &str) -> String {
  s.split('_')
    .map(|part| {
      let mut chars = part.chars();
      match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
      }
    })
    .collect()
}
