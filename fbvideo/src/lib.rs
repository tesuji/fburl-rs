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
//! let mut fb = FbVideo::new(
//!     "https://www.facebook.com/817131355292571/videos/2101344733268123/",
//!     Quality::Hd,
//! );
//! match fb.get_video_url() {
//!     Ok(url) => println!("{:?}", url),
//!     Err(e) => panic!("{:?}", e),
//! }
//! ```

#![deny(rust_2018_idioms)]

use once_cell::sync::Lazy;
use regex::Regex;

/// This struct contains all methods necessary to get video URL or video title
/// from Facebook.
#[derive(Debug)]
pub struct FbVideo<'fb> {
    /// Facebook URL point to a video.
    url: &'fb str,
    /// The quality of downloaded video.
    quality: Quality,
    /// HTML content of that `url`.
    content: Box<str>,
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
    /// Target site has no video link.
    InvalidUrl,
    /// Error is unknown.
    UnknownError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = match self {
            Error::HttpError => "Error is related to HTTP",
            Error::RedirectError => "Error is from a `RedirectPolicy`",
            Error::ClientError => "Error is from a request returning a 4xx error",
            Error::ServerError => "Error is from a request returning a 5xx error",
            Error::TimeoutError => "Error is related to a timeout",
            Error::InvalidUrl => "Target site has no video link",
            Error::UnknownError => "Error is unknown",
        };
        f.write_str(description)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
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
        } else if e.url().is_none() {
            Error::InvalidUrl
        } else {
            Error::UnknownError
        }
    }
}

const SD_RX: &str = r#"sd_src(_no_ratelimit)?:\s*"([^"]+)""#;
const HD_RX: &str = r#"hd_src(_no_ratelimit)?:\s*"([^"]+)""#;
const TITLE_RX: &str = r#"title id="pageTitle">([^<]+)</title>"#;

static URL_SD_RE: Lazy<Regex> = Lazy::new(|| Regex::new(SD_RX).unwrap());
static URL_HD_RE: Lazy<Regex> = Lazy::new(|| Regex::new(HD_RX).unwrap());
static TITLE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(TITLE_RX).unwrap());

impl<'fb> FbVideo<'fb> {
    /// Generate new instance of FbVideo.
    pub fn new(url: &'fb str, quality: Quality) -> Self {
        Self {
            url,
            quality,
            content: String::new().into_boxed_str(),
        }
    }

    /// Get real video URL (often `mp4` format) from Facebook URL.
    pub fn get_video_url(&mut self) -> Result<&str, Error> {
        self.crawl_page_source()?;
        Self::grep_video_url(&self.content, self.quality).ok_or(Error::InvalidUrl)
    }

    /// Get video title from Facebook URL.
    pub fn get_video_title(&mut self) -> Result<&str, Error> {
        self.crawl_page_source()?;
        Self::grep_video_title(&self.content).ok_or(Error::InvalidUrl)
    }

    fn grep_video_url(content: &str, quality: Quality) -> Option<&str> {
        if let Some(caps) = match quality {
            Quality::Sd => &*URL_SD_RE,
            Quality::Hd => &*URL_HD_RE,
        }
        .captures(content)
        {
            Some(caps.get(2).unwrap().as_str())
        } else {
            None
        }
    }

    fn grep_video_title(content: &str) -> Option<&str> {
        if let Some(caps) = TITLE_RE.captures(content) {
            Some(caps.get(1).unwrap().as_str())
        } else {
            None
        }
    }

    fn crawl_page_source(&mut self) -> Result<(), Error> {
        if self.content.is_empty() {
            self.content = Self::make_request(&self.url)
                .map_err(Error::from)?
                .into_boxed_str();
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
}
