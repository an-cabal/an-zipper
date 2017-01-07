pub trait Stack<T> {
    fn push(&mut self, elem: T) -> &mut Self;
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn peek_mut(&mut self) -> Option<&mut T>;
}

//==- singly-linked list -===================================================
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

    #[inline]
    fn new(elem: T) -> Self { Node { elem: elem, next: None } }

}

impl<T> Stack<T> for List<T> {
    fn push(&mut self, elem: T) -> &mut Self {
        self.cons(Box::new(Node::new(elem)))
    }

    fn pop(&mut self) -> Option<T> {
        self.uncons().map(|node| node.elem)
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

    #[inline] fn is_empty(&self) -> bool { self.head.is_none() }

    #[inline] fn new() -> Self {
        List { head: None
             , len: 0 }
    }

    fn cons(&mut self, mut node: Box<Node<T>>) -> &mut Self {
        node.next = self.head.take();
        self.head = Some(node);
        self.len += 1;
        self
    }

    fn uncons(&mut self) -> Link<T> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            self.len -= 1;
            node
        })
    }
}

//==- zip list -=============================================================
pub struct ZipList<T> { left: List<T>
                      , right: List<T>
                      }

impl<T> ZipList<T> {

    // -- wrappers around sublist methods -----------------------------------
    #[inline] pub fn pop_left(&mut self) -> Option<T> { self.left.pop() }
    #[inline] pub fn pop_right(&mut self) -> Option<T> { self.right.pop() }
    #[inline] pub fn peek_left(&self) -> Option<&T> { self.left.peek() }
    #[inline] pub fn peek_right(&self) -> Option<&T> { self.right.peek() }
    #[inline] pub fn peek_left_mut(&mut self) -> Option<&mut T> {
        self.left.peek_mut()
     }
    #[inline] pub fn peek_right_mut(&mut self) -> Option<&mut T> {
        self.right.peek_mut()
    }

    #[inline] pub fn push_left(&mut self, elem: T) -> &mut Self {
         self.left.push(elem);
         self
     }

    #[inline] pub fn push_right(&mut self, elem: T) -> &mut Self {
        self.left.push(elem);
        self
    }

    #[inline] pub fn len(&self) -> usize { self.left.len() + self.right.len() }

    #[inline] pub fn is_empty(&self) -> bool {
        self.left.is_empty() && self.right.is_empty()
    }

    pub fn move_left(&mut self) -> bool {
        self.left.uncons()
            .map(|n| self.right.cons(n))
            .is_some()
    }

    pub fn move_right(&mut self) -> bool {
        self.right.uncons()
            .map(|n| self.left.cons(n))
            .is_some()
    }

    pub fn seek_left(&mut self, amount: usize) {
        unimplemented!()
    }

    pub fn seek_right(&mut self, amount: usize) {
        unimplemented!()
    }

}
