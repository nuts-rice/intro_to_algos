use anyhow::Result;
use diamond_types::list::*;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Method, Request, Response, StatusCode};
use webrtc::peer_connection::RTCPeerConnection;

use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::{todo, unimplemented};

use tokio::sync::Mutex;
use tokio::*;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::PEER_CONNECTION_MUTEX;

type LaTeXFile = Box<File>;

static NOT_FOUND: &[u8] = b"Not Found";
static INDEX: &str = "index.html";
fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(NOT_FOUND.into())
        .unwrap()
}

//TODO: maybe impl Tryfrom for this?
async fn open_file(filename: &str) -> tokio::io::Result<tokio::fs::File> {
    tokio::fs::File::open(filename).await
}

async fn peer_conn_handler() -> Arc<webrtc::peer_connection::RTCPeerConnection> {
    let pc = {
        let pcm = PEER_CONNECTION_MUTEX.lock().await;
        pcm.clone().unwrap()
    };
    pc
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

async fn remote_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let pc = peer_conn_handler();
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => send_file(INDEX).await,
        (&Method::POST, "/createPeerConnection") => todo!(),
        (&Method::POST, "/addNotes") => add_notes().await,
        (&Method::POST, "/removeNotes") => remove_notes().await,

        _ => {
            todo!()
        }
    }
}

async fn add_notes() -> Result<Response<Body>, hyper::Error> {
    unimplemented!()
}

async fn remove_notes() -> Result<Response<Body>, hyper::Error> {
    unimplemented!()
}

async fn edit_file(filename: &str) -> Result<Response<Body>, hyper::Error> {
    unimplemented!()
}

async fn parse_and_check(filename: &str) -> Result<LaTeXFile, hyper::Error> {
    unimplemented!()
}

fn body_from_file(file: tokio::fs::File) -> Body {
    let stream = FramedRead::new(file, BytesCodec::new());
    Body::wrap_stream(stream)
}

async fn create_peer_connection(
    pc: &Arc<RTCPeerConnection>,
    r: Request<Body>,
) -> Result<Response<Body>, hyper::Error> {
    unimplemented!()
}
