use clap::{App, Arg};
use fbvideo::{FbVideo, Quality};

fn main() {
    let mut lists = run();
    for fb in lists.iter_mut() {
        match fb.get_video_url() {
            Ok(url) => println!("{:?}", url),
            Err(e) => eprintln!("{:?}", e),
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
                .help("Get HD quality video URL") // Displayed when showing help info
                .long("hd") // Trigger this arg with "--hd"
                .conflicts_with("sd"), // Opposite of requires(), says "if the
                                       // user uses -a, they CANNOT use 'output'"
                                       // also has a conflicts_with_all(Vec<&str>))
        )
        .arg(
            Arg::with_name("sd")
                .help("Get SD quality video URL") // Displayed when showing help info
                .long("sd") // Trigger this arg with "--sd"
                .conflicts_with("hd"), // Opposite of requires(), says "if the
                                       // user uses -a, they CANNOT use 'output'"
                                       // also has a conflicts_with_all(Vec<&str>))
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
