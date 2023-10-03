use std::ops::Deref;
use std::cell::RefCell;

#[derive(PartialEq, Debug, Clone)]
pub struct Request {
    duration: f64,
    arrived: f64
}

impl Request {
    pub fn new(duration: f64, arrived: f64) -> Request {
        Request { duration, arrived }
    }
}

#[derive(PartialEq, Debug)]
struct Thread {
    busy_until: RefCell<f64>
}

impl Thread {
    fn new() -> Thread {
        Thread { busy_until: RefCell::new(0.0) }
    }
}

pub fn interval_between_requests(request_rate: u32) -> f64 {
    if request_rate > 0 {
        return 1000.0 / request_rate as f64;
    }
    0.0
}

pub fn create_request_queue(request_count: u32, request_duration: u32, interval_between_requests: f64) -> Vec<Request> {
    let mut request_queue = Vec::<Request>::new();

    let mut clock = 0.0;
    for _ in 0..request_count {
        request_queue.push(Request::new(request_duration as f64, clock));
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

fn get_first_available_thread(threads: &Vec<Thread>, clock: f64) -> Option<&Thread> {
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
    pub max_latency: f64,
    pub time_to_send: f64,
    pub time_to_process: f64
}

impl Stats {
    #[cfg(test)]
    fn new(max_latency: f64, time_to_send: f64, time_to_process: f64) -> Stats {
        Stats { max_latency, time_to_send, time_to_process }
    }

    fn empty() -> Stats {
        Stats { max_latency: 0.0, time_to_send: 0.0, time_to_process: 0.0 }
    }

    fn update_max_latency(&mut self, latency: f64) {
        self.max_latency = f64::max(self.max_latency, latency);
    }

    fn update_time_to_send(&mut self, clock: f64) {
        self.time_to_send = f64::max(self.time_to_send, clock);
    }

    fn update_time_to_process(&mut self, clock: f64) {
        self.time_to_process = f64::max(self.time_to_process, clock);
    }
}

pub fn simulate(incoming_requests: &Vec<Request>, thread_count: u32) -> Stats {
    let threads = create_threads(thread_count);

    let mut stats = Stats::empty();
    let mut clock: f64 = 0.0;

    for incoming_request in incoming_requests {
        stats.update_time_to_send(incoming_request.arrived);
        clock = f64::max(clock, incoming_request.arrived);
        'waiting_for_available_thread: loop {
            if let Some(thread) = get_first_available_thread(&threads, clock) {
                let mut busy_until = thread.busy_until.borrow_mut();
                *busy_until = clock + incoming_request.duration;

                stats.update_time_to_process(*busy_until);
                stats.update_max_latency(clock - incoming_request.arrived);

                break 'waiting_for_available_thread;
            } else {
                clock += 1.0;
            }
        }
    }

    stats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_new() {
        let request = Request::new(1.0, 2.0);
        assert_eq!(request.duration, 1.0);
        assert_eq!(request.arrived, 2.0);
    }

    #[test]
    fn thread_new() {
        let thread = Thread::new();
        assert_eq!(*thread.busy_until.borrow().deref(), 0.0);
    }

    #[test]
    fn create_request_queue_basic() {
        let queue = create_request_queue(1, 100, 0.0);
        assert_eq!(queue.len(), 1);

        let r0 = &queue[0];
        assert_eq!(r0.arrived, 0.0);
        assert_eq!(r0.duration, 100.0);
    }

    #[test]
    fn create_request_queue_with_rate_eq_zero() {
        let queue = create_request_queue(2, 100, 0.0);
        assert_eq!(queue.len(), 2);
        assert_eq!(queue, vec![
            Request::new(100.0, 0.0),
            Request::new(100.0, 0.0),
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
            get_first_available_thread(&threads, 0.0).unwrap(),
            &threads[0]);
    }

    #[test]
    fn get_first_available_thread_with_one_busy() {
        let threads = create_threads(2);
        *threads[0].busy_until.borrow_mut() = 100.0;

        assert_eq!(
            get_first_available_thread(&threads, 0.0).unwrap(),
            &threads[1]);

        assert_eq!(
            get_first_available_thread(&threads, 100.0).unwrap(),
            &threads[0]);
    }

    #[test]
    fn get_first_available_thread_with_two_busy() {
        let threads = create_threads(2);
        *threads[0].busy_until.borrow_mut() = 100.0;
        *threads[1].busy_until.borrow_mut() = 50.0;

        assert_eq!(
            get_first_available_thread(&threads, 0.0),
            None);

        assert_eq!(
            get_first_available_thread(&threads, 50.0).unwrap(),
            &threads[1]);

        assert_eq!(
            get_first_available_thread(&threads, 100.0).unwrap(),
            &threads[0]);
    }

    #[test]
    fn simulate_basic_1() {
        let r = create_request_queue(
            1,
            0,
            interval_between_requests(0));
        let s = simulate(&r, 1);
        assert_eq!(s, Stats::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn simulate_basic_2() {
        let r = create_request_queue(
            2,
            1000,
            interval_between_requests(1));
        let s = simulate(&r, 1);
        assert_eq!(s, Stats::new(0.0, 1000.0, 2000.0));
    }

    #[test]
    fn simulate_basic_3() {
        let r = create_request_queue(
            2,
            1000,
            interval_between_requests(2));
        let s = simulate(&r, 1);
        assert_eq!(s, Stats::new(500.0, 500.0, 2000.0,));
    }

    #[test]
    fn simulate_basic_4() {
        let r = create_request_queue(
            100,
            1000,
            interval_between_requests(0));
        let s = simulate(&r, 100);
        assert_eq!(s, Stats::new(0.0, 0.0, 1000.0));
    }

    #[test]
    fn simulate_basic_5() {
        let r = create_request_queue(
            100,
            1000,
            interval_between_requests(0));
        let s = simulate(&r, 50);
        assert_eq!(s, Stats::new(1000.0, 0.0, 2000.0));
    }

    #[test]
    fn simulate_basic_6() {
        let r = create_request_queue(
            101,
            1000,
            interval_between_requests(0));
        let s = simulate(&r, 50);
        assert_eq!(s, Stats::new(2000.0, 0.0, 3000.0));
    }

    #[test]
    fn simulate_basic_7() {
        let r = create_request_queue(
            3,
            11,
            interval_between_requests(100));
        let s = simulate(&r, 1);
        assert_eq!(s, Stats::new(2.0, 20.0, 33.0));
    }
}