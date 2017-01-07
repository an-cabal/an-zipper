use ::Stack;

pub struct List<T> { head: Link<T>
                   , len: usize
                   }

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> { elem: T
               , next: Link<T>
               }

impl<T> Node<T> {
    #[inline]
    fn link(self) -> Link<T> {
        Some(Box::new(self))
    }

}

impl<T> Stack<T> for List<T> {
    fn push(&mut self, elem: T) -> &mut Self {
        self.head = Node { elem: elem, next: self.head.take() }.link();
        self.len += 1;
        self
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take()
            .map(|head| {
                let head = *head;
                self.head = head.next;
                self.len -= 1;
                head.elem
            })
    }

    #[inline]
    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem )
    }

    #[inline]
    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem )
    }
}


impl<T> List<T> {
    #[inline] fn len(&self) -> usize { self.len }
    #[inline] fn is_empty(&self) -> bool { self.len == 0 }
    #[inline] fn new() -> Self {
        List { head: None
             , len: 0 }
    }
}
