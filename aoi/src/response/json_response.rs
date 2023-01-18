use aoi_components::{json::JSON, response::Response};

pub trait JsonResponse<L> {fn ser(&self) -> Result<String, Response>;}

impl<J: for <'j> JSON<'j>> JsonResponse<()> for J {
    fn ser(&self) -> Result<String, Response> {
        self.ser()
    }
}
impl<J: for <'j> JSON<'j>> JsonResponse<&()> for &J {
    fn ser(&self) -> Result<String, Response> {
        Ok(serde_json::to_string(self)?)
    }
}
