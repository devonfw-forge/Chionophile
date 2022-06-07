use anyhow::Result;
use http::Method;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};
use std::fs::File;
use std::io::prelude::*;


#[http_component]
fn visitor(req: Request) -> Result<Response> {
    match req.method() {
        &Method::POST => {
            if req.uri() == "/jumpthequeue/services/rest/visitormanagement/v1/visitor/search/"{
                return Ok(http::Response::builder()
                .status(200)
                .body(Some("Searching POST".into()))?)
            }
            return Ok(http::Response::builder()
                .status(200)
                .body(Some(format!("POST Visitor at {}",req.uri()).into()))?)
        }
        &Method::GET => {
            let id: Vec<&str> = req.uri().path().split('/').collect();
            return Ok(http::Response::builder()
                .status(200)
                .body(Some(format!("Getting {} Id",id[id.len()-2]).into()))?)
        }
        &Method::DELETE => {
            let id: Vec<&str> = req.uri().path().split('/').collect();
            return Ok(http::Response::builder()
                .status(200)
                .body(Some(format!("Deleting {} Id",id[id.len()-2]).into()))?)
        }
        _ => {
            return Ok(http::Response::builder()
                .status(500)
                .body(Some("".into()))?)
        }
    }
}
