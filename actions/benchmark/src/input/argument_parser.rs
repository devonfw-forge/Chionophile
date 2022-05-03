use clap::Parser;

#[derive(Parser, Debug)]
pub struct ArgumentParser {
    #[clap(short='i', long="input", parse(from_os_str))]
    pub path: std::path::PathBuf,
}