use aoi_components::response::{Body, Response};

use super::json_response::JsonResponse;


pub trait IntoOK<OkParam> {fn into_ok(self) -> Result<Option<Body>, Response>;}

impl IntoOK<String> for String {
    fn into_ok(self) -> Result<Option<Body>, Response> {
        Ok(Some(Body::text(self)))
    }
}
impl IntoOK<Option<String>> for Option<String> {
    fn into_ok(self) -> Result<Option<Body>, Response> {
        Ok(self.map(|string| Body::text(string)))
    }
}
impl IntoOK<Result<String, Response>> for Result<String, Response> {
    fn into_ok(self) -> Result<Option<Body>, Response> {
        Ok(Some(Body::text(self?)))
    }
}

impl IntoOK<&String> for &String {
    fn into_ok(self) -> Result<Option<Body>, Response> {
        Ok(Some(Body::text(self.to_owned())))
    }
}
impl IntoOK<&'static str> for &'static str {
    fn into_ok(self) -> Result<Option<Body>, Response> {
        Ok(Some(Body::text(self)))
    }
}
impl IntoOK<Option<&String>> for Option<&String> {
    fn into_ok(self) -> Result<Option<Body>, Response> {
        Ok(self.map(|string| Body::text(string.to_owned())))
    }
}
impl IntoOK<Option<&'static str>> for Option<&'static str> {
    fn into_ok(self) -> Result<Option<Body>, Response> {
        Ok(self.map(|string| Body::text(string)))
    }
}
impl IntoOK<Result<&String, Response>> for Result<&String, Response> {
    fn into_ok(self) -> Result<Option<Body>, Response> {
        Ok(Some(Body::text(self?.to_owned())))
    }
}
impl IntoOK<Result<&'static str, Response>> for Result<&'static str, Response> {
    fn into_ok(self) -> Result<Option<Body>, Response> {
        Ok(Some(Body::text(self?)))
    }
}

impl<J: JsonResponse<Label>, Label> IntoOK<Label> for J {
    fn into_ok(self) -> Result<Option<Body>, Response> {
        Ok(Some(Body::json(self.ser()?)))
    }
}
impl<J: JsonResponse<Label>, Label> IntoOK<Option<Label>> for Option<J> {
    fn into_ok(self) -> Result<Option<Body>, Response> {
        match self {
            Some(json) => Ok(Some(Body::json(json.ser()?))),
            None => Ok(None),
        }
    }
}
impl<J: JsonResponse<Label>, Label> IntoOK<Result<Label, Response>> for Result<J, Response> {
    fn into_ok(self) -> Result<Option<Body>, Response> {
        Ok(Some(Body::json(self?.ser()?)))
    }
}

impl IntoOK<Body> for Body {
    fn into_ok(self) -> Result<Option<Body>, Response> {
        Ok(Some(self))
    }
}
impl IntoOK<Body> for Option<Body> {
    fn into_ok(self) -> Result<Option<Body>, Response> {
        Ok(self)
    }
}
impl IntoOK<Body> for Result<Body, Response> {
    fn into_ok(self) -> Result<Option<Body>, Response> {
        Ok(Some(self?))
    }
}