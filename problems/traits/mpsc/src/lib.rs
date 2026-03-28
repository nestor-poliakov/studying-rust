#![forbid(unsafe_code)]

use std::{cell::RefCell, collections::VecDeque, fmt::Debug, rc::Rc};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("channel is closed")]
pub struct SendError<T> {
    pub value: T,
}

pub struct Sender<T> {
    q: Rc<RefCell<Queue<T>>>,
}

impl<T> Sender<T> {
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        self.q.borrow_mut().push(value)
    }

    pub fn is_closed(&self) -> bool {
        self.q.borrow().is_closed
    }

    pub fn same_channel(&self, other: &Self) -> bool {
        std::ptr::eq(self.q.as_ref(), other.q.as_ref())
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self { q: self.q.clone() }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let refs = Rc::strong_count(&self.q);
        if refs < 3 {
            self.q.borrow_mut().close();
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Error, Debug)]
pub enum ReceiveError {
    #[error("channel is empty")]
    Empty,
    #[error("channel is closed")]
    Closed,
}

pub struct Receiver<T> {
    q: Rc<RefCell<Queue<T>>>,
}

impl<T> Receiver<T> {
    pub fn recv(&self) -> Result<T, ReceiveError> {
        self.q.borrow_mut().pop()
    }

    pub fn close(&self) {
        self.q.borrow_mut().close();
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.close();
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let q = Rc::new(RefCell::new(Queue::new()));
    (Sender { q: q.clone() }, Receiver { q })
}

struct Queue<T> {
    data: VecDeque<T>,
    is_closed: bool,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Self {
            data: VecDeque::new(),
            is_closed: false,
        }
    }
}

impl<T> Queue<T> {
    fn push(&mut self, value: T) -> Result<(), SendError<T>> {
        if self.is_closed {
            return Result::Err(SendError { value });
        }
        self.data.push_back(value);
        Ok(())
    }

    fn close(&mut self) {
        self.is_closed = true
    }

    fn pop(&mut self) -> Result<T, ReceiveError> {
        match (self.data.pop_front(), self.is_closed) {
            (None, false) => Err(ReceiveError::Empty),
            (None, true) => Err(ReceiveError::Closed),
            (Some(val), _) => Ok(val),
        }
    }
}
