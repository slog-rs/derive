//! Custom derives for working with the [`slog`] crate.
//!
//! # The `KV` Derive
//!
//! Say you've got a struct like this,
//!
//! ```rust
//! # use std::path::PathBuf;
//! pub struct Config {
//!   width: f64,
//!   height: f64,
//!   output_file: PathBuf,
//! }
//! ```
//!
//! Sometimes you'll want to log the struct's contents in your application, for
//! example when you've just started and want to record the configuration
//! details for debugging purposes. Usually you'd need to do something like
//! this:
//!
//! ```rust
//! # #[macro_use]
//! # extern crate slog;
//! # use std::path::PathBuf;
//! # fn main() {
//! # let logger = slog::Logger::root(slog::Discard, o!());
//! # struct Config { width: f64, height: f64, output_file: PathBuf }
//! # let cfg = Config { width: 1.0, height: 0.0, output_file: "Foo".into() };
//! debug!(logger, "Loaded Config";
//!     "width" => cfg.width,
//!     "height" => cfg.height,
//!     "output-file" => cfg.output_file.display());
//! # }
//! ```
//!
//! This is where the [`KV`] trait comes in. Implementing it lets you translate
//! the previous log statement into something like this:
//!
//! ```rust
//! # #[macro_use]
//! # extern crate slog;
//! # fn main() {
//! # let logger = slog::Logger::root(slog::Discard, o!());
//! # let cfg = o!();
//! debug!(logger, "Loaded Config"; cfg);
//! # }
//! ```
//!
//! This crate provides a custom derive which will implement [`KV`] for you.
//! It'll just iterate over each field in your `struct` and invoke
//! [`Value::serialize()`] on each.
//!
//! ```rust
//! # #[macro_use]
//! # extern crate slog_derive;
//! # extern crate slog;
//! #[derive(KV)]
//! pub struct Config {
//!   width: f64,
//!   height: f64,
//!   output_file: String,
//! }
//! # fn main() {}
//! ```
//!
//! You can also skip fields using the `#[skip]` attribute, this is useful when
//! you don't want to log complex data structures or the particular field
//! doesn't implement `Value`.
//!
//! ```rust
//! # #[macro_use]
//! # extern crate slog_derive;
//! # extern crate slog;
//! # use std::path::PathBuf;
//! #[derive(KV)]
//! pub struct Config {
//!   width: f64,
//!   height: f64,
//!   #[skip]
//!   output_file: PathBuf,
//! }
//! # fn main() {}
//! ```
//!
//! [`slog`]: https://crates.io/crates/slog
//! [`KV`]: https://docs.rs/slog/2.1.1/slog/trait.KV.html
//! [`Value::serialize()`]: https://docs.rs/slog/2.1.1/slog/trait.Value.html#tymethod.serialize

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

mod derive_kv;
mod utils;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(KV, attributes(skip))]
pub fn derive_kv(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let gen = derive_kv::impl_kv(ast);
    gen.to_string().parse().unwrap()
}
