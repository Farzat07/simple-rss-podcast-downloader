use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <RSS_FEED_URL> [OUTPUT_DIR]", args[0]);
        std::process::exit(1);
    }

    let feed_url = &args[1];
    let output_dir = if args.len() > 2 { &args[2] } else { "." };

    println!("Feed URL: {}", feed_url);
    println!("Output directory: {}", output_dir);
}
