//! Module with struct for barrier
//!
extern crate tokio_zookeeper;
extern crate failure;

use tokio_zookeeper::*;
use tokio::prelude::*;
use std::net::SocketAddr;

#[derive(Copy, Clone)]
pub struct Barrier {
    addr: SocketAddr,
    path: &'static str
}

impl Barrier {
    pub fn new(path: &'static str, addr: SocketAddr) -> Self {
        Barrier {addr: addr, path: path}
    }

    /**
     * Create a new barrier node with the path argument, if it does not exist. Else return file
     * exists error.
     * Assuming a main process will call this for setup before barrier usage.
     */
    pub fn create(self) -> Result<String, error::Create> {
        let mut create_error: Option<error::Create> = None;
        tokio::run(
            ZooKeeper::connect(&self.addr)
            .and_then(move |(zk, _default_watcher)| {
                zk.create(self.path, &b"Barrier Node"[..], Acl::open_unsafe(), CreateMode::Persistent)
            })
            .inspect(move |(_, stat)| {
                match stat {
                    Ok(_) => (),
                    Err(err) => { 
                        create_error = Some(*err);
                    }
                }
            })
            .map(|_| ())
            .map_err(|e| panic!("{:?}", e))
        );
        match create_error {
            Some(err) => Err(err),
            None => Ok(String::from("Barrier node created/already exists"))
        }
    }

    /** Delete barrier node with the path argument, if it exists. Else return file does not
     * exist error.
     * Assuming a main process will call this for tear down after barrier usage.
     */
    pub fn delete(self) -> Result<Result<(), error::Delete>, failure::Error> {
        ZooKeeper::connect(&self.addr)
            .and_then(move |(zk, _default_watcher)| {
                zk
                    .delete(self.path, None)
                    .inspect(|(_, res)| {
                        match res {
                            Err (_) => return ,
                            Ok (_) => (),
                        }
                    })
            }).wait().map(|(_, res)| { res })
    }
    }

