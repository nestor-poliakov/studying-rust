#![forbid(unsafe_code)]
use std::rc::Rc;

pub struct PRef<T> {
    item: Rc<T>,
}

impl<T> Clone for PRef<T> {
    fn clone(&self) -> Self {
        Self {
            item: self.item.clone(),
        }
    }
}

impl<T> std::ops::Deref for PRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.item.as_ref()
    }
}

////////////////////////////////////////////////////////////////////////////////

struct Node<T> {
    item: PRef<T>,
    prev: Option<Rc<Node<T>>>,
}

pub struct PStack<T> {
    node: Option<Rc<Node<T>>>,
    len: usize,
}

impl<T> Default for PStack<T> {
    fn default() -> Self {
        Self { node: None, len: 0 }
    }
}

impl<T> Clone for PStack<T> {
    fn clone(&self) -> Self {
        Self {
            node: self.node.clone(),
            len: self.len,
        }
    }
}

impl<T> PStack<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&self, value: T) -> Self {
        Self {
            node: Some(Rc::new(Node {
                item: PRef {
                    item: Rc::new(value),
                },
                prev: self.node.clone(),
            })),
            len: self.len + 1,
        }
    }

    pub fn pop(&self) -> Option<(PRef<T>, Self)> {
        let node = self.node.as_ref()?;
        Some((
            node.item.clone(),
            Self {
                node: node.prev.clone(),
                len: self.len - 1,
            },
        ))
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = PRef<T>> {
        Iter {
            pstack: self.clone(),
        }
    }
}

struct Iter<T> {
    pstack: PStack<T>,
}

impl<T> Iterator for Iter<T> {
    type Item = PRef<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let (item, ns) = self.pstack.pop()?;
        self.pstack = ns;
        Some(item)
    }
}
