use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::pin::Pin;
use std::time::{Duration, Instant};

use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};

use crate::crawler::{ComputerCrawler, run};
use crate::ops::crawler_config::get_crawler_configs;

const CONFIG_POLL_INTERVAL: Duration = Duration::from_secs(2);

type CrawlerFuture = Pin<Box<dyn Future<Output = Result<usize, String>> + Send>>;
type CrawlerRunner = fn(usize) -> CrawlerFuture;

struct CompletionNotifier {
    crawler_id: String,
    sender: UnboundedSender<(String, Instant)>,
}

impl Drop for CompletionNotifier {
    fn drop(&mut self) {
        let _ = self.sender.send((self.crawler_id.clone(), Instant::now()));
    }
}

/// The single registry mapping crawler IDs to their execution functions.
fn crawler_runner(crawler_id: &str) -> Option<CrawlerRunner> {
    match crawler_id {
        "computer" => Some(run_computer),
        _ => None,
    }
}

pub fn supports_crawler(crawler_id: &str) -> bool {
    crawler_runner(crawler_id).is_some()
}

fn run_computer(notice_count: usize) -> CrawlerFuture {
    Box::pin(async move {
        let crawler = ComputerCrawler::new(notice_count).map_err(|error| error.to_string())?;
        run(&crawler)
            .await
            .map(|saved| saved.len())
            .map_err(|error| error.to_string())
    })
}

fn spawn_crawler(
    crawler_id: String,
    notice_count: usize,
    runner: CrawlerRunner,
    completion_sender: UnboundedSender<(String, Instant)>,
) {
    tokio::spawn(async move {
        let _completion = CompletionNotifier {
            crawler_id: crawler_id.clone(),
            sender: completion_sender,
        };

        match runner(notice_count).await {
            Ok(count) => println!("{crawler_id}: {count} notices saved"),
            Err(error) => eprintln!("{crawler_id}: crawl failed: {error}"),
        }
    });
}

/// Runs configured crawlers, reloading their settings from the database every few seconds.
pub async fn run_daemon() -> Result<(), Box<dyn std::error::Error>> {
    let mut next_run_by_crawler = HashMap::<String, Instant>::new();
    let mut in_flight = HashSet::<String>::new();
    let mut completed_at = HashMap::<String, Instant>::new();
    let (completed_tx, mut completed_rx) = unbounded_channel::<(String, Instant)>();

    loop {
        while let Ok((crawler_id, finished_at)) = completed_rx.try_recv() {
            in_flight.remove(&crawler_id);
            completed_at.insert(crawler_id, finished_at);
        }

        let configs = get_crawler_configs()?;
        let active_crawlers = configs
            .iter()
            .map(|config| config.crawler_id.clone())
            .collect::<HashSet<_>>();
        next_run_by_crawler.retain(|crawler_id, _| active_crawlers.contains(crawler_id));
        completed_at.retain(|crawler_id, _| active_crawlers.contains(crawler_id));

        for config in configs {
            if let Some(finished_at) = completed_at.remove(&config.crawler_id) {
                next_run_by_crawler.insert(
                    config.crawler_id.clone(),
                    finished_at + Duration::from_secs(config.interval_seconds as u64),
                );
            }

            if in_flight.contains(&config.crawler_id) {
                continue;
            }

            let now = Instant::now();
            if next_run_by_crawler
                .get(&config.crawler_id)
                .is_some_and(|next_run| now < *next_run)
            {
                continue;
            }

            let crawler_id = config.crawler_id.clone();
            let notice_count = config.notice_count as usize;
            let completion_sender = completed_tx.clone();

            if let Some(runner) = crawler_runner(&crawler_id) {
                in_flight.insert(crawler_id.clone());
                spawn_crawler(crawler_id, notice_count, runner, completion_sender);
            } else {
                eprintln!("No crawler implementation is registered for '{crawler_id}'");
                next_run_by_crawler.insert(
                    crawler_id,
                    now + Duration::from_secs(config.interval_seconds as u64),
                );
            }
        }

        tokio::time::sleep(CONFIG_POLL_INTERVAL).await;
    }
}
