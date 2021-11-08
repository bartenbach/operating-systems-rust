#![warn(clippy::pedantic)]
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

const ALGORITHMS: [&str; 3] = ["cscan", "sstf", "fifo"]; // known algorithms
const DISK_SIZE: u32 = 1024; // number of cylinders
const LATENCY_MS: f32 = 4.2; // times in milliseconds
const SEEK_MS: f32 = 0.15; // time to seek from one cylinder to the next
const START_MS: f32 = 1.0; // time to start the platter
const STOP_MS: f32 = 1.0; // time to stop the platter

struct Options {
    algorithm: String,
    queue_size: u32,
    input_file: PathBuf,
}

fn calc_seek_time(distance: f32) -> f32 {
    let total_seek_ms = START_MS + (distance * SEEK_MS) + STOP_MS;
    println!("seek time was {}", total_seek_ms);
    return total_seek_ms + LATENCY_MS;
}

fn parse_args(args: &[String]) -> Options {
    let algorithm = String::from(&args[1]);
    if !ALGORITHMS.contains(&&*algorithm) {
        print_help();
        process::exit(1);
    }

    let queue_size: u32 = args[2].parse().expect("Invalid queue size");

    let input_file = PathBuf::from(&args[3]);
    if !input_file.exists() || !input_file.is_file() {
        print_help();
        panic!("Not passed a valid, readable file");
    }

    Options {
        algorithm,
        queue_size,
        input_file,
    }
}

// TODO delete me!
// cscan
// sstf
// fifo
fn main() {
    let options = parse_args(&env::args().collect::<Vec<_>>());
    let mut queue: [u16; 0];
    let data_string =
        fs::read_to_string(options.input_file)
        .expect("Failed to read specified file");
    let mut data_vec: Vec<u32> = data_string
        .split('\n')
        .map(|x| x.trim_end().parse::<u32>().unwrap())
        .collect();
    while ! data_vec.is_empty() {
        let mut chunks = data_vec.chunks(options.queue_size as usize);
        println!("{}", chunks.next().unwrap()[0]);
        process::exit(1);
    }
    //println!("{} {}", options.queue_size, options.algorithm);
    //calc_seek_time(32.2);
    //println!("{}", DISK_SIZE);
}

fn print_help() {
    println!("Usage:");
    println!("  seeker [algorithm] [queue size] [input file]");
    println!("    where:");
    println!("      algorithm in (cscan, sstf, fifo)");
    println!("      queue size is integer");
    println!("      input file is a file");
}
