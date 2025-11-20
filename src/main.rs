use std::env;
use rss::Channel;

/// Parse CLI arguments and return (feed_url, output_dir)
fn parse_args() -> (String, String) {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <RSS_FEED_URL> [OUTPUT_DIR]", args[0]);
        std::process::exit(1);
    }

    let feed_url = args.remove(1);
    let output_dir = if args.len() > 1 { args.remove(1) } else { String::from(".") };

    (feed_url, output_dir)
}

/// Fetch RSS feed content from a URL
fn fetch_feed(url: &str) -> Result<String, reqwest::Error> {
    reqwest::blocking::get(url)?.text()
}

/// Parse RSS XML into a Channel
fn parse_feed(xml: &str) -> Result<Channel, rss::Error> {
    Channel::read_from(xml.as_bytes())
}

// Extract the audio URLs from the given channel
fn get_audio_urls(channel: &Channel) -> Vec<&str> {
    let mut audio_urls = Vec::new();
    for item in channel.items() {
        if let Some(enclosure) = item.enclosure() {
            audio_urls.push(enclosure.url())
        }
    }
    audio_urls
}

fn main() {
    let (feed_url, output_dir) = parse_args();

    println!("Feed RSS feed from: {}", feed_url);

    let xml = fetch_feed(&feed_url).expect("Error fetching feed");
    let channel = parse_feed(&xml).expect("Error parsing feed");
    for url in get_audio_urls(&channel) {
        println!("Audio URL: {}", url)
    }

    println!("Output directory: {}", output_dir);
}
