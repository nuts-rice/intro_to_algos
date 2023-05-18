use anyhow::Result;
use diamond_types::list::*;
use hyper::{Body, Error, Response};
use webrtc::peer_connection::RTCPeerConnection;

use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use tokio::sync::Mutex;
use tokio::*;

type LaTeXFile = Box<File>;

async fn send_file(filename: &str) -> Result<Response<Body>, hyper::Error> {
    unimplemented!()
}

async fn edit_file(filename: &str) -> Result<Response<Body>, hyper::Error> {
    unimplemented!()
}

async fn parse_and_check(filename: &str) -> Result<LaTeXFile, hyper::Error> {
    unimplemented!()
}
