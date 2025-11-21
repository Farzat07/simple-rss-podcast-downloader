# simple-rss-podcast-downloader

A simple Rust CLI application that downloads podcast episodes from an RSS feed.

## Installation

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.

Clone the repository:

```bash
git clone https://cgit.farzat.xyz/public/simple-rss-podcast-downloader
cd simple-rss-podcast-downloader
```

Build the project:

```bash
cargo build --release
```

## Usage

Run the application with:

```bash
simple-rss-podcast-downloader <RSS_FEED_URL> [OUTPUT_DIR]
```

Example:

```bash
simple-rss-podcast-downloader https://feeds.simplecast.com/54nAGcIl ./downloads
```
