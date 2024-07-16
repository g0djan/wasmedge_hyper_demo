#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::{body::HttpBody as _, Client};
use tokio::time::Instant;
// use futures::future::join_all;

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {

    // let url_str = "http://example.com/get?";
    let urls = vec![
        "http://example.com/get?",
        "http://google.com/get?",
        "http://rust-lang.org/get?",
        "http://github.com/get?",
        "http://eu.httpbin.org/get?msg=Hello"
    ];


    let mut ftrs = Vec::new();
    let start = Instant::now();
    for url_str in &urls {
        let url = url_str.parse::<hyper::Uri>().unwrap();
        ftrs.push(fetch_url(url));
    }

    let _results = futures::future::join_all(ftrs).await;
    let end = start.elapsed();

    println!("Elapsed time async: {:.2?}", end);


    let mut sync_results = Vec::new();
    let start = Instant::now();
    for url_str in &urls {
        let url = url_str.parse::<hyper::Uri>().unwrap();
        sync_results.push(fetch_url(url).await?);
    }

    let end = start.elapsed();

    println!("Elapsed time sync: {:.2?}", end);
    Ok(())
}

async fn fetch_url(url: hyper::Uri) -> Result<()> {
    let client = Client::new();
    let mut res = client.get(url).await?;
    while let Some(next) = res.data().await {
        let chunk = next?;
        println!("{:#?}", chunk);
    }

    Ok(())
}

