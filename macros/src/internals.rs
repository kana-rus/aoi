use proc_macro2::TokenStream;
use syn::{Result, parse2};

trait Build {
    fn build(self) -> TokenStream;
}
trait Interpret<As> {
    fn interpret(self) -> As;
}

mod server;
pub(super) fn server(struct_stream: TokenStream) -> Result<TokenStream> {
    Ok(parse2::<server::ServerInput>(struct_stream)?.interpret().build())
}