use darling::{Error, FromAttributes, FromMeta};
use macro_util::{
    ast::{Attributes, Input, Struct, StructKind, Unresolved},
    generics::{generics_with_ident, generics_with_ident_and_bounds, where_clause_with_type_bound},
};
use proc_macro::TokenStream;
use proc_macro_error2::abort;
use quote::{ToTokens, format_ident, quote};
use syn::{
    AngleBracketedGenericArguments, Expr, Meta, Token, Type, TypeGroup, TypePath, parse::Parser,
    parse_quote,
};

pub fn derive(input: TokenStream) -> TokenStream {
    inner_derive(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn inner_derive(input: TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let input = syn::parse(input)?;
    let input = Input::from_syn(&input)?;

    Ok(match input {
        Input::Struct(input) => impl_struct(input)?,
        Input::Enum(input) => abort!(input.node, "Component is not supported for enums.",),
        Input::Union(input) => abort!(input.node, "Component is not supported for unions.",),
    })
}

#[derive(Clone, Default, Debug, FromMeta)]
enum Scope {
    #[default]
    Static,
    Transient,
}

#[derive(Clone, Debug, FromAttributes)]
#[darling(attributes(component))]
struct ComponentAttributes {
    #[darling(default)]
    pub from_state: Option<State>,
    #[darling(default)]
    pub scope: Scope,
    #[darling(default)]
    pub implements: Implements,
}

#[derive(Clone, Debug)]
struct State(syn::Type);

impl FromMeta for State {
    fn from_meta(item: &Meta) -> darling::Result<Self> {
        if let Meta::Path(_) = item {
            return Ok(State(parse_quote!(crate::AppState)));
        }

        let Meta::List(list) = item else {
            return Err(Error::custom("Expected type path").with_span(item));
        };

        let list = syn::punctuated::Punctuated::<Type, Token![,]>::parse_terminated
            .parse2(list.tokens.clone())?;

        if list.len() > 1 {
            return Err(Error::custom("Expected a single state type").with_span(item));
        }

        Ok(State(list[0].clone()))
    }
}

#[derive(Clone, Default, Debug)]
struct Implements(Vec<ImplKind>);

#[derive(Clone, Debug)]
enum ImplKind {
    Single(syn::Type),
    Collection(syn::Type),
}

impl From<syn::Type> for ImplKind {
    fn from(value: syn::Type) -> Self {
        if let syn::Type::Path(TypePath { path, .. }) = &value
            && let syn::PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                args, ..
            }) = &path.segments.last().unwrap().arguments
        {
            let inner_type = syn::parse2(args.to_token_stream()).unwrap();

            if path.segments.last().unwrap().ident.to_string() == "Vec" {
                return ImplKind::Collection(inner_type);
            }
        }

        ImplKind::Single(value)
    }
}

impl FromMeta for Implements {
    fn from_meta(item: &Meta) -> darling::Result<Self> {
        let Meta::List(list) = item else {
            return Err(Error::custom("Expected type path").with_span(item));
        };

        let mut types = Vec::new();

        let list = syn::punctuated::Punctuated::<Type, Token![,]>::parse_terminated
            .parse2(list.tokens.clone())?;

        for item in list {
            types.push(item.into());
        }

        Ok(Self(types))
    }
}

impl Attributes for ComponentAttributes {
    fn from_syn(attributes: &[syn::Attribute]) -> syn::Result<Self> {
        Ok(Self::from_attributes(attributes)?)
    }
}

#[derive(Clone, Debug, FromAttributes)]
#[darling(attributes(inject))]
struct FieldAttributes {
    #[darling(default)]
    pub skip: bool,
    #[darling(default)]
    pub default: bool,
    pub value: Option<Expr>,
}

impl Attributes for FieldAttributes {
    fn from_syn(attributes: &[syn::Attribute]) -> syn::Result<Self> {
        Ok(Self::from_attributes(attributes)?)
    }
}

fn impl_struct(input: Struct) -> syn::Result<proc_macro2::TokenStream> {
    match input.kind {
        StructKind::Named => impl_named_struct(input),
        StructKind::Unnamed => todo!(),
        StructKind::Unit => todo!(),
    }
}

