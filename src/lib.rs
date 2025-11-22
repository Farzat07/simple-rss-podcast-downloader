use rss::Channel;
use std::fs::File;
use std::path::PathBuf;

/// Fetch RSS feed content from a URL
pub fn fetch_feed(url: &str) -> Result<String, reqwest::Error> {
    reqwest::blocking::get(url)?.text()
}

/// Parse RSS XML into a Channel
pub fn parse_feed(xml: &str) -> Result<Channel, rss::Error> {
    Channel::read_from(xml.as_bytes())
}

/// Extract the audio URLs from the given channel
pub fn get_audio_urls(channel: &Channel, newest_first: bool) -> Vec<(usize, &str)> {
    let iter = channel.items().iter();
    if newest_first {
        iter.enumerate()
            .filter_map(|(i, item)| item.enclosure().map(|e| (i + 1, e.url())))
            .collect()
    } else {
        iter.rev()
            .enumerate()
            .filter_map(|(i, item)| item.enclosure().map(|e| (i + 1, e.url())))
            .collect()
    }
}

/// Download the given audio file to the supplied directory
pub fn download_file(
    url: &str,
    output_dir: &str,
    prefix: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut response = reqwest::blocking::get(url)?;
    let filename = url.split('/').next_back().unwrap_or("episode.mp3");
    let filename = if let Some(p) = prefix {
        format!("{}-{}", p, filename)
    } else {
        filename.to_string()
    };
    let mut path = PathBuf::from(output_dir);
    path.push(filename);
    let mut file = File::create(path)?;
    std::io::copy(&mut response, &mut file)?;
    Ok(())
}
