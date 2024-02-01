use std::cmp::Ordering;
use std::iter::Iterator;

#[derive(Debug)]
pub struct Set<T> {
    elements: Vec<T>,
}

impl<T: PartialOrd + Copy> Set<T> {
    pub fn new(elements: Vec<T>) -> Set<T> {
        Set{elements}
    }
    pub fn add(&mut self, value: T) {
        for item in &self.elements {
            match value.partial_cmp(item) {
                Some(Ordering::Equal) => return,
                _ => (),
            }
        }
        self.elements.push(value)
    }

    pub fn remove(&mut self, value: T) {
        if let Some(index) = self.elements.iter().position(|&x| x == value) {
            self.elements.remove(index);
        }
    }

    pub fn union(&self, other: &Set<T>) -> Set<T> {
        let mut set = Set::new(Vec::new());

        for item in &self.elements {
            set.add(*item);
        }

        for item in &other.elements {
            set.add(*item);
        }
        set
    }

    pub fn intersection(&self, other: &Set<T>) -> Set<T> {
        let set = self.difference(other);
        let set = self.difference(&set);
        set
    }

    pub fn difference(&self, other: &Set<T>) -> Set<T> {
        let mut set = Set::new(Vec::new());
        for item in &self.elements {
            set.add(*item);
        }

        for item in &other.elements {
            set.remove(*item);
        }
        set
    }
}


//TODO: genericka mnozina pro ordering typy 
// fuzzy mnozina struct 
