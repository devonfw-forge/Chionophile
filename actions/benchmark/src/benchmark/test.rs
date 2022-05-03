use std::process;
use std::sync::Arc;
use goose::{GooseAttack, GooseError, GooseScheduler, taskset};
use goose::config::GooseConfiguration;
use goose::goose::{GooseTask, GooseTaskFunction, GooseTaskSet};
use serde_json::Value;
use crate::input::http_methods::HttpMethod;
use crate::input::test_group::TestGroup;

pub struct BenchmarkTest {
    test_groups: Vec<TestGroup>,
    goose_configuration: GooseConfiguration
}

impl BenchmarkTest {
    pub fn new(
        test_groups: Vec<TestGroup>,
        goose_configuration: GooseConfiguration
    ) -> Self {
        BenchmarkTest {
            test_groups,
            goose_configuration
        }
    }

    pub async fn launch(self) -> Result<(), GooseError> {
        println!("Initializing goose");
        let attack_res = GooseAttack::initialize_with_config(self.goose_configuration.clone());
        if !attack_res.is_err() {
            println!("Goose initialized");
            let mut attack = attack_res.unwrap();
            println!("Registering tasksets");
            attack = attack.register_taskset(self.create_taskset());
            attack.set_scheduler(GooseScheduler::Serial).execute().await?.print();
        } else {
            println!("{:?}", attack_res.err())
        }

        Ok(())
    }

    pub fn create_taskset(self) -> GooseTaskSet {
        println!("Creating taskset");
        let mut taskset = taskset!("Example name");

        for test_group in self.test_groups {
            let requests: GooseTaskFunction = Arc::new(move |user| {
                let tests = test_group.tests.clone();
                let base_path = test_group.base_path.clone().unwrap_or("".to_string());
                Box::pin (async move {
                    for test in tests {
                        match test.method {
                            HttpMethod::GET
                            | HttpMethod::DELETE => {
                                user.get(format!("{}{}", base_path, test.path).as_ref()).await?;
                            }
                            HttpMethod::POST => {
                                let body = test.body.unwrap_or("".to_string());
                                let response_body : Value;
                                if test.content_type == Some("application/json".to_string()) {
                                    let json : Result<Value, serde_json::Error> = serde_json::from_str(&body);
                                    if json.is_ok() {
                                        let req_response = user.post_json(format!("{}{}", base_path, test.path).as_ref(), &json.unwrap()).await?;
                                        response_body = req_response.response?.json::<Value>().await?;
                                    } else {
                                        log::error!("Invalid JSON format in POST {}\n{}\n{}", test.path, body, json.err().unwrap());
                                        process::exit(1);
                                    }
                                } else {
                                    let req_response = user.post(format!("{}{}", base_path, test.path).as_ref(), body).await?;
                                    response_body = req_response.response?.json::<Value>().await?;
                                }
                                let delete = test.should_delete.unwrap_or(false);
                                if delete {
                                    let slash = test.delete_end_slash.unwrap_or(false);
                                    let path = if slash {
                                        format!("{}{}{}/", base_path, test.path, response_body[test.body_field_for_delete.unwrap_or("".to_string())])
                                    } else {
                                        format!("{}{}{}", base_path, test.path, response_body[test.body_field_for_delete.unwrap_or("".to_string())])
                                    };
                                    user.delete(&path).await?;

                                }
                            }
                        }
                    }

                    Ok(())
                })
            });
            let task = GooseTask::new(requests);
            taskset = taskset.register_task(task);
        }
        println!("Taskset created");
        taskset
    }
}