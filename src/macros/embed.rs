use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{Error, LitBool, LitFloat, LitInt, LitStr, Result};
use toml::Value;

pub fn embed(key: &str, value: &Value, span: Span) -> Result<TokenStream> {
    match value {
        Value::Boolean(b) => Ok(LitBool::new(*b, span).into_token_stream()),
        Value::Integer(int) => Ok(LitInt::new(int.to_string().as_str(), span).into_token_stream()),
        Value::Float(flt) => Ok(LitFloat::new(flt.to_string().as_str(), span).into_token_stream()),
        Value::String(s) => Ok(LitStr::new(s.as_str(), span).into_token_stream()),

        Value::Array(arr) => {
            let els = arr
                .iter()
                .enumerate()
                .map(|(idx, el)| embed(format!("{key}.{idx}").as_str(), el, span))
                .collect::<Result<Vec<_>>>()?;

            Ok(quote! { [#(#els),*] })
        }
        Value::Table(map) => {
            let kvs = map
                .iter()
                .map(|(k, v)| {
                    let value = embed(format!("{key}.{k}").as_str(), v, span)?;
                    let key = LitStr::new(k.as_str(), span);

                    Ok(quote! { (#key, #value) })
                })
                .collect::<Result<Vec<_>>>()?;

            Ok(quote! { [#(#kvs),*] })
        }

        Value::Datetime(dt) => Err(Error::new(
            span,
            format!("Cannot Embed Key `{key}` With Type toml::Value::DateTime `{dt}` Into Rust"),
        )),
    }
}
