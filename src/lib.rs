mod job;
mod trigger;
pub mod worker;

use std::collections::HashMap;
use std::thread;
use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::time::{Duration, Instant};
use crate::job::*;
use crate::trigger::*;
use crate::worker::*;

pub struct Scheduler {
    jobs: HashMap<String, Job>,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            jobs: HashMap::new(),
        }
    }

    pub fn get_job_names(&self) -> Vec<String> {
        return self.jobs.keys().cloned().collect();
    }

    pub fn get_trigger_names(&mut self, job_name: &str) -> Vec<String> {
        if let Some(job) = self.jobs.get_mut(job_name) {
            return job.get_trigger_names();   
        }
        return Vec::new();
    }

    pub fn add_job<T>(&mut self, job_name: &str, worker: T)
    where T: Worker + Send + Sync + 'static,
    {
        if let None = self.jobs.get_mut(job_name) {
            let job: Job = Job::new(worker);
            self.jobs.insert(job_name.to_string(), job);
            self.start();
        }
    }

    pub fn remove_job(&mut self, job_name: &str) {
        if let Some((_, job)) = self.jobs.remove_entry(job_name).as_mut() {
            job.release();
        }
    }

    pub fn add_trigger(&mut self, job_name: &str, trigger_name: &str, interval_sec: u16) {
        if let Some(job) = self.jobs.get_mut(job_name) {
            job.add_trigger(trigger_name, interval_sec);
        }
    }

    pub fn remove_trigger(&mut self, job_name: &str, trigger_name: &str) {
        if let Some(job) = self.jobs.get_mut(job_name) {
            job.remove_trigger(trigger_name);
        }
    }

    pub fn remove_triggers(&mut self, job_name: &str) {
        if let Some(job) = self.jobs.get_mut(job_name) {
            job.remove_triggers();
        }
    }

    pub fn change_trigger_interval(&mut self, job_name: &str, trigger_name: &str, interval_sec: u16) {
        if let Some(job) = self.jobs.get_mut(job_name) {
            job.change_trigger_interval(trigger_name, interval_sec);
        }
    }

    pub fn pause_trigger(&mut self, job_name: &str, trigger_name: &str) {
        if let Some(job) = self.jobs.get_mut(job_name) {
            job.pause_trigger(trigger_name);
        }
    }

    pub fn resume_trigger(&mut self, job_name: &str, trigger_name: &str) {
        if let Some(job) = self.jobs.get_mut(job_name) {
            job.resume_trigger(trigger_name);
        }
    }

    pub fn pause_triggers(&mut self, job_name: &str) {
        if let Some(job) = self.jobs.get_mut(job_name) {
            job.pause_triggers();
        }
    }

    pub fn resume_triggers(&mut self, job_name: &str) {
        if let Some(job) = self.jobs.get_mut(job_name) {
            job.resume_triggers();
        }
    }

    pub fn release(&mut self) {
        for (_, job) in self.jobs.iter_mut() {
            job.release();
        }
        self.jobs.clear();
    }

    fn start(&mut self) {
        for (_, job) in self.jobs.iter_mut() {
            let worker: Arc<dyn Worker + Send + Sync> = job.worker.clone();
            let triggers: Arc<Mutex<HashMap<String, Trigger>>> = job.triggers.clone();
            let close_thread: Arc<AtomicBool> = job.close_thread.clone();
            let receiver: Arc<Mutex<Receiver<()>>> = job.receiver.clone();
            let unlock_tag:Arc<AtomicBool> = job.unlock_tag.clone();

            if job.thread.is_some() {
                continue;
            } 

            job.thread = Some(thread::spawn(move || loop {

                if close_thread.load(Ordering::Relaxed) {
                   return;
                }

                let mut triggers_lock: MutexGuard<'_, HashMap<String, Trigger>> = triggers.lock().unwrap();

                for (name, trigger) in triggers_lock.iter_mut().filter(|(_, trigger)| !trigger.paused) {
                    if unlock_tag.load(Ordering::Relaxed) {
                        break;
                    }

                    let start: Instant = Instant::now();
                    let duration: Option<Duration> = trigger.last_shot.map_or(None, |x|start.checked_duration_since(x));

                    if duration.is_none_or(|d| d.as_secs() >= trigger.interval) {
                        trigger.last_shot = Some(start);
                        worker.run(name);      
                    }
                }

                let mut closest_shot: u64 = u64::MAX;
                let start: Instant = Instant::now();

                for (_, trigger) in triggers_lock.iter_mut().filter(|(_, trigger)| !trigger.paused) {
                    let duration: Option<Duration> = trigger.last_shot.map_or(None, |x|start.checked_duration_since(x));

                    if duration.is_none_or(|d: Duration| d.as_secs() >= trigger.interval) {
                        closest_shot = 0;
                    }
                    else {
                        let d = duration.unwrap();

                        if (trigger.interval - d.as_secs()) < closest_shot {
                            closest_shot = trigger.interval - d.as_secs();
                        }
                    }
                }

                drop(triggers_lock); 

                let rx: MutexGuard<'_, Receiver<()>> = receiver.lock().unwrap();         
                match rx.recv_timeout(Duration::from_secs(closest_shot)) {
                    Ok(_) => (),
                    Err(RecvTimeoutError::Timeout) => (),
                    Err(_) => (),
                }

                drop(rx); 
            }));
        }
    }
}