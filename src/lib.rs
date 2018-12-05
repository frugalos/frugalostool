//! frugalostool
#![warn(missing_docs)]
extern crate cannyls;
extern crate cannyls_rpc;
extern crate fibers;
extern crate fibers_global;
extern crate fibers_rpc;
extern crate futures;
extern crate libfrugalos;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate slog;
extern crate sloggers;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate trackable;

pub mod command;
pub mod error;

/// The result type in this create.
pub type Result<T> = std::result::Result<T, error::Error>;
