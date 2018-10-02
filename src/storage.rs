use config::Config;

// Storage trait defines abstraction
// over storages like mysql for dumping
pub trait Storage {
    fn build(&mut self, conf:Config);
}