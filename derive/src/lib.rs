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

    #[darling(default)]
    bubble: bool,

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
            .filter(|f| f.from || f.bubble || !f.attrs.is_empty())
            .map(|f| &f.ty)
            .next()
            .ok_or_else(|| {
                darling::Error::custom(
                    "Variant should have at least one of: #[source], #[from], #[bubble]",
                )
                .with_span(&self.ident)
            })
    }

    fn reconstruct_build_bubble_from_branch(
        &self,
        top: &syn::Ident,
        // from: &syn::Type,
    ) -> Result<TokenStream> {
        let ident = &self.ident;
        let field_ty = self.field_ty()?;

        let field_ty = Self::extract_from_bubble(field_ty)?;
        Ok(quote! {
            .or_else(|from| Bubble::<#field_ty>::build(from).map(#top::#ident))
        })
    }

    fn extract_from_bubble(field_ty: &syn::Type) -> Result<&syn::Type> {
        while let syn::Type::Path(syn::TypePath { path, .. }) = field_ty {
            if let Some(last_segment) = path.segments.last() {
                if last_segment.ident == quote::format_ident!("Bubble") {
                    if let syn::PathArguments::AngleBracketed(
                        syn::AngleBracketedGenericArguments { args, .. },
                    ) = &last_segment.arguments
                    {
                        if let syn::GenericArgument::Type(ty) = &args[0] {
                            return Ok(ty);
                        }
                    }
                }
            }
        }
        Err(darling::Error::custom("Expected a Bubble type"))
    }

    fn reconstruct_from(&self, top: &syn::Ident) -> Result<TokenStream> {
        let mut field_ty = self.field_ty()?;
        if self.fields.fields.iter().any(|f| f.has_from()) {
            return Ok(quote! {});
        }
        if self.fields.fields.iter().any(|f| f.bubble) {
            field_ty = Self::extract_from_bubble(field_ty)?;
        }

        Ok(quote! {
            impl From<#field_ty> for #top {
                fn from(from: #field_ty) -> Self {
                    use bubble::BuildFrom;
                    Self::build_from(from).unwrap()
                }
            }
        })
    }

    fn reconstruct_build_from(&self, top: &BubbleInput) -> Result<TokenStream> {
        let mut field_ty = self.field_ty()?;
        let ident = &self.ident;
        let bubble_branches = top
            .variants()?
            .iter()
            .filter(|v| v.fields.fields.iter().any(|f| f.bubble))
            .map(|v| v.reconstruct_build_bubble_from_branch(&top.ident))
            .collect::<Result<Vec<_>>>()?;
        let top = &top.ident;

        let from = self.reconstruct_from(top)?;

        let mut final_from = quote! {
            Ok::<#top, #field_ty>(#top::#ident(from))
        };

        if self.fields.fields.iter().any(|f| f.bubble) {
            field_ty = Self::extract_from_bubble(field_ty)?;
            final_from = quote! {
                Bubble::<#field_ty>::build(from).map(#top::#ident)
            };
        }

        Ok(quote! {
            #from
            impl bubble::BuildFrom<#field_ty> for #top {
                fn build_from(from: #field_ty) -> Result<Self, #field_ty> {

                    Err(from)
                        #(#bubble_branches)*
                        .or_else(
                            |from| {
                                #final_from
                            }
                        )


                }
            }
        })
    }
}

impl BubbleInput {
    fn variants(&self) -> Result<&Vec<BubbleVariant>> {
        match &self.data {
            ast::Data::Enum(variants) => Ok(variants),
            _ => Err(darling::Error::custom("Only enums are supported").with_span(&self.ident)),
        }
    }

    fn reconstruct_build_from(&self) -> Result<TokenStream> {
        let variants = self.variants()?;
        let variants = variants
            .iter()
            .map(|v| v.reconstruct_build_from(self))
            .collect::<Result<Vec<_>>>()?;

        Ok(quote! {
            #(#variants)*
        })
    }

    fn reconstruct(&self) -> Result<TokenStream> {
        // let cast_into = self.reconstruct_cast_into()?;
        let build_from = self.reconstruct_build_from()?;

        Ok(quote! {
            // #cast_into
            #build_from
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
