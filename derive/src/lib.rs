extern crate proc_macro;

use darling::{FromDeriveInput, FromField, FromVariant, Result, ast, util};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(FromDeriveInput)]
#[darling(attributes(bubble))]
struct BubbleInput {
    ident: syn::Ident,
    data: ast::Data<BubbleVariant, util::Ignored>,
}

#[derive(FromVariant)]
#[darling(attributes(bubble))]
struct BubbleVariant {
    ident: syn::Ident,
    fields: ast::Fields<BubbleField>,
}

#[derive(FromField, Debug)]
#[darling(attributes(bubble), forward_attrs(from, source))]
struct BubbleField {
    // ident: Option<syn::Ident>,
    ty: syn::Type,

    #[darling(default)]
    from: bool,

    attrs: Vec<syn::Attribute>,
}

impl BubbleField {
    fn has_from(&self) -> bool {
        self.attrs.iter().any(|a| a.path().is_ident("from"))
    }
}

impl BubbleVariant {
    fn field_ty(&self) -> Result<&syn::Type> {
        self.fields
            .fields
            .iter()
            // .inspect(|f | {  dbg!(f); } )
            .filter(|f| f.from || !f.attrs.is_empty())
            .map(|f| &f.ty)
            .next()
            .ok_or_else(|| {
                darling::Error::custom(
                    "Variant should have at least one of: #[source], #[from], #[bubble]",
                )
                .with_span(&self.ident)
            })
    }

    fn reconstruct_from_branch(&self, top: &syn::Ident, bot: &syn::Type) -> Result<TokenStream> {
        let field_ty = self.field_ty()?;

        let ident = &self.ident;

        Ok(quote! {
            .or_else(|bot: #bot| (&mut &mut &mut &mut &bubble::Marker::<#bot, #field_ty>::default()).sbubble(bot)
                // TODO: This assumes the variant has one anonymous field
                .map(#top::#ident)
            )
        })
    }

    fn reconstruct_from(&self, top: &BubbleInput) -> Result<TokenStream> {
        let field_ty = self.field_ty()?;
        let reconstruction = top.reconstruct_from(field_ty)?;
        let top = &top.ident;

        if self.fields.fields.iter().any(|f| f.has_from()) {
            return Ok(quote! {});
        }

        Ok(quote! {
            impl From<#field_ty> for #top {
                fn from(bot: #field_ty) -> Self {
                    use bubble::SBubble;
                    #reconstruction
                }
            }
        })
    }

    fn reconstruct(&self, top: &syn::Ident) -> Result<TokenStream> {
        let field_ty = self.field_ty()?;
        let ident = &self.ident;

        Ok(quote! {
            impl Bubble<#top, bubble::DeriveBubble> for #field_ty {
                fn bubble(t: #top) -> Result<Self, #top> {
                    match t {
                        // TODO it assumes every variant has a single anonymous field
                        #top::#ident (a) => Ok(a),
                        _ => Err(t)
                    }
                }
            }

            // impl bubble::SBubble<#top, #field_ty> for &mut &bubble::Marker<#top, #field_ty> {
            //     fn sbubble(&self, t: #top) -> Result<#field_ty, #top> {
            //         #field_ty::bubble(t)
            //     }
            // }
        })
    }
}

impl BubbleInput {
    fn reconstruct_from(&self, bot: &syn::Type) -> Result<TokenStream> {
        let top = &self.ident;
        let variants = match &self.data {
            ast::Data::Enum(variants) => variants,
            _ => {
                return Err(
                    darling::Error::custom("Only enums are supported").with_span(&self.ident)
                );
            }
        };
        let variants = variants
            .iter()
            .map(|v| v.reconstruct_from_branch(top, bot))
            .collect::<Result<Vec<_>>>()?;

        Ok(quote! {
            Err(bot)
                #(#variants)*
                .unwrap()
        })
    }

    fn reconstruct(&self) -> Result<TokenStream> {
        let ident = &self.ident;
        let variants = match &self.data {
            ast::Data::Enum(variants) => variants,
            _ => {
                return Err(
                    darling::Error::custom("Only enums are supported").with_span(&self.ident)
                );
            }
        };

        let froms = variants
            .iter()
            .map(|v| v.reconstruct_from(self))
            .collect::<Result<Vec<_>>>()?;

        let variants = variants
            .iter()
            .map(|v| v.reconstruct(ident))
            .collect::<Result<Vec<_>>>()?;

        Ok(quote! {
            #(#variants)*
            #(#froms)*
        })
    }
}

#[proc_macro_derive(Bubble, attributes(bubble))]
pub fn derive_bubble(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(_item as syn::DeriveInput);
    let input = match BubbleInput::from_derive_input(&input) {
        Ok(i) => i,
        Err(e) => {
            return e.write_errors().into();
        }
    };
    input
        .reconstruct()
        .unwrap_or_else(|e| e.write_errors())
        .into()
}
