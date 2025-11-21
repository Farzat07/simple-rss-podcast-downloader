use simple_rss_podcast_downloader::*;

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
