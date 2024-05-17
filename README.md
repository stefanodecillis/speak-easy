![Speak Easy — Logging functionalities][splash]

[splash]: https://raw.githubusercontent.com/stefanodecillis/speak-easy/main/assets/crab-contained.jpg

# Speak-Easy

Speak-Easy is a Rust library that provides logging functionalities with different levels and rotation options built on top of tokio-rs tracing.

## Features

- Different log levels
- Log rotation options
- Cleanup functionality

## Usage

First, add the following to your `Cargo.toml`:

```toml
[dependencies]
speak-easy = "0.1.0"
```

Then, use the library in your code like this:


```rust
use speak_easy::{SpeakEasy, SpeakConfig, Rotation};

let speak_config = SpeakConfig::new(Rotation::Minutely, "./logs".to_string(), "my_log".to_string())
    .with_cleanup(24 * 60 * 60, 5);

SpeakEasy::init(Level::INFO, Some(config));

```

Please replace "/path/to/log/files" with the actual path where you want to store your log files.

## License
This project is licensed under the MIT License - see the LICENSE file for details.



