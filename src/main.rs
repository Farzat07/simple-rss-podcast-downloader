use clap::{Parser, ValueEnum};
use std::fs::create_dir_all;
use simple_rss_podcast_downloader::*;

#[derive(ValueEnum, Clone, Debug)]
enum Order {
    Newest,
    Oldest,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    feed_url: String,
    #[arg(default_value = ".")]
    output_dir: String,
    #[arg(short, long)]
    numbered: bool, // Whether to prefix the episode number
    #[arg(long, value_enum, default_value_t = Order::Oldest)]
    order: Order, // Order of download and numbering
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    println!("Fetching RSS feed from: {}", args.feed_url);

    let xml = fetch_feed(&args.feed_url)?;
    let channel = parse_feed(&xml)?;
    create_dir_all(&args.output_dir)?;
    let pad = channel.items().len().to_string().len();
    let newest_first = matches!(args.order, Order::Newest);
    for (i, url) in get_audio_urls(&channel, newest_first) {
        let prefix = if args.numbered {
            Some(format!("{:0width$}", i, width = pad))
        } else {
            None
        };
        println!("Downloading file: {}", url);
        download_file(url, &args.output_dir, prefix.as_deref())?;
    }

    Ok(())
}
