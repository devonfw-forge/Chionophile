use goose::{GooseAttack, GooseError, GooseScheduler};
use goose::config::{GooseDefault, GooseDefaultType};
use crate::benchmark::tasksets::{logic, visitor};

pub async fn launch()  -> Result<(), GooseError> {
    let mut attack = GooseAttack::initialize()?;
    attack = set_defaults(attack)?;
    attack = register_tasksets(attack);
    attack
        .set_scheduler(GooseScheduler::Serial)
        .execute()
        .await?
        .print();
    
    Ok(())
}

fn set_defaults(attack: GooseAttack) -> Result<GooseAttack, GooseError> {
    let attack_with_defaults = attack
        .set_default(GooseDefault::Host, "http://localhost:8081/jumpthequeue/services/rest/")?
        .set_default(GooseDefault::RequestLog, "goose-requests.log")?
        .set_default(GooseDefault::ErrorLog, "goose-error.log")?
        .set_default(GooseDefault::Users, 20)?
        .set_default(GooseDefault::HatchRate, "4")?
        .set_default(GooseDefault::RequestBody, true)?
        .set_default(GooseDefault::RunTime, 600)?
        .set_default(GooseDefault::NoResetMetrics, true)?
        .set_default(GooseDefault::StatusCodes, true)?
        .set_default(GooseDefault::ReportFile, "report.html")?;

        Ok(*attack_with_defaults)
}

fn register_tasksets(attack: GooseAttack) -> GooseAttack {
     attack
         .register_taskset(logic::taskset())
         .register_taskset(visitor::taskset())
}