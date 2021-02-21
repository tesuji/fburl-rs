# This crate doesn't work as expected

Facebook has changed their content site and I haven't had time/interest to
complete with new algorithm.

I am always welcome anybody open pull requests to fix this problem.

# fburl

Get video URLs from Facebook.

[![Build Status][actions-badge]][actions-url]

[actions-badge]: https://github.com/lzutao/fburl-rs/workflows/Rust/badge.svg?branch=master&event=push
[actions-url]: https://github.com/lzutao/fburl-rs/actions

### Usage

```console
$ fburl --help
fburl 0.2.0
Lzu Tao
Get video URLs from Facebook URL.

USAGE:
    fburl [FLAGS] <URL>...

FLAGS:
    -h, --help       Prints help information
        --hd         Get HD quality video URL
        --sd         Get SD quality video URL
    -V, --version    Prints version information

ARGS:
    <URL>...    List of URLs to get video link
```
