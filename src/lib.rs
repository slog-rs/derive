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
//!   #[slog(skip)]
//!   output_file: PathBuf,
//! }
//! # fn main() {}
//! ```
//!
//! # The `SerdeValue` Derive
//!
//! Implementing the [`SerdeValue`] is usually trivial and tedious so it also
//! has a custom derive.
//!
//! ```rust
//! extern crate slog;
//! #[macro_use]
//! extern crate slog_derive;
//! extern crate serde;
//! extern crate erased_serde;
//!
//! use std::path::PathBuf;
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Clone, SerdeValue, Serialize, Deserialize)]
//! pub struct Config {
//!   width: f64,
//!   height: f64,
//!   output_file: PathBuf,
//! }
//! # fn main() {}
//! ```
//!
//! This will require enabling `slog`'s `nested-values` feature flag, as well
//! as implementing (or deriving) `serde::Serialize` for your type. You will
//! also need to pull in the [`erased_serde`] crate because it's part of the
//! `SerdeValue` signature.
//!
//! For convenience this will also generate a `Value` impl for your type (to
//! implement `SerdeValue` you must also implement `Value`). This impl simply
//! calls `Serializer::emit_serde()`, but if you want to write your own `Value`
//! implementation you can add the `#[slog(no_value_impl)]` attribute.
//!
//! ```rust
//! extern crate slog;
//! #[macro_use]
//! extern crate slog_derive;
//! extern crate serde;
//! extern crate erased_serde;
//!
//! use std::path::PathBuf;
//! use slog::{Key, Record, Serializer, Value};
//! use serde::Serialize;
//!
//! #[derive(Clone, SerdeValue, Serialize)]
//! #[slog(no_value_impl)]
//! pub struct Config {
//!   width: f64,
//!   height: f64,
//!   output_file: PathBuf,
//! }
//!
//! impl Value for Config {
//!     fn serialize(&self, _record: &Record, key: Key, ser: &mut Serializer) -> slog::Result {
//!         unimplemented!()
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! [`slog`]: https://crates.io/crates/slog
//! [`KV`]: https://docs.rs/slog/2.1.1/slog/trait.KV.html
//! [`Value::serialize()`]: https://docs.rs/slog/2.1.1/slog/trait.Value.html#tymethod.serialize
//! [`SerdeValue`]: https://docs.rs/slog/2.1.1/slog/trait.SerdeValue.html
//! [`erased_serde`]: https://docs.rs/erased_serde

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

mod derive_kv;
mod derive_serde_value;
mod utils;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(KV, attributes(slog))]
pub fn derive_kv(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let gen = derive_kv::impl_kv(&ast);
    gen.to_string().parse().unwrap()
}

#[proc_macro_derive(SerdeValue, attributes(slog))]
pub fn derive_serde_value(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let gen = derive_serde_value::impl_serde_value(ast);
    gen.to_string().parse().unwrap()
}
