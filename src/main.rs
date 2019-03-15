use clap::{App, Arg};
use fbvideo::{FbVideo, Quality};

fn main() {
    let mut lists = run();
    for fb in lists.iter_mut() {
        match fb.get_video_url() {
            Ok(url) => println!("{}", url),
            Err(e) => eprintln!("{:#?}", e),
        }
    }
}

fn run() -> Vec<FbVideo> {
    let matches = App::new("fbvideo")
        .version("0.1")
        .about("Get video URLs from Facebook URL.")
        .author("Lzu Tao")
        .arg(
            Arg::with_name("hd")
                .help("Get HD quality video URL")
                .long("hd")
                .conflicts_with("sd"),
        )
        .arg(
            Arg::with_name("sd")
                .help("Get SD quality video URL")
                .long("sd")
                .conflicts_with("hd"),
        )
        .arg(
            Arg::with_name("URL")
                .help("List of URLs to get video link")
                .multiple(true) // This flag should allow multiple
                .required(true), // By default this argument MUST be present
        )
        .get_matches();

    let quality = if matches.is_present("sd") {
        Quality::Sd
    } else {
        Quality::Hd
    };

    let mut vecs: Vec<FbVideo> = Vec::new();

    // We can also get the values for those arguments
    if let Some(urls) = matches.values_of("URL") {
        for url in urls {
            let fb = FbVideo::new(url, quality);
            vecs.push(fb);
        }
    }

    vecs
}
