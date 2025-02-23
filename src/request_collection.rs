use std::io::{Error, ErrorKind, Result};
use tokio::io::{AsyncRead, AsyncReadExt};

/// Maximum allowed request size (here 1 MB) to protect against unbounded input.
const MAX_REQUEST_SIZE: usize = 1024 * 1024;

/// Asynchronously reads from the given stream until an entire HTTP request (headers and body) is collected.
/// This function assumes that the request uses a Content-Length header (it does not support
/// chunked transfer encoding) and that the entire request is less than MAX_REQUEST_SIZE bytes.
/// Returns a tuple: (headers as Vec<u8>, body as Vec<u8>).
pub async fn collect_http_request<R: AsyncRead + Unpin>(
    stream: &mut R,
) -> Result<(Vec<u8>, Vec<u8>)> {
    let mut buffer = Vec::new();
    let mut temp = [0; 1024];
    let header_end_pos: usize;

    // --- Step 1: Read until we have the full header (i.e. until "\r\n\r\n") ---
    loop {
        let bytes_read = stream.read(&mut temp).await?;
        if bytes_read == 0 {
            // End-of-stream reached before headers completed.
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                "Stream ended before headers were fully received",
            ));
        }
        buffer.extend_from_slice(&temp[..bytes_read]);

        // Enforce a maximum size to prevent unbounded memory usage.
        if buffer.len() > MAX_REQUEST_SIZE {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Request exceeds maximum allowed size",
            ));
        }

        if let Some(pos) = find_subsequence(&buffer, b"\r\n\r\n") {
            header_end_pos = pos + 4; // position right after the header terminator
            break;
        }
    }

    // Extract the header bytes.
    let headers = buffer[..header_end_pos].to_vec();

    // --- Step 2: Parse the headers to determine the Content-Length (if any) ---
    let headers_str = std::str::from_utf8(&headers)
        .map_err(|_| Error::new(ErrorKind::InvalidData, "Headers are not valid UTF-8"))?;

    // Default to zero if no Content-Length header is found.
    let mut content_length = 0;
    for line in headers_str.lines() {
        let line = line.trim();
        if line.to_lowercase().starts_with("content-length:") {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                content_length = parts[1].trim().parse::<usize>().map_err(|_| {
                    Error::new(ErrorKind::InvalidData, "Invalid Content-Length value")
                })?;
            }
        }
    }

    // --- Step 3: Read the body if there is one ---
    let body_already_read = buffer.len() - header_end_pos;
    let mut body = buffer[header_end_pos..].to_vec();

    if body_already_read < content_length {
        let remaining = content_length - body_already_read;
        let mut body_buffer = vec![0; remaining];
        stream.read_exact(&mut body_buffer).await?;
        body.extend_from_slice(&body_buffer);
    }

    // Final check: if the overall request is too large, return an error.
    if headers.len() + body.len() > MAX_REQUEST_SIZE {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Final request size exceeds allowed maximum",
        ));
    }

    Ok((headers, body))
}

/// Searches for a byte slice `needle` in the `haystack` and returns its starting index if found.
fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}
