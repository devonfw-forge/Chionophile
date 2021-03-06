use suborbital::req;
use suborbital::runnable::*;
use jtq::common::logic::service::Service;
use jtq::accesscode::logic::accesscode_service::AccessCodeService;
use anyhow::Result;

struct DeleteAccessCode{}

impl Runnable for DeleteAccessCode {
    fn run(&self, _input: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        suborbital::resp::content_type("application/json; charset=utf-8");
        let id = req::url_param("id");

        let delete_result = AccessCodeService::delete(id.parse().unwrap_or(-1));
        if let Ok(delete_result) = delete_result {
            Ok(delete_result.to_string().as_bytes().to_vec())
        } else {
            Err(RunErr::new(404, format!("No accesscode with id {}", id).as_str()))
        }
    }
}





// initialize the runner, do not edit below //
static RUNNABLE: &DeleteAccessCode = &DeleteAccessCode{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
