Latency calculator.

```
Usage: latency-simulator --threads <thread_count> [COMMAND]

Commands:
  load-requests      Load requests from a file
  generate-requests  Generates requests based on provided parameters
  help               Print this message or the help of the given subcommand(s)

Options:
  -t, --threads <thread_count>  Number of threads processing requests concurrently
  -h, --help                    Print help
  -V, --version                 Print version
```

Example:

```
$ latency-simulator --threads 300 generate-requests --request-count 100000 --rate 500 --processing-duration 600
Time to send requests: 199.998s
Time to process requests: 200.598s
Max latency: 0.000s
```