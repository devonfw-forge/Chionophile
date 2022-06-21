use suborbital::req;
use suborbital::runnable::*;
use jtq::common::logic::service::Service;
use jtq::accesscode::logic::accesscode_service::AccessCodeService;
use anyhow::Result;

struct GetAccessCode{}

impl Runnable for GetAccessCode {
    fn run(&self, _input: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        suborbital::resp::content_type("application/json; charset=utf-8");
        let id = req::url_param("id");

        let accesscode_eto: Result<Option<Vec<u8>>> = AccessCodeService::get_by_id(id.parse().unwrap_or(-1));
        return match accesscode_eto {
            Ok(accesscode_option) =>
                match accesscode_option {
                    Some(accesscode) => Ok(accesscode),
                    _ =>  Err(RunErr::new(404, format!("No accesscode with id {}", id).as_str()))
                }
            Err(_) => Err(RunErr::new(500, "Internal Server Error"))
        }
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &GetAccessCode = &GetAccessCode{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
