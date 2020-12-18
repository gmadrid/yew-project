pub fn if_option<T, F>(cond: bool, f: F) -> Option<T>
where
    F: FnOnce() -> T,
{
    if cond {
        Some(f())
    } else {
        None
    }
}
