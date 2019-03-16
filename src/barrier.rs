//! Module with struct for barrier
//!
extern crate tokio_zookeeper;
extern crate failure;

use tokio_zookeeper::*;
use tokio::prelude::*;
use std::net::SocketAddr;

pub struct Barrier {
    zookeeper: Option<ZooKeeper>,
    path: &'static str
}

impl Barrier {
    pub fn new(path: &'static str) -> Self {
        Barrier {zookeeper: None, path: path}
    }

    pub fn create(
        mut self,
        addr: &SocketAddr,
        path: &'static str,
        ) -> Result<(Self, Result<String, error::Create>), failure::Error> 
    {
        self.path = path;
        ZooKeeper::connect(addr)
            .and_then(move |(zk, default_watcher)| {
                zk
                    .exists(path)
                    .inspect(|(_, stat)| {
                        match stat {
                            Some(_) => return (),
                            None => (),
                        }
                    })
                .and_then(move |(zk, _)| {
                    zk.create(&path, &b"Barrier Node"[..], Acl::open_unsafe(), CreateMode::Persistent)
                })
            }).wait().map(|(zookeeper, res)| {
                self.zookeeper = Some(zookeeper);
                (self, res)
            })
    }

    pub fn delete(
        self,
        ) ->  Result<Result<(), error::Delete>, failure::Error> {
        self.zookeeper
            .unwrap()
            .delete(self.path, None)
            .inspect(|(_, res)| {
                match res {
                    Err (_) => return (),
                    Ok (_) => (),
                }
            })
        .wait().map(|(_, res)| {
            res
        })
    }
}
