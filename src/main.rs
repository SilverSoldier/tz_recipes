mod barrier;

use barrier::*;
use tokio_zookeeper::*;
use std::net::SocketAddr;

fn main() {
    let addr: SocketAddr = "127.0.0.1:2181".parse().unwrap();
    let barrier = Barrier::new("/bar", addr);
    let res = barrier.create();
    match res {
        Ok(res) => {
            match res {
                Ok(s) => println!("{}", s),
                Err(err) => println!("{:?}", err)
            }
        },
        Err(err) => println!("{:?}", err)
    }
    // barrier.delete();
}
