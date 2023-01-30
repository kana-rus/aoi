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
        #[aoi::__private::change_extern_crate_to_use_postlude]
        #serde_derived_struct
    })
}

pub(super) fn consume_struct(serde_derived_struct: TokenStream) -> Result<TokenStream> {
    let _: ItemStruct = syn::parse2(serde_derived_struct)?;
    Ok(TokenStream::new())
}

pub(super) fn change_extern_crate_to_use_postlude(serde_derives: TokenStream) -> Result<TokenStream> {
    struct SerdeDerives {
        ser: ItemConst,
        de:  ItemConst,
    }
    impl Parse for SerdeDerives {
        fn parse(input: syn::parse::ParseStream) -> Result<Self> {
            Ok(Self {
                ser: input.parse()?,
                de:  input.parse()?,
            })
        }
    }
    fn change(deriver: &mut ItemConst) {
        match &mut *deriver.expr {
            Expr::Block(expr) => {
                expr.block.stmts[0] = parse2(quote!{
                    use aoi::postlude::serde as _serde;
                }).unwrap()
            },
            _ => unreachable!(),
        }
    }

    let SerdeDerives {
        mut ser,
        mut de,
    } = parse2(serde_derives)?;

    change(&mut ser);
    change(&mut de);

    Ok(quote!{
        #ser
        #de
    })
}

