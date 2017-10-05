extern crate curl;
extern crate tokio_core;
extern crate tokio_curl;
extern crate fibers;
extern crate futures;
extern crate futures_cpupool;

use std::io::{self, Write, BufWriter};
use curl::easy::Easy;
use futures::future::*;
use std::fs::File;
use futures_cpupool::CpuPool;
use std::sync::{Mutex, Arc};
use futures::{Future, Stream, Async};
use futures::stream::futures_unordered;
use tokio_core::reactor::Core;
use tokio_curl::{Session, Perform};


fn make_file(x: i32, data: &mut Vec<u8>) -> usize {
    let f = File::create(format!("./data/{}.txt", x)).expect("Unable to open file");
    let mut writer = BufWriter::new(&f);
    writer.write_all(data.as_mut_slice()).unwrap();
    data.len()
}

fn collect_request(x: i32, url: &str, sess: &Session) -> FutureResult<Perform, ()> {
    let mut data = Vec::new();
    let mut easy = Easy::new();
    easy.get(true).unwrap();
    easy.url("https://www.rust-lang.org").unwrap();
    easy.write_function(|data| Ok(data.len())).unwrap();
    make_file(x, &mut data);
    ok(sess.perform(easy))
}

fn main() {
    let url = "https://en.wikipedia.org/wiki/Immanuel_Kant";
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let pool = CpuPool::new_num_cpus();
    let session = Session::new(handle);

    let requests = (0..20).into_iter().map(|x| {
        pool.spawn(collect_request(x, url, &session))
    });

    let performed = futures_unordered(requests).into_future();
}
// let out = requests.into_stream().wait();
