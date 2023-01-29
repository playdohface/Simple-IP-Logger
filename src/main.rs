//! Simple IP logger
//! Gets your current public IP form api.ipify.org
//! writes it to logs/ip_logs.txt 

use anyhow::{Result, Error};
use std::fs::File;
use std::io::{Write};
use std::path::Path;
use chrono::{Utc};

fn main() -> Result<(), impl std::error::Error> {
    let now = Utc::now();
    let res = writeln!(open_file(), "{} :: {:?}" ,fetch_ip().unwrap(),now.time());
    res 
}
fn fetch_ip() -> Result<String, Error> {
    let resp = reqwest::blocking::get("https://api.ipify.org")?;
    //println!("{}", resp.text()?);
    
    Ok(resp.text()?)
}

fn open_file() -> File {
    let file = std::fs::OpenOptions::new()
    .write(true)
    .append(true)
    .create(true)
    .open(Path::new("logs/ip_logs.txt"));
    match file {
        Ok(file) => file,
        Err(_) => {
            std::fs::create_dir_all("logs").expect("could not create folder");
            File::create("logs/ip_logs.txt").expect("could not create file")
        }
    }
    
}
