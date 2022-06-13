use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

mod visitor;
mod queue;
mod accesscode;


#[http_component]
fn router(req: Request) -> Result<Response> {
    let parsed_url: Vec<&str> = req.uri().path().split('/').collect();
    match parsed_url[4] {
        "visitormanagement" => {
            return visitor::router::route(&req, parsed_url)
        }

        "queuemanagement" => {
            return queue::router::route(&req, parsed_url)
        }

        "accesscodemanagement" => {
            return accesscode::router::route(&req, parsed_url)
        }

        _=>{
            return Ok(http::Response::builder()
                        .status(404)
                        .body(Some("".into()))?)
        }

    }
}
