extern crate proc_macro;

use darling::{FromDeriveInput, FromField, FromVariant, ast, util};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(FromDeriveInput)]
struct BubbleInput {
    ident: syn::Ident,
    data: ast::Data<BubbleVariant, util::Ignored>,
}

#[derive(FromVariant)]
struct BubbleVariant {
    ident: syn::Ident,
    fields: ast::Fields<BubbleField>,
}

impl BubbleVariant {
    fn reconstruction(self, top: syn::Ident) -> TokenStream {
        let ident = self.ident;
        let field_ty = self
            .fields
            .fields
            .into_iter()
            .map(|f| f.ty)
            .next()
            .expect("TODO");
        quote! {
            impl Bubble<#top> for #field_ty {
                fn bubble(t: #top) -> Result<Self, #top> {
                    match t {
                        #top::#ident (a) => Ok(a),
                        _ => Err(t)
                    }
                }
            }
        }
    }
}

#[derive(FromField)]
struct BubbleField {
    // ident: Option<syn::Ident>,
    ty: syn::Type,
}

impl BubbleInput {
    fn reconstruction(self) -> TokenStream {
        let ident = self.ident;
        let variants = self.data.take_enum().unwrap();
        let variants = variants
            .into_iter()
            .map(|v| v.reconstruction(ident.clone()));
        quote! {
            #(#variants)*
        }
    }
}

#[proc_macro_derive(Bubble)]
pub fn derive_bubble(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(_item as syn::DeriveInput);
    let input = BubbleInput::from_derive_input(&input).expect("TODO");
    input.reconstruction().into()
}
