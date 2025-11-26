mod analyzer;
mod automation;
mod filters;
mod models;
mod parser;

use analyzer::{Reporter, Scorer};
use anyhow::Result;
use automation::BrowserAutomation;
use clap::{Parser, Subcommand};
use filters::FilterEngine;
use models::Config;
use parser::CsvParser;

#[derive(Parser)]
#[command(name = "linkedin-cleaner")]
#[command(about = "LinkedIn connection and follower management tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Analyze {
        #[arg(short, long, default_value = "data/Connections.csv")]
        input: String,

        #[arg(short, long, default_value = "config.toml")]
        config: String,
    },
    Remove {
        #[arg(short, long, default_value = "data/Connections.csv")]
        input: String,

        #[arg(short, long, default_value = "config.toml")]
        config: String,

        #[arg(short, long)]
        email: String,

        #[arg(short, long)]
        password: String,
    },
    Export {
        #[arg(short, long, default_value = "data/Connections.csv")]
        input: String,

        #[arg(short, long, default_value = "output/unwanted.csv")]
        output: String,

        #[arg(short, long, default_value = "config.toml")]
        config: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { input, config } => {
            analyze_command(&input, &config)?;
        }
        Commands::Remove {
            input,
            config,
            email,
            password,
        } => {
            remove_command(&input, &config, &email, &password)?;
        }
        Commands::Export {
            input,
            output,
            config,
        } => {
            export_command(&input, &output, &config)?;
        }
    }

    Ok(())
}

fn analyze_command(input: &str, config_path: &str) -> Result<()> {
    let config = Config::load(config_path)?;
    let connections = CsvParser::parse_connections(input)?;

    println!("Loaded {} connections", connections.len());

    let filter_engine = FilterEngine::new(config);
    let results: Vec<_> = connections
        .iter()
        .map(|c| filter_engine.filter_connection(c))
        .collect();

    let report = Scorer::analyze(&results);
    Reporter::print_summary(&report);

    Ok(())
}

fn remove_command(input: &str, config_path: &str, email: &str, password: &str) -> Result<()> {
    let config = Config::load(config_path)?;
    let connections = CsvParser::parse_connections(input)?;

    let filter_engine = FilterEngine::new(config.clone());
    let results: Vec<_> = connections
        .iter()
        .map(|c| filter_engine.filter_connection(c))
        .collect();

    let automation = BrowserAutomation::new(config.automation);
    automation.remove_connections(&results, email, password)?;

    Ok(())
}

fn export_command(input: &str, output: &str, config_path: &str) -> Result<()> {
    let config = Config::load(config_path)?;
    let connections = CsvParser::parse_connections(input)?;

    let filter_engine = FilterEngine::new(config);
    let results: Vec<_> = connections
        .iter()
        .map(|c| filter_engine.filter_connection(c))
        .collect();

    Reporter::export_to_csv(&results, output)?;
    println!("Exported to {}", output);

    Ok(())
}
