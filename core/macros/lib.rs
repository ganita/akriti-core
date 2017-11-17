/*
 * Copyright 2017 Sreejith Krishnan R
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/


extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;
use quote::{Tokens};

#[proc_macro_derive(Immutable)]
pub fn make_immutable(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    let gen = impl_make_immutable(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_make_immutable(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    let builder_name = format!("{}Copier", name.as_ref());
    let builder_name = syn::parse_ident(&builder_name).unwrap();

    let fields = match ast.body {
        syn::Body::Struct(ref data) => data.fields(),
        _ => panic!("Immutable can only be used with structs"),
    };

    let mut tokens = Tokens::new();

    for field in fields {
        let attrs = &field.attrs;
        let ident = &field.ident;
        let ty = &field.ty;
        tokens.append(quote!(
            #(#attrs)* #ident: Option<#ty>,
        ))
    }

    let mut setters = Tokens::new();

    for field in fields {
        let ident = &field.ident;
        let ty = &field.ty;
        setters.append(quote! {
            pub fn #ident<'a>(&'a mut self, value: #ty) -> &'a mut #builder_name {
                self.#ident = Some(value);
                self
            }
        })
    }

    let mut copiers = Tokens::new();
    for field in fields {
        let ident = &field.ident;

        copiers.append(quote!{
            if self.#ident.is_some() {
                base.#ident = self.#ident.unwrap();
            }
        })
    }

    let mut getters = Tokens::new();
    for field in fields {
        let ident = &field.ident;
        let ty = &field.ty;

        getters.append(quote! {
            pub fn #ident(&self) -> &#ty {
                &self.#ident
            }
        })
    }

    let base_identifier_name = String::from(builder_name.as_ref()).to_lowercase();
    let base_identifier = syn::parse_ident(&base_identifier_name).unwrap();

    quote! {
        #[derive(Default)]
        pub struct #builder_name {
            #tokens
            #base_identifier: Option<#name>
        }

        impl #builder_name {
            #setters

            pub fn copy(self) -> #name {
                let mut base = self.#base_identifier.unwrap();

                #copiers

                base
            }
        }

        impl #name {
            pub fn copier(&self) -> #builder_name {
                let mut builder = #builder_name::default();
                builder.#base_identifier = Some(self.clone());
                builder
            }

            #getters
        }
    }
}
