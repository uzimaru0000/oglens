use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};
use oglens::{builder::Prefix, input, ogp::Ogp};
use tokio::io::BufReader;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct App {
    /// HTML file path
    #[clap(value_parser)]
    path: Option<PathBuf>,

    /// Displayed in JSON format
    #[clap(short, long)]
    json: bool,

    /// Prefix settings other than OG. Specify key separated by colons. ex: --prefix=twitter:name
    #[clap(short, long, value_parser = Prefix::value_parser)]
    prefix: Option<Vec<Prefix>>,
}

impl App {
    pub async fn run(self) -> Result<()> {
        if self.path.is_none() && !input::is_pipe() {
            Self::command()
                .print_long_help()
                .with_context(|| "failed")?;
            return Ok(());
        }

        let raw = match self.path.clone() {
            Some(p) => {
                let file = tokio::fs::File::open(p).await?;
                let mut reader = BufReader::new(file);
                input::read(&mut reader).await
            }
            None => input::read_stdin().await,
        }?;

        let ogp = Ogp::from(&raw, self.prefix.unwrap_or_default())?;

        let output = if self.json {
            ogp.render()?
        } else {
            format!("{}", ogp)
        };
        input::write_stdout(output.as_bytes()).await?;

        Ok(())
    }
}
