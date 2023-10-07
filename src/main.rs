use crate::simulator::{create_request_queue, interval_between_requests, Request, Thread};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::{thread, time};

mod cli;
mod simulator;

extern crate ncurses;
use ncurses::*;

fn draw_progress_bar(percentage: i32, length: i32, task_name: &str) {
    let filled_amount = (percentage as f32 / 100.0 * length as f32) as i32;
    addstr("[");
    for i in 0..length {
        if i < filled_amount {
            addstr("=");
        } else {
            addstr(" ");
        }
    }
    addstr("]");
    addstr(format!(" {:3}% {}", percentage, task_name).as_str());
}

fn main() {
    let cmd = cli::cli();

    let thread_count = cmd.get_one::<u32>("thread_count").unwrap();
    let visualize = *cmd.get_one::<bool>("visualize").unwrap();
    let delay = cmd.get_one::<u64>("delay").unwrap();

    let mut requests = vec![];
    if let Some(subcommand) = cmd.subcommand_matches("generate-requests") {
        let request_count = subcommand.get_one::<u32>("request_count").unwrap();
        let request_rate = subcommand.get_one::<u32>("request_rate").unwrap();
        let request_duration = subcommand.get_one::<u32>("request_duration").unwrap();

        let interval = interval_between_requests(*request_rate);
        requests = create_request_queue(*request_count, *request_duration, interval);
    } else if let Some(subcommand) = cmd.subcommand_matches("load-requests") {
        let from_file = subcommand.get_one::<String>("from_file").unwrap();
        requests = load_requests(from_file);
    }

    let mut max_x = 0;
    let mut max_y = 0;
    if visualize {
        initscr();
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
    }

    let mut last_clock: f64 = 0.0;
    if let Some(first_request) = requests.first() {
        last_clock = first_request.arrived;
    }

    let mut update_closure: Box<dyn FnMut(f64, &Vec<Thread>) -> ()> =
        Box::new(|clock: f64, threads: &Vec<Thread>| {
            clear();
            while last_clock < clock {
                last_clock += 100.0;
                thread::sleep(time::Duration::from_millis(*delay));

                for (idx, t) in threads.into_iter().enumerate() {
                    let start = *t.start.borrow();
                    let end = *t.busy_until.borrow();
                    let duration = f64::max(0.0, end - start);
                    let elapsed = f64::min(duration, f64::max(0.0, last_clock - start));
                    let percentage = (elapsed / duration * 100.0) as i32;

                    mv(idx as i32, 0);
                    if percentage < 100 {
                        draw_progress_bar(percentage, 30, &t.task_name.borrow());
                    } else {
                        draw_progress_bar(0, 30, "");
                    }
                }
                refresh();
            }
        });

    if !visualize {
        update_closure = Box::new(|_, _| {});
    }

    let stats = simulator::run(&requests, *thread_count, update_closure);

    if visualize {
        getch();
        endwin();
    } else {
        println!(
            "Time to send requests: {:.3}s",
            stats.time_to_send as f32 / 1000.0
        );
        println!(
            "Time to process requests: {:.3}s",
            stats.time_to_process as f32 / 1000.0
        );
        println!("Max latency: {:.3}s", stats.max_latency as f32 / 1000.0);
    }
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
