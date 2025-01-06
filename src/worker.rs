pub trait Worker {
    fn run(&self, _parameter: &str) {}
}