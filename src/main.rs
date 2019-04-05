mod barrier;

use barrier::*;
use tokio_zookeeper::*;
use std::net::SocketAddr;

fn main() {
    let addr: SocketAddr = "127.0.0.1:2181".parse().unwrap();
    let barrier = Barrier::new("/bar", addr);
    let res = barrier.create();
    // barrier.delete();
}
