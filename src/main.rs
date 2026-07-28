mod banner;
mod commands;
mod calculator;
mod models;
mod utils;

use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(
    name = "zitro",
    version = "1.0.0",
    about = "Auditeur d'empreinte carbone numerique pour applications web",
    long_about = "ZITRO CLI audite l'empreinte carbone des applications web \
                  directement dans les pipelines CI/CD selon les standards RSE."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Lancer un audit d'empreinte carbone sur une URL
    Scan {
        /// URL de l'application a auditer (ex: http://localhost:3000)
        url: String,

        /// Code pays ISO pour le mix energetique (ex: TG, CI, SN, FR)
        #[arg(short, long)]
        country: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Scan { url, country }) => {
            banner::display_banner();
            println!();
            
            if let Err(e) = commands::scan::execute(&url, country).await {
                eprintln!();
                eprintln!("{}: {}", "Erreur".red().bold(), e);
                std::process::exit(1);
            }
        }
        None => {
            banner::display_banner();
            println!();
            banner::display_installation_success(env!("CARGO_PKG_VERSION"));
        }
    }
}