use std::path::PathBuf;

pub struct Validator {
    pub path: std::path::PathBuf,
}

impl From<std::path::PathBuf> for Validator {
    fn from(path: PathBuf) -> Self {
        Validator {
            path
        }
    }
}

impl Validator {
    pub fn validate(&self) -> bool  {
        if self.path.is_file() && self.path.extension().is_some() {
            self.path.extension().unwrap() == "yaml"
        } else {
            false
        }
    }
}

