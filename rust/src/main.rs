extern crate curl;
extern crate futures;
extern crate tokio_core;
extern crate tokio_curl;

use tokio_core::reactor::Core;
use futures::{Future, Stream};
use futures::sync::mpsc;
use tokio_curl::Session;
use curl::easy::Easy;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::mem;

fn make_file(x: i32, data: &[u8]) -> usize {
    let f = File::create(format!("./data/{}.txt", x)).expect("Unable to open file");
    let mut writer = BufWriter::new(&f);
    writer.write_all(data).unwrap();
    data.len()
}

fn collect_request(
    x: i32,
    url: &str,
    sess: &Session,
) -> Box<Future<Item = i32, Error = tokio_curl::PerformError>> {
    let buf = Arc::new(Mutex::new(Vec::new()));
    let mut easy = Easy::new();
    easy.get(true).unwrap();
    easy.url(url).unwrap();
    {
        let buf = Arc::clone(&buf);
        easy.write_function(move |data| {
            let mut buf = buf.lock().unwrap();
            buf.extend(data);
            Ok(data.len())
        }).unwrap();
    }

    Box::new(sess.perform(easy).and_then(move |_| {
        let buf = buf.lock().unwrap();
        make_file(x, buf.as_slice());
        Ok(x)
    }))
}

fn main() {
    let url = "https://en.wikipedia.org/wiki/Immanuel_Kant";

    let (tx, rx) = mpsc::channel(800);

    let threads = (0..4)
        .map(|n| {
            let mut tx = tx.clone();
            thread::spawn(move || {
                let mut core = Core::new().unwrap();

                let session = Session::new(core.handle());
                let reqs = futures::stream::futures_unordered((n * 200..n * 200 + 200).map(|x| {
                    collect_request(x, url, &session)
                }));

                core.run(reqs.for_each(
                    move |x| tx.try_send(x).map_err(|e| panic!("{:?}", e)),
                )).unwrap();
            })
        })
        .collect::<Vec<_>>();

    // drop an excess Sender so that `rx` don't wait forever
    mem::drop(tx);

    rx.for_each(|x| {
        println!("Done {}", x);
        Ok(())
    }).wait()
        .unwrap();

    for t in threads {
        t.join().unwrap();
    }
}