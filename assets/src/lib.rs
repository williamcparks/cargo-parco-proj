use proc_macro::TokenStream;

#[proc_macro]
pub fn proj(input: TokenStream) -> TokenStream {
    const CONTENTS: &str = include_str!("{{proj_toml_path}}");
    match cargo_parco_proj::macros::handler(
        cargo_parco_proj::syn::parse_macro_input!(input),
        CONTENTS,
    ) {
        Ok(ok) => ok,
        Err(err) => err.into_compile_error(),
    }
    .into()
}
