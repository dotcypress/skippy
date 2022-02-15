use std::io::prelude::*;
use std::{io::BufReader, net::TcpStream};

use crate::SkippyResult;
use slab::Slab;
use url::Url;

pub enum Adapter {
    Tcp(TcpPhy),
}

impl Adapter {
    pub fn scpi(&mut self, scpi: &str) -> SkippyResult<String> {
        let cmd = format!("{}\n", scpi);
        match self {
            Adapter::Tcp(phy) => phy.send(cmd.as_str()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Device(usize);

pub struct Env {
    phy: Slab<Adapter>,
}

impl Env {
    pub fn new() -> Self {
        Self { phy: Slab::new() }
    }

    pub fn connect(&mut self, addr: &str) -> SkippyResult<Device> {
        let dev = match Url::parse(addr) {
            Ok(url) => match url.scheme() {
                "tcp" => Adapter::Tcp(TcpPhy::connect(url)?),
                _ => return Err("Unsupported protocol".into()),
            },
            Err(_) => return Err("Unsupported address".into()),
        };

        Ok(Device(self.phy.insert(dev)))
    }

    pub fn disconnect(&mut self, dev: Device) -> SkippyResult<()> {
        self.phy.try_remove(dev.0);
        Ok(())
    }

    pub fn scpi(&mut self, dev: Device, raw: &str) -> SkippyResult<String> {
        match self.phy.get_mut(dev.0) {
            Some(dev) => dev.scpi(raw),
            None => Err("Device disconnected".into()),
        }
    }
}

pub struct TcpPhy {
    stream: TcpStream,
}

impl TcpPhy {
    pub fn connect(url: Url) -> SkippyResult<Self> {
        let (_, addr) = url.as_str().split_at(6);
        match TcpStream::connect(addr) {
            Ok(mut stream) => {
                //Read banner
                let _ = stream.read(&mut [0; 256]).unwrap();
                Ok(Self { stream })
            }
            Err(_) => Err("Connect failed".into()),
        }
    }

    fn send(&mut self, raw: &str) -> SkippyResult<String> {
        self.stream.write_all(raw.as_bytes()).unwrap();
        self.stream.flush().unwrap();

        let mut reader = BufReader::new(&self.stream);
        let mut res = String::new();
        reader.read_line(&mut res).unwrap();
        Ok(res.trim().into())
    }
}
