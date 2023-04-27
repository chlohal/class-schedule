use std::{collections::BinaryHeap, cmp::Reverse};

#[derive(Default, Debug)]
pub struct TopN<T: Ord> {
    items: BinaryHeap<Reverse<T>>
}

impl<T: Ord> TopN<T> {
    pub fn new(size: usize) -> TopN<T> {
        TopN {
            items: BinaryHeap::<Reverse<T>>::with_capacity(size + 1)
        }
    }

    pub fn insert(&mut self, item: T) {
        //Add the new item, then remove the smallest if there are too many items
        self.items.push(Reverse(item));

        if self.items.len() == self.items.capacity() {
            self.items.pop();
        }
    }
}

impl<T: Ord> IntoIterator for TopN<T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> std::vec::IntoIter<T> {
        self.items.into_iter().map(|x| x.0).collect::<Vec<_>>().into_iter()
    }
}