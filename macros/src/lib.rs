mod internals;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn server(_: TokenStream, server_struct: TokenStream) -> TokenStream {
    internals::server(server_struct.into())
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}

#[proc_macro_derive(JSON)]
pub fn derive_json(struct_stream: TokenStream) -> TokenStream {
    internals::derive_json(struct_stream.into())
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}

#[proc_macro_attribute]
pub fn consume_struct(_: TokenStream, derived_struct: TokenStream) -> TokenStream {
    internals::consume_struct(derived_struct.into())
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}
