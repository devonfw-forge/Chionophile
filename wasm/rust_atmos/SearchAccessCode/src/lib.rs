use serde::Serialize;
use suborbital::req;
use suborbital::runnable::*;
use jtq::accesscode::logic::api::accesscode_eto::AccessCodeEto;
use jtq::accesscode::logic::api::accesscode_search_criteria::AccessCodeSearchCriteria;
use suborbital::db;
use suborbital::db::query;
use jtq::common::logic::service::Service;
use jtq::accesscode::logic::accesscode_service::AccessCodeService;
use anyhow::Result;

struct SearchAccessCode{}

impl Runnable for SearchAccessCode {
    fn run(&self, input: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        suborbital::resp::content_type("application/json; charset=utf-8");

        let in_string = String::from_utf8(input).unwrap();
        let search_criteria: AccessCodeSearchCriteria = serde_json::from_str(&in_string).unwrap();

        let accesscode_eto: Result<Vec<u8>> = AccessCodeService::search(search_criteria);
        match accesscode_eto {
            Ok(accesscode) => Ok(accesscode),
            Err(e) => Err(RunErr::new(500, "Internal Server Error"))
        }
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &SearchAccessCode = &SearchAccessCode{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
