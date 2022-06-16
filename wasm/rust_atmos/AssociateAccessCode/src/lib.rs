use serde::Serialize;
use suborbital::req;
use suborbital::runnable::*;
use jtq::accesscode::logic::api::accesscode_eto::AccessCodeEto;
use suborbital::db;
use suborbital::db::query;
use jtq::common::logic::service::Service;
use jtq::accesscode::logic::accesscode_service::AccessCodeService;
use anyhow::Result;

struct AssociateAccessCode{}

impl Runnable for AssociateAccessCode {
    fn run(&self, input: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        let in_string = String::from_utf8(input).unwrap();
    
        Ok(String::from(format!("hello associate accesscode {}", in_string)).as_bytes().to_vec())
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &AssociateAccessCode = &AssociateAccessCode{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
