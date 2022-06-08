use anyhow::Result;
use http::Method;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

fn visitor_router(req: &Request, parsed_url: Vec<&str>)-> Result<Response>{
    match req.method() {
        &Method::POST => {
            if parsed_url[parsed_url.len()-2] == "search"{
                return Ok(http::Response::builder()
                .status(200)
                .body(Some("Searching Visitor POST".into()))?)
            }
            return Ok(http::Response::builder()
                .status(200)
                .body(Some(format!("POST Visitor at {} {}",req.uri(), parsed_url[4]).into()))?)
        }
        &Method::GET => {
            return Ok(http::Response::builder()
                .status(200)
                .body(Some(format!("Getting Visitor {} Id",parsed_url[parsed_url.len()-2]).into()))?)
        }
        &Method::DELETE => {
            return Ok(http::Response::builder()
                .status(200)
                .body(Some(format!("Deleting Visitor {} Id",parsed_url[parsed_url.len()-2]).into()))?)
        }
        _ => {
            return Ok(http::Response::builder()
                .status(500)
                .body(Some("".into()))?)
        }
    }
}

fn queue_router(req: &Request, parsed_url: Vec<&str>)-> Result<Response>{
    match req.method() {
        &Method::POST => {
            if parsed_url[parsed_url.len()-2] == "search"{
                return Ok(http::Response::builder()
                .status(200)
                .body(Some("Searching queue POST".into()))?)
            }
            return Ok(http::Response::builder()
                .status(200)
                .body(Some(format!("POST queue at {} {}",req.uri(), parsed_url[4]).into()))?)
        }
        &Method::GET => {
            return Ok(http::Response::builder()
                .status(200)
                .body(Some(format!("Getting queue {} Id",parsed_url[parsed_url.len()-2]).into()))?)
        }
        &Method::DELETE => {
            return Ok(http::Response::builder()
                .status(200)
                .body(Some(format!("Deleting queue {} Id",parsed_url[parsed_url.len()-2]).into()))?)
        }
        _ => {
            return Ok(http::Response::builder()
                .status(500)
                .body(Some("".into()))?)
        }
    }
}

fn accesscode_router(req: &Request, parsed_url: Vec<&str>)-> Result<Response>{
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


#[http_component]
fn router(req: Request) -> Result<Response> {
    let parsed_url: Vec<&str> = req.uri().path().split('/').collect();
    match parsed_url[4] {
        "visitormanagement" => {
            return visitor_router(&req, parsed_url)
        }

        "queuemanagement" => {
            return queue_router(&req, parsed_url)
        }

        "accesscodemanagement" => {
            return accesscode_router(&req, parsed_url)
        }

        _=>{
            return Ok(http::Response::builder()
                        .status(500)
                        .body(Some("".into()))?)
        }

    }
}
