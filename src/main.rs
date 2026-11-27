use cheomseongd::crawler::{ComputerCrawler, Crawler, run};
use cheomseongd::ops;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crawler = ComputerCrawler::new(30)?;
    let saved_notices = run(&crawler).await?;

    println!(
        "{}: {} notices saved",
        crawler.source_id(),
        saved_notices.len()
    );

    for notice in ops::notice::get_notices().unwrap() {
        println!("{:?}", notice);
        println!("-----------------------------------");
        println!("-----------------------------------");
    }

    Ok(())
}
