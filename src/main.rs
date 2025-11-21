use simple_rss_podcast_downloader::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (feed_url, output_dir) = parse_args();

    println!("Feed RSS feed from: {}", feed_url);

    let xml = fetch_feed(&feed_url)?;
    let channel = parse_feed(&xml)?;
    for url in get_audio_urls(&channel) {
        println!("Downloading file: {}", url);
        download_file(url, &output_dir)?;
    }

    Ok(())
}
