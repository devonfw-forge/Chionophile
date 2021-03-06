use suborbital::runnable::*;
use jtq::common::logic::service::Service;
use jtq::accesscode::logic::accesscode_service::AccessCodeService;
use anyhow::Result;

struct AssociateAccessCode{}

impl Runnable for AssociateAccessCode {
    fn run(&self, input: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        suborbital::resp::content_type("application/json; charset=utf-8");
        

        let accesscode_eto: Result<Vec<u8>> = AccessCodeService::create(input);
        match accesscode_eto {
            Ok(accesscode) => Ok(accesscode),
            Err(_) => Err(RunErr::new(500, "Internal Server Error"))
        }
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &AssociateAccessCode = &AssociateAccessCode{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
