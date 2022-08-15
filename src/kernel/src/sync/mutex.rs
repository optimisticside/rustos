/// A mutex is anything that can be locked and unlocked. This can be implemented by a sleep lock,
/// spin lock, etc.
pub trait Mutex<T> {
    fn lock(&mut self) -> MutexGuard<T>;
    fn unlock(&
}
