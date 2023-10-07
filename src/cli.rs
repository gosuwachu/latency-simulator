use clap::{arg, command, value_parser, ArgAction, ArgMatches, Command};

pub fn cli() -> ArgMatches {
    command!()
        .version("1.0.0")
        .about("Latency calculator.")
        .author("Piotr Wach <pwach@bloomberg.net>")
        .arg(arg!(
                thread_count: -t --threads "Number of threads processing requests concurrently"
            )
            .value_parser(value_parser!(u32).range(1..))
            .action(ArgAction::Set)
            .required(true)
        )
        .arg(arg!(
                visualize: --visualize "Visualize request processing"
            )
            .action(ArgAction::SetTrue)
            .required(true)
        )
        .arg(arg!(
                step: --step "Visualization speed-up"
            )
            .value_parser(value_parser!(u64).range(1..))
            .action(ArgAction::Set)
            .required(true)
        )
        .subcommand(
            Command::new("load-requests")
                .about("Load requests from a file")
                .arg(arg!(
                        from_file: -f --"from-file" "Load requests from file"
                    )
                    .value_parser(value_parser!(String))
                    .action(ArgAction::Set)
                    .required(false)
                )
        )
        .subcommand(
            Command::new("generate-requests")
                .about("Generates requests based on provided parameters")
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
        )
        .get_matches()
}
