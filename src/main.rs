mod cli;
mod latency;

fn main() {
    let cmd = cli::cli();

    let request_count = cmd.get_one::<u32>("request_count").unwrap();
    let request_rate = cmd.get_one::<u32>("request_rate").unwrap();
    let request_duration = cmd.get_one::<u32>("request_duration").unwrap();
    let thread_count = cmd.get_one::<u32>("thread_count").unwrap();

    let stats = latency::simulate(*request_count, *request_rate, *request_duration, *thread_count);

    println!("Interval between requests: {:.3}ms", stats.interval_between_requests);
    println!("Time to send requests: {:.3}s", stats.time_to_send as f32 / 1000.0);
    println!("Time to process requests: {:.3}s", stats.time_to_process as f32 / 1000.0);
    println!("Max latency: {:.3}s", stats.max_latency as f32 / 1000.0);
}
