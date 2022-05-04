use goose::config::GooseConfiguration;
use serde::{Serialize, Deserialize};
use crate::input::benchmark::Benchmark;


#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    //pub base_url: String,
    //pub endpoints: Vec<String>
    pub host: String,
    pub port: Option<i32>,
    pub use_https: bool,
    pub base_path: Option<String>,
    #[serde(skip_serializing)]
    complete_path: Option<String>,
    users: usize,
    user_creation_rate: Option<String>,
    startup_time: Option<String>,
    run_time: String,
    goose_log: String,
    report_file: String,
    request_log: String,
    request_body: bool,
    error_log: String,
    debug_log: String,
    no_reset_metrics: Option<bool>,
    max_request_second: i32,
    pub benchmarks: Vec<Benchmark>,
}

impl BenchmarkConfig {
    pub fn get_goose_configuration(self) -> GooseConfiguration {
        GooseConfiguration {
            help: false,
            version: false,
            list: false,
            host: self.complete_path.unwrap(),
            users: Some(self.users),
            hatch_rate: self.user_creation_rate,
            startup_time: self.startup_time.unwrap_or("".to_string()),
            run_time: self.run_time,
            goose_log: self.goose_log,
            log_level: 0,
            verbose: 0,
            running_metrics: None,
            no_reset_metrics: self.no_reset_metrics.unwrap_or(false),
            no_metrics: false,
            no_task_metrics: false,
            no_error_summary: false,
            report_file: self.report_file,
            request_log: self.request_log,
            request_format: None,
            request_body: self.request_body,
            task_log: "".to_string(),
            task_format: None,
            error_log: self.error_log,
            error_format: None,
            debug_log: self.debug_log,
            debug_format: None,
            no_debug_body: false,
            status_codes: false,
            no_telnet: false,
            telnet_host: "".to_string(),
            telnet_port: 0,
            no_websocket: false,
            websocket_host: "".to_string(),
            websocket_port: 0,
            no_autostart: false,
            no_gzip: false,
            timeout: None,
            co_mitigation: None,
            throttle_requests: self.max_request_second as usize,
            sticky_follow: false,
            manager: false,
            expect_workers: None,
            no_hash_check: false,
            manager_bind_host: "".to_string(),
            manager_bind_port: 0,
            worker: false,
            manager_host: "".to_string(),
            manager_port: 0
        }
    }

    pub fn get_complete_path(&mut self) -> String {
        if self.complete_path.is_none() {
            let protocol = if self.use_https { "https://" } else { "http://" };

            let port = if self.port.is_some() {
                self.port.unwrap().to_string()
            } else {
                "".to_string()
            };
            let base_path = if self.base_path.is_some() {
                self.base_path.clone().unwrap()
            } else {
                "".to_string()
            };

            self.complete_path = Some(format!("{}{}:{}{}", protocol, self.host, port, base_path));
        }

        self.complete_path.clone().unwrap()
    }
}