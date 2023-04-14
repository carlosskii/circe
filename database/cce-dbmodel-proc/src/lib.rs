extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;

use proc_macro_error::{proc_macro_error, abort};


#[proc_macro_error]
#[proc_macro_attribute]
pub fn dbnode(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let ast: syn::ItemStruct = syn::parse_macro_input!(item as syn::ItemStruct);

  let name = &ast.ident;
  let visibility = &ast.vis;
  let fields = match &ast.fields {
    syn::Fields::Named(fields) => &fields.named,
    _ => abort!(ast, "Only named fields are supported"),
  };

  fields.iter().for_each(|field| {
    let ident = &field.ident;

    if let Some(ident) = ident {
      if ident == "__id" {
        abort!(field, "Field `__id` is reserved");
      }
    }
  });

  let new_fields: Vec<_> = fields.iter().map(|field| {
    let ident = &field.ident;
    let ty = &field.ty;

    quote! {
      #ident: #ty
    }
  }).collect();

  let field_names: Vec<_> = fields.iter().map(|field| {
    let ident = &field.ident;

    quote! {
      #ident
    }
  }).collect();

  let output = quote! {
    #visibility struct #name {
      pub __id: u64,
      #fields
    }

    impl #name {
      pub fn new(#(#new_fields),*) -> Self {
        Self {
          __id: 0,
          #( #field_names: #field_names ),*
        }
      }
    }

    impl cce_dbmodel::Node for #name {
      fn get_id(&self) -> u64 {
        self.__id
      }
    }
  };

  output.into()
}