#![allow(non_snake_case)]

use std::fmt::Display;

use super::Types::{New, ZenType};

#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    pub value: Vec<ZenType>,
}

impl Array {
    pub fn push(&mut self) -> Push<'_> {
        Push { parent: self }
    }
    pub fn remove(&mut self) -> Remove<'_> {
        Remove { parent: self }
    }
    pub fn get(&mut self) -> Get<'_> {
        Get { parent: self }
    }

    pub fn print(&mut self) {
        let mut log = Vec::new();
        for item in &self.value {
            if let ZenType::Number(num) = item {
               log.push(num.value.to_string()); 
            }
        }
        println!("{}", log.join("\n"));
    }
}

// ----------------------------------------- Functions ----------------------------------------

pub fn RealIndex<T>(index: T, max: T) -> usize
where T: Into<isize>, {
    let inner_index: isize = index.into();
    let inner_max: isize = max.into();
    
    (((inner_index % inner_max) + inner_max) % inner_max) as usize
}

// ------------------------------------------ Structs -----------------------------------------

pub struct Push<'a> { pub parent: &'a mut Array }
impl Push<'_> {
    pub fn toEnd(&mut self, new_element: ZenType) {
        self.parent.value.push(new_element);
    }

    pub fn toStart(&mut self, new_element: ZenType) {
        self.parent.value.insert(0, new_element);
    }
}

pub struct Remove<'a> { pub parent: &'a mut Array }
impl Remove<'_> {
    pub fn byIndex(&mut self, index: i32) {
        let len = self.parent.value.len();
        let real_index = RealIndex(index as isize, len as isize); 

        self.parent.value.remove(real_index as usize);
    }

    pub fn byItem(&mut self, element: ZenType) {
        if let Some(pos) = self.parent.value.iter().position(|x| *x == element) {
            Self::byIndex(self, pos as isize as i32);
        }
    }

    pub fn atEnd(&mut self) {
        if !self.parent.value.is_empty() {
            Self::byIndex(self, -1);
        }
    }

    pub fn atStart(&mut self) {
        if !self.parent.value.is_empty() {
            Self::byIndex(self, 0);
        }
    }
}

pub struct Get<'a> { pub parent: &'a mut Array }
impl Get<'_> {
    pub fn length(&mut self) -> usize {
        self.parent.value.len()
    }

    pub fn indexOf(&mut self, element: ZenType) -> Option<usize> {
        self.parent.value.iter().position(|x| *x == element)
    }
    
    pub fn atIndex(&mut self, index: i32) -> Option<ZenType> {
        let real_index = RealIndex(index as isize, self.parent.value.len() as isize);
        for (i, item) in self.parent.value.iter().enumerate() {
            if i == real_index {
                return Some(item.to_owned());
            }
        }

        None

    }

    pub fn between(&mut self, start: i32, end: i32) -> Vec<ZenType>{
        let mut result = Vec::with_capacity((end - start) as usize);
        let mut inner_start = start;
        let mut inner_end = end;
        if start > end {
            inner_start = end;
            inner_end = start;
        } else if start == end {
            inner_end = inner_end + 1;
        }

        for i in inner_start..inner_end {
            result.push(Self::atIndex(self, i).clone().unwrap());
        };

        result
    }

    pub fn splice(&mut self, _start: i32, _amount: i32) -> Vec<ZenType>{
        todo!()
    }
}
// ------------------------------------------ Traits ------------------------------------------

impl From<Vec<ZenType>> for Array {
    fn from(value: Vec<ZenType>) -> Self {
        Self { value }
    }
}

impl From<Array> for Vec<ZenType> {
    fn from(val: Array) -> Self {
        val.value
    }
}

impl New<Vec<ZenType>> for Array {
    fn enum_from(value: Vec<ZenType>) -> ZenType {
        ZenType::Array(Self { value })
    }

    fn new() -> Self {
        Self { value: vec![] }
    }
}

impl Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}
