pub mod server;
pub mod router;
pub mod result;
pub mod handler;
pub mod response;

pub mod components {
    pub use aoi_components::{
        json,
        header,
        method,
        result,
    };
}

pub mod macros {
    pub use aoi_macros::{
        JSON,
        server,
        consume_struct,
        change_extern_crate_to_use_postlude,
    };
}

pub mod postlude {
    pub use serde;
}
