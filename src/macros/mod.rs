use proc_macro2::TokenStream;
use syn::{Error, LitStr, Result};

mod embed;
mod error;
mod proj;

/// The main entry point for macros: It will parse the `proj_contents`
/// into a proper `Proj.toml` and use the `input` for [`proc_macro2::Span`] and key information
pub fn handler(input: LitStr, proj_contents: &str) -> Result<TokenStream> {
    let span = input.span();
    let proj = match proj::Proj::try_new(proj_contents) {
        Ok(ok) => ok,
        Err(err) => return Err(Error::new(span, err)),
    };

    let key = input.value();
    let value = match proj.try_get(key.as_str()) {
        Ok(ok) => ok,
        Err(err) => return Err(Error::new(span, err)),
    };

    embed::embed(key.as_str(), value, span)
}
