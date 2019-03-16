mod barrier;

use barrier::*;
use tokio_zookeeper::*;

fn main() {
    let barrier = Barrier::new("barrier");
}
