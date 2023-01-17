use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{ItemStruct, token, VisPublic, punctuated::Punctuated, parse2};
use crate::internals::Interpret;
use super::{ServerInput, Server, ServerField, ServerImpl};

impl Interpret<Server> for ServerInput {
    fn interpret(self) -> Server {
        let ServerInput {
            name,
            fields,
            impls,
        } = self;

        let instance = {
            let mut fields_stream = TokenStream::new();
            if let Some(fields) = fields {
                for ServerField { name, _colon, ty } in fields {
                    fields_stream.extend(quote!{
                        #name: #ty,
                    })
                }
            }
            quote!{
                struct #name {
                    #fields_stream
                }
            }
        };

        let struct_impls = {
            let mut struct_impls = TokenStream::new();
            for server_impl in impls {
                match server_impl {
                    ServerImpl::Util(item_fn) => {
                        struct_impls.extend(quote!{#item_fn})
                    },
                    ServerImpl::Handler { req, proccess } => {

                    }
                }
            }
            quote!{
                impl #name {
                    #struct_impls
                }
            }
        };

        Server {
            instance: parse2(instance).unwrap(),
            impls:    parse2(struct_impls).unwrap(),
        }
    }
}