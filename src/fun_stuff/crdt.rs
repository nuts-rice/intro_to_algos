use anyhow::Result;
use diamond_types::list::*;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Response, StatusCode};
use webrtc::peer_connection::RTCPeerConnection;

use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use tokio::sync::Mutex;
use tokio::*;
use tokio_util::codec::{BytesCodec, FramedRead};

type LaTeXFile = Box<File>;

static NOT_FOUND: &[u8] = b"Not Found";
fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(NOT_FOUND.into())
        .unwrap()
}

//TODO: maybe impl Tryfrom for this?
fn body_from_file(file: tokio::fs::File) -> Body {
    let stream = FramedRead::new(file, BytesCodec::new());
    Body::wrap_stream(stream)
}

async fn open_file(filename: &str) -> tokio::io::Result<tokio::fs::File> {
    tokio::fs::File::open(filename).await
}

async fn send_file(filename: &str) -> Result<Response<Body>, hyper::Error> {
    match open_file(filename).await {
        Ok(file) => {
            let body = body_from_file(file);
            Ok(Response::new(body))
        }
        Err(_) => Ok(not_found()),
    }
}

async fn edit_file(filename: &str) -> Result<Response<Body>, hyper::Error> {
    unimplemented!()
}

async fn parse_and_check(filename: &str) -> Result<LaTeXFile, hyper::Error> {
    unimplemented!()
}
