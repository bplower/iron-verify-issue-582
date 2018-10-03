
extern crate iron;

use iron::{Chain, Iron, IronResult, Request, Response, StatusCode};
use iron::typemap::TypeMap;
use iron::headers::{CONTENT_LENGTH, HeaderMap};

/// This function returns a crafted response one might expect to get
/// from a HEAD request. Notably, this response is returning an empty body
/// as well as a CONTENT_LENGTH header with the value 123.
fn index(_: &mut Request) -> IronResult<Response> {
    // Manually set the content length header
    let mut hmap = HeaderMap::new();
    hmap.insert(CONTENT_LENGTH, "123".parse().unwrap());

    // Build response with empty body and headers
    let resp = Response {
        status:     Some(StatusCode::OK),
        body:       None,
        headers:    hmap,
        extensions: TypeMap::new(),
    };
    Ok(resp)
}

fn main() {
    let chain = Chain::new(index);
    Iron::new(chain).http("localhost:3000");
}
