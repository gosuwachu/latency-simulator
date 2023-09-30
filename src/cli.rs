use clap::{arg, ArgAction, ArgMatches, command, value_parser};

pub fn cli() -> ArgMatches {
    command!()
        .version("1.0.0")
        .about("Latency calculator.")
        .author("Piotr Wach <pwach@bloomberg.net>")
        .arg(arg!(
                request_count: -c --"request-count" "Number of requests to send"
            )
            .value_parser(value_parser!(u32))
            .action(ArgAction::Set)
            .required(true)
        )
        .arg(arg!(
                request_rate: -r --rate "Number of requests per second"
            )
            .value_parser(value_parser!(u32))
            .action(ArgAction::Set)
            .required(true)
        )
        .arg(arg!(
                request_duration: -p --"processing-duration" "Request processing duration in milliseconds"
            )
            .value_parser(value_parser!(u32).range(1..))
            .action(ArgAction::Set)
            .required(true)
        )
        .arg(arg!(
                thread_count: -t --threads "Number of threads processing requests concurrently"
            )
            .value_parser(value_parser!(u32).range(1..))
            .action(ArgAction::Set)
            .required(true)
        )
        .get_matches()
}
