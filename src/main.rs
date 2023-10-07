use crate::simulator::interval_between_requests;
use std::{thread, time};

mod cli;
mod simulator;
mod tui;
mod load_requests;
mod types;

extern crate ncurses;
use ncurses::*;
use load_requests::create_request_queue;
use types::Thread;
use crate::types::Request;

fn main() {
    let cmd = cli::cli();

    let thread_count = cmd.get_one::<u32>("thread_count").unwrap();
    let visualize = *cmd.get_one::<bool>("visualize").unwrap();
    let step = cmd.get_one::<u64>("step").unwrap();

    let mut requests = vec![];
    if let Some(subcommand) = cmd.subcommand_matches("generate-requests") {
        let request_count = subcommand.get_one::<u32>("request_count").unwrap();
        let request_rate = subcommand.get_one::<u32>("request_rate").unwrap();
        let request_duration = subcommand.get_one::<u32>("request_duration").unwrap();

        let interval = interval_between_requests(*request_rate);
        requests = create_request_queue(*request_count, *request_duration, interval);
    } else if let Some(subcommand) = cmd.subcommand_matches("load-requests") {
        let from_file = subcommand.get_one::<String>("from_file").unwrap();
        requests = load_requests::load_requests(from_file);
    }

    if visualize {
        run_with_visualization(*thread_count, *step, &requests);
    } else {
        run_with_no_visualization(*thread_count, &requests);
    }
}

fn run_with_visualization(thread_count: u32, step: u64, requests: &Vec<Request>) {
    let mut max_x = 0;
    let mut max_y = 0;

    initscr();
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    let mut last_clock: f64 = 0.0;
    if let Some(first_request) = requests.first() {
        last_clock = first_request.arrived;
    }

    simulator::run(&requests, thread_count, |clock: f64, threads: &Vec<Thread>, latency: f64| {
        while last_clock < clock {
            clear();
            last_clock += step as f64;
            thread::sleep(time::Duration::from_millis(1));

            for (idx, t) in threads.into_iter().enumerate() {
                let start = *t.start.borrow();
                let end = *t.busy_until.borrow();
                let duration = f64::max(0.0, end - start);
                let elapsed = f64::min(duration, f64::max(0.0, last_clock - start));
                let percentage = (elapsed / duration * 100.0) as i32;

                mv(idx as i32, 0);
                if percentage < 100 {
                    tui::draw_progress_bar(percentage, 30, &t.task_name.borrow());
                } else {
                    tui::draw_progress_bar(0, 30, "");
                }
            }

            mv(max_y - 1, 0);
            addstr(format!("Latency: {:.3}s", latency as f32 / 1000.0).as_str());

            refresh();
        }
    });

    getch();
    endwin();
}

fn run_with_no_visualization(thread_count: u32, requests: &Vec<Request>) {
    let stats = simulator::run(&requests, thread_count, |_, _, _| {});

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
