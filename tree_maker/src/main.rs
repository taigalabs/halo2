mod dynamodb;
mod rds;

use chrono::prelude::*;
use hyper::{body::HttpBody as _, Client, Uri};
use hyper::{Body, Method, Request, Response};
use hyper_tls::HttpsConnector;
use log::LevelFilter;
use std::fs::{File, OpenOptions};
use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

pub type TreeMakerError = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), TreeMakerError> {
    println!("Hello, world!");

    dynamodb::retrieve_addresses_in_range().await?;

    // rds::make_tree().await?;

    Ok(())
}