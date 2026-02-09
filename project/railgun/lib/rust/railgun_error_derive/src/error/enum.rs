use convert_case::Case;
use convert_case::Casing;
use macro_util::ast::Enum;
use macro_util::ast::variant::Variant;
use macro_util::generics::generics_with_ident;
use macro_util::generics::generics_with_ident_and_bounds;
use macro_util::generics::where_clause_with_type_bound;
use quote::format_ident;
use quote::quote;
use quote::quote_spanned;
use syn::Ident;
use syn::Path;
use syn::spanned::Spanned;

use crate::attributes::ContainerAttributes;
use crate::attributes::ContainerModuleAttribute;
use crate::attributes::FieldAttributes;
use crate::attributes::VariantAttributes;
use crate::ext::VariantExt;
use crate::get_crate_path;
use crate::meta::VariantMeta;

pub fn impl_enum(input: &Enum) -> syn::Result<proc_macro2::TokenStream> {
    let input = input.resolve_attributes::<ContainerAttributes>()?;

    let ty = &input.name;

    let crate_path = match &input.attributes.crate_path {
        Some(path) => path,
        None => &get_crate_path(input.node.span())?,
    };

    let trait_bounds = generics_with_ident_and_bounds(input.generics);
    let type_params = generics_with_ident(input.generics);
    let where_bounds = where_clause_with_type_bound(input.generics);

    let variants = input
        .variants
        .clone()
        .into_iter()
        .map(Variant::resolve::<VariantAttributes, VariantMeta>)
        .collect::<syn::Result<Vec<_>>>()?;

    let display_fn = impl_enum_display_fn(ty, &variants);
    let next_fn = impl_enum_next_fn(ty, &variants, crate_path);

    let submodule_name = input.attributes.module.as_ref().map(|module| match module {
        ContainerModuleAttribute::Default => input.name.to_string().to_case(Case::Snake),
        ContainerModuleAttribute::WithName(name) => name.clone(),
    });

    let (contexts, from_impls) = variants
        .iter()
        .map(|v| impl_enum_context(ty, v, crate_path /* submodule_name.as_ref() */))
        .collect::<(Vec<_>, Vec<_>)>();

    let contexts = if let Some(module) = &submodule_name {
        let name = format_ident!("{module}");

        quote! {
            pub mod #name {
                use super::*;

                #(#contexts)*
            }
        }
    } else {
        quote!(#(#contexts)*)
    };

    Ok(quote! {
        impl #trait_bounds core::error::Error for #ty #type_params #where_bounds {
        }

        impl #trait_bounds #crate_path::StackedError for #ty #type_params
        #where_bounds
        {
            #display_fn
            #next_fn
        }

        impl #trait_bounds ::core::fmt::Display for #ty #type_params #where_bounds {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                use #crate_path::StackedError;

                let mut buf = vec![];
                self.display(0, &mut buf);

                f.write_str(&buf.join("\n"))
            }
        }

        impl #trait_bounds core::fmt::Debug for #ty #type_params #where_bounds {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use #crate_path::StackedError;

                let mut buf = vec![];
                self.display(0, &mut buf);

                f.write_str(&buf.join("\n"))
            }
        }

        #contexts

        #(#from_impls)*
    })
}

