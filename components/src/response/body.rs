use std::borrow::Cow;

pub struct Body {
    media_type: MediaType,
    content:    Cow<'static, str>,
}
#[allow(non_camel_case_types)]
pub enum MediaType {
    application_json,
    text_plain,
    text_html,
}

impl Body {
    pub fn text<C: Content>(content: C) -> Self {
        Self {
            media_type: MediaType::text_plain,
            content:    content.into_content(),
        }
    }
    pub fn html<C: Content>(content: C) -> Self {
        Self {
            media_type: MediaType::text_html,
            content:    content.into_content(),
        }
    }
    pub fn json<C: Content>(content: C) -> Self {
        Self {
            media_type: MediaType::application_json,
            content:    content.into_content(),
        }
    }

    pub fn content_type(&self) -> &MediaType {
        &self.media_type
    }
    pub fn content_length(&self) -> usize {
        self.content.len()
    }
}

pub trait Content {
    fn into_content(self) -> Cow<'static, str>;
}
impl Content for String {
    fn into_content(self) -> Cow<'static, str> {
        Cow::Owned(self)
    }
}
impl Content for &String {
    fn into_content(self) -> Cow<'static, str> {
        Cow::Owned(self.to_owned())
    }
}
impl Content for &'static str {
    fn into_content(self) -> Cow<'static, str> {
        Cow::Borrowed(self)
    }
}
