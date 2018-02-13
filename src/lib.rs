extern crate prost;
#[macro_use]
extern crate prost_derive;
extern crate clap;
extern crate url;
extern crate uuid;

include!(concat!(env!("OUT_DIR"), "/mesos.rs"));

pub mod process;
