#![feature(linked_list_cursors)]

use std::collections::LinkedList;
use std::time::SystemTime;

pub fn main() {
    println!("{:#?}", SystemTime::now());
    let mut list1 = LinkedList::new();
    list1.push_back('a');

    let mut list2 = LinkedList::new();
    list2.push_back('b');
    list2.push_back('c');

    list1.append(&mut list2);

    let mut cursor = list1.cursor_front();
    println!("cursor: {:?}", cursor);
    loop {
        match cursor.current() {
            Some(c) => {
                println!("{} {}", c, cursor.peek_next().map_or(' ', |f| *f));
                cursor.move_next();
            }
            None => break,
        }
    }
    println!("list1: {:?}", list1);
}
