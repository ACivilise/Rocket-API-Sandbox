use std::convert::Infallible;
use serde_json;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Request, Response};

use crate::domains;

pub async fn map_endpoints(req: Request<impl hyper::body::Body>) -> Result<Response<Full<Bytes>>, Infallible> {
    let (parts, _body) = req.into_parts();
    match (parts.method, parts.uri.path()) {
        (hyper::Method::GET, "/users") => {
            match domains::users::users::get_all_users().await {
                Ok(users) => {
                    match serde_json::to_string(&users) {
                        Ok(users_json) => Ok(Response::new(Full::new(Bytes::from(users_json)))),
                        Err(_) => Ok(Response::new(Full::new(Bytes::from("Error serializing users."))))
                    }
                },
                Err(_) => Ok(Response::new(Full::new(Bytes::from("Error fetching users."))))
            }
        },
        (hyper::Method::GET, "/users") => Ok(Response::new(Full::new(Bytes::from("Hello World!")))),
        _ => Ok(Response::new(Full::new(Bytes::from("Hello World!")))),
    }
}