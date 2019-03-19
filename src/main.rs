use clap::{App, Arg, ArgMatches};
use fbvideo::{FbVideo, Quality};

fn main() {
    run();
}

fn arg_parse<'a>() -> ArgMatches<'a> {
    App::new("fburl")
        .version("0.2.0")
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
        .get_matches()
}

fn run() {
    let matches = arg_parse();

    let quality = if matches.is_present("sd") {
        Quality::Sd
    } else {
        Quality::Hd
    };

    // We can also get the values for those arguments
    if let Some(urls) = matches.values_of("URL") {
        for url in urls {
            match FbVideo::new(url, quality).get_video_url() {
                Ok(url) => println!("{}", url),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}
