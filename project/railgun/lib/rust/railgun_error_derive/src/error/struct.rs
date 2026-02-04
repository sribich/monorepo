use macro_util::{
    ast::{Struct, StructKind},
    generics::{generics_with_ident, generics_with_ident_and_bounds, where_clause_with_type_bound},
};
use quote::{format_ident, quote, quote_spanned};
use syn::{Ident, Path, spanned::Spanned};

use crate::{
    attributes::{ContainerAttributes, FieldAttributes},
    get_crate_path,
    meta::{FieldMeta, StructMeta},
};

type TypedStruct<'syn> =
    Struct<'syn, ContainerAttributes, StructMeta<'syn>, FieldAttributes, FieldMeta>;

pub fn impl_struct(input: Struct) -> syn::Result<proc_macro2::TokenStream> {
    let input = input.resolve::<ContainerAttributes, StructMeta, FieldAttributes, FieldMeta>()?;

    match input.kind {
        StructKind::Named => (),
        StructKind::Unit => (),
        StructKind::Unnamed => {
            return Err(syn::Error::new_spanned(
                input.node,
                "Tuple structs are not supported targets for Error derivation.",
            ));
        },
    };

    match input.kind {
        StructKind::Named => {
            let ty = &input.name;

            let crate_path = match &input.attributes.crate_path {
                Some(path) => path,
                None => &get_crate_path(input.node.span())?,
            };

            let trait_bounds = generics_with_ident_and_bounds(input.generics);
            let where_bounds = where_clause_with_type_bound(input.generics);
            let type_params = generics_with_ident(input.generics);

            let display_fn = impl_struct_display_fn(/* ty, */ &input);
            let next_fn = impl_struct_next_fn(&input, crate_path);

            let context = impl_struct_context(ty, &input, crate_path);

            Ok(quote! {
                impl #trait_bounds core::error::Error for #ty #type_params #where_bounds {
                }

                impl #trait_bounds #crate_path::StackedError for #ty #type_params #where_bounds {
                    #display_fn
                    #next_fn
                }

                impl #trait_bounds core::fmt::Display for #ty #type_params #where_bounds {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

                #context
            })
        },
        StructKind::Unnamed => todo!(),
        StructKind::Unit => todo!(),
    }
}

fn impl_struct_context(
    ty: &Ident,
    r#struct: &TypedStruct<'_>,
    crate_path: &Path,
) -> proc_macro2::TokenStream {
    let name = format_ident!("{}Context", &r#struct.name);
    let ident = &r#struct.name;

    let error_constructor = quote! { #ident };

    let location_impl = r#struct.meta.location_field.as_ref().map(|_| {
        quote! { location: #crate_path::Location::default(), }
    });

    let self_impl = {
        r#struct
            .meta
            .other_fields
            .iter()
            .map(|field| {
                let name = match &field.member {
                    syn::Member::Named(name) => name,
                    syn::Member::Unnamed(_) => unimplemented!(),
                };

                quote!(#name: self.#name,)
            })
            .collect::<Vec<_>>()
    };

    let transform = if r#struct.meta.has_source || r#struct.meta.has_external_cause {
        let field = (r#struct.meta.source_field.as_ref()).or(r#struct.meta.error_field.as_ref()); // .or(&variant.error_field);

        field.map(|field| {
            let target_ty = field.ty;

            let source_or_error = if r#struct.meta.has_source {
                quote!(source)
            } else {
                quote!(error)
            };

            let (source_ty, transform) = if let Some(from) = &field.attributes.from {
                let transform = &from.wrapper;

                (&from.ty, quote! { |v| #transform(v) })
            } else {
                (target_ty, quote! { |v| v })
            };

            quote! {
                impl #crate_path::IntoError<#ty> for #name {
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
            impl #crate_path::IntoError<#ty> for #name {
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

    let constructor = if !r#struct.meta.has_source && !r#struct.meta.has_external_cause {
        quote! {
            impl #name {
                #[must_use]
                #[track_caller]
                pub fn build(self) -> #ty {
                    #error_constructor {
                        #location_impl
                        #(#self_impl)*
                    }
                }

                #[must_use]
                #[track_caller]
                pub fn fail<__T>(self) -> ::core::result::Result<__T, #ty> {
                    ::core::result::Result::Err(self.build())
                }
            }
        }
    } else {
        quote! {}
    };

    let context_fields = r#struct
        .meta
        .other_fields
        .iter()
        .map(|field| {
            let name = match &field.member {
                syn::Member::Named(name) => name,
                syn::Member::Unnamed(_) => unimplemented!(),
            };

            let ty = field.ty;

            quote!(pub #name: #ty,)
        })
        .collect::<Vec<_>>();

    quote! {
        pub struct #name {
            #(#context_fields)*
        }

        #transform
        #constructor
    }
}

fn impl_struct_display_fn(
    // ty: &Ident,
    r#struct: &TypedStruct<'_>,
) -> proc_macro2::TokenStream {
    let body = to_display_body(r#struct);

    quote! {
        fn display(&self, layer: usize, buf: &mut Vec<String>) {
            #body
        }
    }
}

fn impl_struct_next_fn(r#struct: &TypedStruct<'_>, crate_path: &Path) -> proc_macro2::TokenStream {
    let body = to_next_body(r#struct);

    quote! {
        fn next(&self) -> Option<&dyn #crate_path::StackedError> {
            #body
        }
    }
}

pub fn to_display_body(node: &TypedStruct<'_>) -> proc_macro2::TokenStream {
    // let name = &node.name;

    let display = if let Some(attr) = &node.attributes.display {
        let str = &attr.fmt;
        let rest = &attr.paths;

        quote!(#str, #(#rest),*)
    } else {
        quote!("{self:?}")
    };

    // let fields = &node
    //     .data
    //     .fields
    //     .iter()
    //     .map(|f| f.ident.clone().unwrap_or_else(|| Ident::new("_", f.span())))
    //     .collect::<Vec<_>>();

    let span = node.data.struct_token.span();

    match (
        node.meta.has_location,
        node.meta.has_source,
        node.meta.has_external_cause,
    ) {
        (true, true, _) => quote_spanned! {span=>
            buf.push(format!("{layer}: {}, at {}", format!(#display), self.location));
            source.display(layer + 1, buf);
        },
        (true, false, true) => quote_spanned! {span=>
            buf.push(format!("{layer}: {}, at {}", format!(#display), self.location));
            buf.push(format!("{}: {:?}", layer + 1, error));
        },
        (true, false, false) => quote_spanned! {span=>
            buf.push(format!("{layer}: {}, at {}", format!(#display), self.location));
        },
        (false, true, _) => quote_spanned! {span=>
            buf.push(format!("{layer}: {}", format!(#display)));
            source.display(layer + 1, buf);
        },
        (false, false, true) => quote_spanned! {span=>
            buf.push(format!("{layer}: {}", format!(#display)));
            buf.push(format!("{}: {:?}", layer + 1, error));
        },
        (false, false, false) => quote_spanned! {span=>
            buf.push(format!("{layer}: {}", format!(#display)));
        },
    }
}

pub fn to_next_body(node: &TypedStruct<'_>) -> proc_macro2::TokenStream {
    // let name = &node.name;
    //
    // let fields = &node
    //     .data
    //     .fields
    //     .iter()
    //     .map(|f| f.ident.clone().unwrap_or_else(|| Ident::new("_", f.span())))
    //     .collect::<Vec<_>>();

    let span = node.data.struct_token.span();

    if node.meta.has_source {
        quote_spanned! {span=>
            Some(source)
        }
    } else {
        quote_spanned! {span=>
            None
        }
    }
}
