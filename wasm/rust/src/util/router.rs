use spin_sdk::http::{Request, Response};
use anyhow::Result;

pub trait Router<T> {
    fn route(req: &Request, parsed_url: Vec<&str>)-> Result<Response>;
}