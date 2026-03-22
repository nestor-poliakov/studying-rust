#![forbid(unsafe_code)]

use std::{collections::VecDeque, fmt::Debug};

#[derive(Default)]
pub struct MinQueue<T: Debug> {
    data: VecDeque<T>,
    mins: VecDeque<T>,
}

impl<T: Clone + Ord + Debug> MinQueue<T> {
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
            mins: VecDeque::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        self.data.push_back(val.clone());
        if self.data.is_empty() {
            self.mins.push_back(val);
            return;
        }
        let mut to_pop = 0;
        for x in self.mins.iter().rev() {
            if val < *x {
                to_pop += 1;
            } else {
                break;
            }
        }
        for _ in 0..to_pop {
            self.mins.pop_back();
        }
        self.mins.push_back(val.clone());
    }

    pub fn pop(&mut self) -> Option<T> {
        let val = self.data.pop_front();
        val.as_ref()?;
        let m = self.mins.front().unwrap();
        let valv = val.unwrap();
        if *m == valv {
            self.mins.pop_front();
        }
        Option::Some(valv)
    }

    pub fn front(&self) -> Option<&T> {
        self.data.front()
    }

    pub fn min(&self) -> Option<&T> {
        self.mins.front()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
