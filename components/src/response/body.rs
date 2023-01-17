use std::borrow::Cow;

#[allow(non_camel_case_types)]
pub enum Body {
    application_json(Cow<'static, str>),
    text_plain(Cow<'static, str>),
    text_html(Cow<'static, str>),
} impl Body {
    
}