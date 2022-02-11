use crossbeam::deque::{Injector, Stealer, Worker};
use std::{
    iter,
    sync::{Arc, Mutex},
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct WorkPool {
    global: Arc<Injector<Job>>,
    local: Worker<Job>,
    stealers: Arc<Mutex<Vec<Stealer<Job>>>>,
}

impl WorkPool {
    pub fn new() -> Self {
        let local = Worker::new_fifo();
        let stealers = Arc::new(Mutex::new(vec![local.stealer()]));
        let global = Arc::new(Injector::<Job>::new());

        Self {
            global,
            local,
            stealers,
        }
    }

    pub fn find_job(&self) -> Option<Job> {
        self.local.pop().or_else(|| {
            iter::repeat_with(|| {
                self.global.steal_batch_and_pop(&self.local).or_else(|| {
                    self.stealers
                        .lock()
                        .unwrap()
                        .iter()
                        .map(|stealer| stealer.steal())
                        .collect()
                })
            })
            .find(|steal| !steal.is_retry())
            .and_then(|steal| steal.success())
        })
    }

    pub fn push_job(&self, job: Job) {
        self.global.push(job);
    }
}

impl Clone for WorkPool {
    fn clone(&self) -> Self {
        let local = Worker::new_fifo();
        self.stealers.lock().unwrap().push(local.stealer());

        Self {
            global: self.global.clone(),
            local,
            stealers: self.stealers.clone(),
        }
    }
}
