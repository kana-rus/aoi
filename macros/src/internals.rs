use proc_macro2::TokenStream;
use syn::{Result, parse2, ItemStruct, ItemConst, parse::Parse, Expr};
use quote::quote;

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

pub(super) fn derive_json(serde_derived_struct: TokenStream) -> Result<TokenStream> {
    let ItemStruct { ident, .. }
        = syn::parse2(serde_derived_struct.clone())?;

    Ok(quote!{
        impl<'j> aoi::components::json::JSON<'j> for #ident {}

        #[derive(serde::Serialize, serde::Deserialize)]
        #[aoi::__private::consume_struct]
        #serde_derived_struct
    })
}

pub(super) fn consume_struct(serde_derived_struct: TokenStream) -> Result<TokenStream> {
    let _: ItemStruct = parse2(serde_derived_struct)?;
    Ok(TokenStream::new())
}
