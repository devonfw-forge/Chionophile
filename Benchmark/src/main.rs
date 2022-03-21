mod benchmark;
mod models;

use goose::prelude::*;

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    benchmark::load_test::launch().await?;
    Ok(())
}
