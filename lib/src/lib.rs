///! # mailjet-rs
///! 
///! 
extern crate hyper;
use serde_json;

mod api;
mod client;

pub use api::common;
pub use api::v3;
pub use client::*;
pub use serde_json::{
  Map,
  Value
};
