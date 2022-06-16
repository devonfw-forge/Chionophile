use serde::Serialize;
use suborbital::req;
use suborbital::runnable::*;
use jtq::accesscode::logic::api::accesscode_eto::AccessCodeEto;
use suborbital::db;
use suborbital::db::query;
use jtq::common::logic::service::Service;
use jtq::accesscode::logic::accesscode_service::AccessCodeService;
use anyhow::Result;

struct DeleteAccessCode{}

impl Runnable for DeleteAccessCode {
    fn run(&self, input: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        suborbital::resp::content_type("application/json; charset=utf-8");
        let id = req::url_param("id");

        let accesscode_eto: Result<i64> = AccessCodeService::delete(id.parse().unwrap_or(-1));
        return match accesscode_eto {
            Ok(accesscode) => {
                Ok(format!("{}", accesscode).as_bytes().to_vec())
            },
            Err(e) => Err(RunErr::new(500, "Internal Server Error"))
        }
    }
}





// initialize the runner, do not edit below //
static RUNNABLE: &DeleteAccessCode = &DeleteAccessCode{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
