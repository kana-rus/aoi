use proc_macro2::Ident;
use quote::{quote, format_ident};
use syn::{ImplItem, ImplItemMethod, Visibility, Signature, Block, parse2};

use crate::internals::Build;

use super::{Server, Handler, Method, Path, PathSection, RequestBody};

impl Build for Server {
    fn build(self) -> proc_macro2::TokenStream {
        let Server { instance, mut impls, handlers } = self;

        for handler in handlers {
            let Handler { req, proccess } = handler;
            let handler_name = generate_handler_name(req.method, &req.path);

            let args = {
                let mut args = quote!{ &self, request: Request };

                for section in &req.path.0 {
                    match section {
                        PathSection::Param { name, ty, .. } => {
                            args.extend(quote!{
                                , #name: #ty
                            })
                        },
                        PathSection::Str(_) => (),
                    }
                }

                if let Some(req_body) = req.request_body {
                    let RequestBody { name, ty, .. } = req_body;
                    args.extend(quote!{
                        , #name: #ty
                    })
                }

                args
            };

            impls.items.push(parse2(quote!{
                fn #handler_name(#args)
            }).unwrap())
        }

        impls.items.push(

            /*
                method `register_handlers`
            */

            todo!()

        );

        quote!{
            #instance

        }
    }
}

fn generate_handler_name(method: Method, path: &Path) -> Ident {
    format_ident!("handle_{}",
        path.0.iter().fold(
            String::from(match method {
                Method::GET    => "GET",
                Method::POST   => "POST",
                Method::PATCH  => "PATCH",
                Method::DELETE => "DELETE",
            }),
            |it, next| it + "_" + &match next {
                PathSection::Str(ident)   => ident.to_string(),
                PathSection::Param { .. } => "<param>".to_owned(),
            })
    )
}