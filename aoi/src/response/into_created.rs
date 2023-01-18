use aoi_components::{
    json::JSON,
    response::{Body, Response},
};

pub trait IntoCreated<Label> {fn into_created(self) -> Result<Body, Response>;}

impl<J: for <'j> JSON<'j>> IntoCreated<()> for J {
    fn into_created(self) -> Result<Body, Response> {
        Ok(Body::json(self.ser()?))
    }
}
impl<J: for <'j> JSON<'j>> IntoCreated<Result<(), Response>> for Result<J, Response> {
    fn into_created(self) -> Result<Body, Response> {
        Ok(Body::json(self?.ser()?))
    }
}

impl IntoCreated<Body> for Body {
    fn into_created(self) -> Result<Body, Response> {
        Ok(self)
    }
}
impl IntoCreated<Result<Body, Response>> for Result<Body, Response> {
    fn into_created(self) -> Result<Body, Response> {
        self
    }
}