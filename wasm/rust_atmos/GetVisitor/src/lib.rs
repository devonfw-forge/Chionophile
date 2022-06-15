use serde::Serialize;
use suborbital::req;
use suborbital::runnable::*;
use common::entities::visitor::VisitorEntity;

struct GetVisitor{}

impl Runnable for GetVisitor {
    fn run(&self, input: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        let in_string = String::from_utf8(input).unwrap();

        let id = req::url_param("id");
        let visitor: VisitorEntity = VisitorEntity {
            id: 0,
            modification_counter: 0,
            username: None,
            name: None,
            password: None,
            phone_number: None,
            accepted_commercial: None,
            accepted_terms: false,
            user_type: None
        };
        let res = serde_json::to_string(&visitor).unwrap();

        Ok(res.as_bytes().to_vec())
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &GetVisitor = &GetVisitor{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
