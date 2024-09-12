use std::error::Error;

use clap::{Parser, Subcommand};
use elite::journal;
use human_panic::setup_panic;
use time::format_description;

fn main() -> Result<(), Box<dyn Error>> {
    setup_panic!();
    let cli = Cli::parse();

    match cli.command {
        Commands::Path { command } => match command {
            PathCommands::Backpack => println!("{}", journal::backpack_path().display()),
            PathCommands::Cargo => println!("{}", journal::cargo_path().display()),
            PathCommands::Market => println!("{}", journal::market_path().display()),
            PathCommands::ModulesInfo => println!("{}", journal::modules_info_path().display()),
            PathCommands::NavRoute => println!("{}", journal::nav_route_path().display()),
            PathCommands::Outfitting => println!("{}", journal::outfitting_path().display()),
            PathCommands::ShipLocker => println!("{}", journal::ship_locker_path().display()),
            PathCommands::Shipyard => println!("{}", journal::shipyard_path().display()),
            PathCommands::Status => println!("{}", journal::status_path().display()),
            PathCommands::LatestJournal => {
                println!("{}", journal::latest_journal_path()?.display())
            }
        },
        Commands::Test => {
            for event in journal::all_events()? {
                println!("{:?}", event);
            }
        }
        Commands::ChatHistory => {
            let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]")?;

            for event in journal::all_events()? {
                match event {
                    journal::Event::SendText {
                        timestamp,
                        to,
                        message,
                    } => println!("{}\t@{} me: {}", timestamp.format(&format)?, to, message),
                    journal::Event::ReceiveText {
                        timestamp,
                        from,
                        message,
                        channel,
                    } => println!(
                        "{}\t@{:?} {}: {}",
                        timestamp.format(&format)?,
                        channel,
                        from,
                        message
                    ),
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print path to...
    Path {
        #[command(subcommand)]
        command: PathCommands,
    },
    Test,
    ChatHistory,
}

#[derive(Subcommand)]
enum PathCommands {
    Backpack,
    Cargo,
    Market,
    ModulesInfo,
    NavRoute,
    Outfitting,
    ShipLocker,
    Shipyard,
    Status,
    LatestJournal,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}
