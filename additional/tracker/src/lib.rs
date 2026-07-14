use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::parse_quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

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

  let mut variant_names = Vec::new();
  let mut setter_names = Vec::new();

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

    variant_names.push(variant_name);
    setter_names.push(marker_name);
  }

  let bit_count = variant_names.len();
  if bit_count > 64 {
    panic!("TrackChanges supports at most 64 tracked fields");
  }

  let mask_ty: syn::Type = if bit_count <= 8 {
    parse_quote!(u8)
  } else if bit_count <= 16 {
    parse_quote!(u16)
  } else if bit_count <= 32 {
    parse_quote!(u32)
  } else {
    parse_quote!(u64)
  };

  let variant_defs = variant_names.iter().enumerate().map(|(i, v)| {
    let shift = i as u32;
    quote! { #v = 1 << #shift }
  });

  let setters = variant_names
    .iter()
    .zip(setter_names.iter())
    .map(|(variant, marker)| {
      quote! {
          pub fn #marker(&mut self) {
              self.changes |= #enum_name::#variant as #mask_ty;
          }
      }
    });

  let all_mask = quote! {
      0 #( | (#enum_name::#variant_names as #mask_ty) )*
  };

  let expanded = quote! {
      #[repr(#mask_ty)]
      #[derive(Debug, Clone, Copy, PartialEq, Eq)]
      pub enum #enum_name {
          #(#variant_defs),*
      }

      impl #enum_name {
          pub const ALL: #mask_ty = #all_mask;
      }

      impl #struct_name {
          #(#setters)*

          pub fn is_changed(&self, field: #enum_name) -> bool {
              self.changes & (field as #mask_ty) != 0
          }

          pub fn has_changes(&self) -> bool {
              self.changes != 0
          }

          pub fn reset_changes(&mut self) {
              self.changes = 0;
          }
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