#[derive(Debug)]
enum InjectionType {
    Arc { inner: syn::Type },
    Reference { inner: syn::Type },
    // Option { element: Box<Self> },
    // Vec { item: Box<Self> },
    // Lazy { element: Box<Self> },
    PhantomData,
    Raw(syn::Expr),
    // Value { typ: syn::Type },
    Default,
}

fn get_injection_type(ty: &syn::Type) -> InjectionType {
    // matches!(bar, Some(x) if x > 2)
    if let syn::Type::Reference(r) = ty {
        return InjectionType::Reference {
            inner: r.elem.as_ref().clone(),
        };
    }

    // TODO(sr)
    //   if typepath.qself.is_some() || typepath.path.segments.last().unwrap().ident != "Arc" {
    //       return None;
    //   }

    if let syn::Type::Group(TypeGroup { elem, .. }) = ty {
        return get_injection_type(elem);
    }

    if let syn::Type::Path(TypePath { path, .. }) = ty
        && let syn::PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =
            &path.segments.last().unwrap().arguments
    {
        let inner_type = syn::parse2(args.to_token_stream()).unwrap();

        if let Some(result) = match &path.segments.last().unwrap().ident.to_string()[..] {
            "Arc" => Some(InjectionType::Arc { inner: inner_type }),
            // "Option" => Some(InjectionType::Option {
            //     element: Box::new(get_injection_type(&inner_type)),
            // }),
            // "Lazy" => Some(InjectionType::Lazy {
            //     element: Box::new(get_injection_type(&inner_type)),
            // }),
            // "Vec" => Some(InjectionType::Vec {
            //     item: Box::new(get_injection_type(&inner_type)),
            // }),
            "PhantomData" => Some(InjectionType::PhantomData),
            _ => None,
        } {
            return result;
        }
    }

    // TODO: Fix this
    panic!("Unknown type");
    // InjectionType::Value { typ: ty.clone() }
}

