#![warn(clippy::pedantic)]
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
mod driver;
use driver::Driver;
mod cscan;
use cscan::Cscan;
mod fifo;
use fifo::Fifo;

const ALGORITHMS: [&str; 3] = ["cscan", "sstf", "fifo"]; // known algorithms
                                                         //const DISK_SIZE: u32 = 1024; // number of cylinders
const LATENCY_MS: f32 = 4.2; // times in milliseconds
const SEEK_MS: f32 = 0.15; // time to seek from one cylinder to the next
const START_MS: f32 = 1.0; // time to start the platter
const STOP_MS: f32 = 1.0; // time to stop the platter

struct Options {
    algorithm: String,
    queue_size: u32,
    input_file: PathBuf,
}

fn calc_seek_time(distance: u32) -> f32 {
    let total_seek_ms = START_MS + (distance as f32 * SEEK_MS) + STOP_MS;
    return total_seek_ms + LATENCY_MS;
}

fn parse_args(args: &[String]) -> Options {
    if args.len() == 1 {
        print_help();
        process::exit(1);
    }
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

fn main() {
    let options = parse_args(&env::args().collect::<Vec<_>>());
    let data_string =
        fs::read_to_string(options.input_file).expect("Failed to read specified file");
    let data: Vec<u32> = data_string
        .trim()
        .split('\n')
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect();

    let driver: Box<dyn driver::Driver> = get_driver(&options.algorithm);
    let average_time = driver.process_queue(data, options.queue_size);
    println!(
        "Average seek time for {} with queue size {} was: {} ms",
        options.algorithm, options.queue_size, average_time
    );
}

fn get_driver(algorithm: &String) -> Box<dyn Driver> {
    match algorithm.as_str() {
        "cscan" => Box::new(Cscan {}),
        "fifo" => Box::new(Fifo {}),
        //"sstf" => Box::new(Sstf {}),
        _ => unreachable!(),
    }
}

fn print_help() {
    println!("Usage:");
    println!("    where:");
    println!("      algorithm in (cscan, sstf, fifo)");
    println!("      queue size is integer");
    println!("      input file is a file");
}
