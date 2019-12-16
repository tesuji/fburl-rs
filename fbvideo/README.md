# fbvideo

Library to get video URLs from Facebook.

[![Documentation](https://docs.rs/fbvideo/badge.svg)](https://docs.rs/fbvideo)
[![Crates.io](https://img.shields.io/crates/v/fbvideo.svg)](https://crates.io/crates/fbvideo)

### Examples

```rust
use fbvideo::{FbVideo, Quality};
#[tokio::main]
async fn main() {
    let mut fb = FbVideo::new(
        "https://www.facebook.com/817131355292571/videos/2101344733268123/",
        Quality::Hd,
    );
    match fb.get_video_url().await {
        Ok(url) => println!("{:?}", url),
        Err(e) => panic!("{:?}", e),
    }
}
```
