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

#[derive(FromField, Debug)]
#[darling(attributes(bubble), forward_attrs(from, source))]
struct BubbleField {
    // ident: Option<syn::Ident>,
    ty: syn::Type,
    attrs: Vec<syn::Attribute>,
}

impl BubbleVariant {
    fn field_ty(&self) -> &syn::Type {
        self.fields
            .fields
            .iter()
            .inspect(|f | {  dbg!(f); } )
            .filter(|f | !f.attrs.is_empty())
            .map(|f| &f.ty)
            .next()
            .expect("Variant should have at least one of: #[source], #[from]")
    }

    fn from_branch_reconstruction(&self, top: &syn::Ident, bot: &syn::Type) -> TokenStream {
        let field_ty = self.field_ty();

        let ident = &self.ident;

        quote! {
            .or_else(|bot: #bot| (&mut &mut &Marker::<#bot, #field_ty>::new()).sbubble(bot).map(#top::#ident))
        }
    }

    fn from_reconstruction(&self, top: &BubbleInput) -> TokenStream {
        let field_ty = self.field_ty();
        let reconstruction = top.from_reconstruction(&field_ty);
        let top = &top.ident;

        quote! {
            impl From<#field_ty> for #top {
                fn from(bot: #field_ty) -> Self {
                    #reconstruction
                }
            }
        }
    }

    fn reconstruction(&self, top: &syn::Ident) -> TokenStream {
        let field_ty = self.field_ty();
        let ident = &self.ident;

        quote! {
            impl Bubble<#top> for #field_ty {
                fn bubble(t: #top) -> Result<Self, #top> {
                    match t {
                        // TODO it assumes every variant has a single anonymous field
                        #top::#ident (a) => Ok(a),
                        _ => Err(t)
                    }
                }
            }

            impl SBubble<#top, #field_ty> for &mut &Marker<#top, #field_ty> {
                fn sbubble(&self, t: #top) -> Result<#field_ty, #top> {
                    #field_ty::bubble(t)
                }
            }
        }
    }
}


impl BubbleInput {
    fn from_reconstruction(&self, bot: &syn::Type) -> TokenStream {
        let top = &self.ident;
        let variants = match &self.data {
            ast::Data::Enum(variants) => variants,
            _ => panic!("TODO"),
        };
        let variants = variants
            .iter()
            .map(|v| v.from_branch_reconstruction(&top, bot));

        quote! {
            Err(bot)
                #(#variants)*
                .unwrap()
        }
    }

    fn reconstruction(&self) -> TokenStream {
        let ident = &self.ident;
        let variants = match &self.data {
            ast::Data::Enum(variants) => variants,
            _ => panic!("TODO"),
        };

        let froms = variants
            .iter()
            .map(|v| v.from_reconstruction(&self))
            .collect::<Vec<_>>();

        let variants = variants
            .into_iter()
            .map(|v| v.reconstruction(&ident));



        quote! {
            #(#variants)*
            #(#froms)*
        }
    }
}

#[proc_macro_derive(Bubble)]
pub fn derive_bubble(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(_item as syn::DeriveInput);
    let input = BubbleInput::from_derive_input(&input).expect("TODO");
    input.reconstruction().into()
}
