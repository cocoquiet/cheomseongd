use cheomseongd::daemon::{run_daemon, supports_crawler};
use cheomseongd::db::run_migrations;
use cheomseongd::models::crawler_config::NewCrawlerConfig;
use cheomseongd::ops::crawler_config::{set_crawler_config, unset_crawler_config};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "cheomseongd", about = "KNU notice crawler daemon")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Register or update a crawler's schedule.
    Set {
        #[arg(long)]
        crawler: String,
        #[arg(long, value_parser = clap::value_parser!(i64).range(1..))]
        interval: i64,
        #[arg(long, value_parser = clap::value_parser!(i32).range(1..))]
        count: i32,
    },
    /// Remove a crawler from the schedule.
    Unset { crawler: String },
    /// Start the daemon and run configured crawlers.
    Start,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_migrations()?;

    match Cli::parse().command {
        Commands::Set {
            crawler,
            interval,
            count,
        } => {
            if !supports_crawler(&crawler) {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("unsupported crawler '{crawler}'"),
                )
                .into());
            }

            let config = set_crawler_config(NewCrawlerConfig {
                crawler_id: &crawler,
                interval_seconds: interval,
                notice_count: count,
            });
            println!(
                "{} configured: every {} seconds, up to {} notices",
                config.crawler_id, config.interval_seconds, config.notice_count
            );
        }
        Commands::Unset { crawler } => {
            let deleted = unset_crawler_config(&crawler)?;
            if deleted == 0 {
                println!("'{crawler}' was not configured");
            } else {
                println!("'{crawler}' removed from the schedule");
            }
        }
        Commands::Start => run_daemon().await?,
    }

    Ok(())
}
