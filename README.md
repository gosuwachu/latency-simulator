Latency calculator.

```
Usage: latency-simulator --request-count <request_count> --rate <request_rate> --processing-duration <request_duration> --threads <thread_count>

Options:
-c, --request-count <request_count>           Number of requests to send
-r, --rate <request_rate>                     Number of requests per second
-p, --processing-duration <request_duration>  Request processing duration in milliseconds
-t, --threads <thread_count>                  Number of threads processing requests concurrently
-h, --help                                    Print help
-V, --version                                 Print version
```

Example:

```
$ latency-simulator --request-count 100 --rate 100 --processing-duration 600 --threads 30
Time to send requests: 0.990s
Time to process requests: 2.490s
Max latency: 0.900s
```