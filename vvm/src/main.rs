#![warn(clippy::pedantic)]

use ::bounded_vec_deque::BoundedVecDeque;
use clap::Parser;
use std::error::Error;
use std::fs;

#[derive(Parser)]
#[clap(
    version = "0.0.1",
    author = "blake bartenbach <bbartenbach@unomaha.edu>"
)]
struct Opts {
    input_file: String,
    pid1: usize,
    pid2: usize,
    pid3: usize,
    pid4: usize,
}

struct ProcessData {
    queue: BoundedVecDeque<u32>,
    hit: f64,
    miss: f64,
    rate: f64,
}

fn main() {
    let opts: Opts = Opts::parse();
    let data: Result<String, Box<dyn Error>> = parse_file(opts.input_file);
    let values = format!(":{} {} {} {}", opts.pid1, opts.pid2, opts.pid3, opts.pid4);
    let mut p1 = ProcessData {
        queue: BoundedVecDeque::new(opts.pid1),
        hit: 0.0,
        miss: 0.0,
        rate: 0.0,
    };
    let mut p2 = ProcessData {
        queue: BoundedVecDeque::new(opts.pid2),
        ..p1
    };
    let mut p3 = ProcessData {
        queue: BoundedVecDeque::new(opts.pid3),
        ..p1
    };
    let mut p4 = ProcessData {
        queue: BoundedVecDeque::new(opts.pid4),
        ..p1
    };
    for r in data.ok().unwrap().lines().collect::<Vec<&str>>() {
        let mut split = r.split_whitespace();
        match split.next() {
            Some(v) => match v {
                "1" => lru(&mut p1, split.next()),
                "2" => lru(&mut p2, split.next()),
                "3" => lru(&mut p3, split.next()),
                "4" => lru(&mut p4, split.next()),
                &_ => continue,
            },
            None => {}
        }
    }
    generate_reports(&mut Vec::from([p1, p2, p3, p4]));
    println!(" args: {}", values);
}

fn lru(p: &mut ProcessData, req: Option<&str>) {
    let page: u32 = req.unwrap().parse::<u32>().unwrap();
    for (i, val) in p.queue.iter().enumerate() {
        if val == &page {
            p.hit += 1.0;
            p.queue.remove(i);
            p.queue.push_back(page);
            return;
        }
    }
    p.miss += 1.0;
    if p.queue.is_full() {
        p.queue.pop_front();
    }
    p.queue.push_back(page);
}

fn generate_reports(data: &mut Vec<ProcessData>) {
    let mut total = 0.0;
    for p in data {
        p.rate = p.hit / (p.miss + p.hit);
        total += p.rate * 100.0;
        //print!("{} ", p.rate * 100.0);
    }
    print!("{}", total / 4.0);
}

fn parse_file(input_file: String) -> Result<String, Box<dyn Error>> {
    let data: String = fs::read_to_string(input_file)?.parse()?;
    Ok(data)
}
