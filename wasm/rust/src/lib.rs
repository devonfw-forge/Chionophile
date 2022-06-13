use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};
use crate::accesscode::router::AccessCodeRouter;
use crate::queue::router::QueueRouter;
use crate::util::router::Router;
use crate::visitor::router::VisitorRouter;

mod visitor;
mod queue;
mod accesscode;
mod util;


#[http_component]
fn router(req: Request) -> Result<Response> {
    let parsed_url: Vec<&str> = req.uri().path().split('/').collect();
    match parsed_url[4] {
        "visitormanagement" => {
            return VisitorRouter::route(&req, parsed_url)
        }

        "queuemanagement" => {
            return QueueRouter::route(&req, parsed_url)
        }

        "accesscodemanagement" => {
            return AccessCodeRouter::route(&req, parsed_url)
        }

        _=>{
            return Ok(http::Response::builder()
                        .status(404)
                        .body(Some("".into()))?)
        }

    }
}
