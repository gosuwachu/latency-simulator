use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use crate::simulator::{create_request_queue, interval_between_requests, Request};

mod cli;
mod simulator;

fn main() {
    let cmd = cli::cli();

    let thread_count = cmd.get_one::<u32>("thread_count").unwrap();

    let mut requests = vec![];
    if let Some(subcommand) = cmd.subcommand_matches("generate-requests") {
        let request_count = subcommand.get_one::<u32>("request_count").unwrap();
        let request_rate = subcommand.get_one::<u32>("request_rate").unwrap();
        let request_duration = subcommand.get_one::<u32>("request_duration").unwrap();

        let interval = interval_between_requests(*request_rate);
        requests = create_request_queue(
            *request_count,
            *request_duration,
            interval);
    } else if let Some(subcommand) = cmd.subcommand_matches("load-requests") {
        let from_file = subcommand.get_one::<String>("from_file").unwrap();
        requests = load_requests(from_file);
    }

    let stats = simulator::run(&requests, *thread_count);

    println!("Time to send requests: {:.3}s", stats.time_to_send as f32 / 1000.0);
    println!("Time to process requests: {:.3}s", stats.time_to_process as f32 / 1000.0);
    println!("Max latency: {:.3}s", stats.max_latency as f32 / 1000.0);
}

fn load_requests(file_path: &str) -> Vec<Request> {
    let mut requests: Vec<Request> = vec![];
    let result = read_lines(Path::new(file_path));
    match result {
        Ok(lines) => {
            for line in lines {
                if let Ok(line) = line {
                    let tokens: Vec<&str> = line.split(',').collect();
                    if tokens.len() == 3 {
                        let timestamp = tokens[0].parse::<f64>();
                        let duration = tokens[1].parse::<f64>();
                        let name = tokens[2];
                        if let (Ok(timestamp), Ok(duration)) = (timestamp, duration) {
                            requests.push(
                                Request::new(duration, timestamp, name.to_string())
                            );
                        } else {
                            eprintln!("WARN: Invalid request format: {line}")
                        }
                    } else {
                        eprintln!("WARN: Invalid request format: {line}")
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("ERROR: Failed to load: {} - {}", file_path, e.to_string());
            std::process::exit(1);
        }
    }
    println!("Loaded {} requests.", requests.len());
    requests
}

fn read_lines(filename: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
