extern crate curl;
extern crate fibers;
extern crate futures;
extern crate futures_cpupool;

use std::io::{Write, BufWriter};
use curl::easy::Easy;
use futures::future::*;
use std::fs::File;
use futures_cpupool::CpuPool;


fn make_file(x: i32, data: &mut Vec<u8>) {
    let f = File::create(format!("./data/{}.txt", x)).expect("Unable to open file");
    let mut writer = BufWriter::new(&f);
    writer.write_all(data.as_mut_slice()).unwrap();
}

fn collect_request(x: i32, url: &str) -> Result<i32, ()> {
    let mut data = Vec::new();
    let mut easy = Easy::new();
    easy.url(url).unwrap();
    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|d| {
                data.extend_from_slice(d);
                Ok(d.len())
            })
            .unwrap();
        transfer.perform().unwrap();

    }
    make_file(x, &mut data);
    Ok(x)
}

fn main() {
    let url = "https://en.wikipedia.org/wiki/Immanuel_Kant";
    let pool = CpuPool::new(16);
    let output_futures: Vec<_> = (0..100)
        .into_iter()
        .map(|ind| {
            pool.spawn_fn(move || {
                let output = collect_request(ind, url);
                output
            })
        })
        .collect();

    // println!("{:?}", output_futures.Item());
    for i in output_futures {
        i.wait().unwrap();
    }
}