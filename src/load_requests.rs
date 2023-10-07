use std::path::Path;
use std::io;
use std::fs::File;
use std::io::BufRead;
use crate::types::Request;

pub fn load_requests(file_path: &str) -> Vec<Request> {
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
                            requests.push(Request::new(duration, timestamp, name.to_string()));
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

pub fn create_request_queue(
    request_count: u32,
    request_duration: u32,
    interval_between_requests: f64,
) -> Vec<Request> {
    let mut request_queue = Vec::<Request>::new();

    let mut clock = 0.0;
    for i in 0..request_count {
        request_queue.push(Request::new(
            request_duration as f64,
            clock,
            format!("#req{i}"),
        ));
        clock += interval_between_requests;
    }

    request_queue
}
