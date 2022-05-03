use crate::input::test::Test;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestGroup {
    pub tests: Vec<Test>,
    #[serde(skip_serializing)]
    pub base_path: Option<String>
}

impl TestGroup {
    pub fn new(
        tests: Vec<Test>,
        base_path: String
    ) -> Self {
        TestGroup {
            tests,
            base_path: Option::from(base_path)
        }
    }
}