use anyhow::Result;
use http::Method;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

#[http_component]
fn hello(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());
    match req.method() {
        &Method::POST => {
            Ok(http::Response::builder()
                .status(200)
                //.header("foo", "bar")
                .body(Some("POST!\n".into()))?)
        }
        &Method::GET => {
            Ok(http::Response::builder()
                .status(200)
                //.header("foo", "bar")
                .body(Some("GET!\n".into()))?)
        }
        &Method::DELETE => {
            Ok(http::Response::builder()
                .status(200)
                //.header("foo", "bar")
                .body(Some("DELETE!\n".into()))?)
        }
    }

    Ok(bad_request())
}

fn bad_request() -> Response {
    http::Response::builder().status(400).into()
}
