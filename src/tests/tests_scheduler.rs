use crate::scheduler::Scheduler;
use crate::scheduler::Worker;

#[derive(Copy, Clone)]
struct Test;
impl Worker for Test {
    fn run(&self, _trigger_name: &str) {}
}

#[test]
fn test_create_scheduler() {
    let scheduler: Scheduler = Scheduler::new();
    let jobs: Vec<String> = scheduler.get_job_names();
    assert_eq!(jobs.is_empty(), true);
}
#[test]
fn test_add_job() {
    let test: Test = Test {};
    let mut scheduler: Scheduler = Scheduler::new();
    scheduler.add_job("job", test);
    let jobs: Vec<String> = scheduler.get_job_names();
    assert_eq!(jobs.len(), 1);
    assert!(scheduler.get_job_names().contains(&"job".to_string()))
}
#[test]
fn test_add_jobs() {
    let test: Test = Test {};
    let mut scheduler: Scheduler = Scheduler::new();
    scheduler.add_job("job", test);
    scheduler.add_job("job1", test);
    let jobs: Vec<String> = scheduler.get_job_names();
    assert_eq!(jobs.len(), 2);
    assert!(scheduler.get_job_names().contains(&"job".to_string()));
    assert!(scheduler.get_job_names().contains(&"job1".to_string()));
}
#[test]
fn test_remove_job() {
    let test: Test = Test {};
    let mut scheduler: Scheduler = Scheduler::new();
    scheduler.add_job("job", test);
    scheduler.remove_job("job");
    let jobs: Vec<String> = scheduler.get_job_names();
    assert_eq!(jobs.is_empty(), true);
}
#[test]
fn test_remove_jobs() {
    let test: Test = Test {};
    let mut scheduler: Scheduler = Scheduler::new();
    scheduler.add_job("job", test);
    scheduler.add_job("job1", test);
    scheduler.remove_job("job");
    scheduler.remove_job("job1");
    let jobs: Vec<String> = scheduler.get_job_names();
    assert_eq!(jobs.is_empty(), true);
}
#[test]
fn test_add_trigger() {
    let test: Test = Test {};
    let mut scheduler: Scheduler = Scheduler::new();
    scheduler.add_job("job", test);
    scheduler.add_trigger("job", "trigger", 1);
    let trigger: Vec<String> = scheduler.get_trigger_names("job");
    assert_eq!(trigger.len(), 1);
    assert!(scheduler.get_trigger_names("job").contains(&"trigger".to_string()));
}
#[test]
fn test_remove_trigger() {
    let test: Test = Test {};
    let mut scheduler: Scheduler = Scheduler::new();
    scheduler.add_job("job", test);
    scheduler.add_trigger("job", "trigger", 1);
    scheduler.remove_trigger("job", "trigger");
    let trigger: Vec<String> = scheduler.get_trigger_names("job");
    assert_eq!(trigger.is_empty(), true);
}
#[test]
fn test_add_triggers() {
    let test: Test = Test {};
    let mut scheduler: Scheduler = Scheduler::new();
    scheduler.add_job("job", test);
    scheduler.add_trigger("job", "trigger", 1);
    scheduler.add_trigger("job", "trigger1", 1);
    let trigger: Vec<String> = scheduler.get_trigger_names("job");
    assert_eq!(trigger.len(), 2);
    assert!(scheduler.get_trigger_names("job").contains(&"trigger".to_string()));
    assert!(scheduler.get_trigger_names("job").contains(&"trigger1".to_string()));
}
#[test]
fn test_remove_triggers() {
    let test: Test = Test {};
    let mut scheduler: Scheduler = Scheduler::new();
    scheduler.add_job("job", test);
    scheduler.add_trigger("job", "trigger", 1);
    scheduler.add_trigger("job", "trigger1", 1);
    scheduler.remove_triggers("job");
    let trigger: Vec<String> = scheduler.get_trigger_names("job");
    assert_eq!(trigger.is_empty(), true);
}