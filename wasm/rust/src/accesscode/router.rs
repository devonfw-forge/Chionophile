use anyhow::Result;
use http::Method;
use spin_sdk::{
    http::{Request, Response},
};

pub fn route(req: &Request, parsed_url: Vec<&str>)-> Result<Response>{
    match req.method() {
        &Method::POST => {
            if parsed_url[parsed_url.len()-2] == "search"{
                return Ok(http::Response::builder()
                .status(200)
                .body(Some("Searching accesscode POST".into()))?)
            }
            return Ok(http::Response::builder()
                .status(200)
                .body(Some(format!("POST accesscode at {} {}",req.uri(), parsed_url[4]).into()))?)
        }
        &Method::GET => {
            return Ok(http::Response::builder()
                .status(200)
                .body(Some(format!("Getting accesscode {} Id",parsed_url[parsed_url.len()-2]).into()))?)
        }
        &Method::DELETE => {
            return Ok(http::Response::builder()
                .status(200)
                .body(Some(format!("Deleting accesscode {} Id",parsed_url[parsed_url.len()-2]).into()))?)
        }
        _ => {
            return Ok(http::Response::builder()
                .status(500)
                .body(Some("".into()))?)
        }
    }
}