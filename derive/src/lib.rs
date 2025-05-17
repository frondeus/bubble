
extern crate proc_macro;

use quote::quote;
use darling::FromDeriveInput;

#[derive(FromDeriveInput)]
struct BubbleInput {
    ident: syn::Ident,
}

impl BubbleInput {
    fn reconstruction(self) -> proc_macro2::TokenStream {
        quote! {
        }
    }
}

#[proc_macro_derive(Bubble)]
pub fn derive_bubble(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(_item as syn::DeriveInput);
    let input = BubbleInput::from_derive_input(&input).expect("TODO");
    input.reconstruction().into()
}