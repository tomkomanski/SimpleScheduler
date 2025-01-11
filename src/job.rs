use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use crate::trigger::*;
use crate::worker::*;

pub struct Job {
    pub triggers: Arc<Mutex<HashMap<String, Trigger>>>,
    pub worker: Arc<dyn Worker + Sync + Send>,
    pub close_thread: Arc<AtomicBool>,
    pub thread: Option<JoinHandle<()>>,
    pub sender: Sender<()>, 
    pub receiver: Arc<Mutex<Receiver<()>>>,
    pub unlock_tag: Arc<AtomicBool>,
}

impl Job {
    pub fn new<T>(worker: T) -> Self
    where T: Worker + Send + Sync + 'static,
    {
        let (sender, receiver) = mpsc::channel();
        let job: Job = Job {
            triggers: Arc::new(Mutex::new(HashMap::new())),
            worker: Arc::new(worker),
            close_thread: Arc::new(AtomicBool::new(false)),
            thread: None,
            sender: sender,
            receiver: Arc::new(Mutex::new(receiver)),
            unlock_tag: Arc::new(AtomicBool::new(false)),
        };

        return job;
    }

    pub fn get_trigger_names(&mut self) -> Vec<String> {
        self.unlock_thread_loop();
        let triggers: Arc<Mutex<HashMap<String, Trigger>>> = self.triggers.clone();
        let triggers_lock: MutexGuard<'_, HashMap<String, Trigger>> = triggers.lock().unwrap();
        let names: Vec<String> = triggers_lock.keys().cloned().collect::<Vec<String>>();
        self.lock_thread_loop();
        return names;
    }

    pub fn add_trigger(&mut self, trigger_name: &str, interval_sec: u16) {
        self.unlock_thread_loop();
        let trigger: Trigger = Trigger::new(interval_sec);
        let triggers: Arc<Mutex<HashMap<String, Trigger>>> = self.triggers.clone();
        let mut triggers_lock: MutexGuard<'_, HashMap<String, Trigger>> = triggers.lock().unwrap();

        if let None = triggers_lock.get_mut(trigger_name) {
            triggers_lock.insert(trigger_name.to_string(), trigger);
        }

        self.lock_thread_loop();
    }

    pub fn remove_trigger(&mut self, trigger_name: &str) {
        self.unlock_thread_loop();
        let triggers: Arc<Mutex<HashMap<String, Trigger>>> = self.triggers.clone();
        let mut triggers_lock: MutexGuard<'_, HashMap<String, Trigger>> = triggers.lock().unwrap();
        let _ = triggers_lock.remove_entry(trigger_name);
        self.lock_thread_loop();
    }

    pub fn remove_triggers(&mut self) {
        self.unlock_thread_loop();
        let triggers: Arc<Mutex<HashMap<String, Trigger>>> = self.triggers.clone();
        let mut triggers_lock: MutexGuard<'_, HashMap<String, Trigger>> = triggers.lock().unwrap();
        triggers_lock.clear();
        self.lock_thread_loop();
    }

    pub fn change_trigger_interval(&mut self, trigger_name: &str, interval_sec: u16) {
        self.unlock_thread_loop();
        let triggers: Arc<Mutex<HashMap<String, Trigger>>> = self.triggers.clone();
        let mut triggers_lock: MutexGuard<'_, HashMap<String, Trigger>> = triggers.lock().unwrap();

        if let Some(trigger) = triggers_lock.get_mut(trigger_name) {
            trigger.interval = interval_sec as u64;
            trigger.last_shot = None;
        }

        self.lock_thread_loop();
    }

    pub fn pause_trigger(&mut self, trigger_name: &str) {
        self.unlock_thread_loop();
        let triggers: Arc<Mutex<HashMap<String, Trigger>>> = self.triggers.clone();
        let mut triggers_lock: MutexGuard<'_, HashMap<String, Trigger>> = triggers.lock().unwrap();

        if let Some(trigger) = triggers_lock.get_mut(trigger_name) {
            trigger.paused = true;
        }

        self.lock_thread_loop();
    }

    pub fn resume_trigger(&mut self, trigger_name: &str) {
        self.unlock_thread_loop();
        let triggers: Arc<Mutex<HashMap<String, Trigger>>> = self.triggers.clone();
        let mut triggers_lock: MutexGuard<'_, HashMap<String, Trigger>> = triggers.lock().unwrap();

        if let Some(trigger) = triggers_lock.get_mut(trigger_name) {
            trigger.paused = false;
        }

        self.lock_thread_loop();
    }

    pub fn pause_triggers(&mut self) {
        self.unlock_thread_loop();
        let triggers: Arc<Mutex<HashMap<String, Trigger>>> = self.triggers.clone();
        let mut triggers_lock: MutexGuard<'_, HashMap<String, Trigger>> = triggers.lock().unwrap();
        triggers_lock.iter_mut().for_each(|(_, trigger)| trigger.paused = true);
        self.lock_thread_loop();
    } 

    pub fn resume_triggers(&mut self) {
        self.unlock_thread_loop();
        let triggers: Arc<Mutex<HashMap<String, Trigger>>> = self.triggers.clone();
        let mut triggers_lock: MutexGuard<'_, HashMap<String, Trigger>> = triggers.lock().unwrap();
        triggers_lock.iter_mut().for_each(|(_, trigger)| trigger.paused = false);
        self.lock_thread_loop();
    } 

    pub fn release(&mut self) {
        self.close_thread.swap(true, Ordering::Relaxed);
        self.unlock_thread_loop();
        self.thread.take().map(|x| JoinHandle::join(x));
        self.thread = None;
    }

    fn unlock_thread_loop(&mut self) {
        self.unlock_tag.swap(true, Ordering::Relaxed);
        self.sender.send(()).unwrap();
    }

    fn lock_thread_loop(&mut self) {
        self.unlock_tag.swap(false, Ordering::Relaxed);
        self.sender.send(()).unwrap();
    }
}