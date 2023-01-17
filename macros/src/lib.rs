use proc_macro::TokenStream;

mod internals;

#[proc_macro_attribute]
pub fn server(_: TokenStream, server_struct: TokenStream) -> TokenStream {
    internals::server(server_struct.into())
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}