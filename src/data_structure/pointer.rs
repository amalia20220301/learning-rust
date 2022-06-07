/*
* &String 是一个智能指针，因为其对堆上的数据有所有权。&str是一个普通胖指针。
* 在rust中，凡是需要做数据回收的数据结构，并且实现了Deref，DerefMut， Drop，都是智能指针。
*/

use std::ops;

pub struct String {
    vec: Vec<u8>,
}

impl ops::Deref for String {
    type Target = str;
    fn deref(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.vec) }
    }
}

impl ops::DerefMut for String {
    fn deref_mut(&mut self) -> &mut str {
        unsafe { std::str::from_utf8_unchecked_mut(&mut *self.vec) }
    }
}


