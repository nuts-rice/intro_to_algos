use anyhow::Result;
use diamond_types::list::*;
use hyper::{Body, Error, Response};

use std::fs::File;
use std::io::BufReader;

use tokio::*;

async fn send_file(filename: &str) -> Result<Response<Body>, hyper::Error> {
    unimplemented!()
}

async fn edit_file(filename: &str) -> Result<Response<Body>, hyper::Error> {
    unimplemented!()
}
