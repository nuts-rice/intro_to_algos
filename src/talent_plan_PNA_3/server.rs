use crate::talent_plan_PNA_3::common::{GetResponse, RemoveResponse, Request, SetResponse};
use crate::{KVEngine, Result};
use log::{debug, error};
use serde_json::Deserializer;
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

pub struct KVServer<E: KVEngine> {
    engine: E,
}

impl<E: KVEngine> KVServer<E> {
    pub fn new(engine: E) -> Self {
        KVServer { engine }
    }

    pub fn run<A: ToSocketAddrs>(mut self, addr: A) -> Result<()> {
        let listener = TcpListener::bind(addr)?;
        for stream in listener.incoming() {
            match stream {
                Ok(Stream) => {
                    if let Err(e) = self.serve(stream) {
                        error!("Error on serving client: {}", e);
                    }
                }
                Err(e) => error!("Connection failed: {}", e),
            }
        }
        Ok(())
    }

    fn serve(&mut self, tcp: TcpStream) -> Result<()> {
        let peer_addr = tcp.peer_addr()?;
        let reader = BufReader::new(&tcp);
        let mut writer = BufWriter::new(&tcp);
        let request_reader = Deserializer::from_reader(reader).into_iter::<Request>();
        Ok(())
    }
}
