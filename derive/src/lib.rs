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

    fn reconstruct_has_ty(&self, top: &syn::Ident) -> Result<TokenStream> {
        let ident = &self.ident;

        Ok(quote! {
            #top::#ident(t) => t.has_ty(ty),
        })
    }

    fn reconstruct_cast_into(&self, top: &syn::Ident) -> Result<TokenStream> {
        let ident = &self.ident;
        Ok(quote! {
            // TODO: It assumes the variant has one anonymous field
            #top::#ident(t) => t.cast_into(),
        })
    }

    fn reconstruct_build_from_branch(
        &self,
        top: &syn::Ident,
        from: &syn::Type,
    ) -> Result<TokenStream> {
        let ident = &self.ident;
        let field_ty = self.field_ty()?;

        Ok(quote! {
            .or_else(|from| (&mut &mut &bubble::Marker::<#from, #field_ty>::default()).sbuild_from(from).map(#top::#ident))
        })
    }

    fn reconstruct_from(&self, top: &syn::Ident) -> Result<TokenStream> {
        let field_ty = self.field_ty()?;
        if self.fields.fields.iter().any(|f| f.has_from()) {
            return Ok(quote! {});
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
        let field_ty = self.field_ty()?;
        let branches = top
            .variants()?
            .iter()
            .map(|v| v.reconstruct_build_from_branch(&top.ident, field_ty))
            .collect::<Result<Vec<_>>>()?;
        let top = &top.ident;

        let from = self.reconstruct_from(top)?;

        Ok(quote! {
            #from
            impl bubble::BuildFrom<#field_ty> for #top {
                fn build_from(from: #field_ty) -> Result<Self, #field_ty> {
                    use bubble::SBuildFrom;

                    Err(from)
                        #(#branches)*

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

    fn reconstruct_cast_into(&self) -> Result<TokenStream> {
        let ident = &self.ident;
        let variants = self.variants()?;
        let has_ty = variants
            .iter()
            .map(|v| v.reconstruct_has_ty(ident))
            .collect::<Result<Vec<_>>>()?;

        let variants = variants
            .iter()
            .map(|v| v.reconstruct_cast_into(ident))
            .collect::<Result<Vec<_>>>()?;

        Ok(quote! {
            impl bubble::CastInto for #ident {
                fn has_ty(&self, ty: std::any::TypeId) -> bool {
                    match self {
                        #(#has_ty)*
                    }
                }

                fn cast_into(self) -> Box<dyn std::any::Any> {
                    match self {
                        #(#variants)*
                    }
                }
            }
        })
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
        let cast_into = self.reconstruct_cast_into()?;
        let build_from = self.reconstruct_build_from()?;

        Ok(quote! {
            #cast_into
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
