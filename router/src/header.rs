use std::io;
use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader};

const MAX_HEADER_BYTES: usize = 32 * 1024;
const MAX_HEADER_FIELDS: usize = 64;

/// Parsed HTTP/1 request metadata. Values retain bytes and duplicate fields.
#[derive(Clone, Debug)]
pub struct RequestHeader {
    pub method: String,
    pub target: String,
    pub version: u8,
    pub fields: Vec<(String, Vec<u8>)>,
}

pub(crate) async fn read_header<R: AsyncRead + Unpin>(
    reader: &mut BufReader<R>,
) -> io::Result<RequestHeader> {
    let mut bytes = Vec::new();
    loop {
        let available = reader.fill_buf().await?;
        if available.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "incomplete request header",
            ));
        }
        let mut consumed = 0;
        let mut complete = false;
        for &byte in available {
            if bytes.len() == MAX_HEADER_BYTES {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "request header too large",
                ));
            }
            bytes.push(byte);
            consumed += 1;
            if bytes.ends_with(b"\r\n\r\n") {
                complete = true;
                break;
            }
        }
        reader.consume(consumed);
        if complete {
            break;
        }
    }

    let invalid = || io::Error::new(io::ErrorKind::InvalidData, "invalid request header");
    let mut fields = [httparse::EMPTY_HEADER; MAX_HEADER_FIELDS];
    let mut request = httparse::Request::new(&mut fields);
    match request.parse(&bytes).map_err(|_| invalid())? {
        httparse::Status::Complete(_) => Ok(RequestHeader {
            method: request.method.ok_or_else(invalid)?.into(),
            target: request.path.ok_or_else(invalid)?.into(),
            version: request.version.ok_or_else(invalid)?,
            fields: request
                .headers
                .iter()
                .map(|h| (h.name.into(), h.value.to_vec()))
                .collect(),
        }),
        httparse::Status::Partial => Err(invalid()),
    }
}
