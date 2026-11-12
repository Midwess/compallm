use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Data, DeriveInput, Fields, GenericArgument, LitStr, Path, PathArguments, Type,
    parse_macro_input, parse_quote,
};

#[proc_macro_derive(IntoQuery, attributes(query))]
pub fn derive_into_query(input: TokenStream) -> TokenStream {
    expand(parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn option_inner(ty: &Type) -> Option<&Type> {
    let Type::Path(path) = ty else { return None };
    let segment = path.path.segments.last()?;
    if segment.ident != "Option" {
        return None;
    }
    let PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };
    match args.args.first()? {
        GenericArgument::Type(ty) => Some(ty),
        _ => None,
    }
}

fn expand(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let mut runtime: Path = parse_quote!(::compallm_query);
    for attr in &input.attrs {
        if !attr.path().is_ident("query") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("crate") {
                runtime = meta.value()?.parse()?;
                Ok(())
            } else {
                Err(meta.error("expected crate = path"))
            }
        })?;
    }
    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "IntoQuery requires a struct with named fields",
        ));
    };
    let Fields::Named(fields) = &data.fields else {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "IntoQuery requires named fields",
        ));
    };
    let mut generics = input.generics.clone();
    let mut statements = Vec::new();
    for field in &fields.named {
        let name = field.ident.as_ref().unwrap();
        let mut mapped: Option<LitStr> = None;
        let mut kind: Option<String> = None;
        for attr in &field.attrs {
            if !attr.path().is_ident("query") {
                continue;
            }
            attr.parse_nested_meta(|meta| {
                if mapped.is_some() || kind.is_some() {
                    return Err(meta.error("a field can have only one query mapping"));
                }
                if meta.path.is_ident("field") {
                    mapped = Some(meta.value()?.parse()?);
                } else {
                    let key = meta
                        .path
                        .get_ident()
                        .map(ToString::to_string)
                        .unwrap_or_default();
                    if ![
                        "and",
                        "or",
                        "not",
                        "skip",
                        "filter",
                        "order_by",
                        "pagination",
                    ]
                    .contains(&key.as_str())
                    {
                        return Err(meta.error(
                            "expected field, and, or, not, filter, order_by, pagination, or skip",
                        ));
                    }
                    kind = Some(key);
                }
                Ok(())
            })?;
        }
        let kind = kind.unwrap_or_else(|| {
            if mapped.is_none() && ["and", "or", "not"].iter().any(|value| name == *value) {
                name.to_string()
            } else {
                "field".into()
            }
        });
        if kind == "skip" {
            continue;
        }
        let inner = option_inner(&field.ty);
        let body = match kind.as_str() {
            "and" | "or" => {
                let variant = if kind == "and" {
                    quote!(And)
                } else {
                    quote!(Or)
                };
                quote! {
                    let mut __query_group = ::std::vec::Vec::new();
                    for __query_child in __query_value {
                        let __query_child: #runtime::Query = #runtime::IntoQuery::into_query(::std::boxed::Box::new(__query_child))?;
                        __query_group.push(__query_child.into_predicate()?);
                    }
                    __query_predicates.push(#runtime::Predicate::#variant(__query_group));
                }
            }
            "not" => quote! {
                let __query_child: #runtime::Query = #runtime::IntoQuery::into_query(::std::boxed::Box::new(__query_value))?;
                __query_predicates.push(#runtime::Predicate::Not(::std::boxed::Box::new(__query_child.into_predicate()?)));
            },
            "filter" => quote! {
                let __query_child: #runtime::Query = #runtime::IntoQuery::into_query(::std::boxed::Box::new(__query_value))?;
                __query_predicates.push(__query_child.into_predicate()?);
            },
            "order_by" => quote! {
                for __query_order in __query_value {
                    __query_ordering.push(::std::convert::TryInto::try_into(__query_order)?);
                }
            },
            "pagination" => quote! {
                __query_pagination = __query_value;
            },
            _ => {
                let mapped = mapped.unwrap_or_else(|| {
                    LitStr::new(name.to_string().trim_start_matches("r#"), name.span())
                });
                let value_type = inner.unwrap_or(&field.ty);
                generics.make_where_clause().predicates.push(parse_quote!(#value_type: ::std::convert::Into<::std::vec::Vec<#runtime::operation::Operation>>));
                quote! {
                    let __query_operations: ::std::vec::Vec<#runtime::operation::Operation> = ::std::convert::Into::into(__query_value);
                    for __query_operation in __query_operations {
                        __query_predicates.push(#runtime::Predicate::Field { field: ::std::string::String::from(#mapped), operation: __query_operation });
                    }
                }
            }
        };
        statements.push(if inner.is_some() {
            quote! { if let ::std::option::Option::Some(__query_value) = self.#name { #body } }
        } else {
            quote! { { let __query_value = self.#name; #body } }
        });
    }
    let name = &input.ident;
    generics
        .make_where_clause()
        .predicates
        .push(parse_quote!(Self: ::std::marker::Send));
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics #runtime::IntoQuery for #name #type_generics #where_clause {
            fn into_query(self: ::std::boxed::Box<Self>) -> ::std::result::Result<#runtime::Query, #runtime::QueryError> {
                let mut __query_predicates = ::std::vec::Vec::new();
                let mut __query_ordering = ::std::vec::Vec::new();
                let mut __query_pagination = #runtime::Pagination::default();
                #(#statements)*
                ::std::result::Result::Ok(#runtime::Query {
                    predicate: #runtime::Predicate::And(__query_predicates),
                    order_by: __query_ordering,
                    pagination: __query_pagination,
                })
            }
        }
    })
}