fn impl_enum_display_fn(
    ty: &Ident,
    variants: &[Variant<'_, VariantAttributes, VariantMeta>],
) -> proc_macro2::TokenStream {
    let match_arms = variants
        .iter()
        .map(to_display_match_arm)
        .collect::<Vec<_>>();

    quote! {
        fn display(&self, layer: usize, buf: &mut Vec<String>) {
            use #ty::*;

            match self {
                #(#match_arms),*
            }
        }
    }
}

fn impl_enum_next_fn(
    ty: &Ident,
    variants: &[Variant<'_, VariantAttributes, VariantMeta>],
    crate_path: &Path,
) -> proc_macro2::TokenStream {
    let match_arms = variants.iter().map(to_next_match_arm).collect::<Vec<_>>();

    quote! {
        fn next(&self) -> Option<&dyn #crate_path::StackedError> {
            use #ty::*;

            match self {
                #(#match_arms),*
            }
        }
    }
}

fn impl_enum_context(
    ty: &Ident,
    variant: &Variant<'_, VariantAttributes, VariantMeta>,
    crate_path: &Path,
    // submodule: Option<&String>,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let name = format_ident!("{}Context", variant.name);
    let variant_ident = &variant.name;

    let error_constructor = quote! { #ty::#variant_ident };

    let location_impl = variant.meta.location_field.as_ref().map(|_| {
        let location_field = variant.attributes.location_field_ident();

        quote! { #location_field: #crate_path::Location::default(), }
    });

    let source_or_error = if variant.meta.has_source {
        quote!(source)
    } else {
        quote!(error)
    };

    let impl_from = variant
        .meta
        .source_field
        .as_ref()
        .or(variant.meta.error_field.as_ref())
        .filter(|field| {
            field
                .resolve_attributes::<FieldAttributes>()
                .unwrap()
                .attributes
                .impl_from
        });

    let from_impl = if let Some(field) = impl_from {
        let field_ty = field.ty;
        // let submodule_name = if let Some(name) = submodule {
        //     let ident = format_ident!("{name}");
        //     Some(quote!(#ident::))
        // } else {
        //     None
        // };

        Some(quote! {
            impl From<#field_ty> for #ty {
                #[track_caller]
                fn from(value: #field_ty) -> Self {
                    #error_constructor {
                        #source_or_error: value,
                        #location_impl
                    }
                }
            }
        })
    } else {
        None
    };

    let self_impl = {
        variant
            // .fields
            .meta
            .other_fields
            .iter()
            .map(|field| {
                let name = match &field.member {
                    syn::Member::Named(name) => name,
                    syn::Member::Unnamed(_) => unimplemented!(),
                };

                quote!(#name: ::core::convert::Into::into(self.#name),)
            })
            .collect::<Vec<_>>()
    };

    let context_generics = variant.generic_types();
    let context_name_with_generics = variant.name_with_generics(Some(name.clone()));
    let context_where_clause = variant.generic_where();

    let transform = if variant.meta.has_source || variant.meta.has_external_cause {
        let field = (variant.meta.source_field.as_ref()).or(variant.meta.error_field.as_ref()); // .or(&variant.error_field);

        field.map(|field| {
            let target_ty = field.ty;

            let field = field.resolve_attributes::<FieldAttributes>().unwrap();

            let (source_ty, transform) = if let Some(from) = &field.attributes.from {
                let transform = &from.wrapper;

                (&from.ty, quote! { |v| #transform(v) })
            } else {
                (target_ty, quote! { |v| v })
            };

            quote! {
                impl #context_generics #crate_path::IntoError<#ty> for #context_name_with_generics
                where
                    #ty: ::core::error::Error,
                    #context_where_clause
                {
                    type Source = #source_ty;

                    #[track_caller]
                    fn into_error(self, error: Self::Source) -> #ty {
                        let error: #target_ty = (#transform)(error);

                        #error_constructor {
                            #source_or_error: error,
                            #location_impl
                            #(#self_impl)*
                        }
                    }
                }
            }
        })
    } else {
        Some(quote! {
            impl #context_generics #crate_path::IntoError<#ty> for #context_name_with_generics
            where
                #ty: ::core::error::Error,
                #context_where_clause
            {
                type Source = #crate_path::NoneError;

                #[track_caller]
                fn into_error(self, error: Self::Source) -> #ty {
                    #error_constructor {
                        #location_impl
                        #(#self_impl)*
                    }
                }
            }
        })
    };

    let constructor = if !variant.meta.has_source && !variant.meta.has_external_cause {
        quote! {
            impl #context_generics #context_name_with_generics {
                #[must_use]
                #[track_caller]
                pub fn build(self) -> #ty
                where
                    #context_where_clause
                {
                    #error_constructor {
                        #location_impl
                        #(#self_impl)*
                    }
                }

                #[must_use]
                #[track_caller]
                pub fn fail<__T>(self) -> ::core::result::Result<__T, #ty>
                where
                    #context_where_clause
                {
                    ::core::result::Result::Err(self.build())
                }
            }
        }
    } else {
        quote! {}
    };

    let context_fields = variant
        .meta
        .other_fields
        .iter()
        .enumerate()
        .map(|(idx, field)| {
            let name = match &field.member {
                syn::Member::Named(name) => name,
                syn::Member::Unnamed(_) => unimplemented!(),
            };

            let ty = field.ty;
            let generic = format_ident!("__T{}", idx);

            quote!(pub #name: #generic)
        })
        .collect::<Vec<_>>();

    let struct_decl = if context_fields.is_empty() {
        quote! {
            pub struct #name;
        }
    } else {
        quote! {
            pub struct #context_name_with_generics
            // where
            //     #context_where_clause
            {
                #(#context_fields),*
            }
        }
    };

    (
        quote! {
            #struct_decl

            #transform
            #constructor
        },
        quote!(#from_impl),
    )
}

fn to_display_match_arm(
    variant: &Variant<'_, VariantAttributes, VariantMeta>,
) -> proc_macro2::TokenStream {
    let name = &variant.name;

    let display = if let Some(attr) = &variant.attributes.display {
        let str = &attr.fmt;
        let rest = &attr.paths;

        quote!(#str, #(#rest),*)
    } else {
        quote!("Missing display")
    };

    let fields = &variant
        .node
        .fields
        .iter()
        .map(|f| f.ident.clone().unwrap_or_else(|| Ident::new("_", f.span())))
        .collect::<Vec<_>>();

    let span = variant.node.span();

    let location_field = Ident::new(
        &variant
            .attributes
            .location
            .clone()
            .unwrap_or_else(|| "location".to_owned()),
        variant.node.span(),
    );

    match (
        variant.meta.has_location,
        variant.meta.has_source,
        variant.meta.has_external_cause,
    ) {
        (true, true, _) => quote_spanned! {span=>
            #[allow(unused_variables)]
            #name { #(#fields),*, } => {
                buf.push(format!("{layer}: {}, at {}", format!(#display), #location_field));
                source.display(layer + 1, buf);
            }
        },
        (true, false, true) => quote_spanned! {span=>
            #[allow(unused_variables)]
            #name { #(#fields),* } => {
                buf.push(format!("{layer}: {}, at {}", format!(#display), #location_field));
                buf.push(format!("{}: {:?}", layer + 1, error));
            }
        },
        (true, false, false) => quote_spanned! {span=>
            #[allow(unused_variables)]
            #name { #(#fields),* } => {
                buf.push(format!("{layer}: {}, at {}", format!(#display), #location_field));
            }
        },
        (false, true, _) => quote_spanned! {span=>
            #[allow(unused_variables)]
            #name { #(#fields),* } => {
                buf.push(format!("{layer}: {}", format!(#display)));
                source.display(layer + 1, buf);
            }
        },
        (false, false, true) => quote_spanned! {span=>
            #[allow(unused_variables)]
            #name { #(#fields),* } => {
                buf.push(format!("{layer}: {}", format!(#display)));
                buf.push(format!("{}: {:?}", layer + 1, error));
            }
        },
        (false, false, false) => quote_spanned! {span=>
            #[allow(unused_variables)]
            #name { #(#fields),* } => {
                buf.push(format!("{layer}: {}", format!(#display)));
            }
        },
    }
}

fn to_next_match_arm(
    variant: &Variant<'_, VariantAttributes, VariantMeta>,
) -> proc_macro2::TokenStream {
    let name = &variant.name;

    let fields = &variant
        .node
        .fields
        .iter()
        .map(|f| f.ident.clone().unwrap_or_else(|| Ident::new("_", f.span())))
        .collect::<Vec<_>>();

    let span = variant.node.span();

    if variant.meta.has_source {
        quote_spanned! {span=>
            #[allow(unused_variables)]
            #name { #(#fields),* } => {
                Some(source)
            }
        }
    } else {
        quote_spanned! {span=>
            #[allow(unused_variables)]
            #name { #(#fields),* } => {
                None
            }
        }
    }
}
