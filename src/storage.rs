
// Storage trait defines abstraction
// over storages like mysql for dumping
pub trait Storage {
    fn open();
    fn backup();
}