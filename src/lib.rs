pub trait Stack<T> {
    fn push(&mut self, elem: T) -> &mut Self;
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn peek_mut(&mut self) -> Option<&mut T>;
}

pub mod slist;
