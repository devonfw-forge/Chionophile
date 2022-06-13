use anyhow::Result;
use http::Method;
use spin_sdk::{
    http::{Request, Response},
};
use crate::visitor::visitor::VisitorEntity;
use serde_json::Result as JsonResult;
use crate::util::router::Router;

pub struct VisitorRouter;

impl Router<VisitorEntity> for VisitorRouter {
    fn route(req: &Request, parsed_url: Vec<&str>)-> Result<Response> {
        return match req.method() {
            &Method::POST => {
                if parsed_url[parsed_url.len() - 2] == "search" {
                    return Ok(http::Response::builder()
                        .status(200)
                        .body(Some("Searching Visitor POST".into()))?)
                }
                let body = req.body().clone();
                let _body: JsonResult<VisitorEntity> = serde_json::from_slice(&body.unwrap());
                Ok(http::Response::builder()
                    .status(200)
                    .body(Some(format!("POST Visitor at {} {}", req.uri(), parsed_url[4]).into()))?)
            }
            &Method::GET => {
                Ok(http::Response::builder()
                    .status(200)
                    .body(Some(format!("Getting Visitor {} Id", parsed_url[parsed_url.len() - 2]).into()))?)
            }
            &Method::DELETE => {
                Ok(http::Response::builder()
                    .status(200)
                    .body(Some(format!("Deleting Visitor {} Id", parsed_url[parsed_url.len() - 2]).into()))?)
            }
            _ => {
                Ok(http::Response::builder()
                    .status(500)
                    .body(Some("".into()))?)
            }
        }
    }
}
