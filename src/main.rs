#![feature(inner_deref)]
#![allow(dead_code)]
#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

use anyhow::Error;
use clap::App;

use crate::quoth::Quoth;

mod config;
mod errors;
mod quoth;
mod utils;

fn main() -> Result<(), Error> {
    //    utils::read_quotes_database(
    //        "/Users/janani/Downloads/quotesdrivedb.csv",
    //        "quotes_full_database.json",
    //    )?;
    let yaml = load_yaml!("quoth.yml");
    let matches = App::from_yaml(yaml).get_matches();
    Quoth::start(matches)?;
    Ok(())
}
