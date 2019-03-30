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
     * Create a new barrier node with the path argument, if it does not exist. Else do nothing.
     * Ensures first process to execute this creates the parent znode.
     */
    pub fn create(self) {
        ZooKeeper::connect(&self.addr)
            .and_then(move |(zk, default_watcher)| {
                zk
                    .exists(self.path)
                    .inspect(|(_, stat)| {
                        match stat {
                            Some(_) => return ,
                            None => (),
                        }
                    })
                .and_then(move |(zk, _)| {
                    zk.create(self.path, &b"Barrier Node"[..], Acl::open_unsafe(), CreateMode::Persistent)
                })
            });
    }

    /** Delete the parent znode if it exists, else do nothing.
     * Ensures first process deletes the parent znode
     */
    pub fn delete(self) {
        ZooKeeper::connect(&self.addr)
            .and_then(move |(zk, default_watcher)| {
                zk
                    .delete(self.path, None)
                    .inspect(|(_, res)| {
                        match res {
                            Err (_) => return ,
                            Ok (_) => (),
                        }
                    })
            });
    }
}