fn get_depdency_scope(injection_type: &InjectionType) -> proc_macro2::TokenStream {
    match injection_type {
        InjectionType::Arc { inner } | InjectionType::Reference { inner } => {
            quote!(injector.get::<#inner>()?)
        },
        // InjectionType::Option { element } => match element.as_ref() {
        //     InjectionType::Arc { inner } => todo!(),
        //     InjectionType::Value { typ } => ,
        //     _ => {
        //         unimplemented!("Only Option<Arc<>> and Option<Value> are supported")
        //     },
        // },
        // InjectionType::Vec { item } => todo!(),
        // InjectionType::Lazy { .. } => todo!(),
        InjectionType::PhantomData => {
            quote!(::std::marker::PhantomData)
        },
        InjectionType::Default => {
            quote!(Default::default())
        },
        // InjectionType::Value { .. } => todo!(),
        InjectionType::Raw(expr) => quote!(#expr),
    }
}

fn impl_named_struct(input: Struct) -> Result<proc_macro2::TokenStream, syn::Error> {
    let input = input.resolve::<ComponentAttributes, Unresolved, FieldAttributes, Unresolved>()?;

    let ty = &input.name;
    let vis = &input.node.vis;

    let scope = match input.attributes.scope {
        Scope::Static => quote!(::railgun::di::scope::Static),
        Scope::Transient => quote!(::railgun::di::scope::Transient),
    };

    let builder_name = format_ident!("{}Builder", quote!(#ty).to_string());

    let trait_bounds = generics_with_ident_and_bounds(input.generics);
    let where_bounds = where_clause_with_type_bound(input.generics);
    let type_params = generics_with_ident(input.generics);

    let ty_constructor = type_params
        .as_ref()
        .map(|type_params| quote!(:: #type_params));

    let (builder_args, builder_values) = input
        .fields
        .iter()
        .filter(|field| field.attributes.skip)
        .map(|field| {
            let ident = field
                .node
                .ident
                .as_ref()
                .expect("Guaranteed to be a named struct");
            let ty = field.ty;

            (quote!(#ident: #ty), quote!(#ident))
        })
        .collect::<(Vec<_>, Vec<_>)>();

    let (prepare_deps, provide_deps) = input
        .fields
        .iter()
        .map(|field| {
            let name = field.node.ident.as_ref().unwrap();
            let ty = field.ty;

            let injection_type = if field.attributes.default {
                InjectionType::Default
            } else if let Some(expr) = &field.attributes.value {
                InjectionType::Raw(expr.clone())
            } else {
                get_injection_type(&syn::parse2(quote!(#ty)).unwrap())
            };

            let get_dependency = get_depdency_scope(&injection_type);

            let prepare_statement = if field.attributes.skip {
                quote! {}
            } else {
                quote! {
                    let #name = #get_dependency;
                }
            };
            let provide_statement = if field.attributes.skip {
                quote! { self.#name.clone() }
            } else if let InjectionType::Reference { .. } = injection_type {
                quote!(#name.as_ref())
            } else {
                quote!(#name)
            };

            (prepare_statement, provide_statement)
        })
        .collect::<(Vec<_>, Vec<_>)>();

    let interface_bindings = input
        .attributes
        .implements
        .0
        .into_iter()
        .map(|it| match it {
            ImplKind::Single(inner) => {
                quote!(injector.bind::<#inner, #ty #type_params>().unwrap();)
            },
            ImplKind::Collection(inner) => {
                quote!(injector.bind_vec::<#inner, #ty #type_params>().unwrap();)
            },
        })
        .collect::<Vec<_>>();

    let builder_factory = if builder_args.is_empty() {
        let builder_ctor = if type_params.is_some() {
            quote!(#builder_name::#type_params)
        } else {
            quote!(#builder_name)
        };

        quote! {
            impl #trait_bounds ::railgun::di::Component for #ty #type_params #where_bounds {
                type Impl = #ty #type_params;
                type Builder = #builder_name #type_params;

                fn builder() -> Self::Builder {
                    #builder_ctor::new()
                }
            }
        }
    } else {
        quote! {
            impl #trait_bounds #ty #type_params #where_bounds {
                pub fn builder(
                    #(#builder_args),*
                ) -> #builder_name #type_params {
                    #builder_name #ty_constructor {
                        #(#builder_values),*
                    }
                }
            }
        }
    };

    let (builder_marker_field, builder_marker_value) = if type_params.is_some() {
        (
            Some(quote!(__marker: std::marker::PhantomData #type_params,)),
            Some(quote!(__marker: std::marker::PhantomData,)),
        )
    } else {
        (None, None)
    };

    let from_state = if let Some(State(state_ty)) = input.attributes.from_state {
        quote! {
            impl #trait_bounds axum::extract::FromRef<#state_ty> for #ty #type_params #where_bounds {
                fn from_ref(input: &#state_ty) -> Self {
                    #builder_name::new().build(&input.injector).unwrap()
                }
            }

            impl #trait_bounds axum::extract::FromRef<std::sync::Arc<#state_ty>> for #ty #type_params #where_bounds {
                fn from_ref(input: &std::sync::Arc<#state_ty>) -> Self {
                    #builder_name::new().build(&input.injector).unwrap()
                }
            }
        }
    } else {
        quote! {}
    };

    Ok(quote! {
        #from_state

        #[allow(dead_code)]
        #vis struct #builder_name #type_params #where_bounds {
            __scope: #scope,
            /*
             * #(#arg_override_fn_field),*
             */
            // __marker: std::marker::PhantomData #type_params
            #builder_marker_field
        }

        const _: () = {
            #builder_factory

            impl #trait_bounds #ty #type_params #where_bounds {}

            impl #trait_bounds #builder_name #type_params #where_bounds {
                /* #( #meta_vars )* */

                pub fn new(
                    /* #(#explicit_arg_decl),* */
                ) -> Self {
                    Self {
                        __scope: #scope::new(),
                        #builder_marker_value
                        // __marker: std::marker::PhantomData,
                        /* #(#arg_override_fn_field_ctor),* */
                    }
                }

                /* #( #arg_override_setters )* */

                pub fn build(&self, injector: &::railgun::di::Injector) -> Result<#ty #type_params, ::railgun::di::InjectionError> {
                    use ::railgun::di::Spec;

                    #( #prepare_deps )*

                    Ok(#ty #ty_constructor {
                        #( #provide_deps ),*
                        /* #( #arg_name: #arg_provide_dependency, ) */
                        // _marker: std::marker::PhantomData,
                    })
                }
            }

            impl #trait_bounds ::railgun::di::Builder for #builder_name #type_params #where_bounds {

                fn type_id(&self) -> ::std::any::TypeId {
                    ::std::any::TypeId::of::<#ty #type_params>()
                }

                fn type_name(&self) -> &'static str {
                    ::std::any::type_name::<#ty #type_params>()
                }

                fn as_any(
                    &self,
                    injector: &::railgun::di::Injector
                ) -> Result<::std::sync::Arc<dyn ::std::any::Any + Send + Sync>, ::railgun::di::InjectionError> {
                    Ok(::railgun::di::TypedBuilder::get(self, injector)?)
                }

                                /*
                fn interfaces(&self, clb: &mut dyn FnMut(&::dill::InterfaceDesc) -> bool) {
                    #(
                        if !clb(&::dill::InterfaceDesc {
                            type_id: ::std::any::TypeId::of::<#interfaces>(),
                            type_name: ::std::any::type_name::<#interfaces>(),
                        }) { return }
                    )*
                }

                fn metadata<'a>(&'a self, clb: & mut dyn FnMut(&'a dyn std::any::Any) -> bool) {
                    #( #meta_provide )*
                }

                fn check(&self, cat: &::dill::Catalog) -> Result<(), ::dill::ValidationError> {
                    use ::dill::DependencySpec;

                    let mut errors = Vec::new();
                    #(
                        if let Err(err) = #arg_check_dependency {
                            errors.push(err);
                        }
                    )*
                    if errors.len() != 0 {
                        Err(::dill::ValidationError { errors })
                    } else {
                        Ok(())
                    }
                }
                */
            }

            impl #trait_bounds ::railgun::di::TypedBuilder<#ty #type_params> for #builder_name #type_params #where_bounds {
                fn get(&self, injector: &railgun::di::Injector) -> Result<std::sync::Arc<#ty #type_params>, ::railgun::di::InjectionError> {
                    use ::railgun::di::Scope;

                    if let Some(instance) = self.__scope.get() {
                        return Ok(instance.downcast().unwrap())
                    }

                    let instance = ::std::sync::Arc::new(self.build(injector)?);

                    self.__scope.set(instance.clone());

                    Ok(instance)
                }

                fn bind_interfaces(&self, injector: &mut ::railgun::di::InjectorBuilder) {
                    #(#interface_bindings)*
                }
            }

            /*
            #(
                // Allows casting TypedBuilder<T> into TypedBuilder<dyn I> for all declared interfaces
                impl ::dill::TypedBuilderCast<#interfaces> for #builder_name
                {
                    fn cast(self) -> impl ::dill::TypedBuilder<#interfaces> {
                        struct _B(#builder_name);

                        impl ::dill::Builder for _B {
                            fn instance_type_id(&self) -> ::std::any::TypeId {
                                self.0.instance_type_id()
                            }
                            fn instance_type_name(&self) -> &'static str {
                                self.0.instance_type_name()
                            }
                            fn interfaces(&self, clb: &mut dyn FnMut(&::dill::InterfaceDesc) -> bool) {
                                self.0.interfaces(clb)
                            }
                            fn metadata<'a>(&'a self, clb: &mut dyn FnMut(&'a dyn std::any::Any) -> bool) {
                                self.0.metadata(clb)
                            }
                            fn get_any(&self, cat: &::dill::Catalog) -> Result<std::sync::Arc<dyn std::any::Any + Send + Sync>, ::dill::InjectionError> {
                                self.0.get_any(cat)
                            }
                            fn check(&self, cat: &::dill::Catalog) -> Result<(), ::dill::ValidationError> {
                                self.0.check(cat)
                            }
                        }

                        impl ::dill::TypedBuilder<#interfaces> for _B {
                            fn get(&self, cat: &::dill::Catalog) -> Result<::std::sync::Arc<#interfaces>, ::dill::InjectionError> {
                                match self.0.get(cat) {
                                    Ok(v) => Ok(v),
                                    Err(e) => Err(e),
                                }
                            }

                            fn bind_interfaces(&self, cat: &mut ::dill::CatalogBuilder) {
                                self.0.bind_interfaces(cat);
                            }
                        }

                        _B(self)
                    }
                }
            )*
            */
        };
    })
}
