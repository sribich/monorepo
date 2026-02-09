mod attributes;

use attributes::container::ContainerAttributes;
use attributes::variant::VariantAttributes;
use macro_util::ast::Input;
use macro_util::generics::generics_with_ident;
use macro_util::generics::generics_with_ident_and_bounds;
use macro_util::generics::where_clause_with_bounds;
use proc_macro::TokenStream;
use quote::quote;

pub fn derive(input: TokenStream) -> TokenStream {
    inner_derive(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn inner_derive(input: TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let input = syn::parse(input)?;
    let input = Input::from_syn(&input)?;

    let ident = input.ident();
    let generics = input.generics();

    let type_params = generics_with_ident(generics);
    let trait_bounds = generics_with_ident_and_bounds(generics);
    let where_bounds = where_clause_with_bounds(generics, |_| quote!());

    let (match_arms, code_arms) = match &input {
        Input::Struct(_) => todo!(),
        Input::Union(_) => todo!(),
        Input::Enum(input) => {
            let input = input.resolve_attributes::<ContainerAttributes>()?;

            input
                .variants
                .iter()
                .map(|variant| {
                    let refined_variant = variant.resolve_attributes::<VariantAttributes>()?;

                    let variant_name = &variant.name;

                    if variant.fields.len() != 1 {
                        return Err(syn::Error::new_spanned(
                            &variant.name,
                            "Variants must have a single unnamed field.",
                        ));
                    }

                    let field_ty = variant.fields.first().unwrap().ty;
                    let field_inner_ty = match &field_ty {
                        syn::Type::Path(inner) => &inner.path.segments.last().unwrap().arguments,
                        _ => todo!(),
                        syn::Type::Array(_) => todo!(),
                        syn::Type::BareFn(_) => todo!(),
                        syn::Type::Group(_) => todo!(),
                        syn::Type::ImplTrait(_) => todo!(),
                        syn::Type::Infer(_) => todo!(),
                        syn::Type::Macro(_) => todo!(),
                        syn::Type::Never(_) => todo!(),
                        syn::Type::Paren(_) => todo!(),
                        syn::Type::Ptr(_) => todo!(),
                        syn::Type::Reference(_) => todo!(),
                        syn::Type::Slice(_) => todo!(),
                        syn::Type::TraitObject(_) => todo!(),
                        syn::Type::Tuple(_) => todo!(),
                        syn::Type::Verbatim(_) => todo!(),
                    };

                    let status_code = refined_variant.attributes.status_code();
                    let code = refined_variant.attributes.code;

                    Ok((
                        quote! {
                            #variant_name(inner) => {
                                // tracing::error!("{:#?}", inner.source);

                                ApiResponse::<(), ApiErrorInternal #field_inner_ty>::failure(
                                    StatusCode::from_u16(#status_code).unwrap(),
                                    ApiErrorInternal {
                                        code: #code.into(),
                                        kind: inner,
                                    }
                                ).into_response()
                            }
                        },
                        quote! {
                            #variant_name(inner) => #code.to_owned()
                        },
                    ))
                })
                .collect::<syn::Result<(Vec<_>, Vec<_>)>>()?
        }
    };

    let match_arms = if match_arms.is_empty() {
        quote! {
            ApiResponse::<(), ApiErrorInternal<UnknownApiError>>::failure(
                StatusCode::from_u16(500).unwrap(),
                ApiErrorInternal {
                    code: "unknown".to_owned(),
                    kind: ApiErrorKind::<UnknownApiError>::error(UnknownApiError {}, ""),
                }
            ).into_response()
        }
    } else {
        quote! {
            match self {
                #(#match_arms),*
            }
        }
    };

    let code_arms = if code_arms.is_empty() {
        quote! {
            "unknown".to_owned()
        }
    } else {
        quote! {
            match self {
                #(#code_arms),*
            }
        }
    };

    // (StatusCode::OK).into_response()

    Ok(quote! {
        const _: () = {
            use axum::{http::StatusCode, response::{IntoResponse, Response}};
            use railgun::api::json::{ApiResponse, ApiErrorInternal, UnknownApiError};
            use #ident::*;

            #[automatically_derived]
            impl #trait_bounds IntoResponse for #ident #type_params #where_bounds {
                fn into_response(self) -> Response {
                    #match_arms
                }
            }

            #[automatically_derived]
            impl #trait_bounds #ident #type_params #where_bounds {
                pub fn code(&self) -> String {
                    #code_arms
                }
            }
        };
    })
}
