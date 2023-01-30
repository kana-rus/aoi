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
    };
}
pub mod __private {
    pub use aoi_macros::consume_struct;
}

pub mod prelude {
    pub use super::{
        macros::{JSON, server}
    };
}

pub mod postlude {
    pub use serde;
}
