use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Meta, parse_macro_input, parse_quote};

/// Export Validify field rules through Utoipa. Custom validators must supply a
/// sibling `<function>_contract()` returning JSON Schema constraints.
#[proc_macro_derive(ContractSchema, attributes(validate, modify, schema))]
pub fn contract_schema(input: TokenStream) -> TokenStream {
    expand(parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn expand(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    if !input.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &input,
            "ContractSchema requires a concrete input struct",
        ));
    }
    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new_spanned(
            &input,
            "ContractSchema requires a struct",
        ));
    };
    let Fields::Named(fields) = &data.fields else {
        return Err(syn::Error::new_spanned(
            &input,
            "ContractSchema requires named fields",
        ));
    };
    let name = &input.ident;
    let mut helper = input.clone();
    helper.ident = format_ident!("__{}Contract", name);
    helper.vis = syn::Visibility::Inherited;
    helper.attrs.retain(|a| {
        a.path().is_ident("serde") || a.path().is_ident("doc") || a.path().is_ident("schema")
    });
    helper
        .attrs
        .insert(0, parse_quote!(#[derive(::utoipa::ToSchema)]));
    let Data::Struct(helper_data) = &mut helper.data else {
        unreachable!()
    };
    for field in &mut helper_data.fields {
        field.attrs.retain(|a| {
            a.path().is_ident("serde") || a.path().is_ident("doc") || a.path().is_ident("schema")
        });
    }
    let deny_unknown = input.attrs.iter().any(|a| {
        a.path().is_ident("serde") && quote!(#a).to_string().contains("deny_unknown_fields")
    });
    let helper_name = &helper.ident;
    let mut updates = Vec::new();
    for attr in &input.attrs {
        if attr.path().is_ident("validate") {
            let mut function = attr.parse_args::<syn::Path>()?;
            let last = function.segments.last_mut().unwrap();
            last.ident = format_ident!("{}_contract", last.ident);
            updates
                .push(quote! { ::compallm_contract::merge_constraints(&mut schema, #function()); });
        }
    }
    for field in &fields.named {
        let ident = field.ident.as_ref().unwrap();
        // Reject unsupported validation attributes instead of exporting a weaker contract.
        let mut updates_for_field = Vec::new();
        for attr in &field.attrs {
            if attr.path().is_ident("validate") {
                attr.parse_nested_meta(|rule| {
                    if rule.path.is_ident("length") || rule.path.is_ident("range") {
                        let length = rule.path.is_ident("length");
                        if length && !matches!(&field.ty, syn::Type::Path(p) if p.path.is_ident("String")) {
                            return Err(rule.error("length export currently supports String fields only"));
                        }
                        rule.parse_nested_meta(|bound| {
                            let key = if bound.path.is_ident("min") { if length { "minLength" } else { "minimum" } }
                                else if bound.path.is_ident("max") { if length { "maxLength" } else { "maximum" } }
                                else if bound.path.is_ident("equal") && length { "equal" }
                                else { return Err(bound.error("unsupported contract bound")); };
                            let value: syn::Expr = bound.value()?.parse()?;
                            if key == "equal" {
                                updates_for_field.push(quote! { property["minLength"] = ::serde_json::json!(#value); property["maxLength"] = ::serde_json::json!(#value); });
                            } else {
                                updates_for_field.push(quote! { property[#key] = ::serde_json::json!(#value); });
                            }
                            Ok(())
                        })
                    } else if rule.path.is_ident("custom") {
                        let mut function = None;
                        rule.parse_nested_meta(|arg| {
                            if !arg.path.is_ident("function") { return Err(arg.error("custom contract validators require function = path")); }
                            function = Some(arg.value()?.parse::<syn::Path>()?);
                            Ok(())
                        })?;
                        let mut function = function.ok_or_else(|| rule.error("missing custom function"))?;
                        let last = function.segments.last_mut().unwrap();
                        last.ident = format_ident!("{}_contract", last.ident);
                        updates_for_field.push(quote! { ::compallm_contract::merge_constraints(property, #function()); });
                        Ok(())
                    } else {
                        Err(rule.error("unsupported validation rule: add an explicit contract mapping"))
                    }
                })?;
            } else if attr.path().is_ident("modify") {
                attr.parse_nested_meta(|modifier| {
                    if modifier.path.is_ident("custom") {
                        let content;
                        syn::parenthesized!(content in modifier.input);
                        let function: syn::Path = content.parse()?;
                        if quote!(#function).to_string() != "str :: make_ascii_lowercase" {
                            return Err(modifier.error("unsupported custom contract modifier"));
                        }
                        updates_for_field.push(quote! { property["x-compallm-ascii-lowercase"] = ::serde_json::json!(true); });
                        return Ok(());
                    }
                    if !modifier.path.is_ident("trim") {
                        return Err(modifier.error("unsupported contract modifier"));
                    }
                    updates_for_field
                        .push(quote! { property["x-compallm-trim"] = ::serde_json::json!(true); });
                    Ok(())
                })?;
            }
        }
        // Read wire names from Serde metadata (the contracts use camelCase).
        let mut key = ident.to_string();
        for attr in &input.attrs {
            if attr.path().is_ident("serde") {
                for meta in attr.parse_args_with(
                    syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
                )? {
                    if let Meta::NameValue(v) = meta
                        && v.path.is_ident("rename_all")
                    {
                        let syn::Expr::Lit(v) = v.value else { continue };
                        let syn::Lit::Str(v) = v.lit else { continue };
                        if v.value() != "camelCase" {
                            return Err(syn::Error::new_spanned(
                                v,
                                "unsupported contract rename_all",
                            ));
                        }
                        let mut parts = key.split('_');
                        key = parts.next().unwrap().to_owned()
                            + &parts
                                .map(|p| {
                                    let mut c = p.chars();
                                    c.next()
                                        .map(|h| h.to_uppercase().to_string() + c.as_str())
                                        .unwrap_or_default()
                                })
                                .collect::<String>();
                    }
                }
            }
        }
        for attr in &field.attrs {
            if attr.path().is_ident("serde") {
                for meta in attr.parse_args_with(
                    syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
                )? {
                    if let Meta::NameValue(v) = meta
                        && v.path.is_ident("rename")
                        && let syn::Expr::Lit(v) = v.value
                        && let syn::Lit::Str(v) = v.lit
                    {
                        key = v.value();
                    }
                }
            }
        }
        updates.push(quote! {
            let property = schema["properties"].get_mut(#key).expect("contract property must exist");
            #(#updates_for_field)*
        });
    }
    Ok(quote! {
        const _: () = {
            #[allow(dead_code)]
            #helper
            impl ::utoipa::PartialSchema for #name {
                fn schema() -> ::utoipa::openapi::RefOr<::utoipa::openapi::schema::Schema> {
                    let mut schema = ::serde_json::to_value(<#helper_name as ::utoipa::PartialSchema>::schema()).expect("serializable input schema");
                    schema["x-compallm-input"] = ::serde_json::json!(true);
                    if #deny_unknown { schema["additionalProperties"] = ::serde_json::json!(false); }
                    #(#updates)*
                    ::serde_json::from_value(schema).expect("valid contract schema")
                }
            }
            impl ::utoipa::ToSchema for #name {
                fn schemas(schemas: &mut Vec<(String, ::utoipa::openapi::RefOr<::utoipa::openapi::schema::Schema>)>) {
                    <#helper_name as ::utoipa::ToSchema>::schemas(schemas);
                }
            }
        };
    })
}
