#![forbid(unsafe_code)]

use std::{cell::RefCell, collections::VecDeque, fmt::Debug, rc::Rc};

pub struct LazyCycle<I>
where
    I: Iterator,
    I::Item: Clone,
{
    iter: I,
    data: Vec<I::Item>,
    i: Option<usize>,
}

impl<I> LazyCycle<I>
where
    I: Iterator,
    I::Item: Clone,
{
    fn new(iter: I) -> Self {
        Self {
            iter,
            data: Vec::new(),
            i: None,
        }
    }
}

impl<I> Iterator for LazyCycle<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let item = match self.i {
            None => match self.iter.next() {
                None => {
                    self.i = Some(1);
                    if self.data.is_empty() {
                        return None;
                    }
                    self.data[0].clone()
                }
                Some(next_item) => {
                    self.data.push(next_item.clone());
                    next_item
                }
            },
            Some(i) => {
                let index = i;
                self.i = Some((i + 1) % self.data.len());
                self.data[index].clone()
            }
        };
        Some(item)
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct Extract<I: Iterator> {
    iter: I,
    data: Vec<I::Item>,
}

impl<I> Extract<I>
where
    I: Iterator,
{
    fn new(iter: I, data: Vec<I::Item>) -> Self {
        Self { iter, data }
    }
}

impl<I> Iterator for Extract<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.data.pop() {
            None => self.iter.next(),
            Some(item) => Some(item),
        }
    }
}
////////////////////////////////////////////////////////////////////////////////

struct TeeData<I: Iterator> {
    behind: i8,
    data: VecDeque<I::Item>,
    iter: I,
    exhausted: bool,
}

impl<I> TeeData<I>
where
    I: Iterator,
    I::Item: Clone,
{
    fn new(iter: I) -> Self {
        Self {
            behind: 0,
            data: VecDeque::new(),
            iter,
            exhausted: false,
        }
    }
}

pub struct Tee<I>
where
    I: Iterator,
    I::Item: Clone,
{
    data: Rc<RefCell<TeeData<I>>>,
    num: i8,
}

impl<I> Tee<I>
where
    I: Iterator,
    I::Item: Clone,
{
    fn new(data: Rc<RefCell<TeeData<I>>>, num: i8) -> Self {
        Self { data, num }
    }
}

impl<I> Iterator for Tee<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut data = self.data.borrow_mut();
        if data.data.is_empty() {
            if data.exhausted {
                return None;
            }
            let next_item = data.iter.next();
            match next_item {
                None => {
                    data.exhausted = true;
                    return None;
                }
                Some(next_item) => {
                    data.data.push_back(next_item.clone());
                    data.behind = -self.num;
                    return Some(next_item);
                }
            };
        }
        if data.behind == self.num {
            return data.data.pop_front();
        }
        if data.exhausted {
            return None;
        }
        let next_item = data.iter.next();
        match next_item {
            None => {
                data.exhausted = true;
                None
            }
            Some(next_item) => {
                data.data.push_back(next_item.clone());
                Some(next_item)
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct GroupBy<I, F, V>
where
    I: Iterator,
    F: FnMut(&I::Item) -> V,
    V: Eq,
{
    iter: I,
    func: F,
    prev: Option<(I::Item, V)>,
}

impl<I, F, V> GroupBy<I, F, V>
where
    I: Iterator,
    F: FnMut(&I::Item) -> V,
    V: Eq + Debug,
    I::Item: Debug,
{
    fn new(mut iter: I, mut func: F) -> Self {
        let x = match iter.next() {
            None => None,
            Some(item) => {
                let val = func(&item);
                Some((item, val))
            }
        };
        Self {
            iter,
            func,
            prev: x,
        }
    }
}

impl<I, F, V> Iterator for GroupBy<I, F, V>
where
    I: Iterator,
    F: FnMut(&I::Item) -> V,
    V: Eq,
{
    type Item = (V, Vec<I::Item>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.prev.take() {
            None => None,
            Some((prev_item, prev_val)) => {
                let mut res = Vec::new();
                res.push(prev_item);
                loop {
                    let next_item = self.iter.next();
                    match next_item {
                        None => {
                            break;
                        }
                        Some(next_item) => {
                            let next_item_val = (self.func)(&next_item);
                            if next_item_val == prev_val {
                                res.push(next_item);
                                continue;
                            } else {
                                self.prev = Some((next_item, next_item_val));
                                break;
                            }
                        }
                    }
                }
                Some((prev_val, res))
            }
        }
        // let (prev_item, prev_val) = self.prev.take().unwrap();
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait ExtendedIterator: Iterator {
    fn lazy_cycle(self) -> LazyCycle<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        LazyCycle::new(self)
    }

    fn extract(mut self, index: usize) -> (Option<Self::Item>, Extract<Self>)
    where
        Self: Sized,
    {
        let mut vec = Vec::with_capacity(index);
        for _ in 0..index {
            match self.next() {
                None => break,
                Some(item) => {
                    vec.push(item);
                }
            };
        }
        let item = self.next();
        vec.reverse();
        (item, Extract::new(self, vec))
    }

    fn tee(self) -> (Tee<Self>, Tee<Self>)
    where
        Self: Sized,
        Self::Item: Clone,
    {
        let data = Rc::new(RefCell::new(TeeData::new(self)));
        (Tee::new(data.clone(), 1), Tee::new(data, -1))
    }

    fn group_by<F, V>(self, func: F) -> GroupBy<Self, F, V>
    where
        Self::Item: Debug,
        Self: Sized,
        F: FnMut(&Self::Item) -> V,
        V: Eq + Debug,
    {
        GroupBy::new(self, func)
    }
}

// TODO: your code goes here.

impl<I: Iterator> ExtendedIterator for I {}
