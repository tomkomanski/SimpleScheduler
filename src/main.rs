use std::thread;
use std::time::Duration;
use chrono::Local;

extern crate simple_scheduler;
use simple_scheduler::{worker::Worker, Scheduler};

fn main() {
    pub struct Group {
    }

    impl Worker for Group {
        fn run(&self, trigger_name: &str) {
            let now = Local::now();
            println!("Trigger: {}, Time: {}", trigger_name, now);
        }
    }

    let one: Group = Group {
    };

    let two: Group = Group {
    };

    let three: Group = Group {
    };

    let mut scheduler: Scheduler = Scheduler::new();
    scheduler.add_job("thread1",one);
    scheduler.add_job("thread2", two);

    scheduler.add_trigger("thread1", "eat", 1);
    scheduler.add_trigger("thread1", "sleep", 2);
    scheduler.add_trigger("thread2", "code", 3);
    scheduler.add_trigger("thread2", "code learning", 4);

 
    thread::sleep(Duration::from_secs(10));
    scheduler.pause_triggers("thread1");
    scheduler.pause_trigger("thread2", "code");
    thread::sleep(Duration::from_secs(10));
    scheduler.pause_triggers("thread2");
    scheduler.resume_triggers("thread1");
    thread::sleep(Duration::from_secs(10));
    scheduler.pause_triggers("thread1");
    scheduler.pause_triggers("thread2");
    thread::sleep(Duration::from_secs(10));
    println!("jobs: {:?}",scheduler.get_job_names());
    println!("triggers on thread 1: {:?}",scheduler.get_trigger_names("thread1"));
    println!("triggers on thread 2: {:?}",scheduler.get_trigger_names("thread2"));
    thread::sleep(Duration::from_secs(10));
    scheduler.resume_triggers("thread1");
    scheduler.resume_triggers("thread2");
    thread::sleep(Duration::from_secs(10));
    scheduler.remove_job("thread1");
    scheduler.remove_trigger("thread2", "code learning");
    println!("jobs: {:?}",scheduler.get_job_names());
    println!("triggers on thread 1: {:?}",scheduler.get_trigger_names("thread1"));
    println!("triggers on thread 2: {:?}",scheduler.get_trigger_names("thread2"));
    thread::sleep(Duration::from_secs(10));
    scheduler.remove_job("thread2");
    scheduler.add_job("holidays", three);
    scheduler.add_trigger("holidays", "hicking", 1);
    thread::sleep(Duration::from_secs(10));
    scheduler.release();
}
