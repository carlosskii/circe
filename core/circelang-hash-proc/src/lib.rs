/*

Copyright (C) 2023 Carlos Kieliszewski

This file is part of the Circe Project.

Circe is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option)
any later version.

Circe is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
Circe. If not, see <https://www.gnu.org/licenses/>. 

*/

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

use syn::{
    Ident, DeriveInput,
    Data, DataStruct, FieldsNamed, FieldsUnnamed, DataEnum
};


#[proc_macro_derive(CirceHash)]
pub fn circehash_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let name = ast.ident.clone();

    match ast.data {
        Data::Struct(data) => circehash_derive_struct(name, data),
        Data::Enum(data) => circehash_derive_enum(name, data),
        Data::Union(_) => panic!("Unions are not supported")
    }
}

fn circehash_derive_struct(name: Ident, data: DataStruct) -> TokenStream {
    match data.fields {
        syn::Fields::Named(f) => circehash_derive_struct_named(name, f),
        syn::Fields::Unnamed(f) => circehash_derive_struct_unnamed(name, f),
        syn::Fields::Unit => circehash_derive_struct_unit(name)
    }
}

fn circehash_derive_struct_named(name: Ident, f: FieldsNamed) -> TokenStream {
    let (fields_i, fields): (Vec<usize>, Vec<Ident>) = f.named.iter().map(|f| {
        f.ident.clone().unwrap()
    }).enumerate().unzip();

    quote! {
        impl ::circelang_hash::CirceHash for #name {
            fn hash(&self) -> u64 {
                let mut hash: u64 = 0;

                #(hash ^= self.#fields.hash().rotate_right(#fields_i as u32 % 64);)*

                hash
            }
        }
    }.into()
}

fn circehash_derive_struct_unnamed(name: Ident, f: FieldsUnnamed) -> TokenStream {
    let fields_i = (0..f.unnamed.len()).collect::<Vec<usize>>();

    quote! {
        impl ::circelang_hash::CirceHash for #name {
            fn hash(&self) -> u64 {
                let mut hash: u64 = 0;

                #(hash ^= self.#fields_i.hash().rotate_right(#fields_i as u32 % 64);)*

                hash
            }
        }
    }.into()
}

fn circehash_derive_struct_unit(name: Ident) -> TokenStream {
    quote! {
        impl ::circelang_hash::CirceHash for #name {
            fn hash(&self) -> u64 {
                0
            }
        }
    }.into()
}

fn circehash_derive_enum(name: Ident, data: DataEnum) -> TokenStream {
    let variants = data.variants.iter().map(|v| {
        let ident = v.ident.clone();
        let data = v.fields.clone();

        match data {
            syn::Fields::Named(f) => circehash_derive_enum_variant_named(ident, f),
            syn::Fields::Unnamed(f) => circehash_derive_enum_variant_unnamed(ident, f),
            syn::Fields::Unit => circehash_derive_enum_variant_unit(ident)
        }
    });

    quote! {
        impl ::circelang_hash::CirceHash for #name {
            fn hash(&self) -> u64 {
                match self {
                    #(#variants,)*
                }
            }
        }
    }.into()
}

fn circehash_derive_enum_variant_named(name: Ident, f: FieldsNamed) -> proc_macro2::TokenStream {
    let (fields_i, fields): (Vec<usize>, Vec<Ident>) = f.named.iter().map(|f| {
        f.ident.clone().unwrap()
    }).enumerate().unzip();

    quote! {
        Self::#name { #(#fields,)* } => {
            let mut hash: u64 = 0;

            #(hash ^= #fields.hash().rotate_right(#fields_i as u32 % 64);)*

            hash
        }
    }
}

fn circehash_derive_enum_variant_unnamed(name: Ident, f: FieldsUnnamed) -> proc_macro2::TokenStream {
    let fields_i = (0..f.unnamed.len()).collect::<Vec<usize>>();
    let fields = (0..f.unnamed.len()).map(|i| {
        Ident::new(&format!("f{}", i), proc_macro2::Span::call_site())
    }).collect::<Vec<Ident>>();

    quote! {
        Self::#name ( #(#fields,)* ) => {
            let mut hash: u64 = 0;

            #(hash ^= #fields.hash().rotate_right(#fields_i as u32 % 64);)*

            hash
        }
    }
}

fn circehash_derive_enum_variant_unit(name: Ident) -> proc_macro2::TokenStream {
    quote! {
        Self::#name => 0
    }
}
