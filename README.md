![Speak Easy — Logging functionalities][splash]

[splash]: https://raw.githubusercontent.com/stefanodecillis/speak-easy/main/assets/crab-contained.jpg

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/speak-easy)](https://crates.io/crates/speak-easy)

# Speak-Easy

Speak-Easy is a Rust library that provides logging functionalities with different levels and rotation options built on top of tracing and compatible with tokio-rs.

## Features

- Different log levels
- Log rotation options
- Cleanup functionality

## Usage

First, add the following to your `Cargo.toml`:

```toml
[dependencies]
speak-easy = { version = "0.1" }
tokio = { features = ["macros", "rt-multi-thread"], version = "1.37.0" }
```

**Note**

If you want to use Speak-Easy without tokio, you must disable default features:

```toml
[dependencies]
speak-easy = { version = "0.1", default-features = false }
```

Then, use the library in your code like this:


```rust
use speak_easy::speak_easy::SpeakEasy;
use speak_easy::{info, Level};
use speak_easy::{Rotation, SpeakConfig};

#[tokio::main]
async fn main() {
    let speak_config = SpeakConfig::new(
        Rotation::Minutely,
        "./logs".to_string(),
        "my_log".to_string(),
    )
    .with_cleanup(24 * 60 * 60, 5);

    SpeakEasy::init(Level::INFO, Some(speak_config));

    info!("this is a log");
}


```

Please replace "/path/to/log/files" with the actual path where you want to store your log files.

## License
This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing
Feel free to open issues and send PRs. We will evaluate them together in the comment section.



