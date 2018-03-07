pub type ArenaIndex = usize;

/// Append only arena
pub struct Arena<T> {
    inner: Vec<T>,
}

impl<T: Sized> Arena<T> {
    pub fn new() -> Arena<T> {
        Arena {
            inner: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) -> ArenaIndex {
        let index = self.inner.len();
        self.inner.push(item);
        index
    }
}
