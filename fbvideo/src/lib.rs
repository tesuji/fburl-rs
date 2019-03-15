//! This library is used to leak real video URL from Facebook.
//!
//! This crate operates by fetching video page source from Facebook and searching for
//! those string fields: `hd_src_no_ratelimit`, `sd_src_no_ratelimit`.
//!
//! # Networking
//!
//! This crate needs network to fetch page source from Facebook's URL.
//!
//! # Examples
//!
//! ```rust,no_run
//! use fbvideo::{FbVideo, Quality};
//!
//! fn main() {
//!     let mut fb = FbVideo::new(
//!         "https://www.facebook.com/817131355292571/videos/2101344733268123/",
//!         Quality::Hd,
//!     );
//!     match fb.get_video_url() {
//!         Ok(url) => println!("{:?}", url),
//!         Err(e) => panic!("{:?}", e),
//!     }
//! }
//! ```

use lazy_static::lazy_static;
use regex::Regex;
use reqwest;

/// This struct contains all methods necessary to get video URL or video title
/// from Facebook.
#[derive(Debug)]
pub struct FbVideo {
    /// Facebook URL point to a video.
    url: String,
    /// The quality of downloaded video.
    quality: Quality,
    /// HTML content of that `url`.
    content: String,
}

/// The quality of downloaded video.
#[derive(Debug, Clone, Copy)]
pub enum Quality {
    /// Standard Definition quality.
    Sd,
    /// High Definition quality.
    Hd,
}

/// Represent all possible errors encounter in this library.
#[derive(Debug)]
pub enum Error {
    /// Error is related to HTTP.
    HttpError,
    /// Error is from a `RedirectPolicy`.
    RedirectError,
    /// Error is from a request returning a 4xx error.
    ClientError,
    /// Error is from a request returning a 5xx error.
    ServerError,
    /// Error is related to a timeout.
    TimeoutError,
    /// Error is unknown.
    UnknownError,
}

impl FbVideo {
    /// Generate new instance of FbVideo.
    pub fn new(url: &str, quality: Quality) -> Self {
        FbVideo {
            url: String::from(url),
            quality,
            content: String::new(),
        }
    }

    /// Get real video URL (often `mp4` format) from Facebook URL.
    pub fn get_video_url(&mut self) -> Result<&str, Error> {
        self.crawl_page_source()?;
        Ok(FbVideo::grep_video_url(&self.content, self.quality))
    }

    /// Get video title from Facebook URL.
    pub fn get_video_title(&mut self) -> Result<&str, Error> {
        self.crawl_page_source()?;
        Ok(FbVideo::grep_video_title(&self.content))
    }

    fn grep_video_url(content: &str, quality: Quality) -> &str {
        const SD: &str = r#"sd_src(_no_ratelimit)?:\s*"([^"]+)""#;
        const HD: &str = r#"hd_src(_no_ratelimit)?:\s*"([^"]+)""#;
        lazy_static! {
            static ref URL_SD_REGEX: Regex = Regex::new(SD).unwrap();
            static ref URL_HD_REGEX: Regex = Regex::new(HD).unwrap();
        };

        match quality {
            Quality::Sd => &*URL_SD_REGEX,
            Quality::Hd => &*URL_HD_REGEX,
        }
        .captures(content)
        .unwrap()
        .get(2)
        .unwrap()
        .as_str()
    }

    fn grep_video_title(content: &str) -> &str {
        const TITLE: &str = r#"title id="pageTitle">(.+?)<\/title>"#;
        lazy_static! {
            static ref TITLE_REGEX: Regex = Regex::new(TITLE).unwrap();
        }

        TITLE_REGEX
            .captures(content)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
    }

    fn crawl_page_source(&mut self) -> Result<(), Error> {
        if self.content.is_empty() {
            self.content = match FbVideo::make_request(&self.url) {
                Ok(body) => body,
                Err(e) => return Err(FbVideo::handler_error(e)),
            };
        }
        Ok(())
    }

    fn make_request(url: &str) -> Result<String, reqwest::Error> {
        let mut headers = reqwest::header::HeaderMap::new();

        // Disguise as IE 9 on Windows 7.
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static(
                "Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; Trident/5.0)",
            ),
        );

        reqwest::Client::builder()
            .gzip(true)
            .default_headers(headers)
            .build()?
            .get(url)
            .send()?
            .text()
    }

    fn handler_error(e: reqwest::Error) -> Error {
        if e.is_http() {
            Error::HttpError
        } else if e.is_timeout() {
            Error::TimeoutError
        } else if e.is_redirect() {
            Error::RedirectError
        } else if e.is_client_error() {
            Error::ClientError
        } else if e.is_server_error() {
            Error::ServerError
        } else {
            Error::UnknownError
        }
    }
}
