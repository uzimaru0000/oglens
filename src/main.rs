use anyhow::Result;
use clap::Parser;

use crate::app::App;

mod app;

#[tokio::main]
async fn main() -> Result<()> {
    let app = App::parse();
    app.run().await
}
