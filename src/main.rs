use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = false)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Configure gr
    Config {},
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Config {} => {
            println!("config");
        }
    }
}
