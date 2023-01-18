pub trait IntoOK<OkParam> {fn into_ok(self) -> Result<Option<Body>>;}

impl IntoOK<String> for String {
    fn into_ok(self) -> Result<Option<Body>> {
        Ok(Some(Body::text_plain(Cow::Owned(self))))
    }
}
impl IntoOK<Option<String>> for Option<String> {
    fn into_ok(self) -> Result<Option<Body>> {
        Ok(self.map(|string| Body::text_plain(Cow::Owned(string))))
    }
}
impl IntoOK<Result<String>> for Result<String> {
    fn into_ok(self) -> Result<Option<Body>> {
        Ok(Some(Body::text_plain(Cow::Owned(self?))))
    }
}

impl IntoOK<&String> for &String {
    fn into_ok(self) -> Result<Option<Body>> {
        Ok(Some(Body::text_plain(Cow::Owned(self.to_owned()))))
    }
}
impl IntoOK<&'static str> for &'static str {
    fn into_ok(self) -> Result<Option<Body>> {
        Ok(Some(Body::text_plain(Cow::Borrowed(self))))
    }
}
impl IntoOK<Option<&String>> for Option<&String> {
    fn into_ok(self) -> Result<Option<Body>> {
        Ok(self.map(|string| Body::text_plain(Cow::Owned(string.to_owned()))))
    }
}
impl IntoOK<Option<&'static str>> for Option<&'static str> {
    fn into_ok(self) -> Result<Option<Body>> {
        Ok(self.map(|string| Body::text_plain(Cow::Borrowed(string))))
    }
}
impl IntoOK<Result<&String>> for Result<&String> {
    fn into_ok(self) -> Result<Option<Body>> {
        Ok(Some(Body::text_plain(Cow::Owned(self?.to_owned()))))
    }
}
impl IntoOK<Result<&'static str>> for Result<&'static str> {
    fn into_ok(self) -> Result<Option<Body>> {
        Ok(Some(Body::text_plain(Cow::Borrowed(self?))))
    }
}

impl<L: JsonResponseLabel, J: JsonResponse<L>> IntoOK<L> for J {
    fn into_ok(self) -> Result<Option<Body>> {
        Ok(Some(Body::application_json(Cow::Owned(self.ser()?))))
    }
}
impl<L: JsonResponseLabel, J: JsonResponse<L>> IntoOK<Option<L>> for Option<J> {
    fn into_ok(self) -> Result<Option<Body>> {
        match self {
            Some(json) => Ok(Some(Body::application_json(Cow::Owned(json.ser()?)))),
            None => Ok(None),
        }
    }
}
impl<L: JsonResponseLabel, J: JsonResponse<L>> IntoOK<Result<L>> for Result<J> {
    fn into_ok(self) -> Result<Option<Body>> {
        Ok(Some(Body::application_json(Cow::Owned(self?.ser()?))))
    }
}

impl IntoOK<Body> for Body {
    fn into_ok(self) -> Result<Option<Body>> {
        Ok(Some(self))
    }
}
impl IntoOK<Body> for Option<Body> {
    fn into_ok(self) -> Result<Option<Body>> {
        Ok(self)
    }
}
impl IntoOK<Body> for Result<Body> {
    fn into_ok(self) -> Result<Option<Body>> {
        Ok(Some(self?))
    }
}