use std::env;
use std::fs;

fn main(){
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        print_help();
        return;
    }

    let algorithm = &args[1];
    let queue_size = &args[2];
    let input_file = &args[3];

    let data_string = fs::read_to_string(input_file)
        .expect("Failed to read specified file");

    let data_vec: Vec<String> = data_string.lines().collect();
    println!("{} {} {}", algorithm, queue_size, input_file);
}

fn print_help(){
    println!("Usage:");
    println!("  seeker [algorithm] [queue size] [input file]");
    println!("    where:");
    println!("      algorithm in (cscan, sstf, fifo)");
    println!("      queue size is integer");
    println!("      input file is a file");
}
