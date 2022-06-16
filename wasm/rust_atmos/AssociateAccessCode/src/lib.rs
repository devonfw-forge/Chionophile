use serde::Serialize;
use suborbital::req;
use suborbital::runnable::*;
use jtq::accesscode::logic::api::accesscode_eto::AccessCodeEto;
use jtq::accesscode::logic::api::accesscode_insert::AccessCodeInsert;
use suborbital::db;
use suborbital::db::query;
use jtq::common::logic::service::Service;
use jtq::accesscode::logic::accesscode_service::AccessCodeService;
use anyhow::Result;

struct AssociateAccessCode{}

impl Runnable for AssociateAccessCode {
    fn run(&self, input: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        suborbital::resp::content_type("application/json; charset=utf-8");
        let in_string = String::from_utf8(input).unwrap();

        let accesscode_insert: AccessCodeInsert = serde_json::from_str(&in_string).unwrap();

        let accesscode_eto: Result<Option<Vec<u8>>> = AccessCodeService::create(accesscode_insert);
        return match accesscode_eto {
            Ok(accesscode_option) =>
                match accesscode_option {
                    Some(accesscode) => Ok(accesscode),
                    _ =>  Err(RunErr::new(404, format!("No accesscode with id {}", id).as_str()))
                }
            Err(e) => Err(RunErr::new(500, "Internal Server Error"))
        }
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &AssociateAccessCode = &AssociateAccessCode{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
