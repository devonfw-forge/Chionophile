use serde::Serialize;
use suborbital::req;
use suborbital::runnable::*;
use jtq::visitor::logic::api::visitor_eto::VisitorEto;

struct GetVisitor{}

impl Runnable for GetVisitor {
    fn run(&self, input: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        let in_string = String::from_utf8(input).unwrap();

        let id = req::url_param("id");
        let visitor: VisitorEto = VisitorEto {
            id: None,
            modification_counter: None,
            username: None,
            name: None,
            phone_number: None,
            password: None,
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
