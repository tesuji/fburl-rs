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
//! async fn foo() {
//!     let mut fb = FbVideo::new(
//!         "https://www.facebook.com/817131355292571/videos/2101344733268123/",
//!         Quality::Hd,
//!     );
//!     match fb.get_video_url().await {
//!         Ok(url) => println!("{:?}", url),
//!         Err(e) => panic!("{:?}", e),
//!     }
//! }
//! ```

#![deny(rust_2018_idioms)]
#[doc(html_root_url = "https://docs.rs/fbvideo/0.4.0")]
use std::fmt;

use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::Client;

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
    /// Error is from a `RedirectPolicy`.
    Redirect,
    /// Error is related to a timeout.
    Timeout,
    /// Target site has no video link.
    InvalidUrl,
    /// Error is unknown.
    Unknown,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            Error::Redirect => "Error is from a `RedirectPolicy`",
            Error::Timeout => "Error is related to a timeout",
            Error::InvalidUrl => "Target site has no video link",
            Error::Unknown => "Error is unknown",
        };
        f.write_str(description)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            Error::Timeout
        } else if e.is_redirect() {
            Error::Redirect
        } else if e.url().is_none() {
            Error::InvalidUrl
        } else {
            Error::Unknown
        }
    }
}

macro_rules! global_regex {
    ($NAME:ident, $re:literal) => {
        static $NAME: Lazy<Regex> = Lazy::new(|| Regex::new($re).unwrap());
    };
}

global_regex!(URL_SD_RE, r#"sd_src(_no_ratelimit)?:[ \t\n\r\f]*"([^"]+)""#);
global_regex!(URL_HD_RE, r#"hd_src(_no_ratelimit)?:[ \t\n\r\f]*"([^"]+)""#);
global_regex!(TITLE_RE, r#"title id="pageTitle">([^<]+)</title>"#);

static GZIP_CLIENT: Lazy<Client> = Lazy::new(|| {
    use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
    let mut headers = HeaderMap::new();
    // Disguise as IE 9 on Windows 7.
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; Trident/5.0)"),
    );
    Client::builder().gzip(true).default_headers(headers).build().unwrap()
});

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
    pub async fn get_video_url(&mut self) -> Result<&str, Error> {
        self.crawl_page_source().await?;
        grep_video_url(&self.content, self.quality).ok_or(Error::InvalidUrl)
    }

    /// Get video title from Facebook URL.
    pub async fn get_video_title(&mut self) -> Result<&str, Error> {
        self.crawl_page_source().await?;
        grep_video_title(&self.content).ok_or(Error::InvalidUrl)
    }

    async fn crawl_page_source(&mut self) -> Result<(), Error> {
        if self.content.is_empty() {
            self.content = make_request(&self.url).await.map_err(Error::from)?.into_boxed_str();
        }
        Ok(())
    }
}

async fn make_request(url: &str) -> Result<String, reqwest::Error> {
    GZIP_CLIENT.get(url).send().await?.text().await
}

fn grep_video_url(content: &str, quality: Quality) -> Option<&str> {
    let regex = match quality {
        Quality::Sd => &URL_SD_RE,
        Quality::Hd => &URL_HD_RE,
    };

    regex
        .captures(content)
        .and_then(|captures| captures.get(2).map(|m| m.as_str()))
}

fn grep_video_title(content: &str) -> Option<&str> {
    TITLE_RE
        .captures(content)
        .and_then(|captures| captures.get(1).map(|m| m.as_str()))
}
