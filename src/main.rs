use clap::Parser;
use simple_rss_podcast_downloader::*;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    feed_url: String,
    #[arg(default_value = ".")]
    output_dir: String,
    #[arg(short, long)]
    numbered: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    println!("Feed RSS feed from: {}", args.feed_url);

    let xml = fetch_feed(&args.feed_url)?;
    let channel = parse_feed(&xml)?;
    let pad = channel.items().len().to_string().len();
    for (i, url) in get_audio_urls(&channel) {
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
