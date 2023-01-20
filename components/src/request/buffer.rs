use std::{str::{self, Lines}, ops::Index};
use async_std::{net::TcpStream, io::ReadExt};
use super::range::BufRange;

const BUF_SIZE: usize = 512;
pub struct Buffer(
    [u8; BUF_SIZE]
); impl Buffer {
    pub(crate) async fn new(stream: &mut TcpStream) -> Self {
        let mut buffer = [b' '; BUF_SIZE];
        match stream.read(&mut buffer).await {
            Ok(_) => Self(buffer),
            Err(err) => {
                tracing::error!("failed to read TcpStream");
                panic!("err: {}", err.to_string())
            }
        }
    }
    pub(crate) fn lines(&self) -> Lines {
        match std::str::from_utf8(&self.0) {
            Ok(string) => string.trim_end().lines(),
            Err(err) => {
                tracing::error!("invalid TcpStream format");
                panic!("err: {}", err.to_string())
            }
        }
    }
    pub(crate) fn read_str(&self, range: &BufRange) -> &str {
        let target_bytes = &self[*range];
        unsafe {
            std::str::from_utf8_unchecked(target_bytes)
        }
    } 

    pub(crate) async fn from_http_request_str(request: String) -> Self {
        let mut buffer = [b' '; BUF_SIZE];
        let mut request = request.as_bytes();
        assert!(request.len() <= BUF_SIZE, "I can't handle request that's larger than {BUF_SIZE} bytes");
        request.read(&mut buffer).await.expect("failed to read request");
        Self(buffer)
    }
}
impl Index<BufRange> for Buffer {
    type Output = [u8];
    fn index(&self, range: BufRange) -> &Self::Output {
        &self.0[range.as_range()]
    }
}
