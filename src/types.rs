use std::cell::RefCell;

#[derive(PartialEq, Debug, Clone)]
pub struct Request {
    pub duration: f64,
    pub arrived: f64,
    pub name: String,
}

impl Request {
    pub fn new(duration: f64, arrived: f64, name: String) -> Request {
        Request {
            duration,
            arrived,
            name,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Thread {
    pub start: RefCell<f64>,
    pub busy_until: RefCell<f64>,
    pub task_name: RefCell<String>,
}

impl Thread {
    pub fn new() -> Thread {
        Thread {
            start: RefCell::new(0.0),
            busy_until: RefCell::new(0.0),
            task_name: RefCell::new("".to_string()),
        }
    }

    pub fn update(&self, start_: f64, busy_until_: f64, task_name_: String) {
        let mut start = self.start.borrow_mut();
        let mut busy_until = self.busy_until.borrow_mut();
        let mut task_name = self.task_name.borrow_mut();
        *start = start_;
        *busy_until = busy_until_;
        *task_name = task_name_;
    }
}

#[derive(Debug, PartialEq)]
pub struct Stats {
    pub max_latency: f64,
    pub time_to_send: f64,
    pub time_to_process: f64,
}

impl Stats {
    #[cfg(test)]
    pub fn new(max_latency: f64, time_to_send: f64, time_to_process: f64) -> Stats {
        Stats {
            max_latency,
            time_to_send,
            time_to_process,
        }
    }

    pub fn empty() -> Stats {
        Stats {
            max_latency: 0.0,
            time_to_send: 0.0,
            time_to_process: 0.0,
        }
    }

    pub fn update_max_latency(&mut self, latency: f64) {
        self.max_latency = f64::max(self.max_latency, latency);
    }

    pub fn update_time_to_send(&mut self, clock: f64) {
        self.time_to_send = f64::max(self.time_to_send, clock);
    }

    pub fn update_time_to_process(&mut self, clock: f64) {
        self.time_to_process = f64::max(self.time_to_process, clock);
    }
}
