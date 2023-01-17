use syn::{parse::Parse, braced, token, bracketed};
use proc_macro2::Ident;
use super::*;

impl Parse for ServerInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;

        let mut fields = None;
        let mut impls = Vec::new();

        let first_block; braced!(first_block in input);
        if first_block.peek(syn::Ident) {
            fields = Some({
                let mut fields = Vec::new();
                while !first_block.is_empty() {
                    fields.push(first_block.parse::<ServerField>()?)
                }
                fields
            });

            let second_block; braced!(second_block in input);
            impls = {
                let mut impls = Vec::new();
                while !second_block.is_empty() {
                    impls.push(second_block.parse::<ServerImpl>()?)
                }
                impls
            };
        } else {
            impls = {
                let mut impls = Vec::new();
                while !first_block.is_empty() {
                    impls.push(first_block.parse::<ServerImpl>()?)
                }
                impls
            };
        }

        Ok(Self { name, fields, impls })
    }
}

impl Parse for ServerField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name:   input.parse()?,
            _colon: input.parse()?,
            ty:     input.parse()?,
        })
    }
}
    impl Parse for ServerFieldType {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            Err(input.error("server fields is not implemented in current version"))
        }
    }

impl Parse for ServerImpl {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(token::Pound)
        && input.peek2(token::Bracket)
        && (input.peek3(methods::GET)
         || input.peek3(methods::POST)
         || input.peek3(methods::PATCH)
         || input.peek3(methods::DELETE)
        ) {
            Ok(Self::Handler {
                req:      input.parse()?,
                proccess: input.parse()?,
            })
        } else {
            Ok(Self::Util(
                input.parse()?
            ))
        }
    }
}
    impl Parse for RequestInfo {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            input.parse::<token::Pound>();
            let info; bracketed!(info in input);
            Ok(Self {
                method: info.parse()?,
                path:   info.parse()?,
                _semi_colon:  {info.peek(token::Semi)}.then_some(info.parse()?),
                request_body: {!info.is_empty()}.then_some(info.parse()?),
            })
        }
    }
        impl Parse for Method {
            fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                if input.peek(methods::GET) {
                    Ok(Self::GET)
                } else if input.peek(methods::POST) {
                    Ok(Self::POST)
                } else if input.peek(methods::PATCH) {
                    Ok(Self::PATCH)
                } else if input.peek(methods::DELETE) {
                    Ok(Self::DELETE)
                } else {
                    Err(input.error("expected one of： `GET` `POST` `PATCH` `DELETE`"))
                }
            }
        }
        impl Parse for Path {
            fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                Ok(Self(input.parse_terminated(PathSection::parse)?))
            }
        }
            impl Parse for PathSection {
                fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                    if input.peek(token::Brace) {
                        let param;
                        Ok(Self::Param {
                            _bracket: braced!(param in input),
                            name:     param.parse()?,
                            _colon:   param.parse()?,
                            ty:       param.parse()?,
                        })
                    } else {
                        Ok(Self::Str(input.parse()?))
                    }
                }
            }
        impl Parse for RequestBody {
            fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                Ok(Self {
                    name:   input.parse()?,
                    _colon: input.parse()?,
                    ty:     input.parse()?,
                })
            }
        }