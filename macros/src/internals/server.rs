use proc_macro2::Ident;
use syn::{ItemStruct, ItemImpl, token, Block, ItemFn, punctuated::Punctuated, Type};

mod parse;
mod interpret;

pub(super) struct Server {
    instance: ItemStruct,
    impls:    ItemImpl,
}

pub(super) struct ServerInput {
    name:   Ident,
    fields: Option<Vec<ServerField>>,
    impls:  Vec<ServerImpl>,
}
    struct ServerField {
        name:   Ident,
        _colon: token::Colon,
        ty:     ServerFieldType,
    }
        enum ServerFieldType {
            // empty now
        }
    enum ServerImpl {
        Util(
            ItemFn
        ),
        Handler {
            req:      RequestInfo,
            proccess: Box<Block>,
        },
    }
        struct RequestInfo {
            method: Method,
            path:   Path,
            _semi_colon:  Option<token::Semi>,
            request_body: Option<RequestBody>,
        }
            enum Method {
                GET,
                POST,
                PATCH,
                DELETE,
            }
                mod methods {
                    use syn::custom_keyword;
                    custom_keyword!(GET);
                    custom_keyword!(POST);
                    custom_keyword!(PATCH);
                    custom_keyword!(DELETE);
                }
            struct RequestBody {
                name:   Ident,
                _colon: token::Colon,
                ty:     Type,
            }
            struct Path(
                Punctuated<PathSection, token::Div>
            );
                enum PathSection {
                    Str(Ident),
                    Param {
                        _bracket: token::Brace,
                        name:   Ident,
                        _colon: token::Colon,
                        ty:     Type,
                    },
                }