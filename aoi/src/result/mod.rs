use aoi_components::response::Response;

pub type Result<T> = std::result::Result<T, Response>;
pub type HandleResult = Result<Response>;