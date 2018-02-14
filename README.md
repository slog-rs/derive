# Slog Derives

[![Build Status](https://travis-ci.org/slog-rs/derive.svg?branch=master)](https://travis-ci.org/slog-rs/derive)

Custom derives for use with [slog] logging.

## The `KV` Derive

Sometimes you'll want to log the struct's contents in your application, for
example when you've just started and want to record the configuration
details for debugging purposes. Usually you'd need to do something like
this:

```rust
#[macro_use]
extern crate slog;
use std::path::PathBuf;

struct Config {
    width: f64,
    height: f64,
    url: String,
}

let cfg = Config { ... };

debug!(logger, "Loaded Config";
    "width" => cfg.width,
    "height" => cfg.height,
    "url" => cfg.url);
# }
```

This is where the [`KV`] trait comes in. Implementing it lets you log a type
as a bunch of key-value pairs, translating the previous log statement into 
something like this:

```rust
debug!(logger, "Loaded Config"; cfg);
```

This crate provides a custom derive which will implement [`KV`] for you.
It'll just iterate over each field in your `struct` and invoke
[`Value::serialize()`] on each. You can also use the `#[slog(skip)]` attribute
to skip specific fields.

```rust
#[derive(KV)]
pub struct Config {
  width: f64,
  height: f64,
  #[slog(skip)]
  url: String,
}
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.


[`KV`]: https://docs.rs/slog/2.1.1/slog/trait.KV.html
[`Value::serialize()`]: https://docs.rs/slog/2.1.1/slog/trait.Value.html#tymethod.serialize
[slog]: https://github.com/slog-rs/slog