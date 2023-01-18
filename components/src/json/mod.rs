use crate::response::Response;

pub fn ser<S: serde::Serialize>(s: &S) -> Result<String, Response> {
    Ok(serde_json::to_string(s)?)
}
pub fn de<'de, D: serde::Deserialize<'de>>(str: &'de str) -> Result<D, Response> {
    Ok(serde_json::from_str(str)?)
}

pub trait JSON<'j>: serde::Serialize + serde::Deserialize<'j> {
    fn ser(&self) -> Result<String, Response> {
        Ok(serde_json::to_string(self)?)
    }
    fn de(string: &'j str) -> Result<Self, Response> {
        Ok(serde_json::from_str(string)?)
    }
}
impl <'i, J: for <'j> JSON<'j>> JSON<'i> for Vec<J> {}
