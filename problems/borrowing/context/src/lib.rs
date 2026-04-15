#![forbid(unsafe_code)]

use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub struct Context {
    map: HashMap<String, Box<dyn Any>>,
    singletone: HashMap<TypeId, Box<dyn Any>>,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            singletone: HashMap::new(),
        }
    }

    pub fn insert<T: 'static>(&mut self, key: impl Into<String>, obj: T) {
        self.map.insert(key.into(), Box::new(obj));
    }
    pub fn get<T: 'static>(&self, key: impl Into<String>) -> &T {
        let key = key.into();
        self.map.get(&key).unwrap().downcast_ref().unwrap()
    }
    pub fn insert_singletone<T: 'static>(&mut self, obj: T) {
        self.singletone.insert(TypeId::of::<T>(), Box::new(obj));
    }
    pub fn get_singletone<T: 'static>(&self) -> &T {
        let key = TypeId::of::<T>();
        self.singletone.get(&key).unwrap().downcast_ref().unwrap()
    }
}
