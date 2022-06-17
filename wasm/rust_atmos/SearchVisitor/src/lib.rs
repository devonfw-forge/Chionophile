use suborbital::runnable::*;
use jtq::common::logic::service::Service;
use jtq::visitor::logic::api::visitor_search_criteria::VisitorSearchCriteria;
use jtq::visitor::logic::visitor_service::VisitorService;

struct SearchVisitor{}

impl Runnable for SearchVisitor {
    fn run(&self, input: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        suborbital::resp::content_type("application/json; charset=utf-8");

        let criteria_string = String::from_utf8(input).unwrap();
        let criteria: Result<VisitorSearchCriteria, _> = serde_json::from_str(&criteria_string);
        if let Err(_) = criteria {
            return Err(RunErr::new(400, "Bad Request"));
        }
        let results = VisitorService::search(criteria.unwrap());
        match results {
            Ok(res) => Ok(res),
            Err(_) => Err(RunErr::new(500, "Internal Server Error"))
        }
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &SearchVisitor = &SearchVisitor{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
