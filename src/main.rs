//! Simple API logger
//! Logs the response body of a GET-request to file with timestamp

use anyhow::{Result, Error};
use std::fs::File;
use std::io::{Write};
use std::path::Path;
use chrono::{Utc};
use clap::Parser;

#[derive(Parser)]
#[command(name = "Simple API logger")]
#[command(author = "Kalle Enkelmann <mail@karlerikenkelmann.com>")]
#[command(version = "1.0")]
#[command(about = "Logs the response body of an arbitrary GET-request to file and timestamps it", long_about = None)]
struct Cli {
    /// set the output path, defaults to "./log.txt"
    #[arg(short, long)]
    target: Option<String>,

    /// set the URL to query, defaults to https://api.ipify.org
    #[arg(short, long)]
    source: Option<String>,

    /// set this flag to log without timestamps
    #[arg(short, long)]
    notimestamp: bool,
}

fn main() -> Result<(), impl std::error::Error> {
    let cli = Cli::parse();
    let path = cli.target.unwrap_or_else(|| "./log.txt".to_string());
    let source = cli.source.unwrap_or_else(|| "https://api.ipify.org".to_string());
    let ts_flag = cli.notimestamp;
    
    let now = Utc::now();
    if !ts_flag {
        writeln!(open_file(path), 
        "{} :: {:?}   {:?}", 
        fetch(source).unwrap_or_else(|_| "ERROR".to_string()), 
        now.time(), now.date_naive())
    } else {
        writeln!(open_file(path), "{}", fetch(source).unwrap_or_else(|_| "ERROR".to_string()))
    }
}
fn fetch(url: String) -> Result<String, Error> {
    let resp = reqwest::blocking::get(url)?;
    //println!("{}", resp.text()?);
    
    Ok(resp.text()?)
}

fn open_file(path: String) -> File {
    let file = std::fs::OpenOptions::new()
    .write(true)
    .append(true)
    .create(true)
    .open(Path::new(&path));
    match file {
        Ok(file) => file,
        Err(_) => {
            let path = Path::new(&path);
            let dir = path.parent().unwrap();
            
            std::fs::create_dir_all(dir).expect("could not create folder");
            File::create(path).expect("could not create file")
        }
    }
    
}
