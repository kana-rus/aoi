pub trait IntoCreated {fn into_created(self) -> Result<Body>;}

impl<J: for <'j> JSON<'j>> IntoCreated for J {
    fn into_created(self) -> Result<Body> {
        Ok(Body::application_json(Cow::Owned(self.ser()?)))
    }
}
impl<J: for <'j> JSON<'j>> IntoCreated for Result<J> {
    fn into_created(self) -> Result<Body> {
        Ok(Body::application_json(Cow::Owned(self?.ser()?)))
    }
}

impl IntoCreated for Body {
    fn into_created(self) -> Result<Body> {
        Ok(self)
    }
}
impl IntoCreated for Result<Body> {
    fn into_created(self) -> Result<Body> {
        self
    }
}