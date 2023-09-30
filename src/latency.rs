use std::ops::Deref;
use std::cell::RefCell;
use std::cmp::max;

#[derive(PartialEq, Debug, Clone)]
struct Request {
    duration: u32,
    arrived: u32
}

impl Request {
    fn new(duration: u32, arrived: u32) -> Request {
        Request { duration, arrived }
    }
}

#[derive(PartialEq, Debug)]
struct Thread {
    busy_until: RefCell<u32>
}

impl Thread {
    fn new() -> Thread {
        Thread { busy_until: RefCell::new(0) }
    }
}

fn create_request_queue(request_count: u32, request_duration: u32, request_rate: u32) -> Vec<Request> {
    let mut request_queue = Vec::<Request>::new();

    let mut interval_between_requests = 0;
    if request_rate > 0 {
        interval_between_requests = 1000 / request_rate;
    }

    let mut clock: u32 = 0;
    for _ in 0..request_count {
        request_queue.push(Request::new(request_duration, clock));
        clock += interval_between_requests;
    }

    request_queue
}

fn create_threads(thread_count: u32) -> Vec<Thread> {
    let mut threads = Vec::<Thread>::new();
    for _ in 0..thread_count {
        threads.push(Thread::new());
    }
    threads
}

fn get_first_available_thread(threads: &Vec<Thread>, clock: u32) -> Option<&Thread> {
    for thread in threads {
        let busy_until = thread.busy_until.borrow();
        if busy_until.deref() <= &clock {
            return Some(thread)
        }
    }
    None
}

#[derive(Debug, PartialEq)]
pub struct Stats {
    pub max_latency: u32,
    pub time_to_send: u32,
    pub time_to_process: u32
}

impl Stats {
    fn new(max_latency: u32, time_to_send: u32, time_to_process: u32) -> Stats {
        Stats { max_latency, time_to_send, time_to_process }
    }
}

pub fn simulate(request_count: u32, request_rate: u32, request_duration: u32, thread_count: u32) -> Stats {
    let incoming_requests = create_request_queue(request_count, request_duration, request_rate);
    let threads = create_threads(thread_count);

    let mut max_latency: u32 = 0;
    let mut time_to_send: u32 = 0;
    let mut time_to_process: u32 = 0;
    let mut clock: u32 = 0;

    for incoming_request in &incoming_requests {
        time_to_send = incoming_request.arrived;
        clock = max(clock, incoming_request.arrived);
        'waiting_for_available_thread: loop {
            if let Some(thread) = get_first_available_thread(&threads, clock) {
                let mut busy_until = thread.busy_until.borrow_mut();
                *busy_until = clock + incoming_request.duration;
                time_to_process = max(time_to_process, *busy_until);
                let latency = clock - incoming_request.arrived;
                max_latency = max(max_latency, latency);
                break 'waiting_for_available_thread;
            } else {
                clock += 1;
            }
        }
    }

    Stats::new(max_latency, time_to_send, time_to_process)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_new() {
        let request = Request::new(1, 2);
        assert_eq!(request.duration, 1);
        assert_eq!(request.arrived, 2);
    }

    #[test]
    fn thread_new() {
        let thread = Thread::new();
        assert_eq!(*thread.busy_until.borrow().deref(), 0);
    }

    #[test]
    fn create_request_queue_basic() {
        let queue = create_request_queue(1, 100, 0);
        assert_eq!(queue.len(), 1);

        let r0 = &queue[0];
        assert_eq!(r0.arrived, 0);
        assert_eq!(r0.duration, 100);
    }

    #[test]
    fn create_request_queue_with_rate_eq_zero() {
        let queue = create_request_queue(2, 100, 0);
        assert_eq!(queue.len(), 2);
        assert_eq!(queue, vec![
            Request::new(100, 0),
            Request::new(100, 0),
        ]);
    }

    #[test]
    fn create_threads_basic() {
        let threads = create_threads(2);
        assert_eq!(threads.len(), 2);
    }

    #[test]
    fn get_first_available_thread_basic() {
        let threads = create_threads(2);

        assert_eq!(
            get_first_available_thread(&threads, 0).unwrap(),
            &threads[0]);
    }

    #[test]
    fn get_first_available_thread_with_one_busy() {
        let threads = create_threads(2);
        *threads[0].busy_until.borrow_mut() = 100;

        assert_eq!(
            get_first_available_thread(&threads, 0).unwrap(),
            &threads[1]);

        assert_eq!(
            get_first_available_thread(&threads, 100).unwrap(),
            &threads[0]);
    }

    #[test]
    fn get_first_available_thread_with_two_busy() {
        let threads = create_threads(2);
        *threads[0].busy_until.borrow_mut() = 100;
        *threads[1].busy_until.borrow_mut() = 50;

        assert_eq!(
            get_first_available_thread(&threads, 0),
            None);

        assert_eq!(
            get_first_available_thread(&threads, 50).unwrap(),
            &threads[1]);

        assert_eq!(
            get_first_available_thread(&threads, 100).unwrap(),
            &threads[0]);
    }

    #[test]
    fn simulate_basic_1() {
        let s = simulate(1, 0, 0, 1);
        assert_eq!(s, Stats::new(0, 0, 0));
    }

    #[test]
    fn simulate_basic_2() {
        let s = simulate(2, 1, 1000, 1);
        assert_eq!(s, Stats::new(0, 1000, 2000));
    }

    #[test]
    fn simulate_basic_3() {
        let s = simulate(2, 2, 1000, 1);
        assert_eq!(s, Stats::new(500, 500, 2000));
    }

    #[test]
    fn simulate_basic_4() {
        let s = simulate(100, 0, 1000, 100);
        assert_eq!(s, Stats::new(0, 0, 1000));
    }

    #[test]
    fn simulate_basic_5() {
        let s = simulate(100, 0, 1000, 50);
        assert_eq!(s, Stats::new(1000, 0, 2000));
    }

    #[test]
    fn simulate_basic_6() {
        let s = simulate(101, 0, 1000, 50);
        assert_eq!(s, Stats::new(2000, 0, 3000));
    }

    #[test]
    fn simulate_basic_7() {
        let s = simulate(3, 100, 11, 1);
        assert_eq!(s, Stats::new(2, 20, 33));
    }
}