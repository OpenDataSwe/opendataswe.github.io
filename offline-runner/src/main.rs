mod generate_help_from_markdown;
mod generate_municipalities;

use crate::generate_help_from_markdown::generate_html_consts;
use crate::generate_municipalities::generate_municipalities;
use clap::Parser;
use skatt_lib::api::Client;
use skatt_lib::CURRENT_YEAR;
use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
pub struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    GenerateMunicipalities {
        #[clap(long, short)]
        output: PathBuf,
    },
    GenerateHtml {
        #[clap(long, short)]
        input: PathBuf,
        #[clap(long, short)]
        output: PathBuf,
    },
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Args::parse();
    match args.command {
        Command::GenerateMunicipalities { output } => {
            let client = Client::new();
            let mut m = client
                .fetch_available_municipalities(CURRENT_YEAR)
                .await
                .unwrap()
                .into_iter()
                .collect::<Vec<_>>();
            m.sort();
            let out = generate_municipalities(m).unwrap();
            std::fs::write(&output, out).unwrap();
        }
        Command::GenerateHtml { input, output } => {
            let content = generate_html_consts(&input);
            std::fs::write(&output, content).unwrap();
        }
    }
}
