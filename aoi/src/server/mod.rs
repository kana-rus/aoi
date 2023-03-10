use aoi_components::{request::{buffer::Buffer, range::{RangeMap, HeaderRangeMap}}, method::Method};

use crate::{result::HandleResult, response::Responder, router::Router};

#[derive(/* JSON */)]
struct User {
    id:   u64,
    name: String,
}

struct MyServer;
impl Responder for MyServer {
    // no middleware or something in this example
}
impl MyServer {
    // `_` just after `get` is `/`

    async fn handle_get_(&self) -> HandleResult {
        self.OK("Hello!")
    }

    async fn handle_get_health_check(&self) -> HandleResult {
        self.NoContent()
    }

    // ==============================
    // IntoRouter
    // ==============================
    fn into_router(self) -> std::result::Result<Router, String> {
        async fn handler_to_get_(
            buffer:  Buffer,
            queries: RangeMap,
            headers: HeaderRangeMap,
            body:    Option<String>,
        ) -> HandleResult {
            todo!(/*
                prepare

                - struct `Request` public to aoi user
                - trait IntoRouter
                
            */)
        }

        async fn handler_to_get_health_check(
            buffer:  Buffer,
            queries: RangeMap,
            headers: HeaderRangeMap,
            body:    Option<String>,
        ) -> HandleResult {
            todo!()
        }

        let mut router = Router::new();
        router.register(Method::GET, "/",
            Box::new(move
                |buffer, queries, headers, body|
                Box::pin(handler_to_get_(buffer, queries, headers, body))
            )
        )?;
        router.register(Method::GET, "/health_check",
            Box::new(move
                |buffer, queries, headers, body|
                Box::pin(handler_to_get_health_check(buffer, queries, headers, body))
            )
        )?;
        Ok(router)
    }

    // ==============================
}
