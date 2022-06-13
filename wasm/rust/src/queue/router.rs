use anyhow::Result;
use http::Method;
use spin_sdk::{
    http::{Request, Response},
};
use crate::queue::queue::QueueEntity;
use crate::Router;

pub struct QueueRouter;

impl Router<QueueEntity> for QueueRouter {
    fn route(req: &Request, parsed_url: Vec<&str>)-> Result<Response>{
        return match req.method() {
            &Method::POST => {
                if parsed_url[parsed_url.len() - 2] == "search" {
                    return Ok(http::Response::builder()
                        .status(200)
                        .body(Some("Searching queue POST".into()))?)
                }
                Ok(http::Response::builder()
                    .status(200)
                    .body(Some(format!("POST queue at {} {}", req.uri(), parsed_url[4]).into()))?)
            }
            &Method::GET => {
                Ok(http::Response::builder()
                    .status(200)
                    .body(Some(format!("Getting queue {} Id", parsed_url[parsed_url.len() - 2]).into()))?)
            }
            &Method::DELETE => {
                Ok(http::Response::builder()
                    .status(200)
                    .body(Some(format!("Deleting queue {} Id", parsed_url[parsed_url.len() - 2]).into()))?)
            }
            _ => {
                Ok(http::Response::builder()
                    .status(500)
                    .body(Some("".into()))?)
            }
        }
    }
}
